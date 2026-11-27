# 📖 gladeshell Official Technical Wiki

<div align="center">

```
   ██████╗ ██╗      █████╗ ██████╗ ███████╗███████╗██╗  ██╗███████╗██╗     ██╗     
  ██╔════╝ ██║     ██╔══██╗██╔══██╗██╔════╝██╔════╝██║  ██║██╔════╝██║     ██║     
  ██║  ███╗██║     ███████║██║  ██║█████╗  ███████╗███████║█████╗  ██║     ██║     
  ██║   ██║██║     ██╔══██║██║  ██║██╔══╝  ╚════██║██║  ██║██╔══╝  ██║     ██║     
  ╚█████╔╝ ███████╗██║  ██║██████╔╝███████╗███████║██║  ██║███████╗███████╗███████╗
   ╚════╝  ╚══════╝╚═╝  ╚═╝╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝
```

### ⚡ Comprehensive Developer & Technical Guide for gladeshell

_Pure Rust Architecture • Hyper-Optimized CLI Engine • Interactive Ratatui TUIs • Complete Command API_

<br>

[![MIT License](https://img.shields.io/badge/License-MIT-a855f7?style=for-the-badge&logo=opensourceinitiative&logoColor=white)](https://opensource.org/licenses/MIT)
[![Rust Engine](https://img.shields.io/badge/Engine-Pure%20Rust%20100%25-22c55e?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-0ea5e9?style=for-the-badge&logo=linux&logoColor=white)](#)
[![Version](https://img.shields.io/badge/Version-1.2.18-ec4899?style=for-the-badge)](#)
[![Website](https://img.shields.io/badge/Website-gladeshell.netlify.app-22d3ee?style=for-the-badge&logo=netlify&logoColor=white)](https://gladeshell.netlify.app)

</div>

---

## 📌 Table of Contents

- [1. Wiki Overview \& Vision](#1-wiki-overview--vision)
- [2. Architectural Design \& Core Mechanics](#2-architectural-design--core-mechanics)
  - [2.1 High-Level Architecture](#21-high-level-architecture)
  - [2.2 Execution Order \& Lifecycle](#22-execution-order--lifecycle)
  - [2.3 Memory Safety \& Bound Guarantees](#23-memory-safety--bound-guarantees)
- [3. Installation \& Environment Setup](#3-installation--environment-setup)
  - [3.1 Prerequisites](#31-prerequisites)
  - [3.2 Universal One-Line Installer](#32-universal-one-line-installer)
  - [3.3 Cargo Build from Source](#33-cargo-build-from-source)
  - [3.4 Clean Uninstallation Protocol](#34-clean-uninstallation-protocol)
- [4. Smart Prompt \& TUI Theme Engine](#4-smart-prompt--tui-theme-engine)
  - [4.1 Two-Line Layout Structure](#41-two-line-layout-structure)
  - [4.2 Dynamic Context Engine](#42-dynamic-context-engine)
  - [4.3 Real-time System Metrics \& Telemetry](#43-real-time-system-metrics--telemetry)
  - [4.4 55-Theme Switcher Engine](#44-55-theme-switcher-engine)
- [5. Complete Command \& Interactive TUI Suite](#5-complete-command--interactive-tui-suite)
  - [5.1 Multi-Threaded Parallel Compressor (`compressor`)](#51-multi-threaded-parallel-compressor-compressor)
  - [5.2 High-Speed Extractor (`extractor`)](#52-high-speed-extractor-extractor)
  - [5.3 FFmedia 24-in-1 Multimedia Suite (`ffmedia`)](#53-ffmedia-24-in-1-multimedia-suite-ffmedia)
  - [5.4 Interactive Task Manager (`todo`)](#54-interactive-task-manager-todo)
  - [5.5 Terminal Notes Editor (`notes`)](#55-terminal-notes-editor-notes)
  - [5.6 AES-256 Decoy Security Vault (`vault`)](#56-aes-256-decoy-security-vault-vault)
  - [5.7 Process & System Monitor (`process_manager` / `sysmon`)](#57-process--system-monitor-process_manager--sysmon)
  - [5.8 Directory Traversal & Navigation](#58-directory-traversal--navigation)
  - [5.9 Git Version Control Matrix](#59-git-version-control-matrix)
- [6. Multi-Platform & Shell Parity](#6-multi-platform--shell-parity)
  - [6.1 Linux, macOS & Windows Parity](#61-linux-macos--windows-parity)
  - [6.2 Brush Shell (`brush.sh`) & Auto Config](#62-brush-shell-brushsh--auto-config)
- [7. Terminal Shell Integration (`OSC 133` & `OSC 7`)](#7-terminal-shell-integration-osc-133--osc-7)
  - [7.1 Architecture & Multi-Tab Daemon Isolation](#71-architecture--multi-tab-daemon-isolation)
  - [7.2 Supported Protocols & Modes](#72-supported-protocols--modes)
  - [7.3 Conflict Prevention & Edge-Case Guards](#73-conflict-prevention--edge-case-guards)
- [8. Configuration Files & Environment Overrides](#8-configuration-files--environment-overrides)
- [9. Editor & VS Code Integration](#9-editor--vs-code-integration)
- [10. Developer Guide & Contribution Protocol](#10-developer-guide--contribution-protocol)

---

## 1. Wiki Overview & Vision

**gladeshell** is a production-ready, zero-dependency, hyper-optimized pure Rust binary CLI suite engineered specifically for modern full-stack web developers, DevOps engineers, and system administrators.

Built in safe Rust with `#![deny(unsafe_code)]` constraints, **gladeshell** eliminates external subshell execution delays, delivering instantaneous prompt rendering and multi-threaded parallel performance across Linux, macOS, and Windows.

> [!NOTE]
> **Core Technical Promise**: Pure Rust compilation. Zero background subshell latency. Non-destructive shell integration via atomic backup.

---

## 2. Architectural Design & Core Mechanics

### 2.1 High-Level Architecture

The gladeshell engine consists of four primary decoupled layers:

```mermaid
flowchart TD
    subgraph User Interaction
        CLI[User Terminal / Shell Input]
    end

    subgraph Binary Entrypoint & CLI Subcommands
        MAIN[gladeshell Binary Entrypoint]
        SETUP[gladeshell setup / uninstall]
    end

    subgraph Core Engine Modules
        PROMPT[Prompt Telemetry Engine]
        THEMES[55 Ratatui Theme Switcher]
        ALIASES[aliases.toml Engine]
        SHELLINT[Shell Integration OSC 133 / 7]
    end

    subgraph Interactive TUI Suite
        COMP[compressor - Level 1 Parallel ZIP/7z]
        EXT[extractor - Multi-format Extractor]
        FFM[ffmedia - 24-in-1 Media Suite]
        VAULT[vault - AES-256 Decoy Guard]
        NOTES[notes & todo - 2D Viewport Editors]
    end

    CLI --> MAIN
    MAIN --> SETUP
    MAIN --> PROMPT
    MAIN --> THEMES
    MAIN --> ALIASES
    MAIN --> COMP
    MAIN --> EXT
    MAIN --> FFM
    MAIN --> VAULT
    MAIN --> NOTES
```

### 2.2 Memory Safety & Bound Guarantees

gladeshell enforces strict memory safety guarantees:

- **Zero Unsafe Code**: Compiles with `#![deny(unsafe_code)]` across core modules.
- **Bounded Buffers**: Dynamic UI containers calculate exact bounds without hardcoded offsets.
- **Parallel I/O**: Multi-core archive compression utilizes Rayon thread pools and 2MB high-throughput buffered streams.

---

## 3. Installation & Environment Setup

### 3.1 Universal One-Line Installer

Automated installer script auto-detects OS (Linux/macOS/Windows) and CPU architecture (`x86_64` / `aarch64` ARM64):

```bash
curl -fsSL https://gladeshell.netlify.app/install.sh | bash
```

### 3.2 Cargo Build from Source

For Rust developers building directly from source:

```bash
git clone https://github.com/rihadjahanopu/gladeshell.git
cd gladeshell
cargo build --release
make install
```

### 3.3 Clean Uninstallation Protocol

`gladeshell` provides 100% non-destructive uninstallation:

```bash
gladeshell uninstall
# OR via installer script
./install.sh --uninstall
```

---

## 4. Smart Prompt & TUI Theme Engine

### 4.1 Two-Line Layout Structure

The gladeshell prompt renders real-time telemetry with sub-millisecond latency:

```text
🌐 web-project 📂 42M [🌿 main ❗] 🌡️ 48°C 💽 120G free ⚖️ 0.45 ⏱️ 3s
🟢 v20.11.0 │ 📦 10.2.4 │ 🥐 1.1.0 │ 🐧 6.8.0 │ 📅 Sep 30 │ 🧠 4.1G/16G │ 🔋98%
❯❯❯
```

### 4.2 55-Theme Switcher Engine

Select from 55 interactive Ratatui prompt themes (Catppuccin, Nord, Cyberpunk, Tokyo Night, Dracula, Rose Pine) with live terminal preview:

```bash
gladeshell theme
```

---

## 5. Complete Command & Interactive TUI Suite

### 5.1 Multi-Threaded Parallel Compressor (`compressor`)

`gladeshell compressor` runs hyper-optimized parallel multi-core compression:

- **Default Level**: Level 1 (Fast - Max Speed)
- **Supported Formats**: ZIP, 7z, TAR.GZ, TAR.XZ, TAR.BZ2, TAR
- **Smart Store**: Automatically detects pre-compressed media (MP4, MKV, MP3, PNG) to prevent CPU waste.

### 5.2 High-Speed Extractor (`extractor`)

`gladeshell extractor <file>` auto-detects archive signatures and unpacks archives at maximum I/O speed.

### 5.3 FFmedia 24-in-1 Multimedia Suite (`ffmedia`)

24-in-1 interactive media toolkit for video compression, audio extraction, GIF generation, and EXIF metadata stripping.

### 5.4 Interactive Task Manager (`todo`)

Ratatui TUI task manager supporting task creation, priority tagging, filtering, and JSON persistence.

### 5.5 Terminal Notes Editor (`notes`)

Fast terminal note taker featuring 2D viewport navigation, multi-line paste support, and full-text search.

### 5.6 AES-256 Decoy Security Vault (`vault`)

AES-256 encrypted directory vault with decoy password triggers, RAM execution guards, and panic wipe capability.

---

## 6. Multi-Platform & Shell Parity

### 6.1 Linux, macOS & Windows Parity

GladeShell generates zero-fork bootstrap scripts (`gladeshell init <shell>`) and caches them atomically in `~/.gladeshell/cache/init.<ext>` across all **5 supported shells**:

- **Zsh (`5.x+`):** Full ZLE widgets (`autosuggest` ghost text + `highlight`), `add-zsh-hook` (`precmd`/`preexec`/`chpwd`), Unix socket daemon client, and transient prompt.
- **Bash (`4.4+`):** `PROMPT_COMMAND`, `PS0`, `DEBUG` trap timing, lazy system completions, and UTF-8 prompt rendering.
- **Brush (`0.5+`):** Native Rust shell support with `$EPOCHREALTIME` sub-millisecond timing, Reedline `bind -x` widgets (`Ctrl+R`, `Alt+C`), lazy completions, and POSIX function adapters for operator aliases (`rel()`, `path()`, `brushrc`, etc.).
- **Fish (`3.x+`):** `fish_prompt`, `$CMD_DURATION`, `--on-event fish_preexec`, `--on-variable PWD` hooks, and transient prompt repainting.
- **PowerShell (`7+` / `5.1`):** `Prompt` function, `PSReadLine` colors & Enter transient handler, `%USERPROFILE%` fallbacks, and Windows VT console setup.

### 6.2 Brush Shell (`brush.sh`) & Auto Config

GladeShell configures Brush automatically during `gladeshell setup` and installer runs:

- Injects the idempotent init block into `~/.brushrc` (`# >>> glade-brush >>>`).
- Automatically ensures `[ui]\nsyntax-highlighting = true` inside Brush's TOML config (`${XDG_CONFIG_HOME:-$HOME/.config}/brush/config.toml` on Linux/macOS or `%APPDATA%\brush\config.toml` on Windows) without overwriting explicit user preferences or stripping comments.
- Adds cross-shell `brushrc` editor shortcuts (`bash`, `brush`, `zsh`, `fish`) and shell-local `rel` reload functions (`eval "$(gladeshell init brush)"`).

---

## 7. Terminal Shell Integration (`OSC 133` & `OSC 7`)

### 7.1 Architecture & Multi-Tab Daemon Isolation

Because GladeShell's Unix IPC prompt daemon (`src/daemon/mod.rs`) is shared across multiple concurrent terminal tabs (e.g., an IDE terminal, a standalone GPU terminal, and an SSH/TTY session), inspecting terminal environment variables inside the daemon would cross-contaminate sessions.

Instead, **terminal probing and OSC emission execute per shell session** at startup inside generated init hooks (`_fb_init_terminal_integration` in `src/init/shared.rs`) using pure shell builtins (**0 extra subshell forks per prompt**), backed by `src/core/shell_integration.rs`.

### 7.2 Supported Protocols & Modes

```bash
gladeshell integration <off|auto|on|status>
# Aliases: gladeshell shell-integration ..., gladeshell osc ...
```

| Mode | Behavior |
| ---- | -------- |
| `off` *(default)* | Emits zero `OSC 133` / `OSC 7` bytes — 100% safe for legacy TTYs and older emulators |
| `auto` | Activates automatically in verified modern terminals (`WT_SESSION`, Kitty, Ghostty, WezTerm, Konsole, Alacritty, iTerm2, macOS Terminal, Warp, Hyper, Tabby, Rio, Contour, `VTE_VERSION >= 5000`) |
| `on` | Force-enables in all interactive TTY sessions (still suppresses if `TERM=dumb` or non-TTY stdout) |

* **`OSC 133;A` / `133;B` / `133;C` / `133;D[;<exit>]`:** Marks prompt start/end, command execution start, and exit code boundaries across Zsh, Bash, Brush, Fish, and PowerShell (wrapped cleanly inside `%{...%}` in Zsh and `\[...\]` in Bash/Brush `PS1` so cursor column calculations stay exact).
* **`OSC 7` (`file://<host>/<uri>`):** Percent-encodes `$PWD` cleanly and emits only when the working directory changes.
* **`OSC 777` / `OSC 99`:** Built-in desktop & terminal completion notifications in `_fb_notify` (`>= GLADESHELL_NOTIFY_THRESHOLD`).

### 7.3 Conflict Prevention & Edge-Case Guards

- **Empty `<Enter>` accuracy:** Emits bare `\e]133;D\a` (without an exit status code) when no command ran (using `preexec` / `DEBUG` trap on Zsh/Bash/Fish, `PSReadLine` buffer check on PowerShell, and `HISTCMD` guard on Brush).
- **Double-mark suppression (`auto` mode):** Yields cleanly when `VSCODE_SHELL_INTEGRATION=1`, Brush's `config.toml` `experimental.terminal-shell-integration = true`, `Fish 4.*`, GNU `screen`, or `TERM=dumb` is detected.
- **Transient prompt boundary preservation:** Compact transient prompts (`❯ `) remain framed with `133;A` ... `133;B` in Zsh, Fish, and PowerShell.

---

## 8. Configuration Files & Environment Overrides

| Purpose | Config File Path | Environment Variable Fast-Path |
| ------- | ---------------- | ------------------------------ |
| Active Theme | `~/.config/gladeshell/theme` | `GLADESHELL_THEME` |
| Theme Colors | `~/.config/gladeshell/theme_overrides.toml` | — |
| Shell Integration | `~/.config/gladeshell/shell_integration` | `GLADESHELL_TERMINAL_INTEGRATION` (`off`\|`auto`\|`on`) |
| Transient Prompt | Enabled by default | `GLADESHELL_TRANSIENT=0` (disable) |
| Command Alerts | `10000` ms threshold | `GLADESHELL_NOTIFY_THRESHOLD` (`0` disables) |
| Brush UI Config | `${XDG_CONFIG_HOME:-~/.config}/brush/config.toml` (`%APPDATA%\brush\config.toml` on Windows) | `GLADESHELL_BRUSH_KEYBINDS=0` |

---

## 9. Editor & VS Code Integration

Pre-configured VS Code settings (`.vscode/settings.json`) enable `rust-analyzer` live linting:

- **`clippy` on Save**: Automatically runs `cargo clippy` on file save.
- **Debugger**: Pre-configured CodeLLDB launcher (`.vscode/launch.json`).
- **Tasks**: `Ctrl+Shift+B` shortcuts for `cargo build`, `cargo test`, `cargo clippy`, and `cargo fmt`.

---

## 10. Developer Guide & Contribution Protocol

Contributors must verify code before submitting pull requests:

```bash
make check      # Run cargo check across all targets
make clippy     # Run Clippy lints (-D warnings)
make fmt        # Format Rust source code
make test       # Execute unit test suite
```

---

<div align="center">

**Maintained with ❤️ by Rihad Jahan Opu**

[Website](https://gladeshell.netlify.app) • [GitHub Repository](https://github.com/rihadjahanopu/gladeshell) • [Report Issue](https://github.com/rihadjahanopu/gladeshell/issues)

</div>
