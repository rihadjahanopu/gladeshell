# 🆘 Support & Getting Help

Thank you for using **gladeshell**! We want your experience with gladeshell to be as smooth, fast, and delightful as possible. If you encounter an issue, have a question, or need assistance, this document outlines all available support channels and troubleshooting steps.

<div align="center">

[![GitHub Issues](https://img.shields.io/github/issues/rihadjahanopu/gladeshell?style=for-the-badge&color=a855f7&logo=github)](https://github.com/rihadjahanopu/gladeshell/issues)
[![GitHub Discussions](https://img.shields.io/badge/GitHub-Discussions-22c55e?style=for-the-badge&logo=github&logoColor=white)](https://github.com/rihadjahanopu/gladeshell/discussions)
[![Website](https://img.shields.io/badge/Website-gladeshell.netlify.app-22d3ee?style=for-the-badge&logo=netlify&logoColor=white)](https://gladeshell.netlify.app)
[![License MIT](https://img.shields.io/badge/License-MIT-0ea5e9?style=for-the-badge&logo=opensourceinitiative&logoColor=white)](LICENSE)

</div>

---

## 📌 Table of Contents

- [1. Quick Self-Troubleshooting Checklist](#1-quick-self-troubleshooting-checklist)
  - [Icons or Glyphs Are Broken / Displaying Question Marks](#icons-or-glyphs-are-broken--displaying-question-marks)
  - [Binary or Command Not Recognized After Installation](#binary-or-command-not-recognized-after-installation)
  - [Installer Permission Errors or Binary Build Failures](#installer-permission-errors-or-binary-build-failures)
  - [Uninstalling or Restoring Previous Shell Configuration](#uninstalling-or-restoring-previous-shell-configuration)
  - [Cross-Platform Compatibility (Linux, macOS, Windows)](#cross-platform-compatibility-linux-macos-windows)
- [2. Documentation & Technical Manuals](#2-documentation--technical-manuals)
- [3. Where to Get Help](#3-where-to-get-help)
- [4. Submitting a High-Quality Bug Report](#4-submitting-a-high-quality-bug-report)
- [5. Security & Vulnerability Reporting](#5-security--vulnerability-reporting)

---

## 1. Quick Self-Troubleshooting Checklist

Before creating a support ticket, try these quick resolution steps:

### Icons or Glyphs Are Broken / Displaying Question Marks

`gladeshell` uses Nerd Font symbols (e.g., Git branch icons, folder indicators, OS logos). If icons appear as `[?]` or missing rectangles:

1. **Install a Nerd Font**: Download and install a Nerd Font such as [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads) or [JetBrainsMono Nerd Font](https://www.nerdfonts.com/).
2. **Set Terminal Font**: Open your terminal application settings (VS Code, Zed, Alacritty, iTerm2, Windows Terminal, Kitty) and set your font family to your installed Nerd Font (e.g., `FiraCode Nerd Font` or `JetBrainsMono NF`).

---

### Binary or Command Not Recognized After Installation

If running `gladeshell` results in `command not found`:

1. Ensure `~/.cargo/bin` or `~/.local/bin` is in your `$PATH`:
   ```bash
   export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"
   ```
2. Run shell setup to inject interactive aliases:
   ```bash
   gladeshell setup
   ```
3. Restart your terminal subshell session (`exec bash` or `exec zsh`).

---

### Installer Permission Errors or Binary Build Failures

If installing from source via Cargo:

1. Ensure a modern Rust toolchain (1.80+) is installed:
   ```bash
   rustc --version
   ```
2. Build and install binary locally:
   ```bash
   cargo install --path . --force
   ```
3. If using automated installer script:
   ```bash
   curl -fsSL https://gladeshell.netlify.app/install.sh | bash
   ```

---

### Uninstalling or Restoring Previous Shell Configuration

`gladeshell` includes a non-destructive uninstaller:

```bash
gladeshell uninstall
# OR via installer script
./install.sh --uninstall
```

---

### Cross-Platform Compatibility (Linux, macOS, Windows)

`gladeshell` is a 100% pure Rust hyper-optimized binary supporting:

- **Linux**: glibc & static MUSL architectures (`x86_64`, `aarch64`)
- **macOS**: Intel (`x86_64`) & Apple Silicon M-Series (`aarch64`)
- **Windows**: PowerShell 7+ & Windows Command Prompt (`x86_64-pc-windows-msvc`, `aarch64-pc-windows-msvc`)

---

## 2. Documentation & Technical Manuals

| Guide / Manual                          | Purpose                                                                              |
| :-------------------------------------- | :----------------------------------------------------------------------------------- |
| 📖 [README.md](README.md)               | Full command reference, quick start, feature highlights, and interactive tool usage. |
| 🏗️ [ARCHITECTURE.txt](ARCHITECTURE.txt) | Internal project structure, Rust module tree, and engine design.                     |
| 📚 [wiki.md](wiki.md)                   | Detailed user guide and extended configuration manual.                               |
| 🗺️ [ROADMAP.md](ROADMAP.md)             | Project future vision, release milestones, and feature planning.                     |
| 🤝 [CONTRIBUTING.md](CONTRIBUTING.md)   | Developer guidelines for submitting code and pull requests.                          |

---

## 3. Where to Get Help

- **[GitHub Discussions](https://github.com/rihadjahanopu/gladeshell/discussions)** — General questions, shell configuration advice, workflow tips.
- **[GitHub Issues](https://github.com/rihadjahanopu/gladeshell/issues)** — Bug reports with `RUST_BACKTRACE=1` and feature requests.

---

## 4. Submitting a High-Quality Bug Report

Please include the following in your report:

1. **OS & Architecture**: (e.g. Ubuntu 24.04 x86_64, macOS Sonoma arm64, Windows 11)
2. **Version**: Output of `gladeshell --version` and `rustc --version`
3. **Backtrace**: Set `RUST_BACKTRACE=1` when reproducing panics
4. **Terminal Emulator**: (Alacritty, VS Code, WezTerm, Windows Terminal, Kitty)

---

## 5. Security & Vulnerability Reporting

Please do **not** open public GitHub issues for security vulnerabilities. Review **[SECURITY.md](SECURITY.md)** for private disclosure details.
