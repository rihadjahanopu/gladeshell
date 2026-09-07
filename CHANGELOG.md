# 📋 Changelog

All notable changes to **fancybash** are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).  
Versioning follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.2.0] — 2026-08-14

### ✨ Added & Improved
- **FFmpeg Multimedia Suite (`ffmedia`)**:
  - Added an interactive 24-in-1 FFmpeg powerhouse (`ffmedia`, `ffstudio`, `fftool`, `fancy_ffmpeg`).
  - Features 5 comprehensive categories: Video Editing & Size Management, Audio Engineering & Processing, Image/GIF & Thumbnail Utility, Screen Recording & Subtitles, Privacy Metadata & Bulk Automation.
  - Implemented 3-tier interactive UI hierarchy (`gum` → `fzf` → terminal prompts) for smooth file picking and option selection.
  - Dynamic dependency detection and auto-installer integration via `_fb_ensure_dep ffmpeg`.

## [2.1.0] — 2026-08-13

### ✨ Added & Improved
- **Todo Manager (`todo`)**:
  - Added 3-tier fallback hierarchy (`gum` → `fzf` → plain `read` prompt) for full interactivity or standalone execution.
  - Portable line deletion via `_fb_sed_delete_line` supporting both GNU `sed` (Linux) and BSD `sed` (macOS).
  - Added range validation for task index completion to prevent invalid line deletions.
- **Notes Manager (`notes`)**:
  - Switched default note file storage format from `.md` to **`.txt`** plain text for universal editor compatibility.
  - Added cross-platform clipboard support supporting Wayland (`wl-copy`), X11 (`xclip`, `xsel`), and macOS (`pbcopy`).
  - Multi-tier viewer & preview fallback pipeline (`glow` → `bat` → `batcat` → `cat` / `less`).
- **Installer Automation (`install.sh` & `install.zsh`)**:
  - Dynamically detects and auto-installs `notes` and `todo` dependencies (`glow`, `bat`, `xclip`, `wl-clipboard`, `fzf`, `gum`) across all major package managers (`apt`, `pacman`, `dnf`, `apk`, `brew`).
- **Web & Documentation**:
  - Added interactive utilities command reference section to `README.md`.
  - Added dedicated **📋 Todo & Notes** tab and command reference table panel to `web/index.html`.

### 🐛 Fixed
- **`todo` Regex Vulnerability**: Fixed regex injection issue when completing tasks containing special characters (`/`, `&`, `.`, `*`, `[`) using `grep -Fn`.
- **`notes` Subprocess Preview Bug**: Fixed `fzf` preview failures when `bat`/`batcat` is missing or fails in subshell execution.
- **`notes add` Fallback**: Fixed missing fallback when `gum` is not installed during note category creation and entry.
- **Cross-Shell Portability**: Aligned both `config.sh` (Bash) and `config.zsh` (Zsh) with identical portable syntax and verified clean parsing.

---

## [2.0.0] — 2026-06-01

### ✨ Added
- **Docker Swiss Army Knife** — 50+ docker aliases and functions (`dps`, `dsh`, `dwatch`, `dbackup`, `dclean`, `dkill-force`, etc.)
- **Docker tab completion** — press `Tab` after `dsh`, `dlogs`, `dstop` to auto-complete container names
- **`uup` Mega Updater** — interactive `fzf` menu to update OS, Snap, Flatpak, Bun, Node.js, NPM in one shot
- **`uu` Universal Uninstaller** — fuzzy picker across apt/snap/flatpak/AppImage with size & install date info
- **`makecpp`** — advanced C/C++ boilerplate generator (auto cd, Makefile, git init, VSCode tasks)
- **`vite` function** — interactive Vite scaffolder with optional Tailwind CSS v4 setup
- **`ui` function** — Shadcn/UI installer with fzf component picker
- **`css` function** — auto-detects package manager, installs Tailwind + clsx + tailwind-merge
- **`gwip`** — smart WIP commit + auto push to current branch
- **`pg`** — interactive `package.json` generator
- **`run`** — interactive JS/TS file runner via Bun
- **Zsh config** (`config.zsh`) — full port of `config.sh` with zsh-specific features
- **PowerShell config** (`config.ps1`) — initial port (WIP)
- **Linux App Ecosystem page** (`web/linux-setup.html`) — searchable, filterable app browser
- **Zed IDE installer** (`zed/install-settings.sh`) — writes to both Flatpak and native paths
- `ARCHITECTURE.txt` — detailed contributor architecture document
- `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md` — open source standards

### 🔄 Changed
- Prompt system refactored — default is now minimal single-line for speed; full two-line is opt-in
- `pending_updates()` now supports Arch (`checkupdates`), Debian, and fallback apt-get methods
- `cpu_temp()` color thresholds adjusted: green ≤55°C, yellow 55–70°C, red >70°C
- Installer scripts (`install.sh`, `install.zsh`) now validate downloaded file is not an HTML error page
- `setuppc()` updated to include Zsh, fzf, and modern tooling bootstrap

### 🐛 Fixed
- `get_duration()` timer reset issue on rapid command execution
- `ex()` archive extractor — added `.7z` support
- `check_readonly()` false positive on certain network-mounted directories

---

## [1.0.1] — 2025-11-15

### ✨ Added
- `kp <port>` — kill process by port number
- `iploc` — show IP + city/region/org info via ipinfo.io
- `deno` support in `rt()` runtime installer
- `battery_info()` prompt metric for laptop users
- `check_readonly()` — shows 🔒 when directory is not writable

### 🔄 Changed
- `rand_emoji()` now detects `bun`, `py`, `proj` folder patterns
- `gwip` now falls back to "Work in progress (Save Point)" if no message given
- Improved installer header with box-drawing characters

### 🐛 Fixed
- `parse_git_branch()` — no longer prints error in non-git directories
- `ff()` — now correctly skips `.git` and `node_modules`
- `gen()` — fixed output truncation for lengths > 64

---

## [1.0.0] — 2025-08-01

### 🎉 Initial Release
- Core prompt system with `rand_color()`, `rand_emoji()`, `parse_git_branch()`
- System monitoring: `cpu_temp()`, `disk_usage()`, `load_avg()`, `get_duration()`
- Navigation aliases: `..`, `...`, `....`, `dev`, `fr`, `ba`, `fu`
- NPM aliases: `ni`, `nid`, `nr`, `nrd`, `nrb`, `nrs`
- Bun aliases: `bi`, `br`, `brd`, `brb`, `brs`, `html`
- Git shortcuts: `gs`, `ga`, `gcm`, `gps`, `gpl`, `gl`, `gco`, `gcb`, `gd`
- Git stash: `gst`, `gsta`, `gpop`
- Utility tools: `mkd`, `rmd`, `rmf`, `bak`, `trash`, `ex`, `ff`, `gen`, `h`
- System tools: `update`, `clean`, `myip`, `ports`, `serve`, `rel`
- `keep()` — inline command reference cheatsheet
- `init()` — interactive Bun/NPM project initializer
- `next()` — Next.js scaffolder
- `setuppc()` — new PC bootstrap script
- One-line installer with spinner, backup, and idempotency guard
- MIT License
- Static website (`web/`) on Netlify

---

[2.0.0]: https://github.com/rihadjahanopu/fancybash/compare/v1.0.1...v2.0.0
[1.0.1]: https://github.com/rihadjahanopu/fancybash/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/rihadjahanopu/fancybash/releases/tag/v1.0.0
