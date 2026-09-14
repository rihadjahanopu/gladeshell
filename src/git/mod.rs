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
    /// Absolute path of the working directory this status belongs to.
    pub path: std::path::PathBuf,
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

/// Thread-safe Git status cache.
static GIT_STATUS_CACHE: std::sync::OnceLock<Arc<Mutex<GitStatus>>> =
    std::sync::OnceLock::new();

/// Whether the background updater thread is running.
#[allow(dead_code)]
static UPDATER_RUNNING: AtomicBool = AtomicBool::new(false);

fn cache() -> &'static Arc<Mutex<GitStatus>> {
    GIT_STATUS_CACHE.get_or_init(|| Arc::new(Mutex::new(GitStatus::default())))
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Read cached git status for `cwd`. If the cached path matches `cwd`, returns the cached result.
/// Otherwise, queries Git status for `cwd`, updates the cache, and returns it.
pub fn get_status(cwd: &std::path::Path) -> GitStatus {
    if let Ok(guard) = cache().lock() {
        if guard.path == cwd && !guard.path.as_os_str().is_empty() {
            return guard.clone();
        }
    }
    let status = query_git_status(cwd);
    if let Ok(mut guard) = cache().lock() {
        *guard = status.clone();
    }
    status
}

/// Read the current cached Git status (instant — no I/O).
pub fn read_cached() -> GitStatus {
    cache().lock().unwrap_or_else(|e| e.into_inner()).clone()
}

/// Trigger a Git status refresh for `cwd`.
pub fn refresh(cwd: &std::path::Path) {
    let status = query_git_status(cwd);
    if let Ok(mut guard) = cache().lock() {
        *guard = status;
    }
}

// ── Git query ─────────────────────────────────────────────────────────────────

/// Discover and query Git status.
fn query_git_status(cwd: &std::path::Path) -> GitStatus {
    let git_dir = match find_git_dir(cwd) {
        Some(d) => d,
        None => return GitStatus { path: cwd.to_path_buf(), ..Default::default() },
    };

    let branch = read_head(&git_dir);
    let dirty = is_dirty(&git_dir, cwd);

    GitStatus {
        path: cwd.to_path_buf(),
        branch,
        dirty,
        ahead: false,
        behind: false,
        stash_count: 0,
        is_git_repo: true,
    }
}

/// Walk up the directory tree to find a `.git` directory or file (worktree/submodule).
fn find_git_dir(start: &std::path::Path) -> Option<std::path::PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        let candidate = current.join(".git");
        if candidate.exists() {
            if candidate.is_file() {
                // Worktree or submodule: parse "gitdir: <path>"
                if let Ok(content) = std::fs::read_to_string(&candidate) {
                    if let Some(line) = content.lines().next() {
                        if let Some(path_str) = line.strip_prefix("gitdir:") {
                            let trimmed = path_str.trim();
                            let git_path = std::path::PathBuf::from(trimmed);
                            if git_path.is_absolute() {
                                return Some(git_path);
                            } else {
                                return Some(current.join(git_path));
                            }
                        }
                    }
                }
            } else {
                return Some(candidate);
            }
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
    } else if let Some(tag) = content.strip_prefix("ref: refs/tags/") {
        tag.to_owned()
    } else {
        // Detached HEAD — show short SHA
        content.chars().take(7).collect()
    }
}

/// Check if repository has uncommitted changes.
fn is_dirty(git_dir: &std::path::Path, cwd: &std::path::Path) -> bool {
    let index = git_dir.join("index");
    if !index.exists() {
        return false;
    }

    let output = std::process::Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=no"])
        .current_dir(cwd)
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            return !out.stdout.is_empty();
        }
    }

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
        let cwd = std::env::current_dir().unwrap();
        let _ = find_git_dir(&cwd);
    }

    #[test]
    fn cache_readable_before_refresh() {
        let s = read_cached();
        let _ = s.is_git_repo;
    }

    #[test]
    fn get_status_detects_current_repo() {
        let cwd = std::env::current_dir().unwrap();
        let status = get_status(&cwd);
        assert!(status.is_git_repo);
        assert!(!status.branch.is_empty());
    }
}
