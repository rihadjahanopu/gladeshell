# 📋 Changelog

All notable changes to **[gladeshell](https://github.com/rihadjahanopu/gladeshell)** are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

[![Changelog Format](https://img.shields.io/badge/Changelog-Keep%20a%20Changelog-a855f7?style=for-the-badge&logo=markdown)](https://keepachangelog.com/)
[![SemVer](https://img.shields.io/badge/SemVer-2.0.0-0ea5e9?style=for-the-badge&logo=semver)](https://semver.org/)
[![Pure Rust](https://img.shields.io/badge/Rust-2024%20Edition-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)

---

## 🚀 [1.1.0] — 2026-10-02

### ⚡ Added & Core Improvements

- 🐚 **Native Shell Auto-Completions Engine (`gladeshell completions <shell>`)**:
  - Integrated `clap_complete` to generate native auto-completions for **Bash, Zsh, Fish, PowerShell (`pwsh`), and Elvish**.
  - Wired automatic completion loading directly inside `gladeshell init <shell>` (e.g. `eval "$(gladeshell init zsh)"`) for zero-configuration, instant Tab completion.
- ⚡ **Pure Rust Engine Architecture**:
  - Rebuilt the entire core CLI toolkit in 100% pure Rust with memory safety guarantees (`#![deny(unsafe_code)]`).
  - Replaced legacy subshell scripts with a sub-millisecond execution prompt engine (`< 1ms` prompt render time).
- 🖥️ **Native IDE Settings Installers (`zed-setup` & `code-setup`)**:
  - Added single-command bulletproof installers for Zed IDE and VS Code settings across Native, Flatpak, Snap, and Windows AppData locations.
- 🗜️ **Level 1 Fast Parallel Multi-Core Compressor (`cmp` / `compress`)**:
  - Implemented multi-core archive compressor supporting ZIP, 7z, TAR.GZ, TAR.XZ, TAR.BZ2, and TAR formats.
  - Defaults to **Level 1 (Max Speed)** compression and incorporates smart pre-compressed media detection (`is_media_or_compressed_file`) to bypass redundant CPU work.
- 🔐 **AES-256 PBKDF2 Multi-Vault Security Suite (`vault`)**:
  - Hardened memory-guarded directory vault with decoy panic password protection and auto-relock timers.

### 🛡️ CI/CD & Build Pipeline Hardening

- 🤖 **Automated GitHub Release Workflow (`.github/workflows/release.yml`)**:
  - **Version Bump Detection**: Automatically detects version changes in `Cargo.toml` on pushes to `main` and creates/pushes annotated Git tags (`vX.Y.Z`).
  - **Automated Changelog Extractor**: Automatically parses and attaches version notes from `CHANGELOG.md` directly to GitHub Releases.
  - **7-Platform Release Matrix**: Cross-compiles standalone release binaries for Linux (`x86_64` GNU/MUSL, `aarch64` ARM64), macOS (`x86_64` Intel, `aarch64` M-Series), and Windows (`x86_64`, `aarch64`).
  - **Binary Size Stripping**: Automatically strips debug symbols (`strip`) on Unix build runners to maintain sub-4MB executable sizes.
  - **Cross-Platform SHA256 Checksums**: Automatically generates `SHA256SUMS` manifests with PowerShell fallback support on Windows.
  - **Crate Publish Verification**: Performs `cargo publish --dry-run` checks prior to release tagging.

### 🔧 Developer Workflows & Developer Tooling

- **VS Code Workspace Integration**: Added workspace `.vscode/settings.json` (live `clippy` on save), `.vscode/launch.json` (CodeLLDB debugger configuration), and `.vscode/tasks.json` (cargo task shortcuts).
- **Makefile Workflows**: Added `make check`, `make clippy`, `make fmt`, `make test`, `make bench`, and `make install` Makefile targets.
- **Git Hygiene Hooks**: Added `.githooks/pre-commit` (formatting, clippy, web assets auto-sync) and `.githooks/pre-push` (workspace test verification).

### 🐛 Fixed & Cleaned

- **Web Assets Synchronization**: Streamlined `web/` deploy directory to serve static mirror installers (`install.sh`, `install.ps1`) directly at root level.
- **Cargo Lockfile Tracking**: Tracked `Cargo.lock` in version control to ensure 100% reproducible binary builds across all target matrixes.

---

[1.1.0]: https://github.com/rihadjahanopu/gladeshell/releases/tag/v1.1.0
