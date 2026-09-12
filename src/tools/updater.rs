// =============================================================================
//  src/tools/updater.rs — `uup` mega system updater with Ratatui TUI
//
//  Detects available package managers & development runtime updaters:
//    • System: apt, pacman, dnf, brew, snap, flatpak
//    • Runtimes: rustup, bun, npm, pnpm, yarn, pipx
// =============================================================================

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
use std::io::stdout;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct UpdaterTool {
    pub name: &'static str,
    pub command: &'static str,
    pub check_bin: &'static str,
    pub args: &'static [&'static str],
}

const UPDATER_REGISTRY: &[UpdaterTool] = &[
    UpdaterTool {
        name: "APT (Debian/Ubuntu)",
        command: "sudo",
        check_bin: "apt",
        args: &["apt", "update", "&&", "sudo", "apt", "upgrade", "-y"],
    },
    UpdaterTool {
        name: "Pacman (Arch Linux)",
        command: "sudo",
        check_bin: "pacman",
        args: &["pacman", "-Syu", "--noconfirm"],
    },
    UpdaterTool {
        name: "DNF (Fedora/RHEL)",
        command: "sudo",
        check_bin: "dnf",
        args: &["dnf", "upgrade", "-y"],
    },
    UpdaterTool {
        name: "Homebrew (macOS/Linux)",
        command: "brew",
        check_bin: "brew",
        args: &["update"],
    },
    UpdaterTool {
        name: "Snap packages",
        command: "sudo",
        check_bin: "snap",
        args: &["snap", "refresh"],
    },
    UpdaterTool {
        name: "Flatpak packages",
        command: "flatpak",
        check_bin: "flatpak",
        args: &["update", "-y"],
    },
    UpdaterTool {
        name: "Rustup toolchains",
        command: "rustup",
        check_bin: "rustup",
        args: &["update"],
    },
    UpdaterTool {
        name: "Bun JavaScript Runtime",
        command: "bun",
        check_bin: "bun",
        args: &["upgrade"],
    },
    UpdaterTool {
        name: "NPM Global Packages",
        command: "npm",
        check_bin: "npm",
        args: &["update", "-g"],
    },
    UpdaterTool {
        name: "PNPM Package Manager",
        command: "pnpm",
        check_bin: "pnpm",
        args: &["self-update"],
    },
    UpdaterTool {
        name: "Yarn Package Manager",
        command: "yarn",
        check_bin: "yarn",
        args: &["set", "version", "latest"],
    },
    UpdaterTool {
        name: "Pipx Python Applications",
        command: "pipx",
        check_bin: "pipx",
        args: &["upgrade-all"],
    },
];

struct UpdaterApp<'a> {
    available: Vec<&'a UpdaterTool>,
    selected: Vec<bool>,
    cursor: usize,
}

impl<'a> UpdaterApp<'a> {
    fn new(available: Vec<&'a UpdaterTool>) -> Self {
        let count = available.len();
        // Default select all available
        Self {
            available,
            selected: vec![true; count],
            cursor: 0,
        }
    }

    fn toggle_current(&mut self) {
        if !self.selected.is_empty() && self.cursor < self.selected.len() {
            self.selected[self.cursor] = !self.selected[self.cursor];
        }
    }

    fn toggle_all(&mut self) {
        let all_selected = self.selected.iter().all(|&b| b);
        for item in self.selected.iter_mut() {
            *item = !all_selected;
        }
    }

    fn move_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn move_down(&mut self) {
        if !self.available.is_empty() && self.cursor < self.available.len() - 1 {
            self.cursor += 1;
        }
    }
}

/// Probes for installed package managers and runs interactive upgrade workflow.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Scanning for installed package managers and runtime updaters...");

    let available: Vec<&UpdaterTool> = UPDATER_REGISTRY
        .iter()
        .filter(|t| is_cmd_available(t.check_bin))
        .collect();

    if available.is_empty() {
        println!("⚠️ No supported package managers or runtime updaters detected.");
        return Ok(());
    }

    // Launch TUI
    let selected_tools = run_updater_tui(available)?;

    if selected_tools.is_empty() {
        println!("No tools selected for update.");
        return Ok(());
    }

    println!("\n🚀 Starting update suite...\n");

    for tool in selected_tools {
        println!("--------------------------------------------------");
        println!("🔄 Running update for: {}", tool.name);
        println!("--------------------------------------------------");

        let mut cmd;

        // Special case for APT shell chain
        if tool.check_bin == "apt" {
            cmd = Command::new("sh");
            cmd.args(&["-c", "sudo apt update && sudo apt upgrade -y"]);
        } else {
            cmd = Command::new(tool.command);
            cmd.args(tool.args);
        }

        let status = cmd.status();
        match status {
            Ok(s) if s.success() => println!("✅ Finished {}", tool.name),
            Ok(s) => eprintln!("❌ {} failed with status: {}", tool.name, s),
            Err(e) => eprintln!("❌ Failed to execute {}: {}", tool.name, e),
        }
        println!();
    }

    println!("✨ Mega update process finished!");
    Ok(())
}

fn run_updater_tui<'a>(available: Vec<&'a UpdaterTool>) -> Result<Vec<&'a UpdaterTool>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = UpdaterApp::new(available);
    let teal = Color::Rgb(0, 210, 210);

    let res = loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Header banner
                    Constraint::Min(8),    // List area
                    Constraint::Length(3), // Footer status bar
                ])
                .split(f.area());

            // Header
            let header = Paragraph::new(" 🔄 MEGA SYSTEM UPDATER (uup) ")
                .style(Style::default().fg(Color::Black).bg(teal).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(teal)));
            f.render_widget(header, chunks[0]);

            // Tool List
            let items: Vec<ListItem> = app
                .available
                .iter()
                .enumerate()
                .map(|(idx, tool)| {
                    let checked = if app.selected[idx] { "[x]" } else { "[ ]" };
                    let symbol = if idx == app.cursor { "➔ " } else { "  " };
                    let text = format!("{} {} {} ({})", symbol, checked, tool.name, tool.check_bin);

                    let style = if idx == app.cursor {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else if app.selected[idx] {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };

                    ListItem::new(text).style(style)
                })
                .collect();

            let selected_count = app.selected.iter().filter(|&&b| b).count();
            let list_title = format!(" Available Updaters ({}/{} selected) ", selected_count, app.available.len());

            let list_widget = List::new(items)
                .block(Block::default().title(list_title).borders(Borders::ALL).border_style(Style::default().fg(teal)));

            let mut state = ListState::default();
            state.select(Some(app.cursor));
            f.render_stateful_widget(list_widget, chunks[1], &mut state);

            // Footer
            let footer_text = " [Space] Toggle | [a] Select/Deselect All | [↑/↓] Navigate | [Enter] Run Updates | [Esc/q] Quit ";
            let footer = Paragraph::new(footer_text)
                .style(Style::default().fg(teal))
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(teal)));
            f.render_widget(footer, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break Ok(vec![]),
                    KeyCode::Char('c') if is_ctrl => break Ok(vec![]),
                    KeyCode::Up | KeyCode::Char('k') => app.move_up(),
                    KeyCode::Char('p') if is_ctrl => app.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => app.move_down(),
                    KeyCode::Char('n') if is_ctrl => app.move_down(),
                    KeyCode::Char(' ') => app.toggle_current(),
                    KeyCode::Char('a') | KeyCode::Char('A') => app.toggle_all(),
                    KeyCode::Enter => {
                        let selected_tools: Vec<&'a UpdaterTool> = app
                            .available
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, &t)| if app.selected[idx] { Some(t) } else { None })
                            .collect();
                        break Ok(selected_tools);
                    }
                    _ => {}
                }
            }
        }
    };

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn is_cmd_available(cmd: &str) -> bool {
    crate::core::utils::cmd_exists(cmd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_updater_registry_non_empty() {
        assert!(!UPDATER_REGISTRY.is_empty());
    }

    #[test]
    fn test_is_cmd_available_cargo() {
        assert!(is_cmd_available("cargo"));
    }
}
