# ADR 0002: Pure-Rust Gitoxide (`gix`) Integration over Shelling Out to Git

- **Status:** Accepted
- **Deciders:** Core Engineering Team
- **Date:** 2026-10-02

---

## 1. 🎯 Context
Displaying Git repository status (active branch, dirty worktree, staged files, ahead/behind commits) is a signature requirement for modern terminal prompts.
Traditionally, tools either shell out to `git status --porcelain` or link against the C library `libgit2`.
- Shelling out to `git` creates 2–4 separate process spawns per prompt.
- Linking to `libgit2` requires C compilers (GCC/Clang/MSVC) and CMake, breaking pure-Rust cross-compilation and creating OpenSSL linkage hazards.

---

## 2. 💡 Decision
We chose **Gitoxide (`gix`)**, a 100% native Rust implementation of Git protocols and file formats.
Gladeshell integrates `gix` with features `revision`, `status`, and `sha1`, bypassing the `git` binary completely.

---

## 3. 🚀 Consequences

### Positive:
- **No C Dependencies:** Compiles seamlessly on Linux, macOS, and Windows with zero native toolchain dependencies.
- **In-Memory Stat Cache:** Reads `.git/index` and compares stat metadata without spawning processes, executing in sub-5ms.
- **Configurable Bailout:** In gigantic codebases, Gladeshell can cleanly abort stat traversal if it exceeds a 25ms threshold.

### Trade-offs:
- Slightly increases final binary compilation time due to `gix` macro expansions. Mitigated by using pre-compiled cache in GitHub Actions CI (`Swatinem/rust-cache`).
