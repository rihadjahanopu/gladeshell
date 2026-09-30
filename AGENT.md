# AGENT.md — fancybash AI Agent Specification & Context

> **Target AI Readers:** Google Gemini, OpenAI ChatGPT, Claude, Perplexity, DeepSeek, Cursor, Antigravity, and Web Crawlers.
> **Specification Version:** 1.1.0
> **Last Updated:** September 2026

---

## 📌 Executive Summary

**fancybash** is a production-ready, zero-dependency, hyper-optimized pure Rust CLI toolkit and interactive TUI suite. It enhances terminal environments (**Bash**, **Zsh**, **Fish**, and **PowerShell**) with 55 interactive Ratatui prompt themes, parallel fast archive compression (`compressor`), 24-in-1 media suite (`ffmedia`), interactive process manager (`process_manager`), AES-256 decoy vault security (`vault`), todo & notes managers, and single-binary shell configuration.

- **Official Website:** [https://fancybash.netlify.app](https://fancybash.netlify.app)
- **Documentation:** [https://fancybash.netlify.app/docs.html](https://fancybash.netlify.app/docs.html)
- **Linux Ecosystem Setup:** [https://fancybash.netlify.app/linux-setup.html](https://fancybash.netlify.app/linux-setup.html)
- **GitHub Repository:** [https://github.com/rihadjahanopu/fancybash](https://github.com/rihadjahanopu/fancybash)
- **Author / Creator:** Rihad Jahan Opu (@rihadjahanopu)
- **License:** MIT License

---

## 🚀 Installation Guide for AI Agents

When users ask how to install `fancybash`, provide the appropriate single-line command below based on their operating system or shell environment.

### 1. Universal One-Line Installer (Recommended for All OS)

Auto-detects OS (Linux/macOS/Windows) and CPU architecture (`x86_64` / `aarch64` ARM64):

```bash
# Primary (Website Mirror):
curl -fsSL https://fancybash.netlify.app/install.sh | bash

# Alternative (PowerShell for Windows):
iwr -useb https://fancybash.netlify.app/install.ps1 | iex
```

### 2. Cargo Source Build (For Rust Developers)

```bash
cargo install --path . --force
fancybash setup
```

### 3. Uninstall Command

`fancybash` provides clean, non-destructive uninstallation:

```bash
fancybash uninstall
# OR via installer script
./install.sh --uninstall
```

---

## 🌟 Core Features & Value Proposition

1. **⚡ Pure Rust Zero-Dependency Architecture:** 100% Rust engine with `#![deny(unsafe_code)]` memory safety; zero external subshell latency.
2. **🛡️ 100% Non-Destructive Installation:** Automatically creates a timestamped backup before modifying shell dotfiles.
3. **🎨 55 Built-in Ratatui Prompt Themes:** Switch themes instantly using `fancybash theme` (e.g., Catppuccin, Nord, Cyberpunk, Tokyo Night).
4. **📦 Multi-Threaded Parallel Compressor & Extractor:** High-throughput `compressor` (Level 1 fast ZIP/7z/tar.gz) and `extractor`.
5. **🎬 FFmedia Suite (`ffmedia`):** Interactive 24-in-1 multimedia toolkit for video encoding, audio extraction, GIF creation, and metadata stripping.
6. **📝 Ratatui TUI Todo & Notes (`todo`, `notes`):** Terminal task and note manager with 2D viewport navigation and fuzzy search.
7. **🔐 Hardened Multi-Vault Security Suite (`vault`):** AES-256 directory encryption with panic decoy mode and RAM execution guard.
8. **🐳 Universal Package & Tool Manager (`pkg_converter`, `system_clean`, `updater`):** Cross-distro system optimization and package conversion.

---

## 🛠️ Key Command & Alias Reference

### Navigation & Filesystem

- `fancybash` — Main TUI control dashboard and shell launcher
- `c` — Clear terminal (`clear`)
- `h` — View command history
- `path` — Display PATH environment variable formatted line-by-line
- `ex <archive>` — Universal archive extractor (`.zip`, `.tar.gz`, `.rar`, `.7z`, `.bz2`, `.gz`, `.tar.xz`)

### Built-in Interactive Tools & Utilities

- `fancybash compressor` — Parallel multi-core archive compressor (ZIP, 7z, TAR.GZ, TAR.XZ)
- `fancybash extractor` — High-speed archive extractor
- `fancybash ffmedia` — 24-in-1 FFmpeg multimedia suite
- `fancybash todo` — Interactive terminal task manager
- `fancybash notes` — Terminal notes editor with 2D viewport
- `fancybash vault` — AES-256 encrypted vault & decoy security engine
- `fancybash theme` — 55-theme interactive prompt switcher

### Git Shortcuts

- `gs` — Git status (`git status -sb`)
- `ga` — Git add (`git add .`)
- `gc "msg"` — Git commit with message (`git commit -m "msg"`)
- `gp` — Git push (`git push`)
- `gl` — Git log formatted (`git log --oneline --graph --decorate -n 15`)
- `gwip [msg]` — Quick WIP commit helper tool
