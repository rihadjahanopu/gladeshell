// ============================================================================
//  src/plugins/autocomplete.rs — Native Rust Zsh Autocompletion Engine
//
//  SAFETY GUARANTEES:
//   • Zero `.unwrap()` / `.expect()` / raw index slices.
//   • All string slicing is char-boundary-safe via `.get(start..end)`.
//   • All filesystem calls are guarded; errors produce empty results silently.
//   • No stdout/stderr pollution on failure.
// ============================================================================

use std::fs;
use std::path::Path;

/// Split `word` at the last `/` into `(directory_prefix, filename_prefix)`.
/// Returns char-boundary-safe slices; never panics on multi-byte UTF-8.
fn split_at_last_slash(word: &str) -> (&str, &str) {
    // rfind on a char is guaranteed to return a valid char boundary index.
    match word.rfind('/') {
        Some(pos) => {
            // `pos` is the byte index of '/', which is a 1-byte ASCII char.
            // `pos + 1` is therefore always a valid UTF-8 boundary.
            let dir = word.get(..=pos).unwrap_or("./");
            let pre = word.get(pos + 1..).unwrap_or("");
            (dir, pre)
        }
        None => ("./", word),
    }
}

/// Helper to scan directory and add matching file/directory candidates.
fn collect_path_completions(last_word: &str, candidates: &mut Vec<String>) {
    let (dir_part, prefix) = split_at_last_slash(last_word);
    let dir_path = Path::new(dir_part);

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with(prefix) {
                let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                let formatted = if is_dir { format!("{}/", name) } else { name };
                candidates.push(formatted);
            }
        }
    }
}

/// Generate completion candidates for the prompt input buffer.
/// Returns a newline-separated candidate list.
/// Never panics; returns an empty string on any error.
pub fn complete(buffer: &str) -> String {
    let trimmed = buffer.trim_start();
    if trimmed.is_empty() {
        return String::new();
    }

    // last_word: the token after the final whitespace in the buffer.
    let last_word = buffer.split_whitespace().last().unwrap_or("");
    if last_word.is_empty() {
        return String::new();
    }

    let mut candidates: Vec<String> = Vec::new();

    // ── 1. Path & File completions (explicit path syntax) ─────────────────────
    let has_path_prefix =
        last_word.contains('/') || last_word.starts_with('.') || last_word.starts_with('~');
    if has_path_prefix {
        collect_path_completions(last_word, &mut candidates);
    }

    // ── 2. Common CLI Flags ───────────────────────────────────────────────────
    if last_word.starts_with('-') {
        const COMMON_FLAGS: &[&str] = &[
            "--help",
            "--version",
            "--verbose",
            "--all",
            "--force",
            "--quiet",
            "-h",
            "-v",
            "-a",
            "-f",
            "-q",
            "-y",
            "-j",
            "-o",
        ];
        for &flag in COMMON_FLAGS {
            if flag.starts_with(last_word) {
                candidates.push(flag.to_string());
            }
        }
    }

    // ── 3. Git subcommand completions ─────────────────────────────────────────
    if trimmed.starts_with("git ") {
        const GIT_CMDS: &[&str] = &[
            "add",
            "bisect",
            "blame",
            "branch",
            "checkout",
            "cherry-pick",
            "clean",
            "clone",
            "commit",
            "config",
            "describe",
            "diff",
            "fetch",
            "format-patch",
            "gc",
            "grep",
            "init",
            "log",
            "merge",
            "mv",
            "notes",
            "pull",
            "push",
            "rebase",
            "reflog",
            "remote",
            "reset",
            "restore",
            "revert",
            "rm",
            "shortlog",
            "show",
            "stash",
            "status",
            "submodule",
            "switch",
            "tag",
            "worktree",
        ];
        for &cmd in GIT_CMDS {
            if cmd.starts_with(last_word) {
                candidates.push(cmd.to_string());
            }
        }
    }

    // ── 4. Cargo subcommand completions ───────────────────────────────────────
    if trimmed.starts_with("cargo ") {
        const CARGO_CMDS: &[&str] = &[
            "add",
            "bench",
            "build",
            "check",
            "clean",
            "clippy",
            "doc",
            "fix",
            "fmt",
            "generate-lockfile",
            "init",
            "install",
            "metadata",
            "new",
            "package",
            "publish",
            "remove",
            "run",
            "rustc",
            "rustdoc",
            "search",
            "test",
            "tree",
            "uninstall",
            "update",
            "vendor",
        ];
        for &cmd in CARGO_CMDS {
            if cmd.starts_with(last_word) {
                candidates.push(cmd.to_string());
            }
        }
    }

    // ── 5. npm / pnpm / bun subcommand completions ────────────────────────────
    let is_npm_like =
        trimmed.starts_with("npm ") || trimmed.starts_with("pnpm ") || trimmed.starts_with("bun ");
    if is_npm_like {
        const NPM_CMDS: &[&str] = &[
            "install",
            "uninstall",
            "update",
            "run",
            "start",
            "test",
            "build",
            "publish",
            "link",
            "pack",
            "audit",
            "outdated",
            "init",
            "ci",
            "exec",
            "list",
            "info",
        ];
        for &cmd in NPM_CMDS {
            if cmd.starts_with(last_word) {
                candidates.push(cmd.to_string());
            }
        }
    }

    // ── 6. Docker subcommand completions ─────────────────────────────────────
    if trimmed.starts_with("docker ") {
        const DOCKER_CMDS: &[&str] = &[
            "attach",
            "build",
            "commit",
            "container",
            "cp",
            "create",
            "diff",
            "events",
            "exec",
            "export",
            "history",
            "image",
            "images",
            "import",
            "info",
            "inspect",
            "kill",
            "load",
            "login",
            "logout",
            "logs",
            "network",
            "pause",
            "port",
            "ps",
            "pull",
            "push",
            "rename",
            "restart",
            "rm",
            "rmi",
            "run",
            "save",
            "search",
            "start",
            "stats",
            "stop",
            "tag",
            "top",
            "unpause",
            "update",
            "version",
            "volume",
            "wait",
        ];
        for &cmd in DOCKER_CMDS {
            if cmd.starts_with(last_word) {
                candidates.push(cmd.to_string());
            }
        }
    }

    // ── 7. Kubectl subcommand completions ────────────────────────────────────
    if trimmed.starts_with("kubectl ") {
        const KUBECTL_CMDS: &[&str] = &[
            "apply",
            "auth",
            "cluster-info",
            "config",
            "create",
            "delete",
            "describe",
            "diff",
            "edit",
            "exec",
            "explain",
            "get",
            "label",
            "logs",
            "patch",
            "port-forward",
            "replace",
            "rollout",
            "run",
            "scale",
            "top",
            "version",
        ];
        for &cmd in KUBECTL_CMDS {
            if cmd.starts_with(last_word) {
                candidates.push(cmd.to_string());
            }
        }
    }

    // ── 8. GitHub CLI (gh) subcommand completions ────────────────────────────
    if trimmed.starts_with("gh ") {
        const GH_CMDS: &[&str] = &[
            "actions",
            "alias",
            "attestation",
            "auth",
            "browse",
            "codespace",
            "completion",
            "config",
            "extension",
            "gist",
            "issue",
            "org",
            "project",
            "pr",
            "release",
            "repo",
            "run",
            "search",
            "secret",
            "ssh-key",
            "status",
            "variable",
            "workflow",
        ];
        for &cmd in GH_CMDS {
            if cmd.starts_with(last_word) {
                candidates.push(cmd.to_string());
            }
        }
    }

    // ── 9. Systemctl subcommand completions ──────────────────────────────────
    if trimmed.starts_with("systemctl ") {
        const SYSTEMCTL_CMDS: &[&str] = &[
            "daemon-reload",
            "disable",
            "enable",
            "is-active",
            "is-enabled",
            "is-failed",
            "list-units",
            "mask",
            "reload",
            "restart",
            "start",
            "status",
            "stop",
            "unmask",
        ];
        for &cmd in SYSTEMCTL_CMDS {
            if cmd.starts_with(last_word) {
                candidates.push(cmd.to_string());
            }
        }
    }

    // ── 10. Pip / Pip3 subcommand completions ────────────────────────────────
    let is_pip_like = trimmed.starts_with("pip ") || trimmed.starts_with("pip3 ");
    if is_pip_like {
        const PIP_CMDS: &[&str] = &[
            "audit",
            "cache",
            "check",
            "config",
            "download",
            "freeze",
            "hash",
            "index",
            "inspect",
            "install",
            "list",
            "search",
            "show",
            "uninstall",
            "wheel",
        ];
        for &cmd in PIP_CMDS {
            if cmd.starts_with(last_word) {
                candidates.push(cmd.to_string());
            }
        }
    }

    // ── 11. Terraform subcommand completions ─────────────────────────────────
    if trimmed.starts_with("terraform ") {
        const TF_CMDS: &[&str] = &[
            "apply",
            "console",
            "destroy",
            "fmt",
            "force-unlock",
            "get",
            "graph",
            "import",
            "init",
            "login",
            "logout",
            "metadata",
            "output",
            "plan",
            "providers",
            "refresh",
            "show",
            "state",
            "taint",
            "test",
            "untaint",
            "validate",
            "version",
            "workspace",
        ];
        for &cmd in TF_CMDS {
            if cmd.starts_with(last_word) {
                candidates.push(cmd.to_string());
            }
        }
    }

    // ── 12. Fallback Path Completion ─────────────────────────────────────────
    if candidates.is_empty() && !has_path_prefix && !last_word.starts_with('-') {
        collect_path_completions(last_word, &mut candidates);
    }

    // ── Dedup & sort ──────────────────────────────────────────────────────────
    candidates.sort_unstable();
    candidates.dedup();
    candidates.join("\n")
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_buffer_returns_empty() {
        assert_eq!(complete(""), "");
        assert_eq!(complete("   "), "");
    }

    #[test]
    fn test_git_completions() {
        let result = complete("git sta");
        assert!(
            result.contains("status"),
            "should complete 'git sta' → 'status'"
        );
        assert!(
            result.contains("stash"),
            "should complete 'git sta' → 'stash'"
        );
    }

    #[test]
    fn test_cargo_completions() {
        let result = complete("cargo b");
        assert!(
            result.contains("build"),
            "should complete 'cargo b' → 'build'"
        );
        assert!(
            result.contains("bench"),
            "should complete 'cargo b' → 'bench'"
        );
    }

    #[test]
    fn test_kubectl_completions() {
        let result = complete("kubectl g");
        assert!(
            result.contains("get"),
            "should complete 'kubectl g' → 'get'"
        );
    }

    #[test]
    fn test_gh_completions() {
        let result = complete("gh p");
        assert!(result.contains("pr"), "should complete 'gh p' → 'pr'");
    }

    #[test]
    fn test_systemctl_completions() {
        let result = complete("systemctl res");
        assert!(
            result.contains("restart"),
            "should complete 'systemctl res' → 'restart'"
        );
    }

    #[test]
    fn test_pip_completions() {
        let result = complete("pip ins");
        assert!(
            result.contains("install"),
            "should complete 'pip ins' → 'install'"
        );
    }

    #[test]
    fn test_terraform_completions() {
        let result = complete("terraform ap");
        assert!(
            result.contains("apply"),
            "should complete 'terraform ap' → 'apply'"
        );
    }

    #[test]
    fn test_flag_completions() {
        let result = complete("git checkout --h");
        assert!(result.contains("--help"));
    }

    #[test]
    fn test_unicode_last_word_does_not_panic() {
        let _ = complete("ls /tmp/テスト");
        let _ = complete("cat ./⚡dir/");
        let _ = complete("git 日本語");
    }

    #[test]
    fn test_split_at_last_slash_ascii() {
        let (dir, pre) = split_at_last_slash("/usr/bin/git");
        assert_eq!(dir, "/usr/bin/");
        assert_eq!(pre, "git");
    }

    #[test]
    fn test_split_at_last_slash_unicode() {
        let (dir, pre) = split_at_last_slash("/tmp/テスト/ファイル");
        assert_eq!(pre, "ファイル");
        assert!(dir.ends_with('/'));
    }

    #[test]
    fn test_no_duplicates_in_output() {
        let result = complete("git s");
        let lines: Vec<&str> = result.lines().collect();
        let mut sorted = lines.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(lines, sorted, "output must be sorted and deduplicated");
    }

    #[test]
    fn test_docker_completions() {
        let result = complete("docker ru");
        assert!(
            result.contains("run"),
            "should complete 'docker ru' → 'run'"
        );
    }
}
