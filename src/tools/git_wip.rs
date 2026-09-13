// =============================================================================
//  src/tools/git_wip.rs — `gwip` / `gcommit`: Interactive Git Stage & Push
// =============================================================================

use std::io::{self, stdout, Write};
use std::process::{Command, Stdio};

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
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

// ── Design tokens ────────────────────────────────────────────────────────────
const C_ACCENT: Color = Color::Rgb(80, 200, 120);  // mint green
const C_GOLD: Color   = Color::Rgb(255, 200, 60);  // highlight
const C_DIM: Color    = Color::Rgb(120, 130, 140); // muted text
const C_DARK: Color   = Color::Rgb(18, 22, 30);    // near-black bg
const C_WHITE: Color  = Color::Rgb(230, 235, 245); // text
const C_BLUE: Color   = Color::Rgb(100, 160, 255); // info

struct CommitType {
    label:  &'static str,
    prefix: &'static str,
    emoji:  &'static str,
    desc:   &'static str,
}

const COMMIT_TYPES: &[CommitType] = &[
    CommitType { label: "WIP",      prefix: "🚧 wip",      emoji: "🚧", desc: "Work in progress, save point"   },
    CommitType { label: "feat",     prefix: "✨ feat",     emoji: "✨", desc: "A new feature or enhancement"    },
    CommitType { label: "fix",      prefix: "🐛 fix",      emoji: "🐛", desc: "A bug fix"                       },
    CommitType { label: "docs",     prefix: "📝 docs",     emoji: "📝", desc: "Documentation only changes"      },
    CommitType { label: "style",    prefix: "💄 style",    emoji: "💄", desc: "Formatting, missing semicolons"  },
    CommitType { label: "refactor", prefix: "♻️  refactor", emoji: "♻️", desc: "Code refactor, no fix/feat"     },
    CommitType { label: "test",     prefix: "🧪 test",     emoji: "🧪", desc: "Adding or updating tests"        },
    CommitType { label: "chore",    prefix: "🔧 chore",    emoji: "🔧", desc: "Build process or tooling"        },
    CommitType { label: "perf",     prefix: "⚡ perf",     emoji: "⚡", desc: "Performance improvements"        },
    CommitType { label: "ci",       prefix: "🤖 ci",       emoji: "🤖", desc: "CI/CD configuration changes"    },
];

// ── TUI State Machine ─────────────────────────────────────────────────────────
#[derive(PartialEq)]
enum WipStep {
    SelectType,
    EnterMessage,
}

struct WipApp {
    step:       WipStep,
    cursor:     usize,
    msg_input:  String,
    msg_cursor: usize,
}

impl WipApp {
    fn new() -> Self {
        Self { step: WipStep::SelectType, cursor: 0, msg_input: String::new(), msg_cursor: 0 }
    }

    fn selected_type(&self) -> &CommitType { &COMMIT_TYPES[self.cursor] }

    fn preview_commit(&self) -> String {
        let prefix = self.selected_type().prefix;
        let msg = self.msg_input.trim();
        if msg.is_empty() { format!("{prefix}: <auto timestamp>") } else { format!("{prefix}: {msg}") }
    }

    fn insert_char(&mut self, ch: char) {
        let byte_pos = self.msg_input.char_indices().nth(self.msg_cursor)
            .map(|(b, _)| b).unwrap_or(self.msg_input.len());
        self.msg_input.insert(byte_pos, ch);
        self.msg_cursor += 1;
    }

    fn delete_back(&mut self) {
        if self.msg_cursor == 0 { return; }
        let byte_pos = self.msg_input.char_indices().nth(self.msg_cursor - 1)
            .map(|(b, _)| b).unwrap_or(0);
        self.msg_input.remove(byte_pos);
        self.msg_cursor -= 1;
    }

    fn delete_word_back(&mut self) {
        while self.msg_cursor > 0 {
            let byte_pos = self.msg_input.char_indices().nth(self.msg_cursor - 1)
                .map(|(b, _)| b).unwrap_or(0);
            let ch = self.msg_input[byte_pos..].chars().next().unwrap_or(' ');
            self.msg_input.remove(byte_pos);
            self.msg_cursor -= 1;
            if ch == ' ' { break; }
        }
    }
}

// ── Public entry point ────────────────────────────────────────────────────────
pub fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if !cmd_ok("git", &["rev-parse", "--is-inside-work-tree"]) {
        return Err("Not inside a git repository!".into());
    }

    println!("\x1b[1;36m📦 Auto-staging all changes (git add .)…\x1b[0m");
    run_git(&["add", "."])?;

    let full_msg = if args.is_empty() {
        match run_wip_tui()? {
            Some(msg) => msg,
            None => {
                println!("⚠️  Commit cancelled.");
                std::process::exit(0);
            }
        }
    } else {
        cli_commit_msg(args)
    };

    println!("\x1b[1;36m📝 Committing: {full_msg}\x1b[0m");
    run_git(&["commit", "-m", &full_msg])?;

    push_with_retry()
}

// ── Full 2-step TUI ───────────────────────────────────────────────────────────
fn run_wip_tui() -> Result<Option<String>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = WipApp::new();

    let result = loop {
        terminal.draw(|f| draw_ui(f, &app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

                match &app.step {
                    WipStep::SelectType => match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => break Ok(None),
                        KeyCode::Char('c') if ctrl => break Ok(None),
                        KeyCode::Up | KeyCode::Char('k') => { if app.cursor > 0 { app.cursor -= 1; } }
                        KeyCode::Char('p') if ctrl => { if app.cursor > 0 { app.cursor -= 1; } }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if app.cursor < COMMIT_TYPES.len() - 1 { app.cursor += 1; }
                        }
                        KeyCode::Char('n') if ctrl => {
                            if app.cursor < COMMIT_TYPES.len() - 1 { app.cursor += 1; }
                        }
                        KeyCode::Enter | KeyCode::Tab => { app.step = WipStep::EnterMessage; }
                        _ => {}
                    },

                    WipStep::EnterMessage => match key.code {
                        KeyCode::Esc => {
                            app.step = WipStep::SelectType;
                            app.msg_input.clear();
                            app.msg_cursor = 0;
                        }
                        KeyCode::Char('c') if ctrl => break Ok(None),
                        KeyCode::Enter => {
                            let prefix = app.selected_type().prefix.to_string();
                            let msg    = app.msg_input.trim().to_string();
                            break Ok(Some(build_full_msg(&prefix, &msg)));
                        }
                        KeyCode::Char('w') if ctrl => { app.delete_word_back(); }
                        KeyCode::Char('u') if ctrl => { app.msg_input.clear(); app.msg_cursor = 0; }
                        KeyCode::Backspace => { app.delete_back(); }
                        KeyCode::Left  => { if app.msg_cursor > 0 { app.msg_cursor -= 1; } }
                        KeyCode::Right => {
                            if app.msg_cursor < app.msg_input.chars().count() { app.msg_cursor += 1; }
                        }
                        KeyCode::Home | KeyCode::Char('a') if ctrl => { app.msg_cursor = 0; }
                        KeyCode::End  | KeyCode::Char('e') if ctrl => {
                            app.msg_cursor = app.msg_input.chars().count();
                        }
                        KeyCode::Char(ch) => { app.insert_char(ch); }
                        _ => {}
                    },
                }
            }
        }
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

// ── UI Renderer ───────────────────────────────────────────────────────────────
fn draw_ui(f: &mut ratatui::Frame, app: &WipApp) {
    let area = f.area();

    // Background fill
    f.render_widget(Block::default().style(Style::default().bg(C_DARK)), area);

    // Outer layout: header | body | footer
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .margin(1)
        .split(area);

    // ── Header ────────────────────────────────────────────────────────────────
    let step_label = match app.step {
        WipStep::SelectType   => " Step 1/2: Choose Type ",
        WipStep::EnterMessage => " Step 2/2: Write Message ",
    };
    let header = Paragraph::new(Line::from(vec![
        Span::styled(" 🚀 GIT WIP COMMIT  ", Style::default().fg(C_DARK).bg(C_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled(step_label, Style::default().fg(C_ACCENT).add_modifier(Modifier::ITALIC)),
    ]))
    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)
        .border_style(Style::default().fg(C_ACCENT)));
    f.render_widget(header, outer[0]);

    // ── Body: [type list 42% | detail+input 58%] ─────────────────────────────
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(outer[1]);

    draw_type_list(f, app, body[0]);
    draw_right_panel(f, app, body[1]);

    // ── Footer ────────────────────────────────────────────────────────────────
    let hint = match app.step {
        WipStep::SelectType   => " [↑↓ / j k]  Navigate   [Enter / Tab]  Confirm Type   [Esc / q]  Cancel ",
        WipStep::EnterMessage => " [Enter]  Commit & Push   [Esc]  Back   [Ctrl+W]  Del Word   [Ctrl+U]  Clear ",
    };
    let footer = Paragraph::new(hint)
        .style(Style::default().fg(C_DIM))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_DIM)));
    f.render_widget(footer, outer[2]);
}

fn draw_type_list(f: &mut ratatui::Frame, app: &WipApp, area: ratatui::layout::Rect) {
    let active = app.step == WipStep::SelectType;
    let bc = if active { C_ACCENT } else { C_DIM };

    let items: Vec<ListItem> = COMMIT_TYPES.iter().enumerate().map(|(idx, ct)| {
        let sel = idx == app.cursor;
        let (arrow, style) = if sel {
            ("▶ ", Style::default().fg(C_GOLD).add_modifier(Modifier::BOLD))
        } else {
            ("  ", Style::default().fg(C_WHITE))
        };
        ListItem::new(Line::from(vec![
            Span::styled(arrow, style),
            Span::styled(ct.emoji, style),
            Span::raw(" "),
            Span::styled(ct.label, style),
        ]))
    }).collect();

    let title_str = if active { " ◉ Commit Type " } else { " ○ Commit Type " };
    let mut state = ListState::default();
    state.select(Some(app.cursor));

    f.render_stateful_widget(
        List::new(items).block(
            Block::default()
                .title(Span::styled(title_str, Style::default().fg(bc).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(bc)),
        ),
        area,
        &mut state,
    );
}

fn draw_right_panel(f: &mut ratatui::Frame, app: &WipApp, area: ratatui::layout::Rect) {
    let msg_active = app.step == WipStep::EnterMessage;
    let bc = if msg_active { C_ACCENT } else { C_DIM };

    // Outer border for right panel
    f.render_widget(
        Block::default()
            .title(Span::styled(" Details & Message ", Style::default().fg(bc).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(bc)),
        area,
    );

    // Inner layout
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // type info card
            Constraint::Length(3), // message input
            Constraint::Length(3), // preview bar
            Constraint::Min(0),    // padding
        ])
        .margin(1)
        .split(area);

    let ct = app.selected_type();

    // ── Type info card ────────────────────────────────────────────────────────
    f.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::raw("  "),
                Span::styled(ct.emoji, Style::default().fg(C_GOLD).add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled(ct.label, Style::default().fg(C_GOLD).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(ct.desc, Style::default().fg(C_WHITE)),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("prefix: ", Style::default().fg(C_DIM)),
                Span::styled(ct.prefix, Style::default().fg(C_BLUE).add_modifier(Modifier::ITALIC)),
            ]),
        ])
        .block(Block::default()
            .title(Span::styled(" ℹ  Type Info ", Style::default().fg(C_ACCENT)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_ACCENT))),
        inner[0],
    );

    // ── Message input ─────────────────────────────────────────────────────────
    let input_display = if msg_active {
        let chars: Vec<char> = app.msg_input.chars().collect();
        let before: String = chars[..app.msg_cursor].iter().collect();
        let after:  String = chars[app.msg_cursor..].iter().collect();
        let cursor_ch = if app.msg_cursor < chars.len() {
            chars[app.msg_cursor].to_string()
        } else {
            " ".to_string()
        };
        Line::from(vec![
            Span::raw("  "),
            Span::styled(before, Style::default().fg(C_WHITE)),
            Span::styled(cursor_ch, Style::default().fg(C_DARK).bg(C_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled(after, Style::default().fg(C_WHITE)),
        ])
    } else if app.msg_input.is_empty() {
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "(select a type first, then press Enter)",
                Style::default().fg(C_DIM).add_modifier(Modifier::ITALIC),
            ),
        ])
    } else {
        Line::from(vec![Span::raw("  "), Span::styled(app.msg_input.clone(), Style::default().fg(C_WHITE))])
    };

    let msg_title = if msg_active {
        Span::styled(" ◉ Commit Message ", Style::default().fg(bc).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" ○ Commit Message ", Style::default().fg(bc))
    };

    f.render_widget(
        Paragraph::new(input_display)
            .block(Block::default().title(msg_title).borders(Borders::ALL)
                .border_type(BorderType::Rounded).border_style(Style::default().fg(bc))),
        inner[1],
    );

    // ── Preview bar ───────────────────────────────────────────────────────────
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("  ➜  ", Style::default().fg(C_DIM)),
            Span::styled(app.preview_commit(), Style::default().fg(C_BLUE).add_modifier(Modifier::ITALIC)),
        ]))
        .block(Block::default()
            .title(Span::styled(" 👁  Preview ", Style::default().fg(C_DIM)))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_DIM))),
        inner[2],
    );
}

// ── CLI fast path (args provided) ─────────────────────────────────────────────
fn cli_commit_msg(args: &[String]) -> String {
    if args.is_empty() { return default_wip_msg(); }
    let first = args[0].as_str();
    if let Some(prefix) = arg_to_prefix(first) {
        build_full_msg(prefix, &args[1..].join(" "))
    } else if first == "-m" {
        build_full_msg("🚧 wip", &args[1..].join(" "))
    } else {
        build_full_msg("🚧 wip", &args.join(" "))
    }
}

fn arg_to_prefix(arg: &str) -> Option<&'static str> {
    match arg {
        "feat"     | "✨" => Some("✨ feat"),
        "fix"      | "🐛" => Some("🐛 fix"),
        "docs"     | "📝" => Some("📝 docs"),
        "style"    | "💄" => Some("💄 style"),
        "refactor" | "♻️" => Some("♻️  refactor"),
        "test"     | "🧪" => Some("🧪 test"),
        "chore"    | "🔧" => Some("🔧 chore"),
        "perf"     | "⚡" => Some("⚡ perf"),
        "ci"       | "🤖" => Some("🤖 ci"),
        "wip"      | "🚧" => Some("🚧 wip"),
        "-m"             => Some("🚧 wip"),
        _                => None,
    }
}

fn build_full_msg(prefix: &str, msg: &str) -> String {
    let msg = msg.trim();
    if msg.is_empty() { default_wip_msg_with_prefix(prefix) } else { format!("{prefix}: {msg}") }
}

fn default_wip_msg() -> String { default_wip_msg_with_prefix("🚧 wip") }

fn default_wip_msg_with_prefix(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let days = secs / 86400;
    let year = 1970 + days / 365;
    let doy  = days % 365;
    let month = doy / 30 + 1;
    let day   = doy % 30 + 1;
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    format!("{prefix}: save point ({year}-{month:02}-{day:02} {h:02}:{m:02})")
}

// ── Git helpers ───────────────────────────────────────────────────────────────
fn push_with_retry() -> Result<(), Box<dyn std::error::Error>> {
    let cur_branch = current_branch();
    print!("\x1b[1;36m🚀 Pushing to remote");
    io::stdout().flush().ok();

    match push_to_remote(&cur_branch) {
        Ok(true) => {
            println!("\r\x1b[1;32m✅ Everything committed and pushed successfully!\x1b[0m");
            Ok(())
        }
        Ok(false) => {
            println!("\r\x1b[1;33m🔄 Remote has new commits. Auto-syncing (git pull --rebase)…\x1b[0m");
            let origin = cur_branch.as_deref().unwrap_or("HEAD");
            let rebase_ok = Command::new("git").args(["pull", "--rebase", "origin", origin])
                .status().map(|s| s.success()).unwrap_or(false);

            if rebase_ok {
                println!("\x1b[1;36m🚀 Retrying push…\x1b[0m");
                let retry_ok = Command::new("git").args(["push", "-u", "origin", origin])
                    .status().map(|s| s.success()).unwrap_or(false);
                if retry_ok {
                    println!("\x1b[1;32m✅ Synced and pushed successfully!\x1b[0m");
                    return Ok(());
                }
                Err("❌ Push failed after rebase sync.".into())
            } else {
                eprintln!("\x1b[0;31m⚠️  Merge Conflict detected!\x1b[0m");
                eprintln!("\x1b[1;33mPlease resolve conflicts, then run:\x1b[0m");
                eprintln!("  1) \x1b[1;36mgit add .\x1b[0m");
                eprintln!("  2) \x1b[1;36mgit rebase --continue\x1b[0m");
                eprintln!("  3) \x1b[1;36mfancybash gwip\x1b[0m");
                Err("Merge conflict — manual resolution required.".into())
            }
        }
        Err(e) => {
            eprintln!("\x1b[0;31m❌ Push failed: {e}\x1b[0m");
            eprintln!("\x1b[1;33m💡 Your local commit was created successfully.\x1b[0m");
            Err(e)
        }
    }
}

fn push_to_remote(branch: &Option<String>) -> Result<bool, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("git");
    cmd.arg("push").arg("-u");
    if let Some(b) = branch { cmd.args(["origin", b.as_str()]); }
    let output = cmd.stdout(Stdio::inherit()).stderr(Stdio::piped()).output()?;
    if output.status.success() { return Ok(true); }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let is_nff = stderr.contains("non-fast-forward") || stderr.contains("fetch first") || stderr.contains("behind");
    if is_nff { Ok(false) } else {
        if !stderr.is_empty() { eprintln!("\x1b[1;33mGit Error Details:\x1b[0m\n{stderr}"); }
        Err("push failed".into())
    }
}

fn current_branch() -> Option<String> {
    let out = Command::new("git").args(["branch", "--show-current"])
        .stdout(Stdio::piped()).stderr(Stdio::null()).output().ok()?;
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}

fn cmd_ok(prog: &str, args: &[&str]) -> bool {
    Command::new(prog).args(args).stdout(Stdio::null()).stderr(Stdio::null())
        .status().map(|s| s.success()).unwrap_or(false)
}

fn run_git(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git").args(args).status()?;
    if status.success() { Ok(()) } else { Err(format!("git {} failed", args.join(" ")).into()) }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_to_prefix_known() {
        assert_eq!(arg_to_prefix("feat"),     Some("✨ feat"));
        assert_eq!(arg_to_prefix("fix"),      Some("🐛 fix"));
        assert_eq!(arg_to_prefix("wip"),      Some("🚧 wip"));
        assert_eq!(arg_to_prefix("refactor"), Some("♻️  refactor"));
        assert_eq!(arg_to_prefix("perf"),     Some("⚡ perf"));
        assert_eq!(arg_to_prefix("ci"),       Some("🤖 ci"));
    }

    #[test]
    fn test_arg_to_prefix_unknown() {
        assert_eq!(arg_to_prefix("foobar"), None);
    }

    #[test]
    fn test_build_full_msg_with_text() {
        assert_eq!(build_full_msg("✨ feat", "add login"), "✨ feat: add login");
        assert_eq!(build_full_msg("🐛 fix",  "null ptr"),  "🐛 fix: null ptr");
    }

    #[test]
    fn test_build_full_msg_empty_timestamp() {
        let msg = build_full_msg("🚧 wip", "");
        assert!(msg.contains("🚧 wip: save point"));
        assert!(msg.contains('-'));
    }

    #[test]
    fn test_cli_commit_plain() {
        let args: Vec<String> = vec!["my message".into()];
        assert!(cli_commit_msg(&args).starts_with("🚧 wip: my message"));
    }

    #[test]
    fn test_cli_commit_typed() {
        let args: Vec<String> = vec!["feat".into(), "new feature".into()];
        assert_eq!(cli_commit_msg(&args), "✨ feat: new feature");
    }

    #[test]
    fn test_cli_commit_fix() {
        let args: Vec<String> = vec!["fix".into(), "crash on startup".into()];
        assert_eq!(cli_commit_msg(&args), "🐛 fix: crash on startup");
    }

    #[test]
    fn test_wip_app_insert_delete() {
        let mut app = WipApp::new();
        app.insert_char('h');
        app.insert_char('i');
        assert_eq!(app.msg_input, "hi");
        assert_eq!(app.msg_cursor, 2);
        app.delete_back();
        assert_eq!(app.msg_input, "h");
        assert_eq!(app.msg_cursor, 1);
    }

    #[test]
    fn test_wip_app_preview() {
        let mut app = WipApp::new();
        app.cursor = 1; // feat
        app.msg_input = "login page".into();
        app.msg_cursor = 10;
        let preview = app.preview_commit();
        assert!(preview.contains("feat"));
        assert!(preview.contains("login page"));
    }

    #[test]
    fn test_commit_types_completeness() {
        assert!(COMMIT_TYPES.len() >= 10);
        for ct in COMMIT_TYPES {
            assert!(!ct.prefix.is_empty());
            assert!(!ct.emoji.is_empty());
        }
    }

    #[test]
    fn test_delete_word_back() {
        let mut app = WipApp::new();
        for ch in "hello world".chars() { app.insert_char(ch); }
        app.delete_word_back();
        assert_eq!(app.msg_input, "hello");
    }
}
