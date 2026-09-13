// =============================================================================
//  src/tools/universal_clean.rs — Universal System Optimizer & Cleaner (`uc`)
//
//  Interactive Ratatui TUI with:
//    • Auto-detected distro + package manager
//    • Multi-select task checklist (pkg cache, snap, flatpak, journal, docker, temp)
//    • Dual-pane: task list + detail / description pane
//    • Post-run progress log view
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
use std::fs;
use std::io::stdout;
use std::process::Command;

// ── colour palette (teal/emerald — distinct from uup violet) ─────────────────
const C_BG: Color = Color::Rgb(8, 14, 18);
const C_BORDER: Color = Color::Rgb(0, 210, 170); // teal
const C_ACCENT: Color = Color::Rgb(80, 230, 200);
const C_SELECTED_BG: Color = Color::Rgb(0, 40, 38);
const C_SELECTED_FG: Color = Color::Rgb(200, 255, 245);
const C_DIM: Color = Color::Rgb(70, 90, 90);
const C_TEXT: Color = Color::Rgb(210, 230, 225);
const C_GREEN: Color = Color::Rgb(80, 220, 120);
const C_RED: Color = Color::Rgb(255, 90, 90);
const C_YELLOW: Color = Color::Rgb(255, 200, 80);
const C_CYAN: Color = Color::Rgb(80, 200, 255);
const C_WHITE: Color = Color::Rgb(255, 255, 255);
const C_ORANGE: Color = Color::Rgb(255, 160, 60);

// ── distro / pkg-manager detection ───────────────────────────────────────────
#[derive(Debug, Clone)]
struct DistroInfo {
    name: String,
    pkg_mgr: String,
}

fn detect_distro() -> DistroInfo {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        let mut id = String::new();
        let mut name = String::from("Linux");
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("ID=") {
                id = val.trim_matches('"').to_string();
            } else if let Some(val) = line.strip_prefix("NAME=") {
                name = val.trim_matches('"').to_string();
            }
        }
        let mgr = match id.as_str() {
            "ubuntu" | "debian" | "pop" | "mint" | "kali" | "deepin" | "linuxmint" => "apt",
            "fedora" | "rhel" | "centos" | "rocky" | "nobara" | "almalinux" => "dnf",
            "arch" | "manjaro" | "endeavouros" | "cachyos" | "garuda" => "pacman",
            "opensuse-tumbleweed" | "opensuse-leap" | "suse" => "zypper",
            "alpine" => "apk",
            _ => "unknown",
        };
        DistroInfo { name, pkg_mgr: mgr.to_string() }
    } else {
        DistroInfo {
            name: "Linux System".to_string(),
            pkg_mgr: "unknown".to_string(),
        }
    }
}

// ── clean task definitions ────────────────────────────────────────────────────
#[derive(Debug, Clone)]
struct CleanTask {
    name: &'static str,
    emoji: &'static str,
    category: &'static str,
    description: &'static str,
    // check_fn decides if task is relevant at runtime
    check: fn(&DistroInfo) -> bool,
    run: fn(&DistroInfo) -> Result<(), String>,
}

fn always(_: &DistroInfo) -> bool { true }
fn has_apt(d: &DistroInfo) -> bool { d.pkg_mgr == "apt" }
fn has_dnf(d: &DistroInfo) -> bool { d.pkg_mgr == "dnf" }
fn has_pacman(d: &DistroInfo) -> bool { d.pkg_mgr == "pacman" }
fn has_zypper(d: &DistroInfo) -> bool { d.pkg_mgr == "zypper" }
fn has_snap(_: &DistroInfo) -> bool { crate::core::utils::cmd_exists("snap") }
fn has_flatpak(_: &DistroInfo) -> bool { crate::core::utils::cmd_exists("flatpak") }
fn has_journal(_: &DistroInfo) -> bool { crate::core::utils::cmd_exists("journalctl") }
fn has_docker(_: &DistroInfo) -> bool { crate::core::utils::cmd_exists("docker") }
fn has_cargo(_: &DistroInfo) -> bool { crate::core::utils::cmd_exists("cargo") }

fn run_apt(_: &DistroInfo) -> Result<(), String> {
    sh("sudo apt-get autoremove -y && sudo apt-get autoclean")
}
fn run_dnf(_: &DistroInfo) -> Result<(), String> {
    sh("sudo dnf autoremove -y && sudo dnf clean all")
}
fn run_pacman(_: &DistroInfo) -> Result<(), String> {
    sh("sudo pacman -Sc --noconfirm")
}
fn run_zypper(_: &DistroInfo) -> Result<(), String> {
    sh("sudo zypper clean --all")
}
fn run_snap(_: &DistroInfo) -> Result<(), String> {
    sh("snap list --all | awk '/disabled/{print $1, $3}' | while read sn rev; do sudo snap remove \"$sn\" --revision=\"$rev\"; done")
}
fn run_flatpak(_: &DistroInfo) -> Result<(), String> {
    sh("flatpak uninstall --unused -y")
}
fn run_journal(_: &DistroInfo) -> Result<(), String> {
    sh("sudo journalctl --vacuum-time=7d")
}
fn run_docker(_: &DistroInfo) -> Result<(), String> {
    sh("docker system prune -f")
}
fn run_cargo(_: &DistroInfo) -> Result<(), String> {
    sh("cargo cache -a 2>/dev/null || rm -rf ~/.cargo/registry/cache")
}
fn run_tmp(_: &DistroInfo) -> Result<(), String> {
    sh("find /tmp -maxdepth 1 -mindepth 1 -user \"$(whoami)\" -exec rm -rf {} + 2>/dev/null; true")
}
fn run_thumbnail(_: &DistroInfo) -> Result<(), String> {
    sh("rm -rf ~/.cache/thumbnails/* 2>/dev/null; true")
}

fn sh(cmd: &str) -> Result<(), String> {
    let status = Command::new("sh")
        .args(["-c", cmd])
        .status()
        .map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("exited with status {}", status))
    }
}

const ALL_TASKS: &[CleanTask] = &[
    CleanTask {
        name: "APT Cache",
        emoji: "🐧",
        category: "System",
        description: "Removes orphaned APT packages (`autoremove`) and cleans the local package cache (`autoclean`). Safe on Debian/Ubuntu-based systems.",
        check: has_apt,
        run: run_apt,
    },
    CleanTask {
        name: "DNF Cache",
        emoji: "🎩",
        category: "System",
        description: "Removes unused DNF packages (`autoremove`) and purges all cached package data (`clean all`). Targets Fedora, RHEL, CentOS, Rocky.",
        check: has_dnf,
        run: run_dnf,
    },
    CleanTask {
        name: "Pacman Cache",
        emoji: "🎮",
        category: "System",
        description: "Cleans the Pacman package cache (`-Sc`), removing cached packages that are no longer installed. Non-interactive with `--noconfirm`.",
        check: has_pacman,
        run: run_pacman,
    },
    CleanTask {
        name: "Zypper Cache",
        emoji: "🦎",
        category: "System",
        description: "Clears the openSUSE Zypper metadata and package download caches. Frees disk space without removing any installed software.",
        check: has_zypper,
        run: run_zypper,
    },
    CleanTask {
        name: "Snap Revisions",
        emoji: "⚡",
        category: "System",
        description: "Removes all disabled (old) Snap revisions. Snap keeps 2–3 old revisions per app by default — this cleans them up to reclaim space.",
        check: has_snap,
        run: run_snap,
    },
    CleanTask {
        name: "Flatpak Unused",
        emoji: "📦",
        category: "System",
        description: "Removes unused Flatpak runtimes and SDK extensions with `flatpak uninstall --unused -y`. These accumulate over time as apps update.",
        check: has_flatpak,
        run: run_flatpak,
    },
    CleanTask {
        name: "Journal Logs",
        emoji: "📜",
        category: "Logs",
        description: "Vacuums systemd journal logs older than 7 days using `journalctl --vacuum-time=7d`. Safe and instant — no service restart needed.",
        check: has_journal,
        run: run_journal,
    },
    CleanTask {
        name: "Docker Prune",
        emoji: "🐳",
        category: "Dev",
        description: "Prunes stopped containers, dangling images, unused networks and build cache with `docker system prune -f`. Frees significant disk space.",
        check: has_docker,
        run: run_docker,
    },
    CleanTask {
        name: "Cargo Registry",
        emoji: "🦀",
        category: "Dev",
        description: "Clears the Cargo registry download cache (`~/.cargo/registry/cache`). The registry re-downloads on next `cargo build` as needed.",
        check: has_cargo,
        run: run_cargo,
    },
    CleanTask {
        name: "Temp Files",
        emoji: "🗑️",
        category: "User",
        description: "Removes files in `/tmp` owned by the current user. Safe — only removes your own temp files, not those of system daemons.",
        check: always,
        run: run_tmp,
    },
    CleanTask {
        name: "Thumbnails",
        emoji: "🖼️",
        category: "User",
        description: "Clears the `~/.cache/thumbnails` directory. Desktop environments regenerate thumbnails on demand — clearing them is safe.",
        check: always,
        run: run_thumbnail,
    },
];

// ── app state ─────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
enum AppState {
    Selecting,
    Done,
}

struct CleanApp<'a> {
    distro: DistroInfo,
    tasks: Vec<&'a CleanTask>,
    selected: Vec<bool>,
    cursor: usize,
    list_state: ListState,
    state: AppState,
    log: Vec<(String, Color)>,
    tick: u64,
}

impl<'a> CleanApp<'a> {
    fn new(distro: DistroInfo) -> Self {
        let tasks: Vec<&'a CleanTask> = ALL_TASKS
            .iter()
            .filter(|t| (t.check)(&distro))
            .collect();
        let count = tasks.len();
        let mut list_state = ListState::default();
        if count > 0 {
            list_state.select(Some(0));
        }
        Self {
            distro,
            tasks,
            selected: vec![true; count],
            cursor: 0,
            list_state,
            state: AppState::Selecting,
            log: Vec::new(),
            tick: 0,
        }
    }

    fn move_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.list_state.select(Some(self.cursor));
        }
    }

    fn move_down(&mut self) {
        if !self.tasks.is_empty() && self.cursor < self.tasks.len() - 1 {
            self.cursor += 1;
            self.list_state.select(Some(self.cursor));
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

    fn selected_count(&self) -> usize {
        self.selected.iter().filter(|&&b| b).count()
    }
}

fn spinner(tick: u64) -> &'static str {
    let f = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    f[(tick as usize) % f.len()]
}

fn cat_color(cat: &str) -> Color {
    match cat {
        "System" => C_ORANGE,
        "Logs" => C_YELLOW,
        "Dev" => C_CYAN,
        "User" => C_ACCENT,
        _ => C_DIM,
    }
}

fn draw_selecting(f: &mut Frame, app: &mut CleanApp) {
    let area = f.area();
    f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // header (2 rows + border)
            Constraint::Min(6),    // body
            Constraint::Length(3), // footer
        ])
        .split(area);

    // ── Header ──────────────────────────────────────────────────────────────
    let header_text = vec![
        Line::from(vec![
            Span::styled("  🧹  ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled("UNIVERSAL SYSTEM CLEANER", Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
            Span::styled("  uc  ", Style::default().fg(C_DIM)),
        ]),
        Line::from(vec![
            Span::styled(spinner(app.tick), Style::default().fg(C_BORDER)),
            Span::styled(
                format!(
                    "  {}  •  {} tasks  •  {} selected",
                    app.distro.name,
                    app.tasks.len(),
                    app.selected_count()
                ),
                Style::default().fg(C_DIM),
            ),
        ]),
    ];
    let header = Paragraph::new(header_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .style(Style::default().bg(C_BG)),
        )
        .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);

    // ── Body: list | detail ─────────────────────────────────────────────────
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(46), Constraint::Percentage(54)])
        .split(chunks[1]);

    // Left: task checklist
    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .enumerate()
        .map(|(idx, task)| {
            let is_cursor = idx == app.cursor;
            let checked = if app.selected[idx] { "✓" } else { "·" };
            let arrow = if is_cursor { "▶" } else { " " };

            let line = Line::from(vec![
                Span::styled(format!(" {} ", arrow), Style::default().fg(C_ACCENT)),
                Span::styled(
                    format!("{} ", checked),
                    Style::default().fg(if app.selected[idx] { C_GREEN } else { C_DIM }),
                ),
                Span::styled(format!("{} ", task.emoji), Style::default()),
                Span::styled(
                    format!("{:<20}", task.name),
                    if is_cursor {
                        Style::default().fg(C_SELECTED_FG).add_modifier(Modifier::BOLD)
                    } else if app.selected[idx] {
                        Style::default().fg(C_TEXT)
                    } else {
                        Style::default().fg(C_DIM)
                    },
                ),
                Span::styled(
                    format!("[{}]", task.category),
                    Style::default().fg(cat_color(task.category)).add_modifier(Modifier::DIM),
                ),
            ]);

            ListItem::new(line).style(if is_cursor {
                Style::default().bg(C_SELECTED_BG)
            } else {
                Style::default().bg(C_BG)
            })
        })
        .collect();

    let list_title = format!(" Tasks ({}/{} selected) ", app.selected_count(), app.tasks.len());
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
    let detail_lines = if let Some(task) = app.tasks.get(app.cursor) {
        vec![
            Line::from(vec![
                Span::styled(format!("{} ", task.emoji), Style::default()),
                Span::styled(task.name, Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Category  ", Style::default().fg(C_DIM)),
                Span::styled(task.category, Style::default().fg(cat_color(task.category)).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled("Description", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD | Modifier::UNDERLINED))]),
            Line::from(""),
            Line::from(vec![Span::styled(task.description, Style::default().fg(C_TEXT))]),
        ]
    } else {
        vec![Line::from(Span::styled("No task selected", Style::default().fg(C_DIM)))]
    };

    let detail = Paragraph::new(detail_lines)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .title(Span::styled(" 📋 Task Details ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .style(Style::default().bg(C_BG)),
        );
    f.render_widget(detail, body[1]);

    // ── Footer ───────────────────────────────────────────────────────────────
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" [↑↓/jk] ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("Navigate  ", Style::default().fg(C_DIM)),
        Span::styled("[Space] ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("Toggle  ", Style::default().fg(C_DIM)),
        Span::styled("[a] ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled("All  ", Style::default().fg(C_DIM)),
        Span::styled("[Enter] ", Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
        Span::styled("Run Cleanup  ", Style::default().fg(C_DIM)),
        Span::styled("[q/Esc] ", Style::default().fg(C_RED).add_modifier(Modifier::BOLD)),
        Span::styled("Quit ", Style::default().fg(C_DIM)),
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

fn draw_log(f: &mut Frame, app: &CleanApp) {
    let area = f.area();
    f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(6), Constraint::Length(3)])
        .split(area);

    let header = Paragraph::new(Line::from(vec![
        Span::styled("  🧹  ", Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
        Span::styled("CLEANUP LOG", Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
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

// ── main entry point ─────────────────────────────────────────────────────────
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let distro = detect_distro();
    let mut app = CleanApp::new(distro);

    if app.tasks.is_empty() {
        println!("\x1b[1;33m⚠️  No cleanup tasks available for this system.\x1b[0m");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;

    // ── Selection loop ───────────────────────────────────────────────────────
    let confirmed = loop {
        app.tick = app.tick.wrapping_add(1);
        terminal.draw(|f| draw_selecting(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(80))? {
            if let Event::Key(key) = event::read()? {
                let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break false,
                    KeyCode::Char('c') if ctrl => break false,
                    KeyCode::Up | KeyCode::Char('k') => app.move_up(),
                    KeyCode::Char('p') if ctrl => app.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => app.move_down(),
                    KeyCode::Char('n') if ctrl => app.move_down(),
                    KeyCode::Char(' ') => app.toggle_current(),
                    KeyCode::Char('a') | KeyCode::Char('A') => app.toggle_all(),
                    KeyCode::Enter => {
                        if app.selected_count() > 0 {
                            break true;
                        }
                    }
                    _ => {}
                }
            }
        }
    };

    if !confirmed {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        println!("\x1b[1;33m👋 Cleanup cancelled.\x1b[0m");
        return Ok(());
    }

    // ── Run selected tasks ───────────────────────────────────────────────────
    app.log.push(("🧹 Starting cleanup...".into(), C_ACCENT));
    app.state = AppState::Done;

    let distro_clone = app.distro.clone();
    let to_run: Vec<(usize, &&CleanTask)> = app
        .tasks
        .iter()
        .enumerate()
        .filter(|(i, _)| app.selected[*i])
        .collect();

    for (_, task) in &to_run {
        app.log.push(("".into(), C_DIM));
        app.log.push((format!("━━━  {} {}  ━━━", task.emoji, task.name), C_BORDER));
        terminal.draw(|f| draw_log(f, &app))?;

        match (task.run)(&distro_clone) {
            Ok(_) => app.log.push((format!("  ✅ {} — done", task.name), C_GREEN)),
            Err(e) => app.log.push((format!("  ❌ {} — {}", task.name, e), C_RED)),
        }
        terminal.draw(|f| draw_log(f, &app))?;
    }

    app.log.push(("".into(), C_DIM));
    app.log.push(("✨  System cleanup complete!".into(), C_GREEN));

    // ── Wait for dismiss ─────────────────────────────────────────────────────
    loop {
        terminal.draw(|f| draw_log(f, &app))?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_distro_returns_something() {
        let d = detect_distro();
        assert!(!d.name.is_empty());
    }

    #[test]
    fn test_all_tasks_have_description() {
        for t in ALL_TASKS {
            assert!(!t.description.is_empty(), "Task {} missing description", t.name);
            assert!(!t.category.is_empty(), "Task {} missing category", t.name);
        }
    }

    #[test]
    fn test_clean_app_builds() {
        let d = detect_distro();
        let app = CleanApp::new(d);
        // Should have at least the always-true tasks (Temp Files, Thumbnails)
        assert!(app.tasks.len() >= 2);
    }
}
