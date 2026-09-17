// =============================================================================
//  src/tools/git_wip.rs — `gwip` / `gcommit`: Modern Interactive Git Stage & Push
// =============================================================================

use std::io::stdout;
use std::process::{Command, Stdio};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

// ── Modern Dark Violet Design Tokens ─────────────────────────────────────────
const C_BG: Color     = Color::Rgb(10, 10, 18);     // #0A0A12 Deep space dark
const C_CARD: Color   = Color::Rgb(18, 18, 32);   // #121220 Card background
const C_VIOLET: Color = Color::Rgb(180, 100, 255); // #B464FF Primary accent violet
const C_CYAN: Color   = Color::Rgb(0, 229, 255);   // #00E5FF Neon cyan highlight
const C_PINK: Color   = Color::Rgb(255, 100, 200); // #FF64C8 Pink accent
const C_GOLD: Color   = Color::Rgb(255, 200, 60);  // #FFC83C Gold warning
const C_GREEN: Color  = Color::Rgb(80, 220, 140);  // #50DC8C Success green
const C_RED: Color    = Color::Rgb(255, 85, 85);   // #FF5555 Red error
const C_TEXT: Color   = Color::Rgb(235, 240, 255); // Crisp text
const C_MUTED: Color  = Color::Rgb(110, 120, 145); // Slate muted text
const C_BORDER: Color = Color::Rgb(40, 42, 65);     // Card border

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

struct StagedFile {
    status: String,
    path:   String,
}

struct WipApp {
    step:         WipStep,
    cursor:       usize,
    msg_input:    String,
    msg_cursor:   usize,
    staged_files: Vec<StagedFile>,
    branch:       String,
}

impl WipApp {
    fn new() -> Self {
        let staged_files = fetch_staged_files();
        let branch = current_branch().unwrap_or_else(|| "HEAD".to_string());
        Self {
            step: WipStep::SelectType,
            cursor: 0,
            msg_input: String::new(),
            msg_cursor: 0,
            staged_files,
            branch,
        }
    }

    fn selected_type(&self) -> &CommitType { &COMMIT_TYPES[self.cursor] }

    fn preview_commit(&self) -> String {
        let prefix = self.selected_type().prefix;
        let msg = self.msg_input.trim();
        if msg.is_empty() {
            format!("{prefix}: <auto timestamp>")
        } else {
            format!("{prefix}: {msg}")
        }
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

// ── Public Entry Point ────────────────────────────────────────────────────────
pub fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if !cmd_ok("git", &["rev-parse", "--is-inside-work-tree"]) {
        return Err("Not inside a git repository!".into());
    }

    // Auto-stage files silently
    let _ = run_git(&["add", "."]);

    let full_msg = if args.is_empty() {
        match run_wip_tui()? {
            Some(msg) => msg,
            None => {
                println!("\x1b[38;2;110;120;145m⚠️ Commit cancelled.\x1b[0m");
                return Ok(());
            }
        }
    } else {
        cli_commit_msg(args)
    };

    run_git(&["commit", "-m", &full_msg])?;
    push_with_retry()
}

// ── Interactive 2-Step Ratatui TUI ────────────────────────────────────────────
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
                        KeyCode::Up | KeyCode::Char('k') => {
                            if app.cursor > 0 { app.cursor -= 1; }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
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

    // Deep space background fill
    f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

    // Outer vertical layout: Header (3) | Content (Min) | Footer (3)
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .margin(1)
        .split(area);

    // ── Header ────────────────────────────────────────────────────────────────
    let step_pill = match app.step {
        WipStep::SelectType   => Span::styled(" STEP 1/2: CHOOSE COMMIT TYPE ", Style::default().fg(C_BG).bg(C_CYAN).add_modifier(Modifier::BOLD)),
        WipStep::EnterMessage => Span::styled(" STEP 2/2: ENTER COMMIT MESSAGE ", Style::default().fg(C_BG).bg(C_PINK).add_modifier(Modifier::BOLD)),
    };

    let header = Paragraph::new(Line::from(vec![
        Span::styled(" ⚡ FANCYBASH GWIP  ", Style::default().fg(C_BG).bg(C_VIOLET).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        step_pill,
        Span::raw("  "),
        Span::styled(format!("🌿 {}", app.branch), Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
        Span::raw("  •  "),
        Span::styled(format!("📦 {} files staged", app.staged_files.len()), Style::default().fg(C_MUTED)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_VIOLET))
            .style(Style::default().bg(C_CARD)),
    )
    .alignment(Alignment::Left);
    f.render_widget(header, outer[0]);

    // ── Body Layout: Left Pane (38%) | Right Pane (62%) ───────────────────────
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
        .split(outer[1]);

    draw_type_list(f, app, body[0]);
    draw_right_panel(f, app, body[1]);

    // ── Footer ────────────────────────────────────────────────────────────────
    let hint_spans = match app.step {
        WipStep::SelectType => vec![
            Span::styled(" [↑/↓ or k/j] ", Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("Navigate Types  ", Style::default().fg(C_TEXT)),
            Span::styled(" [Enter/Tab] ", Style::default().fg(C_VIOLET).add_modifier(Modifier::BOLD)),
            Span::styled("Select Type  ", Style::default().fg(C_TEXT)),
            Span::styled(" [Esc/q] ", Style::default().fg(C_RED).add_modifier(Modifier::BOLD)),
            Span::styled("Cancel", Style::default().fg(C_MUTED)),
        ],
        WipStep::EnterMessage => vec![
            Span::styled(" [Enter] ", Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled("Commit & Auto-Push  ", Style::default().fg(C_TEXT)),
            Span::styled(" [Esc] ", Style::default().fg(C_GOLD).add_modifier(Modifier::BOLD)),
            Span::styled("Back  ", Style::default().fg(C_TEXT)),
            Span::styled(" [Ctrl+W] ", Style::default().fg(C_PINK).add_modifier(Modifier::BOLD)),
            Span::styled("Del Word  ", Style::default().fg(C_MUTED)),
            Span::styled(" [Ctrl+U] ", Style::default().fg(C_RED).add_modifier(Modifier::BOLD)),
            Span::styled("Clear", Style::default().fg(C_MUTED)),
        ],
    };

    let footer = Paragraph::new(Line::from(hint_spans))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .style(Style::default().bg(C_CARD)),
        );
    f.render_widget(footer, outer[2]);
}

fn draw_type_list(f: &mut ratatui::Frame, app: &WipApp, area: Rect) {
    let active = app.step == WipStep::SelectType;
    let border_color = if active { C_VIOLET } else { C_BORDER };

    let items: Vec<ListItem> = COMMIT_TYPES.iter().enumerate().map(|(idx, ct)| {
        let sel = idx == app.cursor;
        if sel {
            let prefix_style = if active {
                Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(C_TEXT).add_modifier(Modifier::BOLD)
            };
            ListItem::new(Line::from(vec![
                Span::styled("❯ ", Style::default().fg(C_VIOLET).add_modifier(Modifier::BOLD)),
                Span::styled(ct.emoji, Style::default().fg(C_TEXT)),
                Span::raw(" "),
                Span::styled(format!("{:<8}", ct.label), prefix_style),
                Span::styled(ct.desc, Style::default().fg(C_MUTED)),
            ])).style(Style::default().bg(C_CARD))
        } else {
            ListItem::new(Line::from(vec![
                Span::raw("  "),
                Span::styled(ct.emoji, Style::default().fg(C_MUTED)),
                Span::raw(" "),
                Span::styled(format!("{:<8}", ct.label), Style::default().fg(C_MUTED)),
                Span::styled(ct.desc, Style::default().fg(Color::Rgb(70, 78, 100))),
            ]))
        }
    }).collect();

    let title_span = if active {
        Span::styled(" 📌 COMMIT TYPE (ACTIVE) ", Style::default().fg(C_VIOLET).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" 📌 COMMIT TYPE ", Style::default().fg(C_MUTED))
    };

    let mut state = ListState::default();
    state.select(Some(app.cursor));

    f.render_stateful_widget(
        List::new(items).block(
            Block::default()
                .title(title_span)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color))
                .style(Style::default().bg(C_CARD)),
        ),
        area,
        &mut state,
    );
}

fn draw_right_panel(f: &mut ratatui::Frame, app: &WipApp, area: Rect) {
    let msg_active = app.step == WipStep::EnterMessage;

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Type summary info card
            Constraint::Length(3), // Interactive message input box
            Constraint::Length(3), // Live commit preview box
            Constraint::Min(0),    // Staged files list card
        ])
        .split(area);

    let ct = app.selected_type();

    // ── 1. Type Info Card ─────────────────────────────────────────────────────
    f.render_widget(
        Paragraph::new(vec![
            Line::from(vec![
                Span::raw("  "),
                Span::styled(format!("{} {}", ct.emoji, ct.label), Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
                Span::styled("  •  ", Style::default().fg(C_MUTED)),
                Span::styled(ct.desc, Style::default().fg(C_TEXT)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("Git Prefix: ", Style::default().fg(C_MUTED)),
                Span::styled(ct.prefix, Style::default().fg(C_PINK).add_modifier(Modifier::BOLD)),
            ]),
        ])
        .block(
            Block::default()
                .title(Span::styled(" ℹ  SELECTED TYPE DETAILS ", Style::default().fg(C_VIOLET).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .style(Style::default().bg(C_CARD)),
        ),
        inner_layout[0],
    );

    // ── 2. Message Input Box ──────────────────────────────────────────────────
    let input_border = if msg_active { C_PINK } else { C_BORDER };

    let input_line = if msg_active {
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
            Span::styled(before, Style::default().fg(C_TEXT)),
            Span::styled(cursor_ch, Style::default().fg(C_BG).bg(C_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled(after, Style::default().fg(C_TEXT)),
        ])
    } else if app.msg_input.is_empty() {
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "(press Enter to type commit message, or leave empty for auto timestamp)",
                Style::default().fg(C_MUTED).add_modifier(Modifier::ITALIC),
            ),
        ])
    } else {
        Line::from(vec![
            Span::raw("  "),
            Span::styled(app.msg_input.clone(), Style::default().fg(C_TEXT)),
        ])
    };

    let char_count = app.msg_input.chars().count();
    let msg_title = if msg_active {
        Span::styled(
            format!(" ✏️ COMMIT MESSAGE [{char_count} chars] (EDITING) "),
            Style::default().fg(C_PINK).add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(" ✏️ COMMIT MESSAGE ", Style::default().fg(C_MUTED))
    };

    f.render_widget(
        Paragraph::new(input_line).block(
            Block::default()
                .title(msg_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(input_border))
                .style(Style::default().bg(C_CARD)),
        ),
        inner_layout[1],
    );

    // ── 3. Live Commit Preview Box ───────────────────────────────────────────
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::raw("  ➜  "),
            Span::styled(app.preview_commit(), Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
        ]))
        .block(
            Block::default()
                .title(Span::styled(" 👁  LIVE COMMIT PREVIEW ", Style::default().fg(C_MUTED)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .style(Style::default().bg(C_CARD)),
        ),
        inner_layout[2],
    );

    // ── 4. Staged Files Card ─────────────────────────────────────────────────
    let file_items: Vec<ListItem> = if app.staged_files.is_empty() {
        vec![ListItem::new(Line::from(vec![
            Span::raw("  "),
            Span::styled("All changes clean / no modified files detected.", Style::default().fg(C_MUTED).add_modifier(Modifier::ITALIC)),
        ]))]
    } else {
        app.staged_files.iter().map(|f| {
            let (status_color, status_badge) = match f.status.as_str() {
                "M" => (C_GOLD, " MODIFIED "),
                "A" => (C_GREEN, " ADDED    "),
                "D" => (C_RED, " DELETED  "),
                _   => (C_VIOLET, " STAGED   "),
            };
            ListItem::new(Line::from(vec![
                Span::raw("  "),
                Span::styled(status_badge, Style::default().fg(C_BG).bg(status_color).add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled(f.path.as_str(), Style::default().fg(C_TEXT)),
            ]))
        }).collect()
    };

    f.render_widget(
        List::new(file_items).block(
            Block::default()
                .title(Span::styled(
                    format!(" 📦 STAGED FILES ({}) ", app.staged_files.len()),
                    Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .style(Style::default().bg(C_CARD)),
        ),
        inner_layout[3],
    );
}

// ── CLI Fast Path ─────────────────────────────────────────────────────────────
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
    if msg.is_empty() {
        default_wip_msg_with_prefix(prefix)
    } else {
        format!("{prefix}: {msg}")
    }
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

// ── Git & Push Logic ──────────────────────────────────────────────────────────
#[derive(Debug)]
enum PushOutcome {
    Success(String),             // Branch pushed
    Conflict(Vec<String>),       // Merge conflict
    Failed(String, Vec<String>), // Error msg + git stderr lines
}

fn push_with_retry() -> Result<(), Box<dyn std::error::Error>> {
    let cur_branch = current_branch();

    let outcome = match push_once(&cur_branch) {
        Ok(true)  => PushOutcome::Success(cur_branch.clone().unwrap_or_default()),
        Ok(false) => {
            let origin = cur_branch.as_deref().unwrap_or("HEAD");
            let rebase_out = Command::new("git")
                .args(["pull", "--rebase", "origin", origin])
                .stdout(Stdio::piped()).stderr(Stdio::piped()).output();
            match rebase_out {
                Ok(o) if o.status.success() => {
                    match push_once(&cur_branch) {
                        Ok(true) => PushOutcome::Success(cur_branch.clone().unwrap_or_default()),
                        _ => {
                            let lines = vec!["Retry push failed after rebase.".into()];
                            PushOutcome::Failed("Push failed after rebase sync.".into(), lines)
                        }
                    }
                }
                Ok(o) => {
                    let lines: Vec<String> = String::from_utf8_lossy(&o.stderr)
                        .lines().map(|l| l.to_string()).collect();
                    PushOutcome::Conflict(lines)
                }
                Err(e) => PushOutcome::Failed(e.to_string(), vec![]),
            }
        }
        Err((msg, lines)) => PushOutcome::Failed(msg, lines),
    };

    show_push_result(&outcome)
}

fn push_once(branch: &Option<String>) -> Result<bool, (String, Vec<String>)> {
    let mut cmd = Command::new("git");
    cmd.arg("push");
    if let Some(b) = branch {
        cmd.args(["-u", "origin", b.as_str()]);
    } else {
        cmd.arg("-u");
    }
    let out = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| (e.to_string(), vec![]))?;

    if out.status.success() {
        return Ok(true);
    }
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    let is_nff = stderr.contains("non-fast-forward")
        || stderr.contains("fetch first")
        || stderr.contains("behind");
    if is_nff {
        Ok(false)
    } else {
        let lines: Vec<String> = stderr.lines().map(|l| l.to_string()).collect();
        let err_msg = if stderr.contains("has no upstream branch") {
            "Git remote rejected push: Branch has no upstream branch.".to_string()
        } else {
            "Push command rejected by git remote".to_string()
        };
        Err((err_msg, lines))
    }
}

fn show_push_result(outcome: &PushOutcome) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut out = stdout();
    execute!(out, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(out);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| draw_push_result(f, outcome))?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    match outcome {
        PushOutcome::Success(_) => Ok(()),
        PushOutcome::Conflict(_) => Err("Merge conflict — manual resolution required.".into()),
        PushOutcome::Failed(msg, _) => Err(msg.clone().into()),
    }
}

fn draw_push_result(f: &mut ratatui::Frame, outcome: &PushOutcome) {
    let area = f.area();
    f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

    let (border_color, icon, title, body_lines): (Color, &str, &str, Vec<Line>) = match outcome {
        PushOutcome::Success(branch) => (
            C_GREEN, "✅", " PUSH SUCCESSFUL ",
            vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Target Branch: ", Style::default().fg(C_MUTED)),
                    Span::styled(format!("🌿 {branch}"), Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("✅ ", Style::default().fg(C_GREEN)),
                    Span::styled("Everything committed and pushed successfully!", Style::default().fg(C_TEXT).add_modifier(Modifier::BOLD)),
                ]),
            ],
        ),
        PushOutcome::Conflict(lines) => {
            let mut body = vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("⚠️ MERGE CONFLICT DETECTED — RESOLVE MANUALLY:", Style::default().fg(C_GOLD).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![Span::raw("  "), Span::styled("Steps to resolve:", Style::default().fg(C_MUTED))]),
                Line::from(vec![
                    Span::raw("    1) "),
                    Span::styled("git add .", Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::raw("    2) "),
                    Span::styled("git rebase --continue", Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::raw("    3) "),
                    Span::styled("fancybash gwip", Style::default().fg(C_VIOLET).add_modifier(Modifier::BOLD)),
                ]),
            ];
            if !lines.is_empty() {
                body.push(Line::from(""));
                body.push(Line::from(vec![Span::raw("  "), Span::styled("Git Output:", Style::default().fg(C_MUTED))]));
                for l in lines.iter().take(10) {
                    body.push(Line::from(vec![
                        Span::raw("    "),
                        Span::styled(l.as_str(), Style::default().fg(C_MUTED)),
                    ]));
                }
            }
            (C_GOLD, "⚠️", " MERGE CONFLICT ", body)
        }
        PushOutcome::Failed(msg, lines) => {
            let mut body = vec![
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("❌ PUSH FAILED: ", Style::default().fg(C_RED).add_modifier(Modifier::BOLD)),
                    Span::styled(msg.as_str(), Style::default().fg(C_TEXT)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("💡 Note: ", Style::default().fg(C_GOLD).add_modifier(Modifier::BOLD)),
                    Span::styled("Your local commit was created successfully.", Style::default().fg(C_MUTED)),
                ]),
                Line::from(vec![
                    Span::raw("     To push manually later, run: "),
                    Span::styled("git push", Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
                ]),
            ];
            if !lines.is_empty() {
                body.push(Line::from(""));
                body.push(Line::from(vec![Span::raw("  "), Span::styled("Git Error Details:", Style::default().fg(C_MUTED))]));
                for l in lines.iter().take(12) {
                    body.push(Line::from(vec![
                        Span::raw("    "),
                        Span::styled(l.as_str(), Style::default().fg(C_RED)),
                    ]));
                }
            }
            (C_RED, "❌", " PUSH FAILED ", body)
        }
    };

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .margin(1)
        .split(area);

    // Header
    let header = Paragraph::new(Line::from(vec![
        Span::styled(format!(" {icon}  "), Style::default().fg(border_color).add_modifier(Modifier::BOLD)),
        Span::styled("GIT PUSH STATUS  :: ", Style::default().fg(C_TEXT).add_modifier(Modifier::BOLD)),
        Span::styled(title, Style::default().fg(border_color).add_modifier(Modifier::BOLD)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(border_color))
            .style(Style::default().bg(C_CARD)),
    )
    .alignment(Alignment::Center);
    f.render_widget(header, outer[0]);

    // Body
    let body_widget = Paragraph::new(body_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color))
                .style(Style::default().bg(C_CARD)),
        );
    f.render_widget(body_widget, outer[1]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" [Enter / Esc / q] ", Style::default().fg(C_VIOLET).add_modifier(Modifier::BOLD)),
        Span::styled("Dismiss Result Screen", Style::default().fg(C_MUTED)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER))
            .style(Style::default().bg(C_CARD)),
    )
    .alignment(Alignment::Center);
    f.render_widget(footer, outer[2]);
}

fn fetch_staged_files() -> Vec<StagedFile> {
    let out = Command::new("git")
        .args(["status", "--porcelain"])
        .stdout(Stdio::piped()).stderr(Stdio::null()).output();

    if let Ok(o) = out {
        let stdout_str = String::from_utf8_lossy(&o.stdout);
        stdout_str.lines().filter_map(|line| {
            if line.len() >= 4 {
                let status = line[..2].trim().to_string();
                let path = line[3..].to_string();
                let display_status = if status.is_empty() { "M".to_string() } else { status };
                Some(StagedFile { status: display_status, path })
            } else {
                None
            }
        }).collect()
    } else {
        vec![]
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

// ── Unit Tests ────────────────────────────────────────────────────────────────
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
