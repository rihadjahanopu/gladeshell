<div align="center">

<br>

<!-- ```
   ██████╗ ██╗      █████╗ ██████╗ ███████╗███████╗██╗  ██╗███████╗██╗     ██╗
  ██╔════╝ ██║     ██╔══██╗██╔══██╗██╔════╝██╔════╝██║  ██║██╔════╝██║     ██║
  ██║  ███╗██║     ███████║██║  ██║█████╗  ███████╗███████║█████╗  ██║     ██║
  ██║   ██║██║     ██╔══██║██║  ██║██╔══╝  ╚════██║██║  ██║██╔══╝  ██║     ██║
  ╚█████╔╝ ███████╗██║  ██║██████╔╝███████╗███████║██║  ██║███████╗███████╗███████╗
   ╚════╝  ╚══════╝╚═╝  ╚═╝╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝
``` -->

<img src="web/favicon.svg" alt="logo" width="30%">

### ⚡ The Ultimate Pure Rust Shell & Developer Suite for Modern Terminal Users

_100% Rust • Blazing Fast • Zero Bloat • Native Ratatui TUI Suite_

<br>

[![MIT License](https://img.shields.io/badge/License-MIT-a855f7?style=for-the-badge&logo=opensourceinitiative&logoColor=white)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2024%20%7C%201.85%2B-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-0ea5e9?style=for-the-badge&logo=linux&logoColor=white)](#)
[![Stars](https://img.shields.io/github/stars/rihadjahanopu/gladeshell?style=for-the-badge&logo=github&color=f59e0b&logoColor=white)](https://github.com/rihadjahanopu/gladeshell)
[![Version](https://img.shields.io/badge/Version-1.1.0-ec4899?style=for-the-badge)](#)
[![Website](https://img.shields.io/badge/Website-gladeshell.netlify.app-22d3ee?style=for-the-badge&logo=netlify&logoColor=white)](https://gladeshell.netlify.app)

<br>

<img src="https://i.postimg.cc/pXB6h98T/terminal2.png" alt="gladeshell terminal preview 1" width="85%">
<br><br>
<img src="https://i.postimg.cc/tCzMZ1P1/terminal1.png" alt="gladeshell terminal preview 2" width="85%">

<br>

**🌐 Live Website: [gladeshell.netlify.app](https://gladeshell.netlify.app)**

</div>

---

## 📌 Table of Contents

- [✨ What is gladeshell?](#-what-is-gladeshell)
- [🌟 Feature Highlights](#-feature-highlights)
- [⚡ Performance Benchmarks](#-performance-benchmark-matrix)
- [🚀 Quick Install](#-quick-install)
- [🗑️ Uninstall](#️-uninstall)
- [🦄 Zsh Setup Guide](#-zsh-setup-guide)
- [⚙️ Font Setup](#️-font-setup-for-emoji--icons)
- [📟 Smart Prompt](#-smart-prompt-system)
- [🎨 Prompt Themes](#-prompt-themes)
  - [🖼️ Preview All 55 Themes](#️-preview-all-55-themes)
  - [⚡ Switch Themes](#-switch-themes)
  - [📋 All 55 Themes](#-all-55-themes)
- [🛠️ Command Reference](#️-command-reference)
  - [📂 Navigation](#-navigation--movement)
  - [📦 Package Managers](#-npm--bun-commands)
  - [🌿 Git Shortcuts](#-git-version-control)
  - [🔧 Project Setup](#-project-initialization)
  - [⚙️ System Tools](#-system--maintenance)
  - [🔨 Utilities](#-utility-tools)
  - [🚀 Interactive Utilities (GUM & FZF)](#-interactive-utilities-gum--fzf)
    - [📋 Todo Manager](#-todo-manager)
    - [📝 Notes Manager](#-notes-manager)
    - [🎬 FFmedia Multimedia Suite](#-ffmedia-all-in-one-multimedia-suite)
    - [🔐 Hardened Multi-Vault Security Suite](#-hardened-multi-vault-security-suite)
    - [🔀 Other GUM / FZF Utilities](#-other-gum--fzf-utilities)
  - [🐳 Docker & Containers](#-docker--containers)
    - [📊 Dashboard & Monitoring](#-interactive-dashboard--monitoring)
    - [⚡ Service Control](#-service-control)
    - [🔄 Container Lifecycle](#-container-lifecycle)
    - [🐛 Debugging & Building](#-debugging--building)
    - [🧩 Docker Compose](#-docker-compose)
    - [🧪 Quick Test Sandboxes](#-quick-test-sandboxes)
    - [🧠 Advanced Functions](#-advanced-functions)
  - [🐘 PostgreSQL](#-postgresql)
  - [💎 Prisma ORM](#-prisma-orm)
- [🏗️ Project Structure](#️-project-structure)
- [🖥️ Zed IDE Settings](#️-zed-ide-settings)
- [🐧 Linux App Ecosystem](#-linux-app-ecosystem)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)

---

## ✨ What is gladeshell?

**gladeshell** (also known as `gladeshell`) is a high-performance, single-binary shell environment and developer suite written completely in **Pure Rust (2024 edition)**.

It replaces slow shell scripts and heavy framework overhead with a single compiled binary, providing ultra-fast prompt rendering, cross-shell compatibility (Bash, Zsh, Fish, PowerShell), a fast multi-core compression engine (Level 1 Fast), a 24-in-1 FFmpeg multimedia suite, hardened AES-256 security vault, and full Ratatui TUI utilities.

One binary. One install. Absolute speed.

> Built by developers, for developers — with Rust performance, Node.js, Bun, Git, Docker, and Linux/macOS/Windows workflows built-in.

---

## 🌟 Feature Highlights

| Feature                        | Description                                                                                           |
| ------------------------------ | ----------------------------------------------------------------------------------------------------- |
| ⚡ **Pure Rust Engine**        | Compiled Rust binary delivering sub-millisecond execution and zero runtime dependencies               |
| 🛡️ **Safe & Idempotent**       | Universal setup script never overwrites shell configs blindly — creates timestamped backups           |
| 🗜️ **Level 1 Fast Compressor** | Parallel multi-core zstd archive engine optimized for maximum compression speed                       |
| 🖥️ **Ratatui TUI Suite**       | Native terminal GUIs for Todo, Notes, Filetree navigation, Process Manager, and System Clean          |
| 🎬 **24-in-1 FFmedia Suite**   | Comprehensive FFmpeg multimedia processor (compression, trimming, visualizer, GIFs, screen recording) |
| 🔐 **AES-256 PBKDF2 Vault**    | Memory-guarded directory vault with panic password decoy mode and auto-shredding                      |
| 🎨 **55 Prompt Themes**        | Swap between Catppuccin, Tokyo Night, Dracula, Matrix, Starship, Rose Pine, and 49 more in 1 command  |
| 🔄 **Cross-Shell Integration** | Built-in shell integration generator for Bash, Zsh, Fish, and PowerShell (`gladeshell shell-init`)    |
| 📦 **Universal Uninstaller**   | `uu` — interactive fuzzy application remover across apt/snap/flatpak/AppImage                         |
| 🔁 **Mega Updater**            | `uup` — updates system packages, runtimes, and flatpaks in one command                                |

---

## ⚡ Performance Benchmark Matrix

> **Empirical Performance Comparison:** `gladeshell` vs `Starship` vs `Oh My Posh` vs `Oh My Zsh`
>
> Tested on Linux x86_64 / macOS ARM64 using 10,000-iteration sample suites. See [benchmark.md](benchmark.md) for full methodology.

| Performance Metric                |        ⚡ **`gladeshell`**         |   🚀 **`Starship`**   |  🎨 **`Oh My Posh`**  |     🐚 **`Oh My Zsh`**     |
| :-------------------------------- | :--------------------------------: | :-------------------: | :-------------------: | :------------------------: |
| **Engine Architecture**           | **Rust (Pure `gix` / Zero-Alloc)** | Rust (Modular Binary) | Go (GC Static Binary) |       Zsh Scripting        |
| **Core Prompt Latency (`PS1`)**   |     **0.0114 ms (11.4 µs)** 🏆     |   2.40 ms – 8.50 ms   |  8.20 ms – 26.50 ms   |    18.50 ms – 95.00 ms     |
| **Warm Process Launch**           |  **~1.64 ms mean / 0.92 ms min**   |   15.2 ms – 28.5 ms   |   35.0 ms – 72.0 ms   |     N/A (Pure script)      |
| **Bash Hook Source (Cached)**     |   **~6.7 ms mean / 5.9 ms min**    |   18.5 ms – 32.0 ms   |   38.0 ms – 85.0 ms   |      180 ms – 450 ms       |
| **Zsh Hook Source (Cached)**      |  **~32.4 ms mean / 29.2 ms min**   |   30.0 ms – 55.0 ms   |   45.0 ms – 90.0 ms   |      200 ms – 450 ms       |
| **Subshell Process Forks**        |       **0 (Zero subshells)**       |    1 (Exec binary)    |    1 (Exec binary)    | 3 – 8 (git/env subshells)  |
| **Git Repo Overhead (`gix`)**     |  **4.8 µs cache / < 4.6 ms live**  |   8.5 ms – 35.0 ms    |   15.0 ms – 55.0 ms   |     45.0 ms – 250.0 ms     |
| **Tab RSS Memory Footprint**      |   **~3.9 MB (Fat LTO Stripped)**   |  ~12.5 MB – 18.2 MB   |  ~18.5 MB – 32.0 MB   |     ~28.0 MB – 55.0 MB     |
| **Autocompletion Engine Latency** |     **~0.08 ms (Native Rust)**     |          N/A          |          N/A          | ~12.5 ms (`zsh-syntax-hl`) |
| **Included Themes**               |           **55 Themes**            |    Config required    |     JSON Presets      |      Community themes      |

```
Prompt Render Latency (Lower is better):
gladeshell (Pure Rust Engine) : █ 0.0114 ms  [210x FASTER THAN STARSHIP]
Starship (Rust Modular)      : ███████ 2.40 ms
Oh My Posh (Go Engine)       : █████████████████████ 8.20 ms
Oh My Zsh (Zsh Scripting)    : ██████████████████████████████████████████ 18.50 ms
```

> 📖 _For complete micro-benchmarking methodology, IPC socket metrics, and memory profiles, see [benchmark.md](benchmark.md)._

---

## 🚀 Quick Install

### Universal One-Line Install (Recommended)

Auto-detects your operating system (Linux, macOS, Windows) and active shell environment:

**Linux / macOS (Bash / Zsh / Fish):**

```bash
curl -fsSL https://gladeshell.netlify.app/install.sh | bash
```

**Windows (PowerShell / CMD / Run):**

```cmd
powershell -c "irm https://gladeshell.netlify.app/install.ps1 | iex"
```

### Install via Cargo

If you have the Rust toolchain installed:

```bash
cargo install --path .
```

### Build from Source

```bash
git clone https://github.com/rihadjahanopu/gladeshell.git
cd gladeshell
cargo build --release
./target/release/gladeshell --version
```

---

## 🗑️ Uninstall

Cleanly removes gladeshell binary and shell integration blocks without touching your personal configuration:

**Linux / macOS:**

```bash
curl -fsSL https://gladeshell.netlify.app/install.sh | bash -s -- --uninstall
```

Or via binary:

```bash
gladeshell uninstall
```

**Windows (PowerShell / CMD / Run):**

```cmd
powershell -c "irm https://gladeshell.netlify.app/install.ps1 | iex -Uninstall"
```

---

## ⚙️ Font Setup (for Emoji & Icons)

gladeshell uses color emoji and programming ligatures in the prompt for best visual density:

### 1 — Install Fonts

```bash
sudo apt update && sudo apt install -y fonts-noto-color-emoji fonts-firacode fonts-cascadia-code
```

### 2 — Set Font Priority (`/etc/fonts/local.conf`)

```xml
<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <alias>
    <family>monospace</family>
    <prefer>
      <family>Fira Code</family>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
</fontconfig>
```

### 3 — Rebuild Font Cache

```bash
fc-cache -fv
```

---

## 📟 Smart Prompt System

gladeshell renders a responsive, contextual prompt powered by Rust:

```
🚀 myproject [🌿 main ❗]               ← Folder icon + colored folder + git status
❯❯❯                                   ← Fast response line
```

**Prompt features include:**

| Element            | Description                                                                                   |
| ------------------ | --------------------------------------------------------------------------------------------- |
| `rand_emoji`       | Folder-aware emoji — `🌐` for web, `🟢` for node, `🥐` for bun, `🐍` for py, random otherwise |
| `rand_color`       | Rainbow color cycle on every prompt render                                                    |
| `parse_git_branch` | Shows `branchname ❗` when working tree is dirty                                              |
| `cpu_temp`         | 🟢 Green / 🟡 Yellow / 🔴 Red based on temperature thresholds                                 |
| `disk_usage`       | Shows free disk space on `/`                                                                  |
| `load_avg`         | System load average                                                                           |
| `get_duration`     | Shows `⏱️ Ns` for any command taking longer than 1 second                                     |
| `check_readonly`   | Shows `🔒` when current directory is not writable                                             |
| `pending_updates`  | Shows `🆙 N` if system packages need updating                                                 |
| `battery_info`     | Shows battery % when available                                                                |
| `kernel_version`   | Displays current kernel version                                                               |

### ⚙️ Customizing the Prompt Layout

By default, gladeshell/gladezsh renders a clean, minimalistic **single-line** prompt. However, all the dynamic system monitoring metrics listed above (such as CPU temp, disk space, active runtime versions) are already built-in and ready to be used.

You can modify or toggle the prompt layout to your liking by editing your shell configuration file (`~/.bashrc` or `~/.zshrc`) and uncommenting/commenting the lines under the **`🎯 TWO LINE PROMPT`** section:

```bash
# 💡 Uncomment these lines in your ~/.bashrc or ~/.zshrc if you want the full two-line prompt:
# PS1="\$(rand_emoji) \[\033[\$(rand_color)m\]\W\[\033[0m\] "
# PS1+="\$(folder_size) [🌿 \$(parse_git_branch)]\$(cpu_temp) \$(disk_usage) \$(load_avg) \$(get_duration) \$(check_readonly) \$(pending_updates)\n"
# PS1+="\$(node_version) │ \$(npm_version) │ \$(bun_version) │ \$(kernel_version) │ "
# PS1+="\$(time_date) │ \$(sys_info) │ \$(battery_info)\n"
```

Feel free to customize, add, or remove any helper functions (like `node_version`, `cpu_temp`, etc.) from your configuration block to design your own custom layout!

---

## 🎨 Prompt Themes

gladeshell ships with **55 world-class prompt themes** inspired by the best designs from Oh My Zsh, Oh My Posh, Starship, Spaceship, and modern terminal communities. Every theme works on **Bash, Zsh, and Fish** with full color and emoji support.

### 🖼️ Preview All 55 Themes

```bash
# Open interactive scrollable gallery — scroll with ↑↓ / PageUp / PageDown
glade preview

# Same as above:
glade list
```

The gallery opens in a `less`-powered pager showing all 55 themes numbered `[1/55]` through `[55/55]` with full ANSI colors. Press **`q`** to exit.

### ⚡ Switch Themes

```bash
# Interactive picker (gum or fzf UI — arrow keys to select)
glade

# Switch directly by name
glade catppuccin
glade tokyonight
glade dracula
glade cyberpunk
glade starship
glade matrix
glade gruvbox
glade rosepine
glade kanagawa
glade nightowl

# Theme is saved to ~/.gladeshell_theme and persists across sessions
```

### 📋 All 55 Themes

| #   | Theme               | Style                                 | Inspiration            |
| --- | ------------------- | ------------------------------------- | ---------------------- |
| 01  | `minimal`           | 🌈 Emoji + color · 1-line             | gladeshell default     |
| 02  | `full`              | 📊 3-line with all metrics            | gladeshell full info   |
| 03  | `robbyrussell`      | ➜ Green arrow · 1-line                | Oh My Zsh default      |
| 04  | `p10k`              | ╭─ 2-line rich · user@host            | Powerlevel10k          |
| 05  | `agnoster`          | ▓ Powerline segments                  | Oh My Zsh Agnoster     |
| 06  | `catppuccin`        | 🐱 Soft pastel Mocha                  | Oh My Posh Catppuccin  |
| 07  | `tokyonight`        | 🌌 Cyber neon purple/cyan             | Oh My Posh Tokyo Night |
| 08  | `dracula`           | 🧛 Dark pink & purple                 | Oh My Posh Dracula     |
| 09  | `nord`              | ❄️ Arctic frost blue                  | Oh My Posh Nord        |
| 10  | `bira`              | ╭─ Classic 2-line                     | Oh My Zsh Bira         |
| 11  | `pure`              | ·· Ultra quiet minimal                | Sindre Sorhus Pure     |
| 12  | `starship`          | 🚀 Magenta rocket                     | Starship cross-shell   |
| 13  | `cyberpunk`         | ⚡ Neon yellow & cyan                 | Futuristic cyberpunk   |
| 14  | `synthwave`         | 🌅 80s retro magenta                  | Synthwave aesthetic    |
| 15  | `gruvbox`           | 🌴 Warm gold & green                  | Gruvbox retro          |
| 16  | `onedark`           | 🌐 Classic blue/purple                | Atom One Dark          |
| 17  | `sorin`             | → Modern clean arrow                  | Oh My Zsh Sorin        |
| 18  | `spaceship`         | 🚀 Iconic rocket 2-line               | Spaceship ZSH          |
| 19  | `halflife`          | λ Scientist lambda                    | Half-Life orange       |
| 20  | `paradox`           | ⚡ Chevron powerline                  | Oh My Posh Paradox     |
| 21  | `bureau`            | [user@pc] brackets                    | Oh My Zsh Bureau       |
| 22  | `gallifrey`         | ⏳ TARDIS gold & cyan                 | Sci-Fi Doctor Who      |
| 23  | `material`          | 💎 Emerald & cyan                     | Google Material        |
| 24  | `monokai`           | 🔥 Neon green & pink                  | Monokai Pro            |
| 25  | `palenight`         | 🍇 Purple lavender                    | Material Palenight     |
| 26  | `powerlineclassic`  | ▓ Statusline segments                 | Classic Powerline      |
| 27  | `lambda`            | λ Functional one-line                 | Oh My Zsh Lambda       |
| 28  | `hyper`             | ⚡ Lightning pink/cyan                | Hyper Terminal         |
| 29  | `slick`             | ● Dot indicator                       | Ultra slick 1-line     |
| 30  | `matrix`            | 📟 Hacker green                       | The Matrix             |
| 31  | `sunset`            | 🌇 Coral pink/orange                  | Sunset warm tones      |
| 32  | `solarized`         | ☀️ Warm retro cyan & yellow           | Solarized Dark         |
| 33  | `rosepine`          | 🌹 Muted rose & lavender              | Rosé Pine              |
| 34  | `everforest`        | 🌲 Deep forest green & amber          | Everforest             |
| 35  | `kanagawa`          | 🌊 Japanese ink & dragon gold         | Kanagawa Wave          |
| 36  | `nightowl`          | 🦉 Midnight navy & turquoise          | Night Owl              |
| 37  | `cobalt2`           | ⚡ Electric yellow & blue             | Cobalt2 Wes Bos        |
| 38  | `shadesofpurple`    | 🍇 Royal purple & yellow              | Shades of Purple       |
| 39  | `ayu`               | 🎨 Clean orange & bright teal         | Ayu Dark               |
| 40  | `snazzy`            | ✨ Vivid magenta & electric cyan      | Hyper Snazzy           |
| 41  | `outrun`            | 🌆 80s neon sunset pink & cyan        | Outrun 84              |
| 42  | `oceanic`           | 🌊 Deep sea cyan & coral              | Oceanic Next           |
| 43  | `moonlight`         | 🌙 Deep indigo & sky blue             | Moonlight              |
| 44  | `papercolor`        | 📜 High contrast monochrome accent    | PaperColor             |
| 45  | `horizon`           | 🌄 Warm coral & peach sunset          | Horizon Dark           |
| 46  | `catppuccin_frappe` | ☕ Cool muted pastel lavender         | Catppuccin Frappé      |
| 47  | `dracula_pro`       | 🗡️ Neon violet & emerald blade        | Dracula Pro            |
| 48  | `cyber_samurai`     | 🥷 Crimson red & neon cyan            | Cyber Samurai          |
| 49  | `evergreen`         | 🍃 Fresh mint & leaf green            | Evergreen Mint         |
| 50  | `ghost`             | 👻 Translucent grey & phantom white   | Ghost Shell            |
| 51  | `oxide`             | ⚙️ Burnt orange & copper rust         | Oxide Rust             |
| 52  | `neon_pulse`        | 🔮 Vivid lime & electric pink         | Neon Pulse             |
| 53  | `volcano`           | 🌋 Fiery red & obsidian magma         | Volcano Magma          |
| 54  | `sakura`            | 🌸 Cherry blossom pink & pastel green | Sakura Blossom         |
| 55  | `galaxy`            | 🌌 Deep space indigo & nebula purple  | Galaxy Nebula          |

---

## 🛠️ Command Reference

> Run `keep` in your terminal to see this full reference at any time.

---

### 📂 Navigation & Movement

| Command               | Action                                                |
| --------------------- | ----------------------------------------------------- |
| `..`                  | Go up one directory                                   |
| `...`                 | Go up two directories                                 |
| `....`                | Go up three directories                               |
| `dev`                 | Jump to `~/Development`                               |
| `fr` / `ba` / `fu`    | Jump to Frontend / Backend / Fullstack project folder |
| `fig` / `ar` / `de`   | Jump to Figma / Archive / Dev folders                 |
| `des` / `doc` / `dow` | Jump to Desktop / Documents / Downloads               |
| `bv` / `ch` / `gp`    | Jump to Brave / Chrome / Photos Downloads             |

---

### 📦 NPM & Bun Commands

| Alias  | Expands To           |
| ------ | -------------------- |
| `ni`   | `npm install`        |
| `nid`  | `npm install -D`     |
| `nr`   | `npm run`            |
| `nrd`  | `npm run dev`        |
| `nrb`  | `npm run build`      |
| `nrs`  | `npm run start`      |
| `bi`   | `bun install`        |
| `br`   | `bun run`            |
| `brd`  | `bun run dev`        |
| `bhot` | `bun --hot`          |
| `w`    | `bun --watch`        |
| `brb`  | `bun run build`      |
| `brs`  | `bun run start`      |
| `html` | `bun run index.html` |

---

### 🌿 Git Version Control

| Command                 | Description                                    |
| ----------------------- | ---------------------------------------------- |
| `gi`                    | Initialize new git repository                  |
| `gs`                    | Git status (short format)                      |
| `ga`                    | Stage all files (`git add .`)                  |
| `gcm "msg"`             | Commit with message                            |
| `gps` / `gpl`           | Push / Pull from remote                        |
| `gl`                    | Pretty git log with graph                      |
| `gco <branch>`          | Checkout branch                                |
| `gcb <name>`            | Create & checkout new branch                   |
| `gd`                    | View diff                                      |
| `gst` / `gsta` / `gpop` | Stash / Apply stash / Pop stash                |
| `gwip "msg"`            | Quick WIP commit + auto push to current branch |

#### `gwip` — Smart WIP Pusher

```bash
gwip                         # Prompts for message, falls back to "Work in progress (Save Point)"
gwip "add auth middleware"   # Custom message
```

---

### 🔧 Project Initialization

| Command      | Description                                                                      |
| ------------ | -------------------------------------------------------------------------------- |
| `ii`         | Interactive project init — choose Bun or NPM, auto-creates `.gitignore`          |
| `next`       | Scaffold Next.js app (`create-next-app`) via Bun or NPM                          |
| `vite`       | Scaffold Vite project with optional Tailwind CSS v4 setup                        |
| `ui`         | Install & init Shadcn/UI with optional component selection                       |
| `css`        | Auto-detect package manager and install Tailwind CSS + `clsx` + `tailwind-merge` |
| `run`        | Interactive JS/TS file runner via Bun                                            |
| `pg`         | Generate `package.json` for current project                                      |
| `makecpp`    | Advance C/C++ boilerplate generator (auto cd, makefile, git, vscode)             |
| `make run`   | Compile and run the generated C/C++ project                                      |
| `make clean` | Remove compiled binary file                                                      |

#### `vite` example flow:

```bash
vite
# ⚡ Setup Vite with:
# 1) Bun  2) NPM
# Add Tailwind CSS v4? (y/n): y
# → Installs packages, creates src/index.css with @import "tailwindcss"
```

---

### ⚙️ System & Maintenance

| Command     | Description                                                                                          |
| ----------- | ---------------------------------------------------------------------------------------------------- |
| `uup`       | **Mega Updater** — interactive fzf menu: OS core, Snap, Flatpak, Bun, Node.js, NPM, deep clean       |
| `uu`        | **Universal Uninstaller** — fuzzy search across apt/snap/flatpak/AppImage, shows size & install date |
| `uc`        | Universal system clean (cache, orphans, logs)                                                        |
| `update`    | Update system packages                                                                               |
| `clean`     | Clean apt cache & remove orphaned packages                                                           |
| `setuppc`   | Bootstrap a new PC with all essential developer tools                                                |
| `rt`        | Install Node.js (via nvm), Bun, and Deno                                                             |
| `ut`        | Setup optimized CLI tooling for the PC                                                               |
| `rel`       | Reload `.bashrc` configuration                                                                       |
| `myip`      | Show your public IP address                                                                          |
| `iploc`     | Show IP + city/region/org info via `ipinfo.io`                                                       |
| `ports`     | List all open ports                                                                                  |
| `kp <port>` | Kill the process running on a given port                                                             |
| `serve`     | Start a local Python HTTP server in current directory                                                |
| `rn`        | Rename all files — removes special characters (`@`, `%`, `*`, `#`)                                   |

#### `uup` — Interactive Mega Updater

```bash
uup
# Opens fzf menu:
# 0. ALL_MAINTENANCE_TASKS
# 1. Core_System_Update
# 2. Snap_Package_Refresh
# 3. Flatpak_Cleanup_Update
# 4. Bun_Runtime_Upgrade
# 5. Node.js_LTS_Sync
# 6. Global_NPM_Update
# 7. Full_System_Deep_Clean
```

#### `uu` — Universal Uninstaller

```bash
uu
# Opens fzf picker with all installed apps (apt + snap + flatpak + AppImage)
# Columns: IDX | NAME | SOURCE | VERSION | SIZE | INSTALL DATE
# TAB to multi-select, ENTER to purge with animated progress bar
# Automatically runs turbo-clean after removal
```

---

### 🔨 Utility Tools

| Command        | Description                                                 | Example                      |
| -------------- | ----------------------------------------------------------- | ---------------------------- |
| `mkd <name>`   | Create directory and `cd` into it                           | `mkd my-app`                 |
| `t <file>`     | Create a file with feedback                                 | `t index.js`                 |
| `rmd <name>`   | Force remove directory recursively                          | `rmd old-build`              |
| `rmf <file>`   | Safely remove a file                                        | `rmf config.bak`             |
| `bak <file>`   | Create a `.bak` backup copy                                 | `bak .env`                   |
| `trash <file>` | Move file to system trash (safe delete)                     | `trash temp.log`             |
| `ex <archive>` | Extract any archive format                                  | `ex project.tar.gz`          |
| `ff <name>`    | Find file by name (skips `node_modules`, `.git`)            | `ff tsconfig`                |
| `gen <len>`    | Generate a cryptographically secure secret key              | `gen 32`                     |
| `completions`  | Shell auto-completion generator (auto-loaded in shell init) | `gladeshell completions zsh` |
| `h <word>`     | Search command history                                      | `h docker`                   |
| `to`           | Open current directory in VS Code                           |                              |
| `v`            | Play video in terminal                                      |                              |
| `c` / `cls`    | Clear the terminal screen                                   |                              |

#### Archive formats supported by `ex`:

`.tar.bz2` · `.tar.gz` · `.bz2` · `.rar` · `.gz` · `.tar` · `.zip` · `.7z`

---

### 🚀 Interactive Utilities (GUM & FZF)

> These utilities use `gum` and/or `fzf` for rich interactive UIs.
> They degrade gracefully: **gum → fzf → plain `read` prompt** — no tool is strictly required.

#### 📋 Todo Manager

Tasks are saved to `~/.todo_list.txt`.

| Command           | Description                                           |
| ----------------- | ----------------------------------------------------- |
| `todo`            | Open interactive gum menu, or show numbered task list |
| `todo add "Task"` | Add a new task directly                               |
| `todo add`        | Add task via interactive prompt (gum / read)          |
| `todo list`       | Show all pending tasks (numbered)                     |
| `todo done`       | Mark done — fzf picker → gum chooser → ask for number |
| `todo done 2`     | Mark task #2 as done directly                         |
| `todo clear`      | Clear all tasks                                       |
| `todo --help`     | Show usage                                            |

#### 📝 Notes Manager

Notes are stored in `~/.my_notes/<Category>/<Title>.md`.

| Command        | Description                                            |
| -------------- | ------------------------------------------------------ |
| `notes`        | Browse all notes with fzf + live preview               |
| `notes add`    | Add a note — pick category, enter title, write content |
| `notes search` | Full-text search inside all notes with fzf             |
| `notes find`   | Alias for `notes search`                               |
| `notes --help` | Show usage                                             |

**Viewer fallback:** `glow` → `bat` → `batcat` → `less`
**Preview (fzf):** `bat` → `batcat` → `glow` → `cat`
**Clipboard:** `wl-copy` (Wayland) → `xclip` → `xsel` → `pbcopy` (macOS)

#### 🎬 FFmedia All-in-One Multimedia Suite

Interactive FFmpeg powerhouse driven by `gum`, `fzf`, and terminal prompts.

| Command                    | Description                                                  |
| -------------------------- | ------------------------------------------------------------ |
| `ffmedia`                  | Launch interactive 24-in-1 FFmpeg multimedia menu            |
| `ffstudio`                 | Alias for `ffmedia`                                          |
| `fftool`                   | Alias for `ffmedia`                                          |
| `glade_ffmpeg`             | Alias for `ffmedia`                                          |
| `ffmedia compress`         | Compress video preserving quality (50%-80% size reduction)   |
| `ffmedia trim`             | Lossless video trim without re-encoding                      |
| `ffmedia concat`           | Merge multiple video clips into one file                     |
| `ffmedia resolution`       | Convert resolution (1080p/720p) or crop to 9:16 Reels/Shorts |
| `ffmedia speed`            | Slow Motion (0.25x-0.5x) or Time-lapse (2x-8x)               |
| `ffmedia rotate`           | Rotate (90°/180°) or Flip horizontally/vertically            |
| `ffmedia watermark`        | Apply image logo or text banner watermark                    |
| `ffmedia grid`             | Side-by-Side (2 videos) or 2x2 grid (4 videos) comparison    |
| `ffmedia audio-extract`    | Extract audio to MP3, AAC, WAV, FLAC, M4A                    |
| `ffmedia mute`             | Strip audio stream completely from video                     |
| `ffmedia audio-replace`    | Replace or mix background audio with video track             |
| `ffmedia loudness`         | Loudness Normalization (-14 LUFS YouTube / -23 LUFS EBU)     |
| `ffmedia visualizer`       | Generate Waveform or Frequency Spectrum video from audio     |
| `ffmedia audio-speed`      | Change audio playback speed while preserving pitch           |
| `ffmedia snapshot`         | Extract Ultra HD image frame (JPG/PNG) at exact timestamp    |
| `ffmedia bulk-frames`      | Bulk extract video frames as image sequence                  |
| `ffmedia gif`              | Render pro-quality ultra-sharp GIF using 2-pass palette      |
| `ffmedia contact-sheet`    | Generate 3x3 or 4x4 mosaic thumbnail grid image              |
| `ffmedia screen-record`    | Record desktop screen + audio straight from terminal         |
| `ffmedia subtitle-burn`    | Hardcode .srt or .ass subtitle file into video               |
| `ffmedia subtitle-extract` | Extract embedded subtitle tracks from MKV/MP4                |
| `ffmedia privacy-clean`    | Remove EXIF, GPS location, and camera metadata               |
| `ffmedia convert`          | Convert format between MP4, MKV, WEBM, MOV, AVI              |
| `ffmedia batch`            | Run bulk compression/conversion/metadata wiping on a folder  |

#### 🔐 Hardened Multi-Vault Security Suite

Universal Linux & macOS directory vault manager with AES-256 PBKDF2 memory guard, panic password decoy mode, anti-brute-force protection, auto-relock timers, and optional Telegram security alerts.

| Command        | Description                                                                     |
| -------------- | ------------------------------------------------------------------------------- |
| `vault`        | Open interactive GUM/FZF vault menu (Lock, Unlock, List, Config)                |
| `secvault`     | Alias for `vault`                                                               |
| `fvault`       | Alias for `vault`                                                               |
| `vault lock`   | Interactive directory encryption & secure source wiping                         |
| `vault unlock` | Decrypt vault directly into RAM (`/dev/shm`) and symlink to `$HOME`             |
| `vault list`   | List all encrypted vaults in `~/.secret_vaults` with file sizes                 |
| `vault config` | Configure Telegram bot token & chat ID for instant security alert notifications |
| `vault --help` | Show full usage guide & features                                                |

**Key Security Features:**

- **AES-256 CBC + PBKDF2 (500,000 Iterations)** encryption using OpenSSL.
- **RAM Execution Guard:** Decrypts files into `/dev/shm` (or `$TMPDIR`), keeping plain files out of persistent storage.
- **Panic Password Decoy:** Entering a panic password automatically wipes secret data, generates a decoy folder, triggers a Telegram security alert, and logs access.
- **Self-Destruct Sequence:** Permanently wipes vault data after 3 consecutive wrong password attempts.
- **Multi-pass Shredding:** Wipes original source folders with `shred -u -n 3 -z` (Linux) or `rm -P` (macOS).

#### ⚡ Dev Walk & Quick CD (`cf`)

Interactive fuzzy directory navigator and multi-media file launcher with rich live preview and interactive keybindings.

**Usage:** `cf [directory]` (Defaults to current directory `.`)

| Keybinding | Action / Operation                                                            |
| ---------- | ----------------------------------------------------------------------------- |
| `<ENTER>`  | Navigate into selected Directory OR open File in default app/editor           |
| `CTRL-Z`   | Switch candidate list to System Frecent Directories (`zoxide`)                |
| `CTRL-V`   | Play selected Video file with `v()` player                                    |
| `CTRL-P`   | Open selected PDF document in Browser                                         |
| `CTRL-O`   | Open file or directory in Code Editor (`code` / `cursor` / `nvim`)            |
| `CTRL-E`   | Open directory in GUI File Explorer (`nautilus` / `dolphin` / `explorer.exe`) |
| `CTRL-Y`   | Copy path to Clipboard (`wl-copy` / `xclip` / `clip.exe` / `pbcopy`)          |
| `CTRL-H`   | Navigate to Parent Directory (`..`) inside FZF                                |

**Live Preview Features:**

- 📁 **Directories:** Interactive folder tree (`eza` / `tree` / `ls`), Git active branch & recent 3 commits.
- 📄 **Code & Text:** Colorized syntax-highlighted preview via `bat` / `batcat`.
- 🖼️ **Images:** High-resolution terminal thumbnail preview powered by `chafa`.
- 📦 **Archives:** Previews `.zip`, `.tar.gz`, `.7z`, `.rar` contents without extracting.
- 🎵 **Audio & Video:** Audio metadata preview (`mediainfo`) and video quick actions.

#### 🔀 Other GUM / FZF Utilities

| `cf` | Advanced FZF Dev Walk & Quick CD (see above) |
| `gbranch` | Modern interactive Git branch manager (fzf / gum) |
| `fkill` | Advanced interactive process killer (fzf / gum) |
| `fcd` | Fuzzy quick directory jump (fzf / gum) |

---

### 🌿 `gbranch` — Modern Git Branch Manager

> **Usage:** `gbranch [-l] [-r] [-a] [-h]`

Replaces the basic `git branch` workflow with a fully interactive, fuzzy-searchable branch manager. Branches are sorted by **most recently committed** so your active branches are always at the top.

| Flag             | Description                      |
| ---------------- | -------------------------------- |
| _(none)_         | Show all local + remote branches |
| `-l`, `--local`  | Show local branches only         |
| `-r`, `--remote` | Show remote branches only        |
| `-a`, `--all`    | Explicitly show all branches     |
| `-h`, `--help`   | Print usage and keybindings      |

**FZF Keybindings:**

| Key      | Action                                                  |
| -------- | ------------------------------------------------------- |
| `Enter`  | Checkout selected branch (`git checkout`)               |
| `Ctrl-D` | Delete local branch (`git branch -D`) with confirmation |
| `Ctrl-R` | Rebase current branch onto selected (`git rebase`)      |
| `Ctrl-O` | Merge selected branch into current (`git merge`)        |

**Preview Panel:** Live commit graph with hash, author, date, and subject (`git log --graph --color`) shown on the right side as you navigate.

**Smart Remote Checkout:** Selecting `origin/feature-xyz` auto-strips the remote prefix and runs a clean `git checkout feature-xyz`.

**Fallback chain:** `fzf` → `gum filter` → numbered interactive menu (no dependencies required).

---

### ⚡ `fkill` — Advanced Interactive Process Killer

> **Usage:** `fkill [query] [port_number]`

A full-featured process manager in your terminal. Processes are sorted by **CPU usage** (highest first) so resource-hungry processes are instantly visible.

| Argument     | Description                                         |
| ------------ | --------------------------------------------------- |
| _(none)_     | Open all processes sorted by CPU descending         |
| `fkill node` | Pre-filter list to show only `node` processes       |
| `fkill 3000` | Directly find and kill the process on **port 3000** |

**FZF Keybindings:**

| Key      | Action                                                           |
| -------- | ---------------------------------------------------------------- |
| `Tab`    | Multi-select multiple processes                                  |
| `Enter`  | **Soft kill** selected process(es) (SIGTERM — graceful shutdown) |
| `Ctrl-X` | **Force kill** selected process(es) (SIGKILL -9 — immediate)     |
| `Ctrl-R` | Reload live process list without closing fzf                     |

**Process Type Badges:**

- `[APP]` — User GUI Applications (Chrome, VS Code, Slack, Discord, Firefox, VLC, Zed, Spotify, etc.)
- `[SYS]` — System Services & Daemons (`root`, `systemd`, `dbus`, background system workers)
- `[USR]` — User CLI, Scripts, and Shell processes (`node`, `python`, `npm`, `bash`, etc.)

**Preview Panel:** Shows Process Type, PID, User, CPU %, Memory %, Uptime, and open network ports for the highlighted process.

**Port-based kill:** `fkill 3000` uses `ss` / `lsof` to identify the process on that port, confirms with you, tries SIGTERM first, then SIGKILL if needed.

**Fallback chain:** `fzf` (multi-select + preview) → `gum filter` (with confirm) → numbered interactive menu.

---

## 🏗️ Project Structure

```
gladeshell/
├── src/                    # Pure Rust core application codebase
│   ├── main.rs             # CLI entrypoint & subcommand router
│   ├── cli.rs              # Clap CLI definitions & flag parsing
│   ├── config.rs           # Workspace & theme configuration loader
│   ├── theme.rs            # 55 prompt themes & ANSI styling engine
│   └── tools/              # Tool implementations & Ratatui TUIs
│       ├── compressor.rs   # Level 1 Fast multi-core zstd compressor
│       ├── extractor.rs    # Multi-format parallel extractor
│       ├── ffmedia.rs      # 24-in-1 FFmpeg multimedia suite
│       ├── vault.rs        # AES-256 PBKDF2 Multi-Vault Security Suite
│       ├── todo.rs         # Task manager Ratatui TUI
│       ├── notes.rs        # Notes manager Ratatui TUI
│       ├── filetree.rs     # File explorer Ratatui TUI
│       ├── process.rs      # Interactive Process Manager TUI
│       └── cleaner.rs      # System clean Ratatui TUI
│
├── benches/                # Criterion performance benchmarks
├── aliases.toml            # Centralized cross-shell alias registry
├── Cargo.toml              # Rust crate metadata & dependencies
├── Cargo.lock              # Tracked dependency lockfile
├── Makefile                # Cargo workflow shortcuts (check, clippy, test, bench)
│
├── install.sh              # Universal Unix/Linux/macOS setup script
├── install.ps1             # Universal Windows PowerShell setup script
│
├── web/                    # Static website & Netlify deployment
│   ├── index.html          # Landing page
│   ├── docs.html           # Interactive documentation page
│   ├── linux-setup.html    # Linux application ecosystem browser
│   ├── style.css           # Styling system & dark mode
│   ├── install.sh          # Mirrored installer
│   └── install.ps1         # Mirrored installer
│
├── .githooks/              # Automated Git hygiene hooks
│   ├── pre-commit          # Cargo fmt/clippy, sync guard, conflict marker check
│   └── pre-push            # Cargo check & test verification
│
├── .github/                # CI/CD Workflows
│   └── workflows/
│       ├── rust-ci.yml     # Multi-OS Rust compilation & test matrix
│       ├── release.yml     # Cross-platform release binaries
│       ├── shellcheck.yml  # Installer script validation
│       └── label.yml       # PR auto-labeling
│
├── .vscode/                # VS Code workspace settings
│   ├── settings.json       # Live Clippy on save & rust-analyzer config
│   ├── launch.json         # CodeLLDB debugging configuration
│   └── tasks.json          # Cargo task shortcuts
│
├── README.md               # User documentation (you are here)
├── ARCHITECTURE.txt        # Architecture design & system map
├── wiki.md                 # Extended wiki manual
├── ROADMAP.md              # Project roadmap & milestones
├── CHANGELOG.md            # Release version history
├── CONTRIBUTING.md         # Developer contribution guidelines
├── SUPPORT.md              # Support options
├── SECURITY.md             # Security policy & reporting
└── LICENSE                 # MIT License
```

---

## 🖥️ Zed IDE Settings

Install a fully-configured `settings.json` for the [Zed](https://zed.dev) editor — works for native, Flatpak, Snap, and Windows installations in one command.

### Instant Command Setup

Run the built-in installer command directly in your shell:

```bash
zed-setup
# or
gladeshell zed-setup
```

The command will:

1. 💾 **Back up** any existing `settings.json` with a timestamp
2. ✍️ **Write** the new config across all detected installation paths (Native, Flatpak, Snap, and Windows AppData)
3. ✅ Print a confirmation for each updated target path

> **Restart Zed** after running the command for all settings to take effect.

### What's included

| Setting            | Value                                                      |
| ------------------ | ---------------------------------------------------------- |
| Theme              | `BlackFox` (dark) / `Everforest Light Hard` (light)        |
| Buffer Font        | `Cascadia Code` 22px (fallback: JetBrains Mono, Fira Code) |
| UI Font            | `JetBrains Mono` 20px                                      |
| Terminal Font      | `JetBrains Mono` 22px + `FiraCode Nerd Font` fallback      |
| Tab Size           | `2` spaces                                                 |
| Soft Wrap          | `editor_width`                                             |
| Autosave           | `on_focus_change`                                          |
| Keymap             | `VSCode`                                                   |
| Inlay Hints        | Enabled with background                                    |
| Inline Diagnostics | Enabled                                                    |
| Minimap            | `auto`                                                     |
| Prettier           | Allowed                                                    |
| Git Inline Blame   | With commit summary                                        |

---

## 🐧 Linux App Ecosystem

A curated list of essential applications for a Linux development and creative environment.

This project provides a clean, searchable, and filterable web interface to discover and browse recommended Linux applications. It includes categories for Creative Applications, IDEs, Browsers, System Tools, and Dev Tools.

**Features:**

- **Searchable Interface:** Instantly search for applications by name, category, or format.
- **Dynamic Filters:** Filter applications by categories like Creative, Development IDE, Browser, Dev Tools, and Tools.
- **Package Formats:** Displays the recommended package format (Flatpak, DEB, etc.) for each application with color-coded badges.

This page has been integrated into the main `gladeshell` website and can be accessed via the **Linux Apps** link in the navigation menu.

The **[Docs page](https://gladeshell.netlify.app/docs.html)** (`docs.html`) provides a full interactive documentation experience with:

- 🔍 **Trie-based autocomplete** search (O(k) prefix lookup)
- ⚡ **Live Command Explorer** — filter 60+ aliases by category
- 🛠️ **Terminal Simulator** — click chips to demo gladeshell commands
- 🎨 **4 Color Themes** — Cyber Cyan, Matrix Green, Sunset Pink, Nord Frost
- 📱 **Fully Mobile Responsive** — hamburger drawer sidebar, touch-friendly layout

<details>
<summary><b>View the full list of recommended apps</b></summary>

### 🎨 Creative Applications

| Application      | Format  | Description                             |
| ---------------- | ------- | --------------------------------------- |
| **Flatseal**     | Flatpak | Flatpak App Managed Software            |
| **ytDownloader** | Flatpak | Video Downloader Software               |
| **Packet**       | Flatpak | Quick share for Linux                   |
| **Inkscape**     | Flatpak | Vector Image Editor                     |
| **VLC**          | Flatpak | Video Player Software                   |
| **Upscayl**      | Flatpak | Image Upscaling Software                |
| **Pinta**        | Flatpak | General Image Editor                    |
| **Discord**      | Flatpak | Social Media & Voice Chat               |
| **Pods**         | Flatpak | Containers Manager                      |
| **HandBrake**    | Flatpak | Video Compressor                        |
| **OBS Studio**   | Flatpak | Video Recorder & Streamer               |
| **Valot**        | Flatpak | Note & Task tracking with alarm         |
| **Collector**    | Flatpak | Drag and drop everything in one place   |
| **Gitte**        | Flatpak | Git Client Desktop Software             |
| **Kdenlive**     | Flatpak | Video Editor Software                   |
| **Bazaar**       | Flatpak | App store for Flatpak Applications      |
| **Akizip**       | Flatpak | Archive Manager (7z, ZIP, TAR)          |
| **BudsLink**     | Flatpak | Air buds Control for Linux              |
| **Emojify**      | Flatpak | Emoji finder                            |
| **Xournal++**    | Flatpak | Digital notebook / PDF Annotator        |
| **Drawy**        | Flatpak | Draw notebook                           |
| **Gradia**       | Flatpak | Screenshot Utility                      |
| **Scribus**      | Flatpak | Vector Image Print / Desktop Publishing |

### 💻 Development IDEs

| Application     | Format  | Description                              |
| --------------- | ------- | ---------------------------------------- |
| **VS Code**     | DEB     | Powerful code editor by Microsoft        |
| **Qoder**       | DEB     | Code Editor                              |
| **Antigravity** | DEV     | Advanced Agentic Coding Environment      |
| **Zed**         | Flatpak | High-performance multiplayer code editor |
| **VSCodium**    | Flatpak | Telemetry-free VS Code build             |

### 🌐 Browsers

| Application | Format  | Description                 |
| ----------- | ------- | --------------------------- |
| **Chrome**  | DEB     | Google Web Browser          |
| **Brave**   | Flatpak | Privacy-focused Web Browser |

### 🛠️ System Tools & Utilities

| Application   | Format | Category                   |
| ------------- | ------ | -------------------------- |
| **rEFInd**    | DEB    | Dual boot Manager          |
| **Zram**      | DEB    | Memory compression in RAM  |
| **Fzf**       | DEB    | Command-line fuzzy finder  |
| **ls-sensor** | DEB    | Hardware sensor monitoring |
| **Git**       | DEB    | Version Control System     |
| **Node.js**   | DEB    | JavaScript Runtime         |
| **Bun**       | DEB    | Fast JavaScript Runtime    |
| **curl**      | DEB    | Network Data Transfer      |
| **wget**      | DEB    | Network File Retrieval     |

</details>

---

## 🐳 Docker & Containers

> gladeshell includes **The Ultimate Docker Swiss Army Knife** — a full suite of aliases and smart functions for managing containers, images, volumes, and services.

### 📊 Interactive Dashboard & Monitoring

| Command  | Description                                                   |
| -------- | ------------------------------------------------------------- |
| `dman`   | 🐳 Docker Desktop & DevOps Terminal Edition (interactive TUI) |
| `dstats` | Live realtime resource dashboard (CPU, RAM, Net IO, PIDs)     |
| `dps`    | List running containers (clean table format)                  |
| `dpsa`   | List **all** containers including stopped ones                |
| `di`     | List all downloaded Docker images                             |
| `dvl`    | List all Docker volumes                                       |
| `dnl`    | List all Docker networks                                      |
| `dsize`  | Inspect total Docker disk usage                               |
| `dtop`   | Live resource monitor — CPU, RAM, Net & Block I/O             |

#### Sudo variants (for rootless-mode setups)

| Command            | Description                       |
| ------------------ | --------------------------------- |
| `sdps` / `sdpsa`   | `sudo` versions of `dps` / `dpsa` |
| `sdi`              | `sudo docker images`              |
| `sdvl` / `sdnl`    | `sudo` volume / network list      |
| `sdsize` / `sdtop` | `sudo` disk usage / live stats    |

---

### ⚡ Service Control

| Command    | Description                                               |
| ---------- | --------------------------------------------------------- |
| `dstart`   | Start the Docker service                                  |
| `doff`     | Stop the Docker service                                   |
| `dstatus`  | Check Docker service status                               |
| `denable`  | Enable Docker auto-start on boot (docker + docker.socket) |
| `ddisable` | Disable Docker auto-start on boot                         |

---

### 🔄 Container Lifecycle

| Command           | Description                                    |
| ----------------- | ---------------------------------------------- |
| `dstop <name>`    | Stop a container                               |
| `drm <name>`      | Remove a container                             |
| `drmi <image>`    | Remove an image                                |
| `drestart <name>` | Restart a container                            |
| `dkill <name>`    | Force stop + delete a container in one command |
| `dstopall`        | Stop **all** running containers at once        |
| `drmall`          | Remove **all** stopped containers at once      |

---

### 🐛 Debugging & Building

| Command                | Description                                  | Example                  |
| ---------------------- | -------------------------------------------- | ------------------------ |
| `dsh <name>`           | Open an interactive shell inside a container | `dsh myapp`              |
| `dlogs <name>`         | Follow live logs of a container              | `dlogs myapp`            |
| `dbuild <tag>`         | Build a Docker image with a tag              | `dbuild myapp .`         |
| `dbuild-nocache <tag>` | Build image from scratch (no cache)          | `dbuild-nocache myapp .` |
| `dhist <image>`        | View image layer history                     | `dhist myapp`            |
| `dports <name>`        | Check open port bindings of a container      | `dports myapp`           |

---

### 🧩 Docker Compose

| Command  | Description                                              |
| -------- | -------------------------------------------------------- |
| `dcup`   | Start services in detached mode (`docker compose up -d`) |
| `dcdn`   | Stop and remove services (`docker compose down`)         |
| `dclogs` | Follow compose service logs                              |
| `dcupb`  | Rebuild images and start services (`up -d --build`)      |

---

### 🧪 Quick Test Sandboxes

Spin up a temporary container that **auto-deletes on exit**:

| Command        | Launches                  |
| -------------- | ------------------------- |
| `dtest-ubuntu` | `ubuntu:latest` with bash |
| `dtest-node`   | `node:alpine` with sh     |
| `dtest-alpine` | `alpine:latest` with sh   |

---

### 🧠 Advanced Functions

| Function               | Usage                       | Description                                                            |
| ---------------------- | --------------------------- | ---------------------------------------------------------------------- |
| `dfind <term>`         | `dfind nginx`               | Search containers and images by name                                   |
| `droot <name>`         | `droot myapp`               | Enter container as **root** user                                       |
| `dip <name>`           | `dip myapp`                 | Show container's local IP address                                      |
| `dwatch <name>`        | `dwatch myapp`              | Live-track filesystem changes inside container                         |
| `dnetstat <name>`      | `dnetstat myapp`            | Show active network connections inside container                       |
| `dtop-proc <name>`     | `dtop-proc myapp`           | Show process tree inside container                                     |
| `dbackup <vol> <file>` | `dbackup mydata backup.tar` | Backup a Docker volume as a `.tar` file                                |
| `dkill-force`          | `dkill-force`               | Interactively force-kill **all** running containers                    |
| `dclean`               | `dclean`                    | Deep clean — removes all unused containers, images, volumes & networks |

#### `dclean` — Deep Clean

```bash
dclean
# 🧹 Performing deep clean of all unused Docker resources...
# → docker system prune -a --volumes -f
# ✨ System optimization complete!
```

#### `dbackup` — Volume Backup

```bash
dbackup mydata backup.tar
# Backs up 'mydata' volume to backup.tar in current directory
```

> 💡 **Tab Completion** is built-in — press `Tab` after `dsh`, `dlogs`, `dstop`, `dkill`, `drestart`, `dports`, `dwatch`, `dnetstat`, `dtop-proc` to auto-complete container names. Same for `drmi` and `dhist` with image names.

---

## 🐘 PostgreSQL

> gladeshell includes a full suite of **PostgreSQL aliases** for managing your database service with minimal typing.

### 🔌 Service Control

| Alias       | Description                           |
| ----------- | ------------------------------------- |
| `pgstart`   | Start the PostgreSQL service          |
| `pgstop`    | Stop the PostgreSQL service           |
| `pgrestart` | Restart the PostgreSQL service        |
| `pgstatus`  | Check PostgreSQL service status       |
| `pgenable`  | Enable PostgreSQL auto-start on boot  |
| `pgdisable` | Disable PostgreSQL auto-start on boot |
| `pglogs`    | Follow the PostgreSQL log file live   |

### 🗄️ Database Management

| Alias            | Usage                         | Description                           |
| ---------------- | ----------------------------- | ------------------------------------- |
| `pgl`            | `pgl`                         | Login as `postgres` user via `psql`   |
| `pgdb <name>`    | `pgdb mydb`                   | Connect to a specific database        |
| `pgls`           | `pgls`                        | List all databases (`\l`)             |
| `pgtables`       | `pgtables`                    | List all tables in current DB (`\dt`) |
| `pgusers`        | `pgusers`                     | List all users / roles (`\du`)        |
| `pgsize`         | `pgsize`                      | Show size of each database            |
| `pgver`          | `pgver`                       | Show PostgreSQL version               |
| `pgconn`         | `pgconn`                      | Show active connections count         |
| `pgcreate <db>`  | `pgcreate mydb`               | Create a new database                 |
| `pgdrop <db>`    | `pgdrop mydb`                 | Drop / delete a database              |
| `pgdump <db>`    | `pgdump mydb > backup.sql`    | Dump / backup a database              |
| `pgrestore <db>` | `pgrestore mydb < backup.sql` | Restore a database from file          |

---

## 💎 Prisma ORM

> gladeshell includes a complete suite of **Prisma ORM aliases** using the **first letter of each word** for maximum typing speed.
>
> Pattern: `n`(px) + `p`(risma) + sub-command initials → **Node** | `b`(unx) + `p`(risma) + sub-command initials → **Bun**

### 🟢 Node / NPX Prisma — `np*`

| Alias          | Full Command                           | Description                                  |
| -------------- | -------------------------------------- | -------------------------------------------- |
| `np`           | `npx prisma`                           | Base Prisma CLI command                      |
| `npi`          | `npx prisma init`                      | Initialize Prisma project                    |
| `npg`          | `npx prisma generate`                  | Generate Prisma Client                       |
| `nps`          | `npx prisma studio`                    | Open Prisma Studio GUI                       |
| `npmd`         | `npx prisma migrate dev`               | Run dev migrations                           |
| `npmdn <name>` | `npx prisma migrate dev --name <name>` | Run named migration (e.g. `npmdn add_users`) |
| `npmr`         | `npx prisma migrate reset`             | Reset database & re-migrate                  |
| `npmdp`        | `npx prisma migrate deploy`            | Apply migrations in production               |
| `npms`         | `npx prisma migrate status`            | Check migration status                       |
| `npdp`         | `npx prisma db push`                   | Push schema state directly to DB             |
| `npdl`         | `npx prisma db pull`                   | Pull schema from DB / Introspect             |
| `npds`         | `npx prisma db seed`                   | Seed the database                            |
| `npf`          | `npx prisma format`                    | Format `schema.prisma` file                  |
| `npv`          | `npx prisma version`                   | Show Prisma CLI & engine version             |

### 🥐 Bun Runtime Prisma — `bp*`

| Alias          | Full Command                            | Description                         |
| -------------- | --------------------------------------- | ----------------------------------- |
| `bp`           | `bunx prisma`                           | Base Prisma CLI via Bun runner      |
| `bpi`          | `bunx prisma init`                      | Initialize Prisma project via Bun   |
| `bpg`          | `bunx prisma generate`                  | Generate Prisma Client via Bun      |
| `bps`          | `bunx prisma studio`                    | Open Prisma Studio GUI via Bun      |
| `bpmd`         | `bunx prisma migrate dev`               | Run dev migrations via Bun          |
| `bpmdn <name>` | `bunx prisma migrate dev --name <name>` | Run named migration via Bun         |
| `bpmr`         | `bunx prisma migrate reset`             | Reset database via Bun              |
| `bpmdp`        | `bunx prisma migrate deploy`            | Apply migrations in prod via Bun    |
| `bpms`         | `bunx prisma migrate status`            | Check migration status via Bun      |
| `bpdp`         | `bunx prisma db push`                   | Push schema directly to DB via Bun  |
| `bpdl`         | `bunx prisma db pull`                   | Pull schema from DB via Bun         |
| `bpds`         | `bunx prisma db seed`                   | Seed database via Bun               |
| `bpf`          | `bunx prisma format`                    | Format `schema.prisma` file via Bun |
| `bpv`          | `bunx prisma version`                   | Check Prisma version via Bun        |

---

## 🤝 Contributing

Contributions are welcome! Whether it's a new alias, a bug fix, or a feature idea:

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feat/my-feature`
3. **Commit** your changes: `gcm "feat: add my feature"` _(or regular `git commit`)_
4. **Push** and open a **Pull Request**

Please keep functions focused, well-commented, and compatible with **Bash 4+**.

### 📚 Community & Documentation Links

- 🤝 **[CONTRIBUTING.md](CONTRIBUTING.md)** — Step-by-step guidelines for contributing to gladeshell.
- 🆘 **[SUPPORT.md](SUPPORT.md)** — Support options, quick troubleshooting, and issue reporting.
- 🗺️ **[ROADMAP.md](ROADMAP.md)** — Future feature plans, version milestones, and community voting.
- 👥 **[AUTHORS.md](AUTHORS.md)** — Core maintainers and project leadership.
- 🤝 **[CONTRIBUTORS.md](CONTRIBUTORS.md)** — Community contributor recognition wall.
- 🌟 **[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)** — Community standards and covenant.
- 🔒 **[SECURITY.md](SECURITY.md)** — Vulnerability reporting policy.

---

## 📄 License

MIT © [Rihad Jahan Opu](https://github.com/rihadjahanopu)

---

<div align="center">

**If gladeshell saves you time daily, give it a ⭐ — it helps others find it!**

<br>

Made with ❤️ in Bangladesh

[![Website](https://img.shields.io/badge/Website-gladeshell.netlify.app-22d3ee?style=for-the-badge&logo=netlify&logoColor=white)](https://gladeshell.netlify.app)
[![GitHub](https://img.shields.io/badge/GitHub-rihadjahanopu-181717?style=for-the-badge&logo=github)](https://github.com/rihadjahanopu)

© 2026 Rihad Jahan Opu. All rights reserved.

</div>
