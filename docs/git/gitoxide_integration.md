# 🌿 Gitoxide Pure-Rust Engine

- **Primary Source:** [`src/git/mod.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/git/mod.rs)
- **Engine Library:** `gix` (gitoxide v0.88)
- **Feature Gate:** `#[cfg(feature = "prompt-engine")]`

---

## 1. 🎯 Architectural Role

Spawning `git status --porcelain` or `git branch --show-current` on every prompt render spawns an external process that reads the git index and traverses the filesystem. In large codebases, this can consume 100ms+.

Gladeshell integrates **Gitoxide (`gix`)**, the pure-Rust re-implementation of Git. Gitoxide operates entirely in-process by reading the `.git` directory directly from memory:
- **Zero Process Forks:** Eliminates process spawning overhead entirely.
- **Pure Rust:** No C dependencies, no `libgit2` compilation hassles, full cross-platform compatibility.
- **Fast Dirty-Check:** Uses index stat cache comparison (`gix::status::Platform`) instead of full disk hashing.

---

## 2. 📊 Metrics Extracted In-Process

`src/git/mod.rs` collects:
1. **Branch / Tag / Detached SHA:** Extracted from `.git/HEAD` and `.git/refs`.
2. **Dirty Worktree:** Number of modified, deleted, or unstaged files.
3. **Staged Changes:** Number of files added to index.
4. **Untracked Files:** Fast lookahead in root directory.
5. **Ahead / Behind:** Commits ahead/behind upstream tracking branch (`refs/remotes/origin/...`).
6. **Stash Count:** Reads `.git/logs/refs/stash`.

---

## 3. ⏱️ Performance Budget & Bailout

In gigantic repositories (e.g., Linux kernel or Chromium monorepo), reading the full worktree status can still take time. Gladeshell implements an **Early Bailout Guard**:
- If stat comparison exceeds **25ms**, the git inspector stops traversal and returns a partial status with an asynchronous flag.
- The prompt displays the branch name immediately without freezing the shell.
