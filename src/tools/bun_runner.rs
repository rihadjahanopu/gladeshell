// =============================================================================
//  src/tools/bun_runner.rs — Interactive Bun JS/TS File Runner (`run`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::io::stdout;
use std::process::Command;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if ext.eq_ignore_ascii_case("js") || ext.eq_ignore_ascii_case("ts") {
                        if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                            files.push(filename.to_string());
                        }
                    }
                }
            }
        }
    }

    files.sort();

    if files.is_empty() {
        println!("\x1b[0;31m❌ No .js or .ts files found in current directory!\x1b[0m");
        return Ok(());
    }

    if files.len() == 1 {
        let file = &files[0];
        println!("\x1b[1;92m⚡ Running with Bun:\x1b[0m {}", file);
        Command::new("bun").arg(file).status()?;
        return Ok(());
    }

    let selected_idx = match run_bun_tui(&files)? {
        Some(idx) => idx,
        None => return Ok(()),
    };

    if selected_idx < files.len() {
        let selected_file = &files[selected_idx];
        println!("\x1b[1;92m⚡ Running with Bun:\x1b[0m {}", selected_file);
        Command::new("bun").arg(selected_file).status()?;
    }

    Ok(())
}

fn run_bun_tui(files: &[String]) -> Result<Option<usize>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut cursor = 0;
    let bun_gold = Color::Rgb(255, 215, 0);

    let res = loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(8),
                    Constraint::Length(3),
                ])
                .split(f.area());

            let header = Paragraph::new(" ⚡ BUN INTERACTIVE RUNNER ")
                .style(Style::default().fg(Color::Black).bg(bun_gold).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(bun_gold)));
            f.render_widget(header, chunks[0]);

            let items: Vec<ListItem> = files
                .iter()
                .enumerate()
                .map(|(idx, filename)| {
                    let prefix = if idx == cursor { "➔ " } else { "  " };
                    let icon = if filename.ends_with(".ts") { "📘" } else { "📒" };
                    let text = format!("{}{} {}", prefix, icon, filename);
                    let style = if idx == cursor {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(text).style(style)
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .title(format!(" JS/TS Files ({}) ", files.len()))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(bun_gold)),
            );

            let mut state = ListState::default();
            state.select(Some(cursor));
            f.render_stateful_widget(list, chunks[1], &mut state);

            let footer = Paragraph::new(" [↑/↓] Navigate | [Enter] Run File | [Esc/q] Quit ")
                .style(Style::default().fg(bun_gold))
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(bun_gold)));
            f.render_widget(footer, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break Ok(None),
                    KeyCode::Char('c') if is_ctrl => break Ok(None),
                    KeyCode::Up | KeyCode::Char('k') => {
                        if cursor > 0 {
                            cursor -= 1;
                        }
                    }
                    KeyCode::Char('p') if is_ctrl => {
                        if cursor > 0 {
                            cursor -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if cursor < files.len() - 1 {
                            cursor += 1;
                        }
                    }
                    KeyCode::Char('n') if is_ctrl => {
                        if cursor < files.len() - 1 {
                            cursor += 1;
                        }
                    }
                    KeyCode::Enter => break Ok(Some(cursor)),
                    _ => {}
                }
            }
        }
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}
