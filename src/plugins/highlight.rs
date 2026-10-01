// ============================================================================
//  src/plugins/highlight.rs — Native Rust Syntax Highlighting Engine
//
//  SAFETY GUARANTEES:
//   • Zero raw byte-index slicing — all tokenization is char-iterator based.
//   • Highlight offsets are CHARACTER counts, not byte offsets, matching what
//     Zsh's `region_highlight` expects.
//   • PATH binary scan is deferred to a background thread on first call so the
//     hot path (per-keypress) is never blocked.
//   • Mutex poison on PATH_BINARIES is recovered from gracefully.
//   • No stdout/stderr pollution on any failure.
// ============================================================================

use std::collections::HashSet;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::thread;

// ── Static caches ─────────────────────────────────────────────────────────────

static SHELL_BUILTINS: OnceLock<HashSet<&'static str>> = OnceLock::new();

/// Whether the PATH scan background thread has been launched.
static PATH_SCAN_STARTED: AtomicBool = AtomicBool::new(false);
/// Whether the PATH scan has completed and the cache is populated.
static PATH_SCAN_DONE: AtomicBool = AtomicBool::new(false);
/// The populated PATH binary set (None = scan not yet complete).
static PATH_BINARIES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

// ── Builtin set ───────────────────────────────────────────────────────────────

fn get_builtins() -> &'static HashSet<&'static str> {
    SHELL_BUILTINS.get_or_init(|| {
        [
            // POSIX / Bash / Zsh builtins
            ".", ":", "[", "[[", "alias", "autoload", "bg", "bind", "break",
            "builtin", "caller", "cd", "command", "complete", "compdef",
            "compgen", "continue", "declare", "dirs", "disown", "echo",
            "enable", "eval", "exec", "exit", "export", "false", "fc",
            "fg", "getopts", "hash", "help", "history", "jobs", "kill",
            "let", "local", "logout", "mapfile", "popd", "printf", "pushd",
            "pwd", "read", "readarray", "readonly", "return", "set",
            "setopt", "shift", "shopt", "source", "suspend", "test",
            "times", "trap", "true", "type", "typeset", "ulimit", "umask",
            "unalias", "unfunction", "unset", "unsetopt", "wait",
            "zstyle", "zmodload", "zle", "autoload", "gladeshell",
            // Common CLI tools always treated as valid commands
            "bun", "cargo", "cat", "chmod", "chown", "clang", "cp",
            "curl", "cut", "diff", "docker", "env", "find", "g++", "gcc",
            "git", "go", "grep", "head", "install", "kubectl", "la", "less",
            "ll", "ln", "ls", "make", "man", "mkdir", "mv", "node", "npm",
            "npx", "nvim", "pnpm", "podman", "python", "python3", "rm",
            "rmdir", "rsync", "rustc", "sed", "sort", "ssh", "su", "sudo",
            "tail", "tar", "tee", "touch", "tr", "uname", "unzip", "vim",
            "wc", "wget", "which", "xargs", "yarn", "zip", "zsh", "bash",
            "fish", "nu", "pwsh",
        ]
        .into_iter()
        .collect()
    })
}

// ── PATH binary scan ──────────────────────────────────────────────────────────

/// Ensure the PATH binary cache scan has been started (non-blocking).
/// On first call, spawns a background thread to populate the cache.
fn ensure_path_scan_started() {
    if PATH_SCAN_STARTED
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed)
        .is_ok()
    {
        // We won the race — spawn the scan thread
        let _handle = thread::Builder::new()
            .name("fb-path-scan".to_string())
            .stack_size(128 * 1024) // 128 KiB stack is sufficient
            .spawn(|| {
                let mut set = HashSet::with_capacity(2048);
                if let Ok(path_var) = env::var("PATH") {
                    for dir in env::split_paths(&path_var) {
                        if let Ok(entries) = std::fs::read_dir(&dir) {
                            for entry in entries.flatten() {
                                // Only regular files and symlinks count as executables
                                let is_exec = entry
                                    .file_type()
                                    .map(|ft| ft.is_file() || ft.is_symlink())
                                    .unwrap_or(false);
                                if is_exec {
                                    set.insert(entry.file_name().to_string_lossy().into_owned());
                                }
                            }
                        }
                    }
                }
                // Store result — if this is the first init, we set it;
                // subsequent calls to OnceLock::get_or_init are no-ops.
                PATH_BINARIES.get_or_init(|| Mutex::new(set));
                PATH_SCAN_DONE.store(true, Ordering::Release);
            });
        // If thread spawn fails, leave PATH_SCAN_DONE = false — highlighting
        // will simply not validate PATH binaries (graceful degradation).
    }
}

/// Check whether a command token represents a valid command.
/// Uses builtins (instant) and PATH cache (only when scan is complete).
fn is_valid_command(cmd: &str) -> bool {
    if cmd.is_empty() {
        return false;
    }

    // Absolute or relative path: ./foo or /usr/bin/bar
    if cmd.starts_with('/') || cmd.starts_with("./") || cmd.starts_with("../") {
        // Existence check only on path-like tokens to limit stat() calls
        return std::path::Path::new(cmd).exists();
    }

    // Fast builtin lookup (no syscall)
    if get_builtins().contains(cmd) {
        return true;
    }

    // PATH cache lookup — only after scan is complete to avoid stale reads
    ensure_path_scan_started();
    if PATH_SCAN_DONE.load(Ordering::Acquire) {
        if let Some(cache_mutex) = PATH_BINARIES.get() {
            // Recover from poison
            let cache = match cache_mutex.lock() {
                Ok(g) => g,
                Err(p) => p.into_inner(),
            };
            if cache.contains(cmd) {
                return true;
            }
        }
    }

    false
}

// ── Token highlight descriptor ────────────────────────────────────────────────

/// Highlight span in **character** offsets (not byte offsets).
struct TokenSpan {
    /// Character offset of the first character of the span.
    char_start: usize,
    /// Character offset one past the last character of the span.
    char_end: usize,
    /// Zsh region_highlight style string.
    style: &'static str,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a command buffer and return Zsh `region_highlight` entries.
///
/// Output format: one `"<char_start> <char_end> <style>"` entry per line.
/// Character offsets match Zsh's 0-based character counting so that CJK,
/// emoji, and all other multi-byte codepoints are positioned correctly.
///
/// Never panics; returns an empty string on any error.
pub fn highlight(buffer: &str) -> String {
    if buffer.trim().is_empty() {
        return String::new();
    }

    let mut spans: Vec<TokenSpan> = Vec::with_capacity(16);

    // We tokenize by iterating over (char_index, char) pairs.
    // This is inherently UTF-8 safe — no byte indexing required.
    let chars: Vec<char> = buffer.chars().collect();
    let total = chars.len();
    let mut i = 0usize; // current character index
    let mut is_first_token = true;

    while i < total {
        let ch = chars[i];

        // ── Skip whitespace ──────────────────────────────────────────────────
        if ch.is_whitespace() {
            i += 1;
            continue;
        }

        let span_start = i;

        // ── Pipe, Semicolon, Ampersand ───────────────────────────────────────
        if ch == '|' || ch == ';' || ch == '&' {
            let mut end = i + 1;
            if end < total
                && (chars[end] == '|' || chars[end] == '&' || chars[end] == '>')
            {
                end += 1;
            }
            spans.push(TokenSpan {
                char_start: span_start,
                char_end: end,
                style: "fg=yellow,bold",
            });
            i = end;
            is_first_token = true; // token after pipe/semicolon is a command
            continue;
        }

        // ── Redirection: > >> < ──────────────────────────────────────────────
        if ch == '>' || ch == '<' {
            let mut end = i + 1;
            if end < total && chars[end] == '>' {
                end += 1;
            }
            spans.push(TokenSpan {
                char_start: span_start,
                char_end: end,
                style: "fg=yellow",
            });
            i = end;
            continue;
        }

        // ── Quoted string: '...' or "..." ────────────────────────────────────
        if ch == '\'' || ch == '"' {
            let quote = ch;
            i += 1;
            while i < total && chars[i] != quote {
                if chars[i] == '\\' && i + 1 < total {
                    i += 2; // skip escaped character
                } else {
                    i += 1;
                }
            }
            if i < total {
                i += 1; // consume closing quote
            }
            spans.push(TokenSpan {
                char_start: span_start,
                char_end: i,
                style: "fg=cyan",
            });
            is_first_token = false;
            continue;
        }

        // ── Variable: $VAR or ${VAR} or $() ─────────────────────────────────
        if ch == '$' {
            i += 1;
            if i < total && chars[i] == '{' {
                i += 1;
                while i < total && chars[i] != '}' {
                    i += 1;
                }
                if i < total {
                    i += 1; // consume '}'
                }
            } else if i < total && chars[i] == '(' {
                // Command substitution $(...): skip to matching ')'
                let mut depth = 1usize;
                i += 1;
                while i < total && depth > 0 {
                    if chars[i] == '(' {
                        depth += 1;
                    } else if chars[i] == ')' {
                        depth -= 1;
                    }
                    i += 1;
                }
            } else {
                // Plain variable: $VARNAME
                while i < total
                    && (chars[i].is_alphanumeric() || chars[i] == '_')
                {
                    i += 1;
                }
            }
            spans.push(TokenSpan {
                char_start: span_start,
                char_end: i,
                style: "fg=magenta,bold",
            });
            is_first_token = false;
            continue;
        }

        // ── Comment: # ... (only at start of token) ──────────────────────────
        if ch == '#' && is_first_token {
            // Rest of line is a comment
            spans.push(TokenSpan {
                char_start: span_start,
                char_end: total,
                style: "fg=8", // dim/dark gray
            });
            break;
        }

        // ── Regular word token ───────────────────────────────────────────────
        while i < total
            && !chars[i].is_whitespace()
            && chars[i] != '|'
            && chars[i] != ';'
            && chars[i] != '&'
            && chars[i] != '>'
            && chars[i] != '<'
        {
            i += 1;
        }
        let span_end = i;

        // Reconstruct the token string from the char vec slice (safe — no byte indexing)
        let token_str: String = chars[span_start..span_end].iter().collect();

        let style: &'static str = if is_first_token {
            if is_valid_command(&token_str) {
                "fg=green,bold"
            } else {
                "fg=red,bold"
            }
        } else if token_str.starts_with('-') {
            // Flag / option
            "fg=magenta"
        } else if token_str.starts_with('/') || token_str.starts_with("./") {
            // Path-like argument
            "fg=blue,underline"
        } else if token_str.chars().all(|c| c.is_ascii_digit()) {
            // Numeric literal
            "fg=cyan,bold"
        } else {
            // Plain argument
            "none"
        };

        spans.push(TokenSpan {
            char_start: span_start,
            char_end: span_end,
            style,
        });
        is_first_token = false;
    }

    // ── Serialize to Zsh region_highlight format ──────────────────────────────
    // One entry per line: "<char_start> <char_end> <style>"
    let mut out = String::with_capacity(spans.len() * 28);
    for (idx, span) in spans.iter().enumerate() {
        if idx > 0 {
            out.push('\n');
        }
        out.push_str(&format!("{} {} {}", span.char_start, span.char_end, span.style));
    }
    out
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_and_whitespace_returns_empty() {
        assert_eq!(highlight(""), "");
        assert_eq!(highlight("   "), "");
        assert_eq!(highlight("\t\n"), "");
    }

    #[test]
    fn test_builtin_command_highlighted_green() {
        let result = highlight("echo hello");
        // "echo" at chars 0..4 should be fg=green,bold
        assert!(result.contains("0 4 fg=green,bold"), "expected echo highlighted green, got: {result}");
    }

    #[test]
    fn test_unknown_command_highlighted_red() {
        let result = highlight("zzz_nonexistent_cmd_xyz arg1");
        assert!(
            result.contains("fg=red,bold"),
            "expected unknown command highlighted red, got: {result}"
        );
    }

    #[test]
    fn test_quoted_string_cyan() {
        let result = highlight("echo \"hello world\"");
        assert!(result.contains("fg=cyan"), "expected quoted string highlighted cyan, got: {result}");
    }

    #[test]
    fn test_flag_highlighted_magenta() {
        let result = highlight("ls -la");
        assert!(result.contains("fg=magenta"), "expected flag -la highlighted magenta, got: {result}");
    }

    #[test]
    fn test_pipe_highlighted_yellow_bold() {
        let result = highlight("ls | grep foo");
        assert!(result.contains("fg=yellow,bold"), "expected pipe highlighted yellow,bold, got: {result}");
    }

    #[test]
    fn test_variable_highlighted_magenta_bold() {
        let result = highlight("echo $HOME");
        assert!(result.contains("fg=magenta,bold"), "expected $HOME highlighted magenta,bold");
    }

    #[test]
    fn test_unicode_emoji_does_not_panic() {
        // Must not panic on multi-byte input
        let _ = highlight("echo ⚡");
        let _ = highlight("ls テスト");
        let _ = highlight("cat 日本語ファイル.txt");
        let _ = highlight("git commit -m '🚀 release'");
    }

    #[test]
    fn test_char_offsets_are_character_not_byte() {
        // "echo ⚡" — '⚡' is 3 bytes but 1 character
        // echo is at chars 0..4, ⚡ is at chars 5..6
        let result = highlight("echo ⚡");
        // The second token (⚡) should start at char index 5, not byte index 5
        assert!(
            result.contains("5 6"),
            "char offsets must be character-based not byte-based, got: {result}"
        );
    }

    #[test]
    fn test_command_substitution_does_not_panic() {
        let _ = highlight("echo $(git rev-parse HEAD)");
        let _ = highlight("VAR=$(cat file.txt)");
    }

    #[test]
    fn test_comment_line() {
        let result = highlight("# this is a comment");
        assert!(result.contains("fg=8"), "comment should be dim gray");
    }

    #[test]
    fn test_cjk_char_offsets() {
        // "ls 日本" — 日 is at char index 3, 本 at 4
        let result = highlight("ls 日本");
        // "ls" is at 0..2, "日本" is at 3..5
        assert!(result.contains("3 5"), "CJK token must use char offsets, got: {result}");
    }

    #[test]
    fn test_no_panic_on_unclosed_quote() {
        // Unclosed quotes must not panic
        let _ = highlight("echo 'unclosed");
        let _ = highlight("echo \"unclosed");
    }

    #[test]
    fn test_output_has_no_trailing_newline() {
        let result = highlight("echo hello");
        assert!(!result.ends_with('\n'), "output should not have trailing newline");
    }
}
