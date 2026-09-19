// =============================================================================
//  src/tools/history_search.rs — Native Ratatui Interactive History Search (`fh`)
//
//  Replaces 3rd-party `fzf` binary with pure Rust Ratatui + Crossterm TUI.
//  Reads ~/.bash_history, ~/.zsh_history, ~/.local/share/fish/fish_history.
// =============================================================================

use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

/// Entry point for `fancybash fh` or `fancybash history`
pub fn run() -> Result<(), Box<dyn Error>> {
    let history_items = load_shell_history();
    if history_items.is_empty() {
        eprintln!("\x1b[0;33m⚠️  No shell history entries found.\x1b[0m");
        return Ok(());
    }

    let mut app = HistoryApp::new(history_items);

    // Setup terminal raw mode & alternate screen
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = app.run_loop(&mut terminal);

    // Restore terminal state safely
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Ok(Some(selected_cmd)) = res {
        println!("{selected_cmd}");
    }

    Ok(())
}

// ── History File Parser ───────────────────────────────────────────────────────

fn load_shell_history() -> Vec<String> {
    let mut entries = Vec::with_capacity(4096);
    let home = match dirs_home() {
        Some(h) => h,
        None => return entries,
    };

    // 1. Zsh history (~/.zsh_history)
    let zsh_path = home.join(".zsh_history");
    if zsh_path.exists() {
        if let Ok(content) = fs::read_to_string(&zsh_path) {
            for line in content.lines() {
                let clean = clean_zsh_line(line);
                if !clean.trim().is_empty() {
                    entries.push(clean.to_string());
                }
            }
        }
    }

    // 2. Bash history (~/.bash_history)
    let bash_path = home.join(".bash_history");
    if bash_path.exists() {
        if let Ok(content) = fs::read_to_string(&bash_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    entries.push(trimmed.to_string());
                }
            }
        }
    }

    // 3. Fish history (~/.local/share/fish/fish_history)
    let fish_path = home.join(".local/share/fish/fish_history");
    if fish_path.exists() {
        if let Ok(content) = fs::read_to_string(&fish_path) {
            for line in content.lines() {
                if let Some(cmd) = line.strip_prefix("- cmd: ") {
                    let trimmed = cmd.trim();
                    if !trimmed.is_empty() {
                        entries.push(trimmed.to_string());
                    }
                }
            }
        }
    }

    // 4. PowerShell history
    let pwsh_paths = [
        home.join("AppData/Roaming/Microsoft/Windows/PowerShell/PSReadLine/ConsoleHost_history.txt"),
        home.join(".config/powershell/PSReadLine/ConsoleHost_history.txt"),
    ];
    for pwsh_path in &pwsh_paths {
        if pwsh_path.exists() {
            if let Ok(content) = fs::read_to_string(pwsh_path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        entries.push(trimmed.to_string());
                    }
                }
            }
        }
    }

    // Deduplicate from end to beginning to preserve newest unique commands
    let mut seen = std::collections::HashSet::new();
    let mut unique_reversed = Vec::with_capacity(entries.len());

    for item in entries.into_iter().rev() {
        if seen.insert(item.clone()) {
            unique_reversed.push(item);
        }
    }

    unique_reversed
}

fn clean_zsh_line(line: &str) -> &str {
    if line.starts_with(": ") {
        if let Some(idx) = line.find(';') {
            return &line[idx + 1..];
        }
    }
    line
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

// ── Ratatui App State ────────────────────────────────────────────────────────

struct HistoryApp {
    all_items: Vec<String>,
    filtered: Vec<usize>,
    query: String,
    list_state: ListState,
}

impl HistoryApp {
    fn new(items: Vec<String>) -> Self {
        let indices: Vec<usize> = (0..items.len()).collect();
        let mut list_state = ListState::default();
        if !indices.is_empty() {
            list_state.select(Some(0));
        }

        Self {
            all_items: items,
            filtered: indices,
            query: String::new(),
            list_state,
        }
    }

    fn filter_items(&mut self) {
        let q = self.query.to_lowercase();
        if q.is_empty() {
            self.filtered = (0..self.all_items.len()).collect();
        } else {
            self.filtered = self
                .all_items
                .iter()
                .enumerate()
                .filter(|(_, item)| fuzzy_match(&item.to_lowercase(), &q))
                .map(|(idx, _)| idx)
                .collect();
        }

        if self.filtered.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(0));
        }
    }

    fn run_loop<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<Option<String>> {
        loop {
            terminal.draw(|f| self.render_ui(f))?;

            if let Event::Key(key) = event::read()? {
                // Ignore key release events
                if key.kind != event::KeyEventKind::Press {
                    continue;
                }

                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        return Ok(None);
                    }
                    (KeyCode::Enter, _) => {
                        if let Some(sel) = self.list_state.selected() {
                            if sel < self.filtered.len() {
                                let orig_idx = self.filtered[sel];
                                return Ok(Some(self.all_items[orig_idx].clone()));
                            }
                        }
                        return Ok(None);
                    }
                    (KeyCode::Up, _) => {
                        self.move_select(-1);
                    }
                    (KeyCode::Down, _) => {
                        self.move_select(1);
                    }
                    (KeyCode::PageUp, _) => {
                        self.move_select(-10);
                    }
                    (KeyCode::PageDown, _) => {
                        self.move_select(10);
                    }
                    (KeyCode::Backspace, _) => {
                        self.query.pop();
                        self.filter_items();
                    }
                    (KeyCode::Char(c), _) => {
                        self.query.push(c);
                        self.filter_items();
                    }
                    _ => {}
                }
            }
        }
    }

    fn move_select(&mut self, delta: i32) {
        if self.filtered.is_empty() {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0) as i32;
        let len = self.filtered.len() as i32;
        let next = (current + delta).clamp(0, len - 1);
        self.list_state.select(Some(next as usize));
    }

    fn render_ui(&mut self, frame: &mut ratatui::Frame) {
        let size = frame.area();

        // Centered modal rect
        let area = centered_rect(85, 80, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Input box
                Constraint::Min(5),    // Matching list
                Constraint::Length(1), // Help footer
            ])
            .split(area);

        // Background block
        let outer_block = Block::default()
            .title(Span::styled(
                " 🔍 fancybash Interactive History Search ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));
        frame.render_widget(outer_block, area);

        // Input field widget
        let input_widget = Paragraph::new(format!("Search > {}", self.query))
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Filter ")
                    .border_style(Style::default().fg(Color::DarkGray)),
            );
        frame.render_widget(input_widget, chunks[0]);

        // List items widget
        let items: Vec<ListItem> = self
            .filtered
            .iter()
            .map(|&idx| {
                let line = &self.all_items[idx];
                ListItem::new(Line::from(vec![
                    Span::styled("  ❯ ", Style::default().fg(Color::DarkGray)),
                    Span::raw(line),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Matches ({}) ", self.filtered.len()))
                    .border_style(Style::default().fg(Color::DarkGray)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(" ⚡ ");

        frame.render_stateful_widget(list, chunks[1], &mut self.list_state);

        // Help footer widget
        let help_text = Line::from(vec![
            Span::styled(" [Enter]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" Execute  "),
            Span::styled(" [Esc/Ctrl+C]", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw(" Cancel  "),
            Span::styled(" [↑/↓]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" Navigate "),
        ]);
        let footer = Paragraph::new(help_text);
        frame.render_widget(footer, chunks[2]);
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn fuzzy_match(text: &str, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    // Substring or sequential character match
    if text.contains(query) {
        return true;
    }
    let mut text_chars = text.chars();
    for q_char in query.chars() {
        match text_chars.position(|tc| tc == q_char) {
            Some(_) => continue,
            None => return false,
        }
    }
    true
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
