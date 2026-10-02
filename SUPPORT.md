# 🆘 Support & Troubleshooting Guide

Welcome to the **[gladeshell](https://github.com/rihadjahanopu/gladeshell)** Support Hub. We are committed to providing a zero-latency, rock-solid developer shell experience. 

If you encounter an issue, have a question, or need setup assistance across Linux, macOS, or Windows, this guide outlines quick self-troubleshooting steps, diagnostic tools, and community support channels.

<div align="center">

[![GitHub Issues](https://img.shields.io/github/issues/rihadjahanopu/gladeshell?style=for-the-badge&color=a855f7&logo=github)](https://github.com/rihadjahanopu/gladeshell/issues)
[![GitHub Discussions](https://img.shields.io/badge/GitHub-Discussions-22c55e?style=for-the-badge&logo=github&logoColor=white)](https://github.com/rihadjahanopu/gladeshell/discussions)
[![Website Portal](https://img.shields.io/badge/Website-gladeshell.netlify.app-22d3ee?style=for-the-badge&logo=netlify&logoColor=white)](https://gladeshell.netlify.app)
[![License MIT](https://img.shields.io/badge/License-MIT-0ea5e9?style=for-the-badge&logo=opensourceinitiative&logoColor=white)](LICENSE)

</div>

---

## 📌 Table of Contents

- [1. Quick Self-Troubleshooting Checklist](#1-quick-self-troubleshooting-checklist)
  - [Icon & Glyph Rendering Issues (Nerd Fonts)](#icon--glyph-rendering-issues-nerd-fonts)
  - [Command Not Found (`gladeshell: command not found`)](#command-not-found-gladeshell-command-not-found)
  - [Shell Auto-Completions Not Triggering](#shell-auto-completions-not-triggering)
  - [IDE Integration Setup (VS Code & Zed)](#ide-integration-setup-vs-code--zed)
  - [Uninstallation & Clean Shell Restoration](#uninstallation--clean-shell-restoration)
- [2. System Diagnostics Command (`gladeshell pc-info`)](#2-system-diagnostics-command-gladeshell-pc-info)
- [3. Documentation & Technical Manuals](#3-documentation--technical-manuals)
- [4. Support Channels & Community Guidelines](#4-support-channels--community-guidelines)
- [5. Submitting a High-Quality Bug Report](#5-submitting-a-high-quality-bug-report)
- [6. Security & Vulnerability Policy](#6-security--vulnerability-policy)

---

## 1. Quick Self-Troubleshooting Checklist

Before creating a support ticket, review these quick resolution steps for common shell and environment configurations:

### Icon & Glyph Rendering Issues (Nerd Fonts)

`gladeshell` uses Nerd Font glyphs (Git branch symbols, OS logos, folder indicators). If symbols appear as broken rectangles `[?]` or missing characters:

1. **Install a Nerd Font**: Download and install a recommended Nerd Font such as [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads) or [JetBrainsMono Nerd Font](https://www.nerdfonts.com/).
2. **Set Terminal Font**: Open your terminal application settings (VS Code, Zed, Alacritty, iTerm2, Windows Terminal, Kitty, WezTerm) and configure the buffer font family to your installed Nerd Font (e.g., `"FiraCode Nerd Font"` or `"JetBrainsMono NF"`).
3. **Rebuild Font Cache** (Linux):
   ```bash
   fc-cache -fv
   ```

---

### Command Not Found (`gladeshell: command not found`)

If running `gladeshell` returns `command not found`:

1. Verify that `~/.cargo/bin` and `~/.local/bin` are in your PATH environment variable:
   ```bash
   export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"
   ```
2. Auto-inject the shell bootstrap integration:
   ```bash
   gladeshell setup
   ```
3. Reload your active terminal subshell:
   ```bash
   exec $SHELL
   ```

---

### Shell Auto-Completions Not Triggering

`gladeshell` includes a native auto-completion engine (`gladeshell completions <shell>`) for **Bash, Zsh, Fish, PowerShell, and Elvish**.

- **Automatic Registration**: `eval "$(gladeshell init <shell>)"` automatically registers in-memory Tab completion hooks.
- **Manual Verification**:
  - **Zsh**: Verify `autoload -Uz compinit && compinit` is active in `~/.zshrc`.
  - **Bash**: Verify `bash-completion` is installed (`sudo apt install bash-completion`).
  - **Fish**: Test completion output via `gladeshell completions fish | source`.

---

### IDE Integration Setup (VS Code & Zed)

To automatically apply bulletproof, high-density developer settings for your preferred IDE:

- **Zed Editor**: Run `zed-setup` (or `gladeshell zed-setup`) to configure Flatpak, Snap, Native, and Windows AppData paths.
- **VS Code**: Run `code-setup` (or `gladeshell code-setup`) to write workspace settings and install recommended developer extensions.

---

### Uninstallation & Clean Shell Restoration

`gladeshell` is designed to be 100% non-destructive. To cleanly remove the binary and restore your original shell config without losing custom settings:

```bash
# Via native binary
gladeshell uninstall

# OR via universal installer script
curl -fsSL https://gladeshell.netlify.app/install.sh | bash -s -- --uninstall
```

---

## 2. System Diagnostics Command (`gladeshell pc-info`)

When requesting support, you can gather complete hardware, OS, and shell environment diagnostics instantly using the built-in diagnostic profiler:

```bash
gladeshell pc-info
```

To export clean JSON diagnostics for bug reports:

```bash
gladeshell pc-info --json > system_report.json
```

---

## 3. Documentation & Technical Manuals

| Manual | Purpose |
| :--- | :--- |
| 📖 **[README.md](README.md)** | Full user guide, feature overview, prompt theme gallery, and command reference. |
| 📋 **[CHANGELOG.md](CHANGELOG.md)** | Detailed version history, release notes, and feature changes per release. |
| 🏗️ **[ARCHITECTURE.txt](ARCHITECTURE.txt)** | Internal system architecture, Rust module layout, and zero-latency prompt design. |
| 🗺️ **[ROADMAP.md](ROADMAP.md)** | Technical milestone progress, upcoming features, and long-term vision. |
| 🔒 **[SECURITY.md](SECURITY.md)** | Security architecture, vault cryptography, zero-telemetry rules, and disclosure policy. |
| 🤝 **[CONTRIBUTING.md](CONTRIBUTING.md)** | Developer workflow, Makefile targets, and pull request guidelines. |

---

## 4. Support Channels & Community Guidelines

| Support Channel | Intended Use | SLA Response Time |
| :--- | :--- | :--- |
| 💬 **[GitHub Discussions](https://github.com/rihadjahanopu/gladeshell/discussions)** | Questions, configuration tips, theme customization, feature ideas | Within **24–48 hours** |
| 🐛 **[GitHub Issues](https://github.com/rihadjahanopu/gladeshell/issues)** | Bug reports, panic tracebacks, reproducible build failures | Within **24–48 hours** |
| 🔒 **[Private Security Email](SECURITY.md)** | Sensitive vulnerability reports (`rihadjahanopu@gmail.com`) | Within **24 hours** |

---

## 5. Submitting a High-Quality Bug Report

To help maintainers resolve your issue on the first attempt, please include:

1. **Environment**: Operating System & Architecture (e.g., Ubuntu 24.04 `x86_64`, macOS Sonoma `aarch64`, Windows 11 `x86_64-pc-windows-msvc`).
2. **Version**: Output of `gladeshell --version` and `rustc --version`.
3. **Panic Backtrace**: If `gladeshell` crashed, run with `RUST_BACKTRACE=1` and paste the full traceback log:
   ```bash
   RUST_BACKTRACE=1 gladeshell <command>
   ```
4. **Terminal Emulator**: Name and version of your terminal (Alacritty, VS Code, WezTerm, Kitty, iTerm2, Windows Terminal).

---

## 6. Security & Vulnerability Policy

Do **NOT** open public GitHub issues for security vulnerabilities or cryptographic concerns. Please consult our **[SECURITY.md](SECURITY.md)** for private advisory reporting instructions.

Thank you for supporting **gladeshell**! 🚀
