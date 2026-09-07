# 🆘 Support & Getting Help

Thank you for using **fancybash**! We want your experience with fancybash to be as smooth, fast, and delightful as possible. If you encounter an issue, have a question, or need assistance customizing your shell environment, this document outlines all the available support channels and troubleshooting steps.

<div align="center">

[![GitHub Issues](https://img.shields.io/github/issues/rihadjahanopu/fancybash?style=for-the-badge&color=a855f7&logo=github)](https://github.com/rihadjahanopu/fancybash/issues)
[![GitHub Discussions](https://img.shields.io/badge/GitHub-Discussions-22c55e?style=for-the-badge&logo=github&logoColor=white)](https://github.com/rihadjahanopu/fancybash/discussions)
[![Website](https://img.shields.io/badge/Website-fancybash.netlify.app-22d3ee?style=for-the-badge&logo=netlify&logoColor=white)](https://fancybash.netlify.app)
[![License MIT](https://img.shields.io/badge/License-MIT-0ea5e9?style=for-the-badge&logo=opensourceinitiative&logoColor=white)](LICENSE)

</div>

---

## 📌 Table of Contents

- [1. Quick Self-Troubleshooting Checklist](#1-quick-self-troubleshooting-checklist)
  - [Icons or Glyphs Are Broken / Displaying Question Marks](#icons-or-glyphs-are-broken--displaying-question-marks)
  - [Aliases or Commands Not Recognized After Installation](#aliases-or-commands-not-recognized-after-installation)
  - [Installer Permission Errors or Script Failures](#installer-permission-errors-or-script-failures)
  - [Uninstalling or Restoring Previous Shell Configuration](#uninstalling-or-restoring-previous-shell-configuration)
  - [Cross-Shell Compatibility Issues (Zsh, Fish, PowerShell)](#cross-shell-compatibility-issues-zsh-fish-powershell)
- [2. Documentation & Technical Manuals](#2-documentation--technical-manuals)
- [3. Where to Get Help](#3-where-to-get-help)
  - [💡 GitHub Discussions (Questions & Usage Advice)](#-github-discussions-questions--usage-advice)
  - [🐛 GitHub Issues (Bug Reports & Feature Requests)](#-github-issues-bug-reports--feature-requests)
- [4. Submitting a High-Quality Bug Report](#4-submitting-a-high-quality-bug-report)
- [5. Security & Vulnerability Reporting](#5-security--vulnerability-reporting)
- [6. Community Expectations & Response Times](#6-community-expectations--response-times)

---

## 1. Quick Self-Troubleshooting Checklist

Before creating a support ticket, try these quick resolution steps for common issues:

### Icons or Glyphs Are Broken / Displaying Question Marks

`fancybash` uses Nerd Font symbols (e.g., Git branch icons, folder indicators, OS logos). If icons appear as `[?]` or missing rectangles:
1. **Install a Nerd Font**: Download and install a Nerd Font such as [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads) or [JetBrainsMono Nerd Font](https://www.nerdfonts.com/).
2. **Set Terminal Font**: Open your terminal application settings (VS Code, Zed, Alacritty, iTerm2, Windows Terminal, Kitty) and set your font family to your installed Nerd Font (e.g., `FiraCode Nerd Font` or `JetBrainsMono NF`).

> [!TIP]
> Check our dedicated [Font Setup Guide in README.md](README.md#-font-setup-for-emoji--icons) for OS-specific instructions.

---

### Aliases or Commands Not Recognized After Installation

If running a shortcut like `gs`, `nrd`, `dps`, or `todo` results in `command not found`:
1. Reload your current shell environment:
   ```bash
   source ~/.bashrc   # For Bash
   # OR
   source ~/.zshrc    # For Zsh
   ```
2. Alternatively, start a fresh shell session:
   ```bash
   exec bash
   ```
3. Verify that `fancybash` section exists inside `~/.bashrc`:
   ```bash
   grep -i "fancybash" ~/.bashrc
   ```

---

### Installer Permission Errors or Script Failures

If `install.sh` fails during execution:
1. Ensure the installer script has execute permissions:
   ```bash
   chmod +x install.sh
   ./install.sh
   ```
2. If installing interactively via `curl`:
   ```bash
   bash -c "$(curl -fsSL https://raw.githubusercontent.com/rihadjahanopu/fancybash/main/i.sh)"
   ```
3. **No Root Required**: `fancybash` installs entirely inside your user `$HOME` directory (`~/.fancybash`). Do **not** run the installer with `sudo`.

---

### Uninstalling or Restoring Previous Shell Configuration

`fancybash` is designed with 100% safe, non-destructive installation. Every installation creates a timestamped backup of your original configuration file (e.g., `~/.bashrc.bak.YYYYMMDD_HHMMSS`).

To uninstall completely and restore your original backup:
```bash
./uninstall.sh
```

---

### Cross-Shell Compatibility Issues (Zsh, Fish, PowerShell)

`fancybash` supports multiple shell environments:
* **Bash**: Default engine using [`config.sh`](config.sh).
* **Zsh**: Native Zsh integration using [`config.zsh`](config.zsh) & [`install.zsh`](install.zsh).
* **Fish**: Native Fish configuration using [`config.fish`](config.fish) & [`install.fish`](install.fish).
* **PowerShell 7+**: Windows & cross-platform PowerShell support using [`config.ps1`](config.ps1) & [`install.ps1`](install.ps1).

Ensure you ran the installer appropriate for your shell (e.g., `./install.zsh` for Zsh users).

---

## 2. Documentation & Technical Manuals

Detailed technical documentation is maintained within the repository:

| Guide / Manual | Purpose |
| :--- | :--- |
| 📖 [README.md](README.md) | Full command reference, quick start, feature highlights, and interactive tool usage. |
| 🏗️ [ARCHITECTURE.txt](ARCHITECTURE.txt) | Internal project structure, section banners, and modular architecture. |
| ⚡ [DSA.md](DSA.md) | Performance principles ($O(1)$ prompt execution rules, subshell optimization). |
| 📚 [wiki.md](wiki.md) | Detailed user guide and extended configuration manual. |
| 🗺️ [ROADMAP.md](ROADMAP.md) | Project future vision, release milestones, and feature planning. |
| 🤝 [CONTRIBUTING.md](CONTRIBUTING.md) | Developer guidelines for submitting code and pull requests. |

---

## 3. Where to Get Help

If your issue is not resolved by the troubleshooting guide above:

### 💡 GitHub Discussions (Questions & Usage Advice)

Use [GitHub Discussions](https://github.com/rihadjahanopu/fancybash/discussions) for:
* Asking general "How do I...?" questions.
* Seeking help with custom shell configurations or alias additions.
* Sharing feedback, workflow tips, and terminal setups with the community.
* Requesting community guidance on custom TUI utilities.

👉 **[Join GitHub Discussions](https://github.com/rihadjahanopu/fancybash/discussions)**

---

### 🐛 GitHub Issues (Bug Reports & Feature Requests)

Use [GitHub Issues](https://github.com/rihadjahanopu/fancybash/issues) to report reproducible bugs or propose formal feature enhancements.

👉 **[Open an Issue](https://github.com/rihadjahanopu/fancybash/issues/new/choose)**

---

## 4. Submitting a High-Quality Bug Report

To help us diagnose and fix your issue quickly, please include the following details in your issue report:

1. **Operating System & Architecture**: (e.g., Ubuntu 24.04 LTS x86_64, macOS Sonoma M2, Windows 11 WSL2).
2. **Shell & Version**: (e.g., GNU bash version 5.2.21, zsh 5.9).
3. **Terminal Emulator**: (e.g., VS Code Terminal, Zed IDE, Alacritty, Windows Terminal, Kitty, iTerm2).
4. **Expected vs Actual Behavior**: Clear description of what happened versus what you expected.
5. **Exact Terminal Output / Error Stack**: Copy-paste terminal text or attach screenshots.
6. **Reproduction Steps**: Minimal set of commands to trigger the bug.

---

## 5. Security & Vulnerability Reporting

> [!IMPORTANT]
> Please **do not** open public GitHub issues for security vulnerabilities.

If you discover a security vulnerability or potential privilege escalation bug in `fancybash` (e.g., installer script injection or unsafe variable expansion), please review our **[SECURITY.md](SECURITY.md)** guidelines and report it directly to the lead maintainer via security disclosure channels.

---

## 6. Community Expectations & Response Times

* **Volunteer Effort**: `fancybash` is an open-source project maintained by volunteers and community contributors. While we endeavor to review issues and pull requests promptly, response times may vary.
* **Respectful Interaction**: All support interactions are governed by our **[Code of Conduct](CODE_OF_CONDUCT.md)**. Please maintain an empathetic, constructive, and friendly tone.

<br>

<div align="center">

**Thank you for using `fancybash`! 🚀**  
*Crafted with ❤️ for developers worldwide.*

</div>
