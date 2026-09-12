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
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

struct CommitType {
    label: &'static str,
    prefix: &'static str,
}

const COMMIT_TYPES: &[CommitType] = &[
    CommitType {
        label: "✏️  Custom…",
        prefix: "",
    },
    CommitType {
        label: "🚧 WIP: Work in progress",
        prefix: "🚧 WIP",
    },
    CommitType {
        label: "✨ feat: New feature",
        prefix: "✨ feat",
    },
    CommitType {
        label: "🐛 fix: Bug fix",
        prefix: "🐛 fix",
    },
    CommitType {
        label: "📝 docs: Documentation",
        prefix: "📝 docs",
    },
    CommitType {
        label: "💄 style: Styling",
        prefix: "💄 style",
    },
    CommitType {
        label: "♻️  refactor: Refactoring",
        prefix: "♻️ refactor",
    },
    CommitType {
        label: "🧪 test: Adding tests",
        prefix: "🧪 test",
    },
    CommitType {
        label: "🔧 chore: Maintenance",
        prefix: "🔧 chore",
    },
];

fn arg_to_prefix(arg: &str) -> Option<&'static str> {
    match arg {
        "feat" | "✨" => Some("✨ feat"),
        "fix" | "🐛" => Some("🐛 fix"),
        "docs" | "📝" => Some("📝 docs"),
        "style" | "💄" => Some("💄 style"),
        "refactor" | "♻️" => Some("♻️ refactor"),
        "test" | "🧪" => Some("🧪 test"),
        "chore" | "🔧" => Some("🔧 chore"),
        "wip" | "🚧" => Some("🚧 WIP"),
        "-m" => Some("🚧 WIP"),
        _ => None,
    }
}

pub fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if !cmd_ok("git", &["rev-parse", "--is-inside-work-tree"]) {
        return Err("Not inside a git repository!".into());
    }

    println!("\x1b[1;36m📦 Auto-staging all changes (git add .)…\x1b[0m");
    run_git(&["add", "."])?;

    let full_msg = if args.is_empty() {
        interactive_commit_msg()?
    } else {
        cli_commit_msg(args)
    };

    println!("\x1b[1;36m📝 Committing: {full_msg}\x1b[0m");
    run_git(&["commit", "-m", &full_msg])?;

    push_with_retry()
}

fn interactive_commit_msg() -> Result<String, Box<dyn std::error::Error>> {
    let chosen_idx = match run_commit_type_tui()? {
        Some(idx) => idx,
        None => {
            println!("⚠️ Commit cancelled.");
            std::process::exit(0);
        }
    };

    let chosen = COMMIT_TYPES[chosen_idx].label;

    let prefix = if chosen.contains("Custom") {
        print!("Type your custom commit prefix: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let v = input.trim();
        if v.is_empty() {
            println!("⚠️ Commit cancelled.");
            std::process::exit(0);
        }
        v.to_string()
    } else {
        COMMIT_TYPES[chosen_idx].prefix.to_string()
    };

    print!("Enter commit message (empty = auto timestamp): ");
    io::stdout().flush()?;
    let mut msg_input = String::new();
    io::stdin().read_line(&mut msg_input)?;

    Ok(build_full_msg(&prefix, msg_input.trim()))
}

fn run_commit_type_tui() -> Result<Option<usize>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut cursor = 0;
    let theme_green = Color::Rgb(80, 200, 120);

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

            let header = Paragraph::new(" 🚀 GIT COMMIT & AUTO PUSH (gwip) ")
                .style(Style::default().fg(Color::Black).bg(theme_green).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme_green)));
            f.render_widget(header, chunks[0]);

            let items: Vec<ListItem> = COMMIT_TYPES
                .iter()
                .enumerate()
                .map(|(idx, item)| {
                    let prefix = if idx == cursor { "➔ " } else { "  " };
                    let style = if idx == cursor {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(format!("{}{}", prefix, item.label)).style(style)
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .title(" Select Commit Type ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme_green)),
            );

            let mut state = ListState::default();
            state.select(Some(cursor));
            f.render_stateful_widget(list, chunks[1], &mut state);

            let footer = Paragraph::new(" [↑/↓] Navigate | [Enter] Select Type | [Esc/q] Cancel ")
                .style(Style::default().fg(theme_green))
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme_green)));
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
                        if cursor < COMMIT_TYPES.len() - 1 {
                            cursor += 1;
                        }
                    }
                    KeyCode::Char('n') if is_ctrl => {
                        if cursor < COMMIT_TYPES.len() - 1 {
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

fn cli_commit_msg(args: &[String]) -> String {
    if args.is_empty() {
        return default_wip_msg();
    }

    let first = args[0].as_str();

    if let Some(prefix) = arg_to_prefix(first) {
        let rest = args[1..].join(" ");
        build_full_msg(prefix, &rest)
    } else if first == "-m" {
        let rest = args[1..].join(" ");
        build_full_msg("🚧 WIP", &rest)
    } else {
        let rest = args.join(" ");
        build_full_msg("🚧 WIP", &rest)
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

fn default_wip_msg() -> String {
    default_wip_msg_with_prefix("🚧 WIP")
}

fn default_wip_msg_with_prefix(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = secs / 86400;
    let year = 1970 + days / 365;
    let day_of_year = days % 365;
    let month = day_of_year / 30 + 1;
    let day = day_of_year % 30 + 1;
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    format!("{prefix}: Save point ({year}-{month:02}-{day:02} {h:02}:{m:02})")
}

fn push_with_retry() -> Result<(), Box<dyn std::error::Error>> {
    let cur_branch = current_branch();

    print!("\x1b[1;36m🚀 Pushing to remote");
    io::stdout().flush().ok();

    let push_result = push_to_remote(&cur_branch);

    match push_result {
        Ok(true) => {
            println!("\r\x1b[1;32m✅ Everything committed and pushed successfully!\x1b[0m");
            Ok(())
        }
        Ok(false) => {
            println!("\r\x1b[1;33m🔄 Remote has new commits. Auto-syncing (git pull --rebase)…\x1b[0m");
            let origin = cur_branch.as_deref().unwrap_or("HEAD");

            let rebase_ok = Command::new("git")
                .args(["pull", "--rebase", "origin", origin])
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            if rebase_ok {
                println!("\x1b[1;36m🚀 Retrying push…\x1b[0m");
                let retry_ok = Command::new("git")
                    .args(["push", "-u", "origin", origin])
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false);

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

    if let Some(b) = branch {
        cmd.args(["origin", b.as_str()]);
    }

    let output = cmd.stdout(Stdio::inherit()).stderr(Stdio::piped()).output()?;

    if output.status.success() {
        return Ok(true);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let is_nff = stderr.contains("non-fast-forward") || stderr.contains("fetch first") || stderr.contains("behind");

    if is_nff {
        Ok(false)
    } else {
        if !stderr.is_empty() {
            eprintln!("\x1b[1;33mGit Error Details:\x1b[0m\n{stderr}");
        }
        Err("push failed".into())
    }
}

fn current_branch() -> Option<String> {
    let out = Command::new("git")
        .args(["branch", "--show-current"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn cmd_ok(prog: &str, args: &[&str]) -> bool {
    Command::new(prog)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn run_git(args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git").args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("git {} failed", args.join(" ")).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_to_prefix_feat() {
        assert_eq!(arg_to_prefix("feat"), Some("✨ feat"));
        assert_eq!(arg_to_prefix("fix"), Some("🐛 fix"));
        assert_eq!(arg_to_prefix("wip"), Some("🚧 WIP"));
    }

    #[test]
    fn test_arg_to_prefix_unknown_returns_none() {
        assert_eq!(arg_to_prefix("foobar"), None);
    }

    #[test]
    fn test_build_full_msg_with_msg() {
        assert_eq!(build_full_msg("✨ feat", "add login"), "✨ feat: add login");
    }

    #[test]
    fn test_build_full_msg_empty_msg_has_timestamp() {
        let msg = build_full_msg("🚧 WIP", "");
        assert!(msg.contains("🚧 WIP: Save point"));
        assert!(msg.contains('-'));
    }

    #[test]
    fn test_cli_commit_msg_no_type() {
        let args: Vec<String> = vec!["my message".into()];
        let msg = cli_commit_msg(&args);
        assert!(msg.starts_with("🚧 WIP: my message"));
    }

    #[test]
    fn test_cli_commit_msg_with_type() {
        let args: Vec<String> = vec!["feat".into(), "new feature".into()];
        let msg = cli_commit_msg(&args);
        assert_eq!(msg, "✨ feat: new feature");
    }

    #[test]
    fn test_commit_types_non_empty() {
        assert!(COMMIT_TYPES.len() >= 9);
    }
}
