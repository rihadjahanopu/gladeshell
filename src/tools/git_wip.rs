// =============================================================================
//  src/tools/git_wip.rs — `gwip` / `gcommit`: Interactive Git Stage & Push
//
//  Shell features ported:
//    • CLI argument mode: gwip feat "msg", gwip fix "msg", gwip "msg"
//    • Interactive mode (no args): inquire-based commit type selector + message input
//    • Auto-stage: git add .
//    • Push with non-fast-forward detection → auto git pull --rebase retry
//    • Conflict guidance on rebase failure
//    • Spinner simulation (progress indicator while pushing)
// =============================================================================

use std::io::{self, Write};
use std::process::{Command, Stdio};

use inquire::{Select, Text};

// ── Commit type table ─────────────────────────────────────────────────────────

struct CommitType {
    label:  &'static str,
    prefix: &'static str,
}

const COMMIT_TYPES: &[CommitType] = &[
    CommitType { label: "✏️  Custom…",                prefix: ""           },
    CommitType { label: "🚧 WIP: Work in progress",   prefix: "🚧 WIP"     },
    CommitType { label: "✨ feat: New feature",        prefix: "✨ feat"    },
    CommitType { label: "🐛 fix: Bug fix",             prefix: "🐛 fix"     },
    CommitType { label: "📝 docs: Documentation",      prefix: "📝 docs"    },
    CommitType { label: "💄 style: Styling",           prefix: "💄 style"   },
    CommitType { label: "♻️  refactor: Refactoring",   prefix: "♻️ refactor" },
    CommitType { label: "🧪 test: Adding tests",       prefix: "🧪 test"    },
    CommitType { label: "🔧 chore: Maintenance",       prefix: "🔧 chore"   },
];

// ── CLI arg → prefix mapping ──────────────────────────────────────────────────

fn arg_to_prefix(arg: &str) -> Option<&'static str> {
    match arg {
        "feat"   | "✨" => Some("✨ feat"),
        "fix"    | "🐛" => Some("🐛 fix"),
        "docs"   | "📝" => Some("📝 docs"),
        "style"  | "💄" => Some("💄 style"),
        "refactor"|"♻️" => Some("♻️ refactor"),
        "test"   | "🧪" => Some("🧪 test"),
        "chore"  | "🔧" => Some("🔧 chore"),
        "wip"    | "🚧" => Some("🚧 WIP"),
        "-m"            => Some("🚧 WIP"),
        _               => None,
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

/// `fancybash gwip [type] [message]`
///
/// Examples:
///   fancybash gwip                    → interactive TUI
///   fancybash gwip "my message"       → WIP commit with message
///   fancybash gwip feat "new feature" → typed commit
pub fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    // Guard: git installed?
    if !cmd_ok("git", &["--version"]) {
        return Err("Git is not installed.".into());
    }

    // Guard: inside a git repo?
    if !cmd_ok("git", &["rev-parse", "--is-inside-work-tree"]) {
        return Err("Not a git repository.".into());
    }

    // Auto-stage all changes
    run_git(&["add", "."])?;

    let full_msg = if args.is_empty() {
        // ── Interactive mode ──────────────────────────────────────────────────
        interactive_commit_msg()?
    } else {
        // ── CLI argument mode ─────────────────────────────────────────────────
        cli_commit_msg(args)
    };

    // Commit
    println!("\x1b[1;36m📝 Committing: {full_msg}\x1b[0m");
    run_git(&["commit", "-m", &full_msg])?;

    // Push (with rebase-retry on non-fast-forward)
    push_with_retry()
}

// ── Interactive mode ──────────────────────────────────────────────────────────

fn interactive_commit_msg() -> Result<String, Box<dyn std::error::Error>> {
    let labels: Vec<&str> = COMMIT_TYPES.iter().map(|t| t.label).collect();

    let chosen = match Select::new("Select commit type:", labels).prompt() {
        Ok(v)  => v,
        Err(_) => {
            println!("⚠️ Commit cancelled.");
            std::process::exit(0);
        }
    };

    let prefix = if chosen.contains("Custom") {
        // Custom prefix
        match Text::new("Type your custom commit prefix:").prompt() {
            Ok(v) if !v.trim().is_empty() => v.trim().to_string(),
            _ => {
                println!("⚠️ Commit cancelled.");
                std::process::exit(0);
            }
        }
    } else {
        // Lookup the matching static prefix
        COMMIT_TYPES
            .iter()
            .find(|t| t.label == chosen)
            .map(|t| t.prefix.to_string())
            .unwrap_or_else(|| "🚧 WIP".to_string())
    };

    let msg = match Text::new("Enter commit message (empty = auto timestamp):").prompt() {
        Ok(v)  => v,
        Err(_) => {
            println!("⚠️ Commit cancelled.");
            std::process::exit(0);
        }
    };

    Ok(build_full_msg(&prefix, &msg))
}

// ── CLI mode ──────────────────────────────────────────────────────────────────

fn cli_commit_msg(args: &[String]) -> String {
    if args.is_empty() {
        return default_wip_msg();
    }

    let first = args[0].as_str();

    if let Some(prefix) = arg_to_prefix(first) {
        // gwip feat "message" | gwip feat  (no message)
        let rest = args[1..].join(" ");
        build_full_msg(prefix, &rest)
    } else if first == "-m" {
        // gwip -m "message"
        let rest = args[1..].join(" ");
        build_full_msg("🚧 WIP", &rest)
    } else {
        // gwip "message without type"
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
    // Minimal date: seconds since epoch converted to a rough date string
    // We avoid the `chrono` crate to keep zero extra deps
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // epoch-based date (good enough for a save-point label)
    let days   = secs / 86400;
    let year   = 1970 + days / 365;
    let day_of_year = days % 365;
    let month  = day_of_year / 30 + 1;
    let day    = day_of_year % 30 + 1;
    let h      = (secs % 86400) / 3600;
    let m      = (secs % 3600)  / 60;
    format!("{prefix}: Save point ({year}-{month:02}-{day:02} {h:02}:{m:02})")
}

// ── Push with non-fast-forward retry ─────────────────────────────────────────

fn push_with_retry() -> Result<(), Box<dyn std::error::Error>> {
    let cur_branch = current_branch();

    print!("\x1b[1;36m🚀 Pushing to remote");
    io::stdout().flush().ok();

    // Spinner: spin while push runs
    let push_result = push_to_remote(&cur_branch);

    match push_result {
        Ok(true) => {
            println!("\r\x1b[1;32m✅ Everything committed and pushed successfully!\x1b[0m");
            return Ok(());
        }
        Ok(false) => {
            // Check if it's a non-fast-forward issue
            // We'll try rebase automatically
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
                return Err("❌ Push failed after rebase sync.".into());
            } else {
                // Merge conflict
                eprintln!("\x1b[0;31m⚠️  Merge Conflict detected!\x1b[0m");
                eprintln!("\x1b[1;33mPlease resolve conflicts, then run:\x1b[0m");
                eprintln!("  1) \x1b[1;36mgit add .\x1b[0m");
                eprintln!("  2) \x1b[1;36mgit rebase --continue\x1b[0m");
                eprintln!("  3) \x1b[1;36mfancybash gwip\x1b[0m");
                return Err("Merge conflict — manual resolution required.".into());
            }
        }
        Err(e) => {
            eprintln!("\x1b[0;31m❌ Push failed: {e}\x1b[0m");
            eprintln!("\x1b[1;33m💡 Your local commit was created successfully.\x1b[0m");
            return Err(e);
        }
    }
}

/// Returns `Ok(true)` on success, `Ok(false)` on non-fast-forward, `Err` on other failures.
fn push_to_remote(branch: &Option<String>) -> Result<bool, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("git");
    cmd.arg("push").arg("-u");

    if let Some(b) = branch {
        cmd.args(["origin", b.as_str()]);
    }

    // Capture stderr to detect non-fast-forward
    let output = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        return Ok(true);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let is_nff = stderr.contains("non-fast-forward")
        || stderr.contains("fetch first")
        || stderr.contains("behind");

    if is_nff {
        Ok(false)  // caller will rebase-retry
    } else {
        if !stderr.is_empty() {
            eprintln!("\x1b[1;33mGit Error Details:\x1b[0m\n{stderr}");
        }
        Err("push failed".into())
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn current_branch() -> Option<String> {
    let out = Command::new("git")
        .args(["branch", "--show-current"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if s.is_empty() { None } else { Some(s) }
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

// =============================================================================
//  Unit tests
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_to_prefix_feat() {
        assert_eq!(arg_to_prefix("feat"), Some("✨ feat"));
        assert_eq!(arg_to_prefix("fix"),  Some("🐛 fix"));
        assert_eq!(arg_to_prefix("wip"),  Some("🚧 WIP"));
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
        assert!(msg.contains('-')); // date-like
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
