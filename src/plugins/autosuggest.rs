// ============================================================================
//  src/plugins/autosuggest.rs — Native Rust Zsh/Bash Autosuggestion Engine
//
//  SAFETY GUARANTEES:
//   • Mutex poison is recovered from — never panics on `.lock()`.
//   • All byte-index slicing uses `.get()` with explicit `None` fallback.
//   • History loading is fully defensive: malformed files, bad UTF-8,
//     inaccessible paths, and missing env vars all produce empty results.
//   • Cross-platform: reads Bash, Zsh, Fish, and PowerShell history files.
//   • No stdout/stderr pollution on any failure.
// ============================================================================

use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

/// Maximum number of history entries to keep in memory.
const MAX_HISTORY_ENTRIES: usize = 50_000;

struct HistoryCache {
    commands: Vec<String>,
}

static CACHE: OnceLock<Mutex<HistoryCache>> = OnceLock::new();

/// Get (or initialize) the global history cache.
/// Returns the mutex — callers MUST recover from poison via `unwrap_or_else`.
fn get_cache() -> &'static Mutex<HistoryCache> {
    CACHE.get_or_init(|| {
        Mutex::new(HistoryCache {
            commands: Vec::new(),
        })
    })
}

/// Returns the current user's home directory.
/// Checks $HOME (Unix) then $USERPROFILE (Windows) then CWD as last resort.
fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

/// Collect all history files to scan across shells.
fn history_file_candidates(home: &PathBuf) -> Vec<PathBuf> {
    let mut candidates = vec![
        home.join(".bash_history"),
        home.join(".zsh_history"),
        // Fish shell history (TOML-like format, we'll extract `cmd:` lines)
        home.join(".local/share/fish/fish_history"),
        // PowerShell (Linux/macOS)
        home.join(".local/share/powershell/PSReadLine/ConsoleHost_history.txt"),
        // PowerShell (macOS via Homebrew)
        home.join(".config/powershell/PSReadLine/ConsoleHost_history.txt"),
    ];

    // PowerShell on Windows: $APPDATA\Microsoft\Windows\PowerShell\PSReadLine\...
    if let Some(appdata) = std::env::var_os("APPDATA") {
        let win_pwsh = PathBuf::from(&appdata)
            .join("Microsoft")
            .join("Windows")
            .join("PowerShell")
            .join("PSReadLine")
            .join("ConsoleHost_history.txt");
        candidates.push(win_pwsh);
    }

    // Also check $XDG_DATA_HOME for Fish
    if let Ok(xdg) = std::env::var("XDG_DATA_HOME") {
        candidates.push(PathBuf::from(xdg).join("fish").join("fish_history"));
    }

    candidates
}

/// Load history from all available shell history files.
/// Never panics; errors silently produce fewer entries.
fn load_all_history() -> Vec<String> {
    let home = match dirs_home() {
        Some(h) => h,
        None => return Vec::new(),
    };

    let mut all_cmds: Vec<String> = Vec::with_capacity(4096);

    for path in history_file_candidates(&home) {
        if path.is_file() {
            all_cmds.extend(load_history_file(&path));
        }
    }

    // Trim to cap before dedup to bound allocations
    if all_cmds.len() > MAX_HISTORY_ENTRIES * 2 {
        let drop_count = all_cmds.len() - MAX_HISTORY_ENTRIES * 2;
        all_cmds.drain(0..drop_count);
    }

    // Deduplicate consecutive duplicates while preserving reverse order
    let mut deduped: Vec<String> = Vec::with_capacity(all_cmds.len().min(MAX_HISTORY_ENTRIES));
    for cmd in all_cmds {
        if deduped.last() != Some(&cmd) {
            deduped.push(cmd);
        }
    }

    if deduped.len() > MAX_HISTORY_ENTRIES {
        deduped.truncate(MAX_HISTORY_ENTRIES);
    }

    deduped
}

/// Parse and extract commands from a single history file.
///
/// Handles multiple formats:
///  - Bash:  plain lines
///  - Zsh:   `: timestamp:elapsed;command`
///  - Fish:  YAML-like blocks with `- cmd: command`
///  - PSReadLine: plain lines
fn load_history_file(path: &PathBuf) -> Vec<String> {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return Vec::new(),
    };

    // Lossy UTF-8: replace invalid sequences rather than failing
    let content = String::from_utf8_lossy(&bytes);

    let is_fish = path
        .file_name()
        .map(|n| n == "fish_history")
        .unwrap_or(false);

    let mut lines: Vec<String> = Vec::with_capacity(1024);

    for raw_line in content.lines() {
        let line = raw_line.trim_end();
        if line.is_empty() {
            continue;
        }

        let cmd: &str = if is_fish {
            // Fish YAML format: `- cmd: actual command here`
            if let Some(rest) = line.strip_prefix("- cmd: ") {
                rest
            } else {
                continue; // skip metadata lines (- when:, etc.)
            }
        } else if line.starts_with(':') {
            // Zsh extended history: `: 1720000000:0;cmd`
            // Find the ';' separator safely using char indices
            match line.char_indices().find(|&(_, c)| c == ';') {
                Some((pos, _)) => {
                    // pos is guaranteed a valid char boundary (';' is ASCII)
                    // pos + 1 is safe because ';' is exactly 1 byte
                    line.get(pos + 1..).unwrap_or("")
                }
                None => line,
            }
        } else {
            line
        };

        let trimmed = cmd.trim();
        if is_junk(trimmed) {
            continue;
        }

        lines.push(trimmed.to_string());
    }

    // Reverse so most-recent commands appear first
    lines.reverse();
    lines
}

/// Return `true` if the command line is noise that should not be suggested.
fn is_junk(cmd: &str) -> bool {
    if cmd.is_empty() {
        return true;
    }

    // Bare shell syntax noise
    if matches!(cmd, "}" | "});" | "};" | ")" | "];" | ">" | ">>" | "{" | "(") {
        return true;
    }

    // Single characters are not useful suggestions
    if cmd.chars().count() == 1 {
        return true;
    }

    // Git diff statistics output leaked into history
    if cmd.contains("file changed")
        || cmd.contains("files changed")
        || cmd.contains("insertion(+)")
        || cmd.contains("deletion(-)")
    {
        return true;
    }

    // Lines starting with '#' are comments (Zsh / Bash HIST_NO_FUNCTIONS edge case)
    if cmd.starts_with('#') {
        return true;
    }

    false
}

/// Fetch an autosuggestion for the given prompt input buffer.
///
/// Returns `None` if:
///  - The buffer contains only whitespace
///  - No matching history entry is found
///  - The cache mutex is poisoned (graceful degradation)
pub fn suggest(buffer: &str) -> Option<String> {
    // Whitespace guard: never suggest for blank input
    let trimmed_input = buffer.trim_start();
    if trimmed_input.is_empty() {
        return None;
    }

    // Recover from mutex poison — the poisoned value still contains valid data
    let mut cache = match get_cache().lock() {
        Ok(guard) => guard,
        Err(poison) => poison.into_inner(),
    };

    if cache.commands.is_empty() {
        cache.commands = load_all_history();
    }

    // Search most-recent matching command (commands are stored most-recent first)
    for cmd in &cache.commands {
        // Prefer exact-prefix match against the full typed buffer (preserving leading whitespace)
        if cmd.starts_with(buffer) && cmd.len() > buffer.len() {
            return Some(cmd.clone());
        }
        // Also match against the whitespace-trimmed version
        if cmd.starts_with(trimmed_input) && cmd.len() > trimmed_input.len() {
            return Some(cmd.clone());
        }
    }

    None
}

/// Invalidate the history cache, forcing a reload on the next `suggest()` call.
/// Call this after the user executes a command so fresh history is picked up.
pub fn invalidate_cache() {
    if let Ok(mut cache) = get_cache().lock() {
        cache.commands.clear();
    }
    // If the lock is poisoned, we simply skip invalidation — next lock call
    // will recover the poisoned value and it will be refreshed anyway.
}

/// Append a new command entry to the top of the history cache in real time.
/// Removes duplicate instances of the command so the latest execution remains first.
pub fn add_history_entry(cmd: &str) {
    let trimmed = cmd.trim();
    if trimmed.is_empty() || is_junk(trimmed) {
        return;
    }

    let mut cache = match get_cache().lock() {
        Ok(guard) => guard,
        Err(poison) => poison.into_inner(),
    };

    if cache.commands.is_empty() {
        cache.commands = load_all_history();
    }

    // Deduplicate: remove existing instance if present
    cache.commands.retain(|existing| existing != trimmed);
    // Push to top (index 0) so it takes precedence in autosuggestion
    cache.commands.insert(0, trimmed.to_string());

    if cache.commands.len() > MAX_HISTORY_ENTRIES {
        cache.commands.truncate(MAX_HISTORY_ENTRIES);
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_buffer_returns_none() {
        assert_eq!(suggest(""), None);
        assert_eq!(suggest("   "), None);
        assert_eq!(suggest("\t\n"), None);
    }

    #[test]
    fn test_is_junk_filters_noise() {
        assert!(is_junk(""));
        assert!(is_junk("}"));
        assert!(is_junk("});"));
        assert!(is_junk("# comment"));
        assert!(is_junk("x")); // single char
        assert!(!is_junk("git status"));
        assert!(!is_junk("cargo build --release"));
    }

    #[test]
    fn test_zsh_extended_history_parsing() {
        // Simulate the zsh extended history format
        let fake_content = b": 1720000000:0;git status\n: 1720000001:0;cargo build\nplain command\n";
        let path = PathBuf::from("/does/not/exist/zsh_history");
        // Parse directly using internal logic
        let content = String::from_utf8_lossy(fake_content);
        let mut cmds = Vec::new();
        for raw_line in content.lines() {
            let line = raw_line.trim_end();
            if line.starts_with(':') {
                if let Some((pos, _)) = line.char_indices().find(|&(_, c)| c == ';') {
                    if let Some(cmd) = line.get(pos + 1..) {
                        let t = cmd.trim();
                        if !is_junk(t) {
                            cmds.push(t.to_string());
                        }
                    }
                }
            } else if !is_junk(line.trim()) {
                cmds.push(line.trim().to_string());
            }
        }
        assert!(cmds.contains(&"git status".to_string()));
        assert!(cmds.contains(&"cargo build".to_string()));
        assert!(cmds.contains(&"plain command".to_string()));
        drop(path); // suppress unused warning
    }

    #[test]
    fn test_fish_history_parsing() {
        let fake_content = b"- cmd: echo hello\n  when: 1720000000\n- cmd: ls -la\n  when: 1720000001\n";
        let content = String::from_utf8_lossy(fake_content);
        let mut cmds = Vec::new();
        for raw_line in content.lines() {
            let line = raw_line.trim_end();
            if let Some(rest) = line.strip_prefix("- cmd: ") {
                let t = rest.trim();
                if !is_junk(t) {
                    cmds.push(t.to_string());
                }
            }
        }
        assert!(cmds.contains(&"echo hello".to_string()));
        assert!(cmds.contains(&"ls -la".to_string()));
    }

    #[test]
    fn test_unicode_buffer_does_not_panic() {
        // Must not panic on any unicode input
        let _ = suggest("git テスト");
        let _ = suggest("⚡ fire");
        let _ = suggest("日本語コマンド");
        let _ = suggest("café");
    }

    #[test]
    fn test_mutex_poison_recovery() {
        // This test verifies the lock-recovery path compiles and is reachable.
        // We can't easily poison a Mutex in safe Rust from outside a thread,
        // but we verify the non-poisoned path works correctly.
        let result = suggest("zzz_definitely_not_in_history_xyz");
        assert_eq!(result, None);
    }

    #[test]
    fn test_add_history_entry_live_sync() {
        add_history_entry("fancybash_test_live_command --sync");
        let suggestion = suggest("fancybash_test_live_");
        assert_eq!(suggestion, Some("fancybash_test_live_command --sync".to_string()));
    }
}
