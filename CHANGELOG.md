# 📋 Changelog

All notable changes to **[gladeshell](https://github.com/rihadjahanopu/gladeshell)** are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

[![Changelog Format](https://img.shields.io/badge/Changelog-Keep%20a%20Changelog-a855f7?style=for-the-badge&logo=markdown)](https://keepachangelog.com/)
[![SemVer](https://img.shields.io/badge/SemVer-2.0.0-0ea5e9?style=for-the-badge&logo=semver)](https://semver.org/)
[![Pure Rust](https://img.shields.io/badge/Rust-2024%20Edition-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)

## 🚀 [1.2.14] — 2026-10-22

### 🛠️ Editor Priority & Navigation Fixes
- 💻 **`Ctrl+O` Priority Order**:
  - Reordered editor launching across all shells (Bash, Zsh, Fish, PowerShell) to prioritize **VS Code** (`code`) before `cursor`, `zed`, `codium`, and `nvim`.
- 🗂️ **Zed Dedicated Workspace Support**:
  - Added `-n` flag when launching Zed (`zed -n "$open_path"`), ensuring Zed opens the selected folder in its own dedicated workspace instead of attaching to the parent Git root.

### 🎨 Prompt Engine Enhancements
- 🎲 **Universal Random Prompt Emoji**:
  - Removed keyword pinning (`node`, `bun`, `web`, `py`) from prompt emoji generator so every folder cycles dynamically through `RANDOM_EMOJIS` on each prompt render.

### ⚙️ Developer Tools & Project Setup
- ⚡ **Code-Runner Integration**:
  - Added customized `.vscode/settings.json` Code-Runner configuration to `makecpp` (with `bin/main` output and `-Iinclude`) and `makerust` (`cargo run`).
  - Pre-created `bin/` directory and added `src/main` to `.gitignore` and Makefile clean rules.

### 🛡️ Security
- 🔒 **Dependency Security Patch**:
  - Upgraded `rustls-webpki` to `0.103.16` (patching `RUSTSEC-2026-0382`).
  - Upgraded `webpki-roots` to `1.0.10` (patching `RUSTSEC-2026-0381`).

## 🚀 [1.2.13] — 2026-10-20

### 📂 `cf` (Fuzzy CD) Overhaul & Folder Diving
- 🔄 **In-Session Folder Diving Restored**:
  - `Enter` on any directory dives into that folder in the same TUI session without exiting.
  - `Enter` on `..` backtracks up the directory hierarchy.
  - `Enter` on `.` confirms `cd` to the shell.
  - `Ctrl+Enter` (along with `Shift+Enter`, `Alt+Enter`, `Ctrl+D`) executes immediate shell `cd` into the selected directory.
- 📐 **Rebalanced 65% / 35% Layout**:
  - Widened list view to 65% and preview panel to 35% for cleaner typography and comfortable previewing.
- 🏷️ **Compact List View & Right-Aligned Badges**:
  - List displays compact `parent/folder` to prevent clutter and text clipping, while the bottom `📍 Full Path` status bar shows the full path.

### 🔍 `ff` (File Find) Compact Display & Hardening
- 📄 **Clean `parent/filename` Rendering**:
  - Files render with compact parent folder hierarchy, neatly aligned size badges, and complete path shown in bottom details pane.

### 🛡️ Production Hardening (10k Users Scale)
- 🔒 **RAII `TerminalGuard`**:
  - Implemented drop guards in both `cf` and `ff` ensuring alternate screen exit, raw mode disable, mouse capture disable, and cursor restore occur reliably even during panics.
- 🚫 **Panic-Free Resizing & Index Safety**:
  - Added zero-area screen guards (`area.width < 10 || area.height < 5`) and safe `.get(index)` array indexing across search views.
- 🧵 **Mutex Poisoning Resilience**:
  - Safe lock acquisition in multi-threaded file search preventing crashes if background threads abort.
- 🐚 **POSIX Shell Hardening**:
  - Hardened shell wrappers with `cd -- "$target"` to safely handle directory names starting with `-`.

### 🪟 Cross-Platform & Windows CI Reliability
- 🛠️ **Windows-Specific Clippy & Lint Fixes**:
  - Fixed Windows console raw mode handles in `input_parser.rs`.
  - Cleaned up native file opener syntax in `file_find.rs` and Windows symlink handling in `vault.rs`.
- 🌐 **Cross-Platform Virtual Path Handling**:
  - Normalized path separators (`\` to `/`) in system root exclusion logic so virtual paths (`/proc`, `/sys`, `/dev`) and Windows paths (`$recycle.bin`, `system volume information`, `winsxs`) pass across all OS runners.

---

## 🚀 [1.2.12] — 2026-10-19

### 🦀 Modern Project Generators
- 🆕 **`makerust` Tool**:
  - Full-featured Rust project generator supporting `bin` (Application), `lib` (Library Crate), `cli` (Modular CLI with clap/anyhow), and `wasm` (WebAssembly with wasm-bindgen).
  - Generates `Cargo.toml`, multi-target `Makefile` (`run`, `test`, `check`, `clippy`, `release`), `README.md`, `.gitignore`, and VS Code / rust-analyzer integration.
  - Integrated into CLI (`gladeshell makerust`), `keep` menu, and shell aliases.
- 🐛 **`makecpp` Makefile Fix**:
  - Replaced immediate variable evaluation (`:=`) with deferred evaluation (`=`) for `RUN_CMD` and `RM_CMD`.
  - Fixes the `make: .//: Permission denied (Error 127)` bug on both Linux and Windows.

### 🔍 Dynamic Toolchain & Version Detection
- ⚡ **Subdirectory Project Root Walk-Up**:
  - `detect_project_toolchain` now traverses parent directories to identify project root (Node.js, Rust, Bun, Python, Go, PHP, Ruby, etc.) even when navigating deep nested subdirectories.
- 📁 **`.nvmrc` & Toolchain File Detection**:
  - Added support for reading specific required versions directly from `.nvmrc` or project manifests.
- 🔄 **Multi-Version Manager Fallback & SemVer Resolution**:
  - Added comprehensive fallback discovery across FNM, NVM, Volta, ASDF, Mise, PNPM, Homebrew, and Windows paths with proper SemVer numerical sorting.

### 🪟 Windows CI & Cross-Platform Reliability
- 🛡️ **Zero-Warning Windows Target Compilation**:
  - Guarded Unix-only daemon client/server imports behind `#[cfg(unix)]`.
  - Scoped non-Linux `libc_statfs` and `raw_statfs` stubs to `#[cfg(all(unix, not(target_os = "linux")))]`.
  - Added `#[allow(unused_variables)]` on cross-platform abstractions in `auto_ls` and `file_find`.
- 🧪 **Test Isolation & Sandbox Safety**:
  - Protected test/cargo/rust temporary folders (`fb_test*`, `gladeshell*`, `cargo*`, `rust*`) in `clean_temp_dirs` from being deleted during execution.
  - Isolated `system_clean` unit tests to private sandbox directories to prevent concurrent test race conditions.
  - Made `git init` calls in project generators resilient with automatic partial directory cleanup on restricted environments.

### 📝 Zed IDE Integration
- 🧩 **`make` Extension Support**:
  - Added `make` extension to `auto_install_extensions` in Zed setup for syntax highlighting and Makefile support.
- ⚙️ **`clangd` Language Server Configuration**:
  - Configured `clangd` with `--query-driver` argument in `lsp` settings to seamlessly resolve compiler-builtin headers (`stddef.h`, `stdio.h`) without missing header diagnostics.

---

## 🚀 [1.2.11] — 2026-10-17

### ⚡ World-Class Performance & Latency Optimization
- 🏎️ **P50 Latency < 2ms via Socket Daemon IPC**:
  - Layer-3 packed binary IPC protocol for instant sub-millisecond prompt rendering.
  - In-memory background cache refresh for Git status and system metrics.
  - Zero-allocation string formatting optimizations across core prompt pipelines.

---

## 🚀 [1.2.10] — 2026-10-15

### 🔧 CI & Cross-Platform Stability
- ⚡ **Fixed CI benchmark verification across all platforms**:
  - Replaced `cargo bench --workspace --no-run` with `cargo check --benches --all-features`.
  - Eliminates Cargo panic strategy mismatch (`abort` in release profile vs `unwind` in criterion runner, rust-lang/cargo#4741).
  - Eliminates `cdylib` and `rlib` dead-code symbol collision when building workspace targets under release profile (rust-lang/cargo#6313).
- 🛡️ **MSRV updated to 1.95**:
  - Aligned with `sysinfo` and `is_multiple_of` stability requirements.
- 🔒 **Security hardening**:
  - Patched `rustls` 0.23.46 (resolving 4 CVEs).
  - Pinned `Swatinem/rust-cache@v2.8.0` for deterministic caching.

## 🚀 [1.2.9] — 2026-10-15

### 🛡️ Bash Shell Fixes
- 🐛 **Removed broken `clear()` shell wrapper**:
  - Eliminated `clear()` function override and `bind -x '"\C-l": clear'` from bash init.
  - `bind -x` does not trigger `PROMPT_COMMAND`, causing `_fb_skip_render` to stick at `1`
    and the next prompt after `Ctrl+L` to render stale cached PS1.
  - Emoji change on `clear` is intentional behavior — matches zsh parity.
- 🧹 **Removed unused variables**: `_fb_cached_ps1`, `_fb_cached_pwd`, `_fb_skip_render`.

### 📋 `keep` Command Center — Full Parity with Shell Script
- ✅ **BUN**: Added `html` alias (`bun run index.html`).
- ✅ **GIT**: Added `gd` (view diff) and `gst / gsta / gpop` (stash / apply / pop).
- ✅ **SYSTEM**: Added `setuppc`, `rel`, `myip / iploc`, `ports`, `serve`.
- ✅ **ADVANCED TUIs**: Added `cf` with full sub-key bindings (`ENTER`, `CTRL+O`, `CTRL+Y`, `CTRL+H`),
  `gbranch` sub-keys (`ENTER`, `CTRL+D`, `CTRL+R`, `CTRL+M`),
  `fkill` sub-keys (`Tab`, `ENTER`, `CTRL+X`, `CTRL+R`), and `fcd`.
- 🆕 **New category — UTILITY TOOLS**: `ex`, `ff`, `gen`, `h`, `c / cls`, `v`, `pg`.
- 🆕 **New category — DOCKER & CONTAINERS**: 39 docker commands
  (`dman`, `dstats`, `dps`, `dbuild`, `dclean`, `dstart`, `ddisable`, …).
- 🆕 **New category — POSTGRESQL DATABASE**: 17 commands
  (`pgstart / pgstop`, `pgl`, `pgcreate`, `pgdump`, `pgrestore`, …).
- 🆕 **New category — PRISMA ORM**: 14 commands
  (`np / bp`, `npmd`, `npmdn`, `npdp`, `npf`, `npv`, …).

### 🛡️ GitHub Actions — Production-Grade CI Hardening
- 🚨 **Security audit now BLOCKING**: Removed `continue-on-error: true` from `rustsec/audit-check`.
- 🆕 **MSRV job**: Verify compilation on Rust **1.80** minimum supported version.
- 🆕 **`--no-default-features` check**: Ensures crate compiles without optional features.
- 🆕 **Benchmark compile check**: `cargo bench --no-run` verifies all benches always compile.
- 🆕 **Binary smoke-test job** (Linux): Validates `--version`, `--help`, `init bash/zsh/fish`,
  `prompt`, `highlight`, `suggest`, `correct`, and binary size guard.
- 🔗 **Release gate**: `release.yml` now waits for CI Pipeline to complete before building
  cross-platform binaries — prevents broken releases from reaching users.
- 📌 **`Cargo.lock` tracked** in CI path triggers.
- ⚙️ **`CARGO_INCREMENTAL=0`** set for clean, reproducible CI builds.

### ✅ Quality Verification
- 319/319 unit tests pass.
- 7/7 benchmark sanity checks pass.
- `cargo doc` generates with zero warnings.
- `cargo clippy` — 0 warnings.

---

## 🚀 [1.2.8] — 2026-10-14

### 🪟 Windows PowerShell 5.1 PSReadLine & History Range Fixes

- 🛡️ **MaximumHistoryCount Limit**:
  - Bound `$MaximumHistoryCount` to `32767` (`Int16.MaxValue`), preventing PowerShell 5.1's `ValidateSetFailure` error when exceeding the engine limit.
- ⚙️ **PSReadLine Parameter Guarding**:
  - Guarded `-PredictionSource` and `-PredictionViewStyle` parameter calls using `$psrl.Parameters.ContainsKey()`, avoiding binding errors on older PSReadLine versions (< 2.2).
- 🧹 **Installer Multi-Layer Sanitizer**:
  - Extended `install.ps1` in-memory regex sanitizer to automatically patch any legacy init cache files.

---

## 🚀 [1.2.7] — 2026-10-14

### 🪟 Windows PowerShell 5.1 Syntax & Expression Compatibility

- 🛡️ **Expression Splatting Fix**:
  - Eliminated invalid `@args` appending on PowerShell expressions such as `$env:PATH -split ';'`, fixing `Unexpected token '@args' in expression or statement`.
- 🔗 **Pipeline Chain Operator (`&&`) Parser Fix**:
  - Restricted Linux-specific Docker `sudo systemctl` aliases (`dstart`, `doff`, `dstatus`, `denable`, `ddisable`) to Unix shells (`bash`, `zsh`, `fish`).
  - Added native transformer in PowerShell alias generator to convert any `&&` into PowerShell 5.1-compatible `; if ($?) { ... }` blocks.
- 🧹 **Installer Cache Sanitizer**:
  - Added in-memory sanitization in `install.ps1` before writing `init.ps1`, protecting users against older cached binaries.

---

## 🚀 [1.2.6] — 2026-10-14

### 🪟 Windows PowerShell Parity & Bug Fixes

- 🛡️ **Zero-Notepad Native Script Execution (`init.ps1`)**:
  - Renamed PowerShell cache bootstrap script from `init.pwsh` to `init.ps1` so Windows PowerShell 5.1 and PowerShell 7 natively dot-source the file without triggering Windows Shell file-association popups (Notepad).
  - Added automatic removal of obsolete `init.pwsh` cache files during installation.
- 🎨 **Top-Priority Prompt Registration & Cross-Platform Parity**:
  - Moved `function Prompt` and UTF-8 console output setup to the very top of generated `init.ps1`, guaranteeing immediate terminal prompt rendering.
  - Added explicit `--shell pwsh` flag to `gladeshell prompt` invocation, eliminating ZSH escape sequences on Windows.
  - Enabled Windows Console Host Virtual Terminal processing (`VirtualTerminalLevel = 1`) for true-color ANSI support.
- ⚡ **Global Command Aliases & Splatting (`glade`, `theme`, `z`, `cf`)**:
  - Aliases now register directly in `global:` scope with proper argument splatting (`@args`), allowing single-word shortcuts like `glade` and `theme` to work immediately.
  - Hardened `Set-Alias` with `-Option AllScope -Force -ErrorAction SilentlyContinue`.
- 🧩 **Parser Safety & Clean Completions**:
  - Fully qualified AST types in PowerShell completion generator to eliminate `using namespace` parser errors.
  - Switched PowerShell environment variables from `$env:HOME` to `$HOME` and guarded Unix paths with `Test-Path`.

---

## 🚀 [1.2.5] — 2026-10-13

### ⚡ CLI Enhancements

- 🏷️ **Short Version Flag (`-v`)**:
  - Added support for `gladeshell -v` to quickly inspect the installed version (similar to `bun -v` and `node -v`).

---

## 🚀 [1.2.4] — 2026-10-12

### 🛡️ Bulletproof Self-Upgrade Pipeline (`gladeshell upgrade`)

- 🔄 **Atomic Cross-Device Binary Replacement**:
  - Eliminated `Text file busy (os error 26)` (`ETXTBSY`) on Linux by staging directly in target directory and using atomic `rename(2)` instead of `fs::copy`.
  - Added Windows NTFS executable locking support via temporary `.old.exe` rotation with automatic startup cleanup.
- 🌐 **GitHub API Rate-Limit Resilience**:
  - Added transparent fallback to GitHub web releases tag redirect (`302 Location`) when GitHub API encounters HTTP 403 rate limits.
  - Configured resilient HTTP connect (15s), response (30s), and download (180s) timeouts.
- 🐧 **Cross-OS & Libc Compatibility**:
  - Added Alpine Linux / MUSL libc target detection (`gladeshell-linux-*-musl`).
  - Added macOS Universal/Fat Mach-O binary validation and automatic quarantine clearing (`xattr -d com.apple.quarantine`).
  - Implemented safe symlink canonicalization and early directory writability checks (`PermissionDenied`).
- 🛡️ **Zero Orphaned Files Guarantee**:
  - Implemented RAII `TempFileGuard` to guarantee automatic staging cleanup on errors or interruptions.

---

## 🚀 [1.2.1] — 2026-10-09

### 🐛 Bug Fixes & Refinements

- 🎨 **Clean Git WIP Commit Messages (`gwip`)**:
  - Removed emojis/icons from actual `git commit` messages for all commit types (e.g. `fix: message`, `feat: message`).
  - Preserved rich emoji badges and styling inside the interactive Ratatui TUI.

---

## 🚀 [1.2.0] — 2026-10-05

### ⚡ Performance & UX Enhancements

- 🚀 **Zero Subshell Shell Init Loading**:
  - Replaced subshell binary forks (`$(gladeshell socket-path)`) during `.zshrc`/`.bashrc`/`config.fish` load with pure shell variable expansion (`${TMPDIR:-/tmp}/gladeshell_${UID:-${USER:-default}}.sock`).
  - Eliminates cold terminal tab opening black screen delays and lag.
- ⚡ **Instant Cold Startup & Socket Fallback**:
  - Added early `!path.exists()` check in IPC client daemon before attempting OS Unix socket connection.
  - Reduced socket IPC read/write timeouts to 10ms for instant offline fallback.
- 🎨 **Zero-Flicker Prompt Pre-Rendering**:
  - Pre-populates prompt variables prior to initial frame draw in Zsh, Bash, Fish, and PowerShell.
- 🔒 **Subcommands**: Added CLI flags `socket-path`, `suggest`, `highlight`, and `complete`.

---

## 🩹 [1.1.2] — 2026-10-03

### 🛠️ CI/CD Fixes

- 🤖 **aarch64 Linux Build Pipeline — Native Runner Migration**:
  - Replaced `cross` + Docker strategy with a **native `ubuntu-24.04-arm` GitHub Actions runner** for aarch64 Linux builds — eliminates all Docker-in-Docker overhead and C dependency issues.
  - Reverted `arboard` to default features (native ARM runner has all required system C libraries available; `cargo-zigbuild` workaround no longer needed).
  - Removed `Cross.toml` config (no longer required after native runner migration).
  - Replaced `taiki-e/install-action@cross` with direct native build — simpler, faster, and more reliable.
  - Replaced `cargo-zigbuild` (aarch64 cross-compilation workaround) with native `cargo build` on ARM runner.
- 🔒 **Cargo.lock Updated**: Regenerated lockfile to reflect dependency graph after CI tooling changes.

---

## 🩹 [1.1.1] — 2026-10-02

### 🐛 Bug Fixes

- 🔴 **Critical: Shell config wipe on install** (`install.sh` + `cleaner.rs`):
  - Fixed backup order: backup now runs **before** `remove_old_config`, not after. Previously the backup was taken from the already-modified file.
  - `backup_shell_rc` now backs up **all** known shell configs (`~/.bashrc`, `~/.bash_profile`, `~/.zshrc`, `~/.config/fish/config.fish`) since `remove_old_config` touches all of them — previously only the active shell's config was backed up.
  - `remove_old_config` now uses `awk` + atomic temp-file + `mv` instead of in-place `sed`, preventing potential file truncation on some systems.
  - Fixed `clean_specific_rc_file` (Rust): previously **reordered** content by moving the glade block to the end of file, displacing any user content written after the block. Now only **removes** the block, preserving all content in original order.
- 🔴 **Critical: GLIBC version mismatch on older distros** (`install.sh` + `release.yml`):
  - Linux pre-built binaries now compiled on `ubuntu-20.04` (GLIBC 2.31) instead of `ubuntu-latest` (GLIBC 2.39+) — works on any distro from 2020 onward (Debian 11+, Ubuntu 20.04+, Deepin 25, etc.).
  - Added `aarch64-unknown-linux-musl` release target (fully static, zero GLIBC dependency).
  - Renamed musl asset from `gladeshell-linux-musl` → `gladeshell-linux-amd64-musl` for consistency.
  - `install.sh` now tries **musl (static) binary first** on Linux — works on every distro regardless of GLIBC version.
  - Added automatic source-build fallback: if no pre-built binary works, installer auto-installs Rust via `rustup` and builds from source.

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

[1.1.2]: https://github.com/rihadjahanopu/gladeshell/releases/tag/v1.1.2
[1.1.1]: https://github.com/rihadjahanopu/gladeshell/releases/tag/v1.1.1
[1.1.0]: https://github.com/rihadjahanopu/gladeshell/releases/tag/v1.1.0
