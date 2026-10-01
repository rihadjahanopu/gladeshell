# 📋 Changelog

All notable changes to **gladeshell** are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] — 2026-09-30

### ✨ Added & Improved

- **Pure Rust Engine Architecture (`gladeshell`)**:
  - Rebuilt the entire core CLI toolkit in 100% pure Rust with `#![deny(unsafe_code)]` memory safety guarantees.
  - Replaced legacy subshell scripts with zero-latency sub-millisecond execution engine.
- **Parallel Multi-Core Compressor (`compressor`)**:
  - Implemented parallel archive compressor supporting ZIP, 7z, TAR.GZ, TAR.XZ, TAR.BZ2, and TAR formats.
  - Set **Level 1 (Fast - Max Speed)** as the default compression level for maximum throughput.
  - Added smart media detection mode (`is_media_or_compressed_file`) to store pre-compressed video, audio, and images without wasting CPU cycles.
- **Developer Workflows & Tooling**:
  - **VS Code Suite**: Added `.vscode/settings.json` (live `clippy` on save), `.vscode/launch.json` (CodeLLDB debugging), `.vscode/tasks.json` (cargo build/test shortcuts), and `.vscode/extensions.json`.
  - **Makefile Upgrades**: Added `make check`, `make clippy`, `make fmt`, `make test`, `make bench`, and `make hooks` targets.
  - **Git Hooks**: Updated `.githooks/pre-commit` (security guard, `install.sh`/`install.ps1` auto-sync to `web/`, cargo fmt/clippy/check) and `.githooks/pre-push` (workspace compilation & full unit test suite).
  - **GitHub Actions**: Added 7-target release compilation pipeline (`x86_64` and `aarch64` ARM64 binaries for Linux glibc/musl, macOS Intel/M-Series, and Windows MSVC).

### 🐛 Fixed & Cleaned

- **Web Directory Cleanup**: Streamlined `web/` deploy root to contain `install.sh` and `install.ps1` directly, purging legacy `web/public/` references and `.zwc` cache files.
- **Cargo.lock Tracking**: Removed `Cargo.lock` from `.gitignore` to guarantee 100% reproducible application binary builds across CI/CD and release matrixes.

---

## [2.1.0] — 2026-08-13

### ✨ Added & Improved

- **Todo Manager (`todo`)**:
  - Added 3-tier fallback hierarchy (`gum` → `fzf` → plain `read` prompt) for full interactivity or standalone execution.
- **Notes Manager (`notes`)**:
  - Switched default note file storage format from `.md` to **`.txt`** plain text for universal editor compatibility.
- **Installer Automation**:
  - Dynamically detects and auto-installs `notes` and `todo` dependencies (`glow`, `bat`, `xclip`, `wl-clipboard`, `fzf`, `gum`) across package managers.

---

## [2.0.0] — 2026-06-01

### ✨ Added

- **Docker Swiss Army Knife** — 50+ docker aliases and functions (`dps`, `dsh`, `dwatch`, `dbackup`, `dclean`, `dkill-force`).
- **`uup` Mega Updater** — interactive menu to update OS, Snap, Flatpak, Bun, Node.js in one shot.
- **`uu` Universal Uninstaller** — fuzzy picker across apt/snap/flatpak/AppImage.
- **Zsh & PowerShell parity** — initial multi-shell support.
