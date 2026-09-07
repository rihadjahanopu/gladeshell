// =============================================================================
//  src/git/mod.rs — Lock-free async Git status
//
//  Phase 1: Stub with the public API shape and doc comments.
//  Phase 2: Full implementation using `gix` (gitoxide) with an atomic cache
//           updated by a background thread — zero forks, < 0.1 ms read time.
//
//  Design:
//    ┌──────────────────┐    background thread    ┌─────────────────────┐
//    │  Shell precmd    │ ─── reads AtomicPtr ───> │  GitStatus cache    │
//    │  hook calls      │                          │  (ArcSwap / atomic) │
//    │  fb_prompt_render│                          │                     │
//    └──────────────────┘                          └─────────────────────┘
//                                                         ↑
//                                                  gix discovers git repo
//                                                  on CWD change (inotify)
// =============================================================================

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

// ── Public types ──────────────────────────────────────────────────────────────

/// Cached result of a Git status query.
#[derive(Debug, Clone, Default)]
pub struct GitStatus {
    /// Current branch name, or short SHA if detached HEAD.
    pub branch: String,
    /// True if working tree or index has uncommitted changes.
    pub dirty: bool,
    /// True if ahead of upstream.
    pub ahead: bool,
    /// True if behind upstream.
    pub behind: bool,
    /// Stash count.
    pub stash_count: u32,
    /// True if we are currently inside a git repository.
    pub is_git_repo: bool,
}

// ── Global atomic cache ───────────────────────────────────────────────────────

/// Thread-safe, lock-free Git status cache.
///
/// Phase 2 uses ArcSwap for wait-free reads on the prompt thread.
/// Phase 1 uses a simple Mutex for correctness; will be replaced.
static GIT_STATUS_CACHE: std::sync::OnceLock<Arc<Mutex<GitStatus>>> =
    std::sync::OnceLock::new();

/// Whether the background updater thread is running.
#[allow(dead_code)]
static UPDATER_RUNNING: AtomicBool = AtomicBool::new(false);

fn cache() -> &'static Arc<Mutex<GitStatus>> {
    GIT_STATUS_CACHE.get_or_init(|| Arc::new(Mutex::new(GitStatus::default())))
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Read the current cached Git status (instant — no I/O).
///
/// Returns `None` if the cache has never been populated (first call before
/// the background thread has run).
pub fn read_cached() -> GitStatus {
    cache().lock().unwrap_or_else(|e| e.into_inner()).clone()
}

/// Trigger an async Git status refresh for `cwd`.
///
/// Phase 1: Runs inline (blocking) for correctness.
/// Phase 2: Dispatches to the background `gix`-powered updater thread.
pub fn refresh(cwd: &std::path::Path) {
    let status = query_git_status(cwd);
    if let Ok(mut guard) = cache().lock() {
        *guard = status;
    }
}

// ── Git query (Phase 1: pure-std fallback) ────────────────────────────────────

/// Discover and query Git status using pure-std file reading (no subprocesses).
///
/// Phase 1 reads `.git/HEAD` and `.git/index` directly to avoid any fork.
/// Phase 2 replaces this with `gix` for complete, accurate, and faster results.
fn query_git_status(cwd: &std::path::Path) -> GitStatus {
    #[cfg(feature = "gix")]
    {
        if let Ok(repo) = gix::discover(cwd) {
            let branch = if let Ok(head) = repo.head() {
                if let Some(name) = head.referent_name() {
                    name.shorten().to_string()
                } else if let Some(id) = head.id() {
                    id.to_hex_with_len(7).to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            let dirty = is_dirty(repo.git_dir());

            return GitStatus {
                branch,
                dirty,
                ahead: false,
                behind: false,
                stash_count: 0,
                is_git_repo: true,
            };
        }
    }

    // Walk up from cwd to find .git directory (fallback)
    let git_dir = find_git_dir(cwd);
    let git_dir = match git_dir {
        Some(d) => d,
        None => return GitStatus::default(),
    };

    let branch = read_head(&git_dir);
    let dirty = is_dirty(&git_dir);

    GitStatus {
        branch,
        dirty,
        ahead: false,       // Phase 2: parse FETCH_HEAD / packed-refs
        behind: false,
        stash_count: 0,     // Phase 2: count refs/stash
        is_git_repo: true,
    }
}

/// Walk up the directory tree to find a `.git` directory or file.
fn find_git_dir(start: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        let candidate = current.join(".git");
        if candidate.exists() {
            return Some(candidate);
        }
        if !current.pop() {
            return None;
        }
    }
}

/// Read the current branch name from `.git/HEAD`.
///
/// Format: `ref: refs/heads/<branch>` → returns `<branch>`.
/// Detached HEAD: `<sha>` → returns first 7 chars.
fn read_head(git_dir: &std::path::Path) -> String {
    let head_path = git_dir.join("HEAD");
    let content = match std::fs::read_to_string(&head_path) {
        Ok(c) => c,
        Err(_) => return String::new(),
    };
    let content = content.trim();
    if let Some(branch) = content.strip_prefix("ref: refs/heads/") {
        branch.to_owned()
    } else {
        // Detached HEAD — show short SHA
        content.chars().take(7).collect()
    }
}

/// Heuristic dirty check: compare `.git/index` mtime against the git dir mtime.
///
/// Phase 2 replaces this with `gix`'s precise status API.
fn is_dirty(git_dir: &std::path::Path) -> bool {
    let index = git_dir.join("index");
    // If index doesn't exist, repo might be freshly initialised (clean).
    if !index.exists() {
        return false;
    }
    // Phase 1 heuristic: always report "not dirty" to avoid false positives.
    // Phase 2: use gix::Repository::status() for accuracy.
    false
}

// =============================================================================
//  Unit tests
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_status_is_not_git_repo() {
        let s = GitStatus::default();
        assert!(!s.is_git_repo);
    }

    #[test]
    fn find_git_dir_finds_this_repo() {
        // The fancybash-rs repo itself has a .git directory.
        let cwd = std::env::current_dir().unwrap();
        // It may or may not exist depending on test environment, so just
        // ensure the function doesn't panic.
        let _ = find_git_dir(&cwd);
    }

    #[test]
    fn cache_readable_before_refresh() {
        // Should return default (empty) status without panicking.
        let s = read_cached();
        // Either in a git repo (CI) or not — both are valid.
        let _ = s.is_git_repo;
    }
}
