// =============================================================================
//  src/tools/fuzzy_cd.rs — Interactive Fuzzy Directory Navigator (`cf`)
//
//  Pure Rust Ratatui + Crossterm dual-pane directory navigator with live item
//  preview, mode switching (Dev Walk / Recent Dirs), fast search & match counter.
// =============================================================================

use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

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
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchMode {
    DevWalk,
    RecentDirs,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub rel_path: String,
    pub is_dir: bool,
    pub size: u64,
}

pub struct FuzzyCdApp {
    pub current_dir: PathBuf,
    pub mode: SearchMode,
    pub all_items: Vec<FileEntry>,
    pub filtered_indices: Vec<usize>,
    pub list_state: ListState,
    pub query: String,
}

impl FuzzyCdApp {
    pub fn new(current_dir: PathBuf) -> Self {
        let mut app = Self {
            current_dir,
            mode: SearchMode::DevWalk,
            all_items: Vec::new(),
            filtered_indices: Vec::new(),
            list_state: ListState::default(),
            query: String::new(),
        };
        app.load_items();
        app
    }

    pub fn load_items(&mut self) {
        self.all_items.clear();

        match self.mode {
            SearchMode::DevWalk => self.load_dev_walk(),
            SearchMode::RecentDirs => self.load_recent_dirs(),
        }

        self.filter_items();
    }

    fn load_dev_walk(&mut self) {
        // Always include current directory
        self.all_items.push(FileEntry {
            path: self.current_dir.clone(),
            rel_path: "./".to_string(),
            is_dir: true,
            size: 4096,
        });

        let max_entries = 10000;
        let walker = WalkDir::new(&self.current_dir)
            .max_depth(5)
            .into_iter()
            .filter_entry(|e| {
                if let Some(name) = e.file_name().to_str() {
                    if name.starts_with('.') && name != "." && name != ".." {
                        return false;
                    }
                    if name == "node_modules" || name == "target" || name == "vendor" || name == "dist" || name == "build" {
                        return false;
                    }
                }
                true
            });

        for entry in walker.flatten() {
            if self.all_items.len() >= max_entries {
                break;
            }

            let path = entry.path().to_path_buf();
            if path == self.current_dir {
                continue;
            }

            let is_dir = entry.file_type().is_dir();
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);

            let rel_path = match path.strip_prefix(&self.current_dir) {
                Ok(rel) => {
                    let s = rel.to_string_lossy();
                    if is_dir {
                        format!("./{}/", s)
                    } else {
                        format!("./{}", s)
                    }
                }
                Err(_) => {
                    if is_dir {
                        format!("{}/", path.display())
                    } else {
                        format!("{}", path.display())
                    }
                }
            };

            self.all_items.push(FileEntry {
                path,
                rel_path,
                is_dir,
                size,
            });
        }
    }

    fn load_recent_dirs(&mut self) {
        let mut candidates = Vec::new();

        // Check zoxide database if available (~/.local/share/zoxide/db)
        if let Some(home) = dirs_home() {
            let zoxide_db = home.join(".local/share/zoxide/db");
            if zoxide_db.is_file() {
                if let Ok(content) = fs::read_to_string(&zoxide_db) {
                    for line in content.lines() {
                        let parts: Vec<&str> = line.split('|').collect();
                        if let Some(&dir_str) = parts.first() {
                            let p = PathBuf::from(dir_str.trim());
                            if p.is_dir() {
                                candidates.push(p);
                            }
                        }
                    }
                }
            }
        }

        // Add fallback common directories from home
        if candidates.is_empty() {
            if let Some(home) = dirs_home() {
                if let Ok(entries) = fs::read_dir(&home) {
                    for entry in entries.flatten() {
                        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                            candidates.push(entry.path());
                        }
                    }
                }
            }
        }

        for path in candidates {
            let is_dir = path.is_dir();
            let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(4096);
            let rel_path = if let Ok(rel) = path.strip_prefix(&self.current_dir) {
                format!("./{}/", rel.display())
            } else {
                format!("{}/", path.display())
            };

            self.all_items.push(FileEntry {
                path,
                rel_path,
                is_dir,
                size,
            });
        }

        if self.all_items.is_empty() {
            self.load_dev_walk();
        }
    }

    pub fn filter_items(&mut self) {
        let q = self.query.to_lowercase();
        if q.is_empty() {
            self.filtered_indices = (0..self.all_items.len()).collect();
        } else {
            self.filtered_indices = self
                .all_items
                .iter()
                .enumerate()
                .filter(|(_, item)| item.rel_path.to_lowercase().contains(&q))
                .map(|(i, _)| i)
                .collect();
        }

        if self.filtered_indices.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(0));
        }
    }

    pub fn run_loop<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<Option<String>> {
        loop {
            terminal.draw(|f| self.render_ui(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != event::KeyEventKind::Press {
                    continue;
                }

                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        return Ok(None);
                    }
                    (KeyCode::Char('z'), KeyModifiers::CONTROL) => {
                        self.mode = match self.mode {
                            SearchMode::DevWalk => SearchMode::RecentDirs,
                            SearchMode::RecentDirs => SearchMode::DevWalk,
                        };
                        self.query.clear();
                        self.load_items();
                    }
                    (KeyCode::Enter, _) => {
                        if let Some(sel) = self.list_state.selected() {
                            if sel < self.filtered_indices.len() {
                                let orig_idx = self.filtered_indices[sel];
                                let entry = &self.all_items[orig_idx];
                                return Ok(Some(entry.path.to_string_lossy().to_string()));
                            }
                        }
                        return Ok(None);
                    }
                    (KeyCode::Up, _) | (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                        self.move_select(-1);
                    }
                    (KeyCode::Down, _) | (KeyCode::Char('j'), KeyModifiers::CONTROL) => {
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
                    (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                        self.query.push(c);
                        self.filter_items();
                    }
                    _ => {}
                }
            }
        }
    }

    fn move_select(&mut self, delta: i32) {
        if self.filtered_indices.is_empty() {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0) as i32;
        let len = self.filtered_indices.len() as i32;
        let next = (current + delta).clamp(0, len - 1);
        self.list_state.select(Some(next as usize));
    }

    fn render_ui(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();

        // Outer container block matching exact screenshot styling
        let header_title = match self.mode {
            SearchMode::DevWalk => " [ENTER] Cd/Open | [CTRL-Z] Recent Dirs (Zoxide/History) ",
            SearchMode::RecentDirs => " [ENTER] Cd/Open | [CTRL-Z] Dev Walk (Current Tree) ",
        };

        let outer_block = Block::default()
            .title(Span::styled(
                header_title,
                Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));
        frame.render_widget(outer_block, area);

        // Inner layout: Top Search/Status bar + Dual Pane Body
        let inner_margin = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Prompt Line (⚡ Dev Walk: ... 4944/4944)
                Constraint::Min(4),    // Dual pane body
            ])
            .split(inner_margin);

        // 1. Render Prompt Header Row
        let mode_label = match self.mode {
            SearchMode::DevWalk => "⚡ Dev Walk: ",
            SearchMode::RecentDirs => "🕒 Recent Dirs: ",
        };

        let total_count = self.all_items.len();
        let match_count = self.filtered_indices.len();
        let counter_str = format!("{}/{}", match_count, total_count);

        let available_width = main_chunks[0].width as usize;
        let prompt_prefix_len = mode_label.chars().count() + self.query.chars().count() + 1;
        let pad_len = available_width.saturating_sub(prompt_prefix_len + counter_str.len());

        let prompt_line = Line::from(vec![
            Span::styled(mode_label, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(&self.query, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("|", Style::default().fg(Color::Green)),
            Span::raw(" ".repeat(pad_len)),
            Span::styled(counter_str, Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
        ]);
        frame.render_widget(Paragraph::new(prompt_line), main_chunks[0]);

        // 2. Render Dual Pane Layout (Left: File List, Right: Live Preview)
        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(52), // Left List
                Constraint::Percentage(48), // Right Preview
            ])
            .split(main_chunks[1]);

        // Render Left File List
        let list_items: Vec<ListItem> = self
            .filtered_indices
            .iter()
            .enumerate()
            .map(|(i, &orig_idx)| {
                let is_selected = self.list_state.selected() == Some(i);
                let item = &self.all_items[orig_idx];

                if is_selected {
                    ListItem::new(Line::from(vec![
                        Span::styled("> ", Style::default().fg(Color::Rgb(255, 0, 128)).add_modifier(Modifier::BOLD)),
                        Span::styled(
                            &item.rel_path,
                            Style::default()
                                .fg(Color::Rgb(255, 0, 128))
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]))
                } else {
                    ListItem::new(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(&item.rel_path, Style::default().fg(Color::LightGreen)),
                    ]))
                }
            })
            .collect();

        let list_widget = List::new(list_items)
            .block(Block::default().borders(Borders::NONE));

        frame.render_stateful_widget(list_widget, body_chunks[0], &mut self.list_state);

        // Render Right Preview Panel
        let selected_entry = self
            .list_state
            .selected()
            .and_then(|idx| self.filtered_indices.get(idx))
            .map(|&orig_idx| &self.all_items[orig_idx]);

        render_preview_panel(frame, body_chunks[1], selected_entry);
    }
}

fn render_preview_panel(frame: &mut ratatui::Frame, area: Rect, entry: Option<&FileEntry>) {
    let title = match entry {
        Some(e) if e.is_dir => format!(" 📁 Contents of: '{}' ", e.rel_path),
        Some(e) => format!(" 📄 Preview of: '{}' ", e.rel_path),
        None => " Preview ".to_string(),
    };

    let preview_block = Block::default()
        .title(Span::styled(
            title,
            Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let inner = preview_block.inner(area);
    frame.render_widget(preview_block, area);

    if inner.height < 2 || inner.width < 5 {
        return;
    }

    let mut lines = Vec::new();

    if let Some(item) = entry {
        if item.is_dir {
            let (children, size_str) = get_dir_preview(&item.path);
            lines.push(Line::from(Span::styled(
                "─".repeat(inner.width as usize),
                Style::default().fg(Color::Green),
            )));
            if children.is_empty() {
                lines.push(Line::from(Span::styled("  (empty directory)", Style::default().fg(Color::DarkGray))));
            } else {
                for child in children.into_iter().take((inner.height as usize).saturating_sub(4)) {
                    let color = if child.ends_with('/') || child.ends_with('@') {
                        Color::Cyan
                    } else {
                        Color::LightGreen
                    };
                    lines.push(Line::from(vec![
                        Span::styled(child, Style::default().fg(color)),
                    ]));
                }
            }
            lines.push(Line::from(Span::styled(
                "─".repeat(inner.width as usize),
                Style::default().fg(Color::Green),
            )));
            lines.push(Line::from(vec![
                Span::styled("📊 Size: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(size_str, Style::default().fg(Color::Green)),
            ]));
        } else {
            let (file_lines, size_str) = get_file_preview(&item.path);
            lines.push(Line::from(Span::styled(
                "─".repeat(inner.width as usize),
                Style::default().fg(Color::Green),
            )));
            for l in file_lines.into_iter().take((inner.height as usize).saturating_sub(4)) {
                lines.push(Line::from(Span::styled(l, Style::default().fg(Color::LightGreen))));
            }
            lines.push(Line::from(Span::styled(
                "─".repeat(inner.width as usize),
                Style::default().fg(Color::Green),
            )));
            lines.push(Line::from(vec![
                Span::styled("📊 Size: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::styled(size_str, Style::default().fg(Color::Green)),
            ]));
        }
    } else {
        lines.push(Line::from(Span::styled("No item selected", Style::default().fg(Color::DarkGray))));
    }

    frame.render_widget(Paragraph::new(lines), inner);
}

fn get_dir_preview(dir_path: &Path) -> (Vec<String>, String) {
    let mut items = Vec::new();
    let mut total_size: u64 = 0;

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                total_size += meta.len();
            }
            let name = entry.file_name().to_string_lossy().to_string();
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let is_symlink = entry.file_type().map(|t| t.is_symlink()).unwrap_or(false);

            if is_dir {
                items.push(format!("{}/", name));
            } else if is_symlink {
                items.push(format!("{}@", name));
            } else {
                items.push(name);
            }
        }
    }

    items.sort();
    let size_str = format_size(total_size);
    (items, size_str)
}

fn get_file_preview(file_path: &Path) -> (Vec<String>, String) {
    let mut lines = Vec::new();
    let mut size_str = "0B".to_string();

    if let Ok(meta) = fs::metadata(file_path) {
        size_str = format_size(meta.len());
    }

    if let Ok(file) = fs::File::open(file_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().take(30) {
            if let Ok(l) = line {
                lines.push(l);
            } else {
                lines.push("[Binary file or non-UTF8 text preview]".to_string());
                break;
            }
        }
    }
    (lines, size_str)
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}K", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1}M", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1}G", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    let mut app = FuzzyCdApp::new(current_dir);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = app.run_loop(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Ok(Some(selected_path)) = res {
        println!("{}", selected_path);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500B");
        assert_eq!(format_size(4096), "4.0K");
        assert_eq!(format_size(1048576), "1.0M");
    }

    #[test]
    fn test_fuzzy_cd_app_init() {
        let cur = std::env::current_dir().unwrap();
        let app = FuzzyCdApp::new(cur);
        assert!(!app.all_items.is_empty());
    }
}

