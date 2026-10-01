<div align="center">

<br>

```
   ██████╗ ██╗    █████╗ ██████╗ ███████╗███████╗██╗  ██╗███████╗██╗   ██╗
  ██╔════╝ ██║   ██╔══██╗██╔══██╗██╔════╝██╔════╝██║  ██║██╔════╝██║   ██║
  ██║  ███╗██║   ███████║██║  ██║█████╗  ███████╗███████║█████╗  ██║   ██║
  ██║   ██║██║   ██╔══██║██║  ██║██╔══╝  ╚════██║██║  ██║██╔══╝  ██║   ██║
  ╚██████╔╝██████╗██║  ██║██████╔╝███████╗███████║██║  ██║███████╗██████╗██████╗
   ╚═════╝ ╚═════╝╚═╝  ╚═╝╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝╚══════╝╚═════╝╚═════╝
```

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
- [🚀 Quick Install](#-quick-install)
- [🗑️ Uninstall](#️-uninstall)
- [⚙️ Font Setup](#️-font-setup-for-emoji--icons)
- [📟 Smart Prompt System](#-smart-prompt-system)
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
  - [🔨 Utility Tools](#-utility-tools)
  - [🗜️ Fast Multi-Core Compressor & Extractor](#️-fast-multi-core-compressor--extractor)
  - [🎬 FFmedia Multimedia Suite](#-ffmedia-all-in-one-multimedia-suite)
  - [🔐 Hardened Multi-Vault Security Suite](#-hardened-multi-vault-security-suite)
  - [🖥️ Native Ratatui TUI Modules](#️-native-ratatui-tui-modules)
  - [🐳 Docker & Containers](#-docker--containers)
  - [🐘 PostgreSQL](#-postgresql)
  - [💎 Prisma ORM](#-prisma-orm)
- [🏗️ Project Structure](#️-project-structure)
- [💻 Development & Testing](#-development--testing)
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
| 🔄 **Cross-Shell Integration** | Built-in shell integration generator for Bash, Zsh, Fish, and PowerShell (`gladeshell shell-init`)     |
| 📦 **Universal Uninstaller**   | `uu` — interactive fuzzy application remover across apt/snap/flatpak/AppImage                         |
| 🔁 **Mega Updater**            | `uup` — updates system packages, runtimes, and flatpaks in one command                                |

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

**Built-in dynamic prompt features:**

- Folder-aware emoji matching (Web `🌐`, Node `🟢`, Bun `🥐`, Python `🐍`, Rust `🦀`).
- Git dirty status (`❗`), active branch detection, and unpushed commit counters.
- System metrics: CPU temperature, free disk space, load average, and process counters.
- Command execution timer (`⏱️ Ns` for commands taking >1s).
- Read-only filesystem warning indicator (`🔒`).

---

## 🎨 Prompt Themes

gladeshell includes **55 built-in themes** inspired by popular terminal prompts.

### 🖼️ Preview All 55 Themes

```bash
gladeshell theme preview
# or shorthand:
glade preview
```

### ⚡ Switch Themes

```bash
# Interactive TUI picker:
glade

# Direct selection:
glade catppuccin
glade tokyonight
glade dracula
glade starship
glade matrix
glade rosepine
```

### 📋 All 55 Themes

| #   | Theme               | Style                                 | Inspiration            |
| --- | ------------------- | ------------------------------------- | ---------------------- |
| 01  | `minimal`           | 🌈 Emoji + color · 1-line             | Gladeshell default      |
| 02  | `full`              | 📊 3-line with all metrics            | Gladeshell full info    |
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

Run `gladeshell --help` to view all CLI tool modules.

---

### 📂 Navigation & Movement

| Command               | Action                                        |
| --------------------- | --------------------------------------------- |
| `..`                  | Go up one directory                           |
| `...`                 | Go up two directories                         |
| `....`                | Go up three directories                       |
| `dev`                 | Jump to `~/Development`                       |
| `fr` / `ba` / `fu`    | Jump to Frontend / Backend / Fullstack folder |
| `des` / `doc` / `dow` | Jump to Desktop / Documents / Downloads       |

---

### 📦 NPM & Bun Commands

| Alias | Expands To       |
| ----- | ---------------- |
| `ni`  | `npm install`    |
| `nid` | `npm install -D` |
| `nr`  | `npm run`        |
| `nrd` | `npm run dev`    |
| `bi`  | `bun install`    |
| `brd` | `bun run dev`    |
| `brb` | `bun run build`  |

---

### 🌿 Git Version Control

| Command       | Description                    |
| ------------- | ------------------------------ |
| `gi`          | Initialize new git repository  |
| `gs`          | Git status (short format)      |
| `ga`          | Stage all files (`git add .`)  |
| `gcm "msg"`   | Commit with message            |
| `gps` / `gpl` | Push / Pull from remote        |
| `gl`          | Pretty git log graph           |
| `gwip "msg"`  | Work-in-progress commit & push |

---

### 🔧 Project Initialization

| Command | Description                                           |
| ------- | ----------------------------------------------------- |
| `ii`    | Interactive project init (Bun or NPM, auto gitignore) |
| `next`  | Scaffold Next.js app                                  |
| `vite`  | Scaffold Vite project with optional Tailwind CSS v4   |
| `ui`    | Install & initialize Shadcn/UI                        |
| `css`   | Install Tailwind CSS + `clsx` + `tailwind-merge`      |

---

### ⚙️ System & Maintenance

| Command     | Description                                           |
| ----------- | ----------------------------------------------------- |
| `uup`       | Mega Updater — OS, Snap, Flatpak, Bun, Node.js update |
| `uu`        | Universal Uninstaller — fuzzy application remover     |
| `uc`        | Universal system clean                                |
| `myip`      | Display public IP address                             |
| `ports`     | List all active open ports                            |
| `kp <port>` | Kill process running on a specific port               |

---

### 🔨 Utility Tools

| Command        | Description                                  | Example         |
| -------------- | -------------------------------------------- | --------------- |
| `mkd <name>`   | Create directory and `cd` into it            | `mkd my-app`    |
| `rmd <name>`   | Recursive directory removal                  | `rmd build`     |
| `bak <file>`   | Create `.bak` timestamped copy               | `bak .env`      |
| `gen <len>`    | Generate cryptographically secure secret key | `gen 32`        |
| `ex <archive>` | Multi-format archive extractor               | `ex app.tar.gz` |

---

### 🗜️ Fast Multi-Core Compressor & Extractor

`gladeshell` includes a high-throughput parallel multi-core compression engine:

```bash
# Compress folder using Level 1 Fast parallel mode:
gladeshell compressor /path/to/source output.tar.zst

# Extract archive:
gladeshell extractor archive.tar.zst /path/to/destination
```

**Compression Level:** **Level 1 (Fast)** — configured for ultra-fast archive generation utilizing all CPU threads.

---

### 🎬 FFmedia All-in-One Multimedia Suite

Launch via `gladeshell ffmedia` or `ffmedia`:

| Command / Option        | Description                                                 |
| ----------------------- | ----------------------------------------------------------- |
| `ffmedia compress`      | High-efficiency video compression preserving visual quality |
| `ffmedia trim`          | Lossless video trim without re-encoding                     |
| `ffmedia concat`        | Join multiple video files                                   |
| `ffmedia visualizer`    | Audio waveform & spectrum video generation                  |
| `ffmedia gif`           | High-quality 2-pass palette GIF renderer                    |
| `ffmedia screen-record` | Terminal-driven desktop & audio recording                   |
| `ffmedia privacy-clean` | Strip EXIF, GPS location, and camera metadata               |

---

### 🔐 Hardened Multi-Vault Security Suite

Launch via `gladeshell vault` or `vault`:

```bash
# Interactive vault manager:
gladeshell vault

# Direct commands:
gladeshell vault lock /path/to/folder
gladeshell vault unlock /path/to/vault
gladeshell vault list
```

**Key Security Features:**

- AES-256 CBC + PBKDF2 (500,000 iterations) OpenSSL encryption.
- Decrypts directly into RAM (`/dev/shm`) to keep plaintext off disk storage.
- Panic password decoy mode with automated security notification.
- Multi-pass source folder shredding (`shred -u -n 3 -z`).

---

### 🖥️ Native Ratatui TUI Modules

`gladeshell` features native terminal user interfaces powered by Ratatui:

```bash
gladeshell todo             # Interactive Task Manager TUI
gladeshell notes            # Interactive Markdown Notes TUI
gladeshell filetree         # Interactive Filetree & Directory Explorer TUI
gladeshell process_manager  # Interactive Process Killer & Resource TUI
gladeshell system_clean     # Interactive System Disk Cleanup TUI
```

---

### 🐳 Docker & Containers

| Command           | Description                                      |
| ----------------- | ------------------------------------------------ |
| `dman`            | Interactive Docker TUI Manager                   |
| `dstats`          | Realtime container resource dashboard            |
| `dps` / `dpsa`    | Container listing                                |
| `dstart` / `doff` | Docker daemon service control                    |
| `dclean`          | Deep clean of unused volumes, containers, images |

---

### 🐘 PostgreSQL

| Alias                | Description                      |
| -------------------- | -------------------------------- |
| `pgstart` / `pgstop` | Service management               |
| `pgl`                | Connect via `psql` as `postgres` |
| `pgls`               | List databases                   |
| `pgdump <db>`        | Backup database                  |

---

### 💎 Prisma ORM

| Node (`np*`) | Bun (`bp*`) | Description          |
| ------------ | ----------- | -------------------- |
| `npg`        | `bpg`       | `prisma generate`    |
| `npmd`       | `bpmd`      | `prisma migrate dev` |
| `nps`        | `bps`       | `prisma studio`      |

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

## 💻 Development & Testing

### Cargo Commands & Makefile

```bash
# Check code syntax & dependencies:
make check       # or: cargo check

# Run Linter with strict warnings:
make clippy      # or: cargo clippy --all-targets -- -D warnings

# Format codebase:
make fmt         # or: cargo fmt

# Run test suite:
make test        # or: cargo test

# Run benchmarks:
make bench       # or: cargo bench
```

### Git Hooks Setup

To activate pre-commit and pre-push hooks:

```bash
make hooks       # or: git config core.hooksPath .githooks
```

The `pre-commit` hook automatically checks:

1. Installer synchronization (`install.sh` and `install.ps1` matching `web/`).
2. Absence of unresolved merge conflict markers.
3. Code formatting via `cargo fmt -- --check`.
4. Linting via `cargo clippy`.

---

## 🐧 Linux App Ecosystem

gladeshell includes a curated guide to desktop applications for Linux developers:

- **Creative:** Inkscape, Kdenlive, OBS Studio, HandBrake, Upscayl.
- **IDEs:** VS Code, Zed, Antigravity.
- **Browsers:** Google Chrome, Brave.
- **Utilities:** Flatseal, Fzf, Zram.

Explore the searchable interactive web database on [gladeshell.netlify.app/linux-setup.html](https://gladeshell.netlify.app/linux-setup.html).

---

## 🤝 Contributing

Contributions are welcome!

1. **Fork** the repository.
2. **Create** a feature branch (`git checkout -b feat/new-feature`).
3. **Commit** your changes (`git commit -m "feat: add feature"`).
4. **Verify** with `make check && make test`.
5. **Push** and submit a **Pull Request**.

Refer to [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

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
