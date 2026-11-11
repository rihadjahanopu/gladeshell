# 🏗️ Project Scaffolding Suite

- **Primary Source:** [`src/tools/project_setup/`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/project_setup/)
- **Core Commands:** `project`, `makecpp`, `makerust`, `vite`, `next`, `ui`, `css`
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Overview

The Project Scaffolding Suite automates the initialization of modern, production-ready developer projects with preconfigured Linters, Git repos, VS Code settings, and Makefiles in seconds.

---

## 2. 🧩 Scaffolding Modules

### A. Project Setup Hub (`hub.rs`)
An interactive Ratatui wizard launched via `project`. Allows choosing project categories (Frontend, Backend, Systems, Mobile, Static Web) and framework presets.

### B. Advance C/C++ Generator (`makecpp.rs`)
- Scaffolds complete modular C or C++ projects (`src/`, `include/`, `build/`).
- Generates a production `Makefile` with automatic header dependency generation (`-MMD -MP`).
- Automatically configures `.vscode/launch.json` and `.vscode/c_cpp_properties.json` for CodeLLDB debugging.

### C. Advanced Rust Boilerplate (`makerust.rs`)
- Generates binary (`bin`), library (`lib`), or CLI applications.
- Preconfigures `rust-toolchain.toml`, `clippy` lints, benchmark targets, and GitHub CI workflow templates.

### D. Modern Frontend Scaffolder (`vite.rs`, `next.rs`, `shadcn_ui.rs`, `tailwind.rs`)
- Auto-detects package manager (prioritizes Bun for 10x speed, falls back to NPM).
- Installs Tailwind CSS v4 with modern `@import "tailwindcss"` CSS syntax.
- Optionally installs and configures Shadcn/UI with customizable component selection.

---

## 3. 💻 CLI Usage

```bash
# Launch interactive Project Hub
project

# Scaffold instant C++ project with Makefile & VS Code debugging
makecpp my_project

# Scaffold modern Rust workspace
makerust my_cli --bin

# Scaffold Vite React with Tailwind v4
vite

# Initialize Shadcn/UI
ui
```
