// =============================================================================
//  src/tools/updater.rs — `uup` Mega System Updater (Modern fkill-Style TUI)
//
//  Detects available package managers & development runtime updaters:
//    • System: apt, pacman, dnf, zypper, brew, snap, flatpak
//    • Runtimes: rustup, bun, npm, pnpm, yarn, pipx, cargo-update
// =============================================================================

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io::stdout;
use std::process::Command;

// ── colour palette (violet, matching fkill aesthetic) ────────────────────────
const C_BG: Color = Color::Rgb(10, 10, 18);
const C_BORDER: Color = Color::Rgb(180, 100, 255); // violet
const C_ACCENT: Color = Color::Rgb(200, 140, 255);
const C_SELECTED_BG: Color = Color::Rgb(45, 20, 70);
const C_SELECTED_FG: Color = Color::Rgb(240, 210, 255);
const C_DIM: Color = Color::Rgb(90, 80, 110);
const C_TEXT: Color = Color::Rgb(220, 220, 230);
const C_GREEN: Color = Color::Rgb(80, 220, 120);
const C_YELLOW: Color = Color::Rgb(255, 200, 80);
const C_RED: Color = Color::Rgb(255, 90, 90);
const C_CYAN: Color = Color::Rgb(80, 220, 210);
const C_WHITE: Color = Color::Rgb(255, 255, 255);
const C_ORANGE: Color = Color::Rgb(255, 160, 60);

#[derive(Debug, Clone)]
pub struct UpdaterTool {
    pub name: &'static str,
    pub emoji: &'static str,
    pub command: &'static str,
    pub check_bin: &'static str,
    pub args: &'static [&'static str],
    pub category: &'static str,
    pub description: &'static str,
}

const UPDATER_REGISTRY: &[UpdaterTool] = &[
    UpdaterTool {
        name: "APT",
        emoji: "🐧",
        command: "sh",
        check_bin: "apt",
        args: &["-c", "sudo apt update && sudo apt upgrade -y"],
        category: "System",
        description: "Debian/Ubuntu package manager. Runs `apt update` then `apt upgrade -y` to bring all system packages up to date.",
    },
    UpdaterTool {
        name: "Pacman",
        emoji: "🎮",
        command: "sudo",
        check_bin: "pacman",
        args: &["pacman", "-Syu", "--noconfirm"],
        category: "System",
        description: "Arch Linux package manager. Syncs databases and upgrades all installed packages without confirmation prompts.",
    },
    UpdaterTool {
        name: "DNF",
        emoji: "🎩",
        command: "sudo",
        check_bin: "dnf",
        args: &["dnf", "upgrade", "-y"],
        category: "System",
        description: "Fedora/RHEL package manager. Downloads and installs all available upgrades in a single non-interactive pass.",
    },
    UpdaterTool {
        name: "Zypper",
        emoji: "🦎",
        command: "sudo",
        check_bin: "zypper",
        args: &["zypper", "update", "-y"],
        category: "System",
        description: "openSUSE package manager. Runs a full system update resolving dependencies automatically.",
    },
    UpdaterTool {
        name: "Homebrew",
        emoji: "🍺",
        command: "brew",
        check_bin: "brew",
        args: &["upgrade"],
        category: "System",
        description: "macOS/Linux third-party package manager. Updates Homebrew itself and upgrades all installed formulae and casks.",
    },
    UpdaterTool {
        name: "Snap",
        emoji: "⚡",
        command: "sudo",
        check_bin: "snap",
        args: &["snap", "refresh"],
        category: "System",
        description: "Canonical Snap package manager. Refreshes all installed snaps to their latest channel revisions.",
    },
    UpdaterTool {
        name: "Flatpak",
        emoji: "📦",
        command: "flatpak",
        check_bin: "flatpak",
        args: &["update", "-y"],
        category: "System",
        description: "Sandboxed application runtime. Updates all installed Flatpak apps and their runtime dependencies.",
    },
    UpdaterTool {
        name: "Rustup",
        emoji: "🦀",
        command: "rustup",
        check_bin: "rustup",
        args: &["update"],
        category: "Runtime",
        description: "Rust toolchain manager. Updates stable, beta, and nightly toolchains plus rustfmt, clippy, and other components.",
    },
    UpdaterTool {
        name: "Bun",
        emoji: "🥐",
        command: "bun",
        check_bin: "bun",
        args: &["upgrade"],
        category: "Runtime",
        description: "Bun JavaScript runtime & toolkit. Self-upgrades to the latest release from the official install channel.",
    },
    UpdaterTool {
        name: "NPM",
        emoji: "🟢",
        command: "npm",
        check_bin: "npm",
        args: &["update", "-g"],
        category: "Runtime",
        description: "Node.js package manager. Updates all globally installed npm packages to their latest compatible versions.",
    },
    UpdaterTool {
        name: "PNPM",
        emoji: "⚙️",
        command: "pnpm",
        check_bin: "pnpm",
        args: &["self-update"],
        category: "Runtime",
        description: "Performant Node.js package manager. Updates pnpm itself to the latest available release.",
    },
    UpdaterTool {
        name: "Yarn",
        emoji: "🧶",
        command: "yarn",
        check_bin: "yarn",
        args: &["set", "version", "latest"],
        category: "Runtime",
        description: "Yarn package manager. Migrates to the latest Yarn release using Corepack / Berry version management.",
    },
    UpdaterTool {
        name: "Pipx",
        emoji: "🐍",
        command: "pipx",
        check_bin: "pipx",
        args: &["upgrade-all"],
        category: "Runtime",
        description: "Python application installer. Upgrades all user-installed Python CLI tools running in isolated virtual environments.",
    },
];

#[derive(Debug, Clone, PartialEq)]
enum AppState {
    Selecting,
    Running,
    Done,
}

struct UpdaterApp<'a> {
    available: Vec<&'a UpdaterTool>,
    selected: Vec<bool>,
    cursor: usize,
    list_state: ListState,
    state: AppState,
    log: Vec<(String, Color)>, // (message, color)
    tick: u64,
}

impl<'a> UpdaterApp<'a> {
    fn new(available: Vec<&'a UpdaterTool>) -> Self {
        let count = available.len();
        let mut list_state = ListState::default();
        if count > 0 {
            list_state.select(Some(0));
        }
        Self {
            available,
            selected: vec![true; count],
            cursor: 0,
            list_state,
            state: AppState::Selecting,
            log: Vec::new(),
            tick: 0,
        }
    }

    fn toggle_current(&mut self) {
        if self.cursor < self.selected.len() {
            self.selected[self.cursor] = !self.selected[self.cursor];
        }
    }

    fn toggle_all(&mut self) {
        let all_on = self.selected.iter().all(|&b| b);
        for s in self.selected.iter_mut() {
            *s = !all_on;
        }
    }

    fn move_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.list_state.select(Some(self.cursor));
        }
    }

    fn move_down(&mut self) {
        if !self.available.is_empty() && self.cursor < self.available.len() - 1 {
            self.cursor += 1;
            self.list_state.select(Some(self.cursor));
        }
    }

    fn selected_count(&self) -> usize {
        self.selected.iter().filter(|&&b| b).count()
    }

    fn current_tool(&self) -> Option<&&UpdaterTool> {
        self.available.get(self.cursor)
    }
}

fn spinner_frame(tick: u64) -> &'static str {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    frames[(tick as usize) % frames.len()]
}

fn draw_selecting(f: &mut Frame, app: &mut UpdaterApp) {
    let area = f.area();

    // background
    f.render_widget(
        Block::default().style(Style::default().bg(C_BG)),
        area,
    );

    // outer layout: header | body | footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // header
            Constraint::Min(6),    // body
            Constraint::Length(3), // footer keybinds
        ])
        .split(area);

    // ── Header ──────────────────────────────────────────────────────────────
    let tick_spin = spinner_frame(app.tick);
    let header_lines = vec![
        Line::from(vec![
            Span::styled("  🔄  ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled("MEGA SYSTEM UPDATER", Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
            Span::styled("  uup  ", Style::default().fg(C_DIM)),
        ]),
        Line::from(vec![
            Span::styled(tick_spin, Style::default().fg(C_BORDER)),
            Span::styled(
                format!("  {} tool(s) detected  •  {} selected", app.available.len(), app.selected_count()),
                Style::default().fg(C_DIM),
            ),
        ]),
    ];
    let header = Paragraph::new(header_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .style(Style::default().bg(C_BG)),
        )
        .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // ── Body: list | detail pane ─────────────────────────────────────────────
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(48), Constraint::Percentage(52)])
        .split(chunks[1]);

    // Left: tool checklist
    let items: Vec<ListItem> = app
        .available
        .iter()
        .enumerate()
        .map(|(idx, tool)| {
            let is_cursor = idx == app.cursor;
            let checked = if app.selected[idx] { "✓" } else { "·" };
            let arrow = if is_cursor { "▶" } else { " " };
            let cat_color = category_color(tool.category);

            let line = Line::from(vec![
                Span::styled(format!(" {} ", arrow), Style::default().fg(C_ACCENT)),
                Span::styled(format!("{} ", checked), Style::default().fg(if app.selected[idx] { C_GREEN } else { C_DIM })),
                Span::styled(format!("{} ", tool.emoji), Style::default()),
                Span::styled(
                    format!("{:<18}", tool.name),
                    if is_cursor {
                        Style::default().fg(C_SELECTED_FG).add_modifier(Modifier::BOLD)
                    } else if app.selected[idx] {
                        Style::default().fg(C_TEXT)
                    } else {
                        Style::default().fg(C_DIM)
                    },
                ),
                Span::styled(
                    format!("[{}]", tool.category),
                    Style::default().fg(cat_color).add_modifier(Modifier::DIM),
                ),
            ]);

            let style = if is_cursor {
                Style::default().bg(C_SELECTED_BG)
            } else {
                Style::default().bg(C_BG)
            };

            ListItem::new(line).style(style)
        })
        .collect();

    let sel_count = app.selected_count();
    let list_title = format!(" Updaters ({}/{} selected) ", sel_count, app.available.len());
    let list_widget = List::new(items).block(
        Block::default()
            .title(Span::styled(list_title, Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER))
            .style(Style::default().bg(C_BG)),
    );
    f.render_stateful_widget(list_widget, body[0], &mut app.list_state);

    // Right: detail pane
    let detail_lines = if let Some(tool) = app.current_tool() {
        let cat_color = category_color(tool.category);
        vec![
            Line::from(vec![
                Span::styled(format!("{} ", tool.emoji), Style::default()),
                Span::styled(tool.name, Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Category  ", Style::default().fg(C_DIM)),
                Span::styled(tool.category, Style::default().fg(cat_color).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Binary    ", Style::default().fg(C_DIM)),
                Span::styled(tool.check_bin, Style::default().fg(C_CYAN)),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled("About", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD | Modifier::UNDERLINED))]),
            Line::from(""),
            Line::from(vec![Span::styled(tool.description, Style::default().fg(C_TEXT))]),
            Line::from(""),
            Line::from(vec![Span::styled("Command", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD | Modifier::UNDERLINED))]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!("$ {} {}", tool.command, tool.args.join(" ")),
                    Style::default().fg(C_YELLOW).add_modifier(Modifier::ITALIC),
                ),
            ]),
        ]
    } else {
        vec![Line::from(Span::styled("No tool selected", Style::default().fg(C_DIM)))]
    };

    let detail = Paragraph::new(detail_lines)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(Span::styled(" 📋 Tool Details ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .style(Style::default().bg(C_BG)),
        );
    f.render_widget(detail, body[1]);

    // ── Footer ───────────────────────────────────────────────────────────────
    let footer_spans = Line::from(vec![
        Span::styled(" [↑↓/jk] ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("Navigate  ", Style::default().fg(C_DIM)),
        Span::styled("[Space] ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("Toggle  ", Style::default().fg(C_DIM)),
        Span::styled("[a] ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("Select All  ", Style::default().fg(C_DIM)),
        Span::styled("[Enter] ", Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
        Span::styled("Run Updates  ", Style::default().fg(C_DIM)),
        Span::styled("[q/Esc] ", Style::default().fg(C_RED).add_modifier(Modifier::BOLD)),
        Span::styled("Quit ", Style::default().fg(C_DIM)),
    ]);
    let footer = Paragraph::new(footer_spans)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_DIM))
                .style(Style::default().bg(C_BG)),
        )
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

fn draw_done(f: &mut Frame, app: &UpdaterApp) {
    let area = f.area();
    f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(6), Constraint::Length(3)])
        .split(area);

    // Header
    let header = Paragraph::new(Line::from(vec![
        Span::styled("  ✨  ", Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
        Span::styled("UPDATE LOG", Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
        Span::styled("  Press [q] to exit  ", Style::default().fg(C_DIM)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_GREEN))
            .style(Style::default().bg(C_BG)),
    )
    .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // Log
    let log_items: Vec<ListItem> = app
        .log
        .iter()
        .map(|(msg, color)| ListItem::new(Line::from(Span::styled(msg.as_str(), Style::default().fg(*color)))))
        .collect();
    let log_list = List::new(log_items).block(
        Block::default()
            .title(Span::styled(" 📜 Results ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER))
            .style(Style::default().bg(C_BG)),
    );
    f.render_widget(log_list, chunks[1]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("[q / Esc] ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("Exit", Style::default().fg(C_DIM)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_DIM))
            .style(Style::default().bg(C_BG)),
    )
    .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

fn category_color(cat: &str) -> Color {
    match cat {
        "System" => C_ORANGE,
        "Runtime" => C_CYAN,
        _ => C_DIM,
    }
}

/// Probes for installed package managers and runs interactive upgrade workflow.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let available: Vec<&UpdaterTool> = UPDATER_REGISTRY
        .iter()
        .filter(|t| is_cmd_available(t.check_bin))
        .collect();

    if available.is_empty() {
        println!("\x1b[1;33m⚠️  No supported package managers or runtime updaters detected.\x1b[0m");
        return Ok(());
    }

    // ── Launch TUI ──────────────────────────────────────────────────────────
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;

    let mut app = UpdaterApp::new(available);

    // ── Event loop ──────────────────────────────────────────────────────────
    let selected_tools: Vec<&UpdaterTool> = loop {
        app.tick = app.tick.wrapping_add(1);
        terminal.draw(|f| draw_selecting(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(80))? {
            if let Event::Key(key) = event::read()? {
                let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break vec![],
                    KeyCode::Char('c') if ctrl => break vec![],
                    KeyCode::Up | KeyCode::Char('k') => app.move_up(),
                    KeyCode::Char('p') if ctrl => app.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => app.move_down(),
                    KeyCode::Char('n') if ctrl => app.move_down(),
                    KeyCode::Char(' ') => app.toggle_current(),
                    KeyCode::Char('a') | KeyCode::Char('A') => app.toggle_all(),
                    KeyCode::Enter => {
                        let tools: Vec<&UpdaterTool> = app
                            .available
                            .iter()
                            .enumerate()
                            .filter_map(|(i, &t)| if app.selected[i] { Some(t) } else { None })
                            .collect();
                        break tools;
                    }
                    _ => {}
                }
            }
        }
    };

    if selected_tools.is_empty() {
        // Restore terminal before printing
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        println!("\x1b[1;33m👋 No tools selected — update cancelled.\x1b[0m");
        return Ok(());
    }

    // ── Run updates (show results in log pane) ───────────────────────────────
    app.state = AppState::Running;
    app.log.push(("🚀 Starting update suite...".into(), C_ACCENT));

    for tool in &selected_tools {
        app.log.push((format!(""), C_DIM));
        app.log.push((
            format!("━━━  {} {}  ━━━", tool.emoji, tool.name),
            C_BORDER,
        ));

        terminal.draw(|f| draw_done(f, &app))?;

        let status = if tool.check_bin == "apt" {
            Command::new("sh")
                .args(["-c", "sudo apt update && sudo apt upgrade -y"])
                .status()
        } else {
            Command::new(tool.command).args(tool.args).status()
        };

        match status {
            Ok(s) if s.success() => {
                app.log.push((format!("  ✅ {} — done", tool.name), C_GREEN));
            }
            Ok(s) => {
                app.log.push((format!("  ❌ {} — exited {}", tool.name, s), C_RED));
            }
            Err(e) => {
                app.log.push((format!("  ❌ {} — error: {}", tool.name, e), C_RED));
            }
        }
        terminal.draw(|f| draw_done(f, &app))?;
    }

    app.log.push(("".into(), C_DIM));
    app.log.push(("✨  Mega update process complete!".into(), C_GREEN));
    app.state = AppState::Done;

    // ── Wait for user to dismiss result view ─────────────────────────────────
    loop {
        terminal.draw(|f| draw_done(f, &app))?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
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

    #[test]
    fn test_all_tools_have_category() {
        for t in UPDATER_REGISTRY {
            assert!(!t.category.is_empty(), "Tool {} has no category", t.name);
            assert!(!t.description.is_empty(), "Tool {} has no description", t.name);
        }
    }
}
