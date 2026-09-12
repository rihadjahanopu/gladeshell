// =============================================================================
//  src/tools/runtime_installer.rs — Interactive JS Runtime & NVM Installer (`rt`)
// =============================================================================

use std::error::Error;
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

const RUNTIME_OPTIONS: &[&str] = &[
    "NVM (Node Version Manager)",
    "Node.js (LTS Version)",
    "Bun (Fast JS Runtime)",
    "Deno (Secure JS Runtime)",
];

pub fn run() -> Result<(), Box<dyn Error>> {
    let chosen = match run_runtime_tui()? {
        Some(opt) => opt,
        None => {
            println!("👋 Exit");
            return Ok(());
        }
    };

    match chosen {
        "NVM (Node Version Manager)" => {
            let home = std::env::var("HOME").unwrap_or_default();
            let nvm_path = format!("{home}/.nvm");
            if std::path::Path::new(&nvm_path).exists() {
                println!("\x1b[1;32m✅ NVM is already installed at {nvm_path}\x1b[0m");
            } else {
                println!("\x1b[1;36m📥 Installing NVM...\x1b[0m");
                let status = Command::new("bash")
                    .arg("-c")
                    .arg("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash")
                    .status()?;
                if status.success() {
                    println!("\x1b[1;32m✨ NVM installed successfully!\x1b[0m");
                }
            }
        }
        "Node.js (LTS Version)" => {
            println!("\x1b[1;36m📦 Installing Node.js LTS via NVM...\x1b[0m");
            let status = Command::new("bash")
                .arg("-c")
                .arg("export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] && \\. \"$NVM_DIR/nvm.sh\" && nvm install --lts && nvm use --lts")
                .status()?;
            if status.success() {
                println!("\x1b[1;32m✨ Node.js LTS installed!\x1b[0m");
            } else {
                eprintln!("\x1b[1;31m❌ Failed to install Node.js. Please install NVM first.\x1b[0m");
            }
        }
        "Bun (Fast JS Runtime)" => {
            if cmd_exists("bun") {
                println!("\x1b[1;32m✅ Bun is already installed.\x1b[0m");
            } else {
                println!("\x1b[1;36m🥐 Installing Bun...\x1b[0m");
                let status = Command::new("bash")
                    .arg("-c")
                    .arg("curl -fsSL https://bun.sh/install | bash")
                    .status()?;
                if status.success() {
                    println!("\x1b[1;32m✨ Bun installed successfully!\x1b[0m");
                }
            }
        }
        "Deno (Secure JS Runtime)" => {
            if cmd_exists("deno") {
                println!("\x1b[1;32m✅ Deno is already installed.\x1b[0m");
            } else {
                println!("\x1b[1;36m🦕 Installing Deno...\x1b[0m");
                let status = Command::new("bash")
                    .arg("-c")
                    .arg("curl -fsSL https://deno.land/install.sh | sh")
                    .status()?;
                if status.success() {
                    println!("\x1b[1;32m✨ Deno installed successfully!\x1b[0m");
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn run_runtime_tui() -> Result<Option<&'static str>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut cursor = 0;
    let theme_cyan = Color::Rgb(0, 200, 220);

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

            let header = Paragraph::new(" 🚀 ULTIMATE RUNTIME & TOOL INSTALLER (rt) ")
                .style(Style::default().fg(Color::Black).bg(theme_cyan).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme_cyan)));
            f.render_widget(header, chunks[0]);

            let items: Vec<ListItem> = RUNTIME_OPTIONS
                .iter()
                .enumerate()
                .map(|(idx, &opt)| {
                    let prefix = if idx == cursor { "➔ " } else { "  " };
                    let text = format!("{}{}", prefix, opt);
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
                    .title(" Select Runtime to Install ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme_cyan)),
            );

            let mut state = ListState::default();
            state.select(Some(cursor));
            f.render_stateful_widget(list, chunks[1], &mut state);

            let footer = Paragraph::new(" [↑/↓] Navigate | [Enter] Select & Install | [Esc/q] Quit ")
                .style(Style::default().fg(theme_cyan))
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme_cyan)));
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
                        if cursor < RUNTIME_OPTIONS.len() - 1 {
                            cursor += 1;
                        }
                    }
                    KeyCode::Char('n') if is_ctrl => {
                        if cursor < RUNTIME_OPTIONS.len() - 1 {
                            cursor += 1;
                        }
                    }
                    KeyCode::Enter => break Ok(Some(RUNTIME_OPTIONS[cursor])),
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

fn cmd_exists(name: &str) -> bool {
    crate::core::utils::cmd_exists(name)
}
