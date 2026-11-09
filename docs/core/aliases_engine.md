# ⚡ Aliases Engine & Cross-Shell Transpiler

- **Primary Sources:**
  - [`src/core/aliases/mod.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/core/aliases/mod.rs)
  - [`src/core/aliases/categories/`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/core/aliases/categories/)
  - Root configuration: [`aliases.toml`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/aliases.toml)
- **Architectural Domain:** Centralized Alias Registry & Dynamic Multi-Shell Transpilation

---

## 1. 🎯 Architectural Role

Developers frequently maintain duplicated aliases across `.bashrc`, `.zshrc`, `config.fish`, and PowerShell `$PROFILE`. Gladeshell centralizes all developer shortcuts in a single master registry (`aliases.toml` and Rust category files) and dynamically compiles them into native syntax for each shell during startup:
- **Bash:** Emits native `alias name='expansion'`
- **Zsh:** Emits `alias -g` or `alias name='expansion'`
- **Fish:** Emits `abbr --add name 'expansion'`
- **PowerShell:** Transpiles multi-word aliases into valid PowerShell functions, eliminating syntax errors

---

## 2. 📂 17 Domain-Driven Categories

All aliases are organized across 17 structured category modules in `src/core/aliases/categories/`:

| Category Module | File Path | Scope & Focus |
| :--- | :--- | :--- |
| **bun** | `bun.rs` | Fast Bun runtime commands (`bi`, `br`, `brd`, `bhot`, `w`) |
| **developer** | `developer.rs` | Project management, workspace jumping, build runners |
| **docker** | `docker.rs` | Containers, images, volumes, Compose shortcuts (`dco`, `dcu`, `dcd`) |
| **editor** | `editor.rs` | Editor launchers (VS Code, Zed, Neovim, Sublime) |
| **git** | `git.rs` | Git version control (`gs`, `ga`, `gcm`, `gps`, `gpl`, `gl`, `gwip`) |
| **maintenance**| `maintenance.rs`| OS garbage collection, memory flush, disk cleaners |
| **modern_cli** | `modern_cli.rs` | Rust CLI drop-in upgrades (`eza`, `bat`, `fd`, `rg`, `dust`) |
| **navigation** | `navigation.rs` | High-frequency directory jumps (`..`, `...`, `dev`, `fr`, `ba`) |
| **npm** | `npm.rs` | Node package manager (`ni`, `nid`, `nr`, `nrd`, `nrb`) |
| **pnpm_yarn** | `pnpm_yarn.rs` | Alternative JS package managers (`pi`, `px`, `y`, `ya`) |
| **postgres** | `postgres.rs` | PostgreSQL database clients, start/stop, DB creation |
| **prisma** | `prisma.rs` | Prisma ORM migrations, studio, generate, db push |
| **python** | `python.rs` | Virtualenv, pip, pytest, and python3 shortcuts |
| **rust** | `rust.rs` | Cargo commands (`cb`, `ct`, `cr`, `cc`, `ccl`, `cup`) |
| **system** | `system.rs` | Service control (`systemctl`), reboot, process kill |
| **tools** | `tools.rs` | Gladeshell native tool shortcuts (`rmd`, `rmf`, `gw`, `keep`) |
| **mod** | `mod.rs` | Category aggregator and registry table |

---

## 3. 🛡️ PowerShell Transpilation Safety

In PowerShell, aliases cannot accept arguments natively (e.g., `Set-Alias gcm "git commit -m"` fails because PowerShell does not allow spaces in alias targets).

Gladeshell's transpiler (`render_pwsh`) detects multi-word commands and emits functions with `$args` forwarding:
```powershell
function gcm { git commit -m $args }
```
Single-word commands remain lightweight aliases:
```powershell
Set-Alias -Name gs -Value git -Option AllScope
```
This guarantees 100% clean parsing without syntax warnings.
