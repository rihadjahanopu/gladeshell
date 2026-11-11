# 🖥️ IDE & Editor Setup Suites (`code_setup` & `zed_setup`)

- **Primary Sources:**
  - VS Code Setup: [`src/tools/code_setup.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/code_setup.rs) (1,606 LoC)
  - Zed Setup: [`src/tools/zed_setup.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/zed_setup.rs) (857 LoC)
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Role

Developers waste hours configuring new editor installations with optimal fonts, formatter settings, linting rules, and essential extensions.

These setup suites detect the host platform and editor installations (Native, Flatpak, Snap, Homebrew, Windows AppData) and install curated, production-tuned configurations in one command.

---

## 2. ⚡ VS Code Automated Configurer (`code_setup`)

- **JSON Merge Engine:** Intelligently merges settings into `settings.json` without destroying existing custom keybindings.
- **Font & Ligatures:** Configures JetBrains Mono / Fira Code font ligatures, line heights, and smooth cursor animations.
- **Language Configurations:** Sets up format-on-save, Rust Analyzer, ESLint, Prettier, Tailwind CSS, Clangd, and Go language servers.
- **Essential Extensions Auto-Installer:** Optionally batch-installs verified developer extensions:
  - `rust-lang.rust-analyzer`
  - `vadimcn.vscode-lldb`
  - `bradlc.vscode-tailwindcss`
  - `esbenp.prettier-vscode`
  - `dbaeumer.vscode-eslint`

---

## 3. ⚡ Zed IDE Setup Suite (`zed_setup`)

- Configures Zed's `settings.json` and `keymap.json`.
- Detects Flatpak (`~/.var/app/dev.zed.Zed`), Snap (`~/snap/zed`), macOS (`~/Library/Application Support/Zed`), and Windows (`%APPDATA%\Zed`).
- Synchronizes theme with Gladeshell's active prompt palette (e.g. Catppuccin, Tokyo Night, Dracula, Gruvbox).

---

## 4. 💻 CLI Usage

```bash
# Configure VS Code settings & extensions
code-setup

# Configure Zed editor settings & language servers
zed-setup
```
