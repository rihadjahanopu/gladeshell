# 🛠️ System Utilities & Productivity TUIs

- **Primary Source:** [`src/tools/`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/)
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Overview

Gladeshell bundles essential daily developer utilities directly into its binary, replacing dozens of disparate CLI tools with unified, lightning-fast Rust implementations.

---

## 2. 🧰 Utility Inventory

### A. Theme Picker TUI (`theme_picker.rs`)
- **Command:** `glade`
- **Description:** Full-screen interactive Ratatui browser for all 55 built-in themes. Displays a live ANSI preview of the user's actual username, hostname, and current working directory as themes are highlighted.
- **Persistence:** Selected theme is written to `~/.gladeshell_theme` and persists across sessions.

### B. Interactive Process Killer (`fkill.rs`)
- **Command:** `fkill`
- **Description:** Interactive process manager and searcher. Filters processes by PID or name, sorted by CPU and memory consumption. Pressing `Enter` terminates the selected process cleanly (`SIGTERM`), with force-kill fallback (`SIGKILL`).

### C. Frecency Directory Jumper (`z_jumper.rs`)
- **Command:** `z <query>`
- **Description:** Smart directory navigation using a frecency algorithm (frequency + recency). Scores decay over time, prioritizing frequently visited active project directories.

### D. System Cleaners (`system_clean.rs`, `universal_clean.rs`)
- **Commands:** `cleaner`, `uup`
- **Description:** Interactive TUI for clearing development caches (Cargo build caches, `node_modules`, npm/bun caches, system `/tmp` directories, package manager caches).

### E. Hardware Diagnostics (`pc_info.rs`, `pc_optimizer.rs`)
- **Commands:** `pc-info`, `pc-opt`
- **Description:** Complete system specs viewer (motherboard, GPU, memory slots, disks, network interfaces) using native OS APIs.

### F. Terminal Notes & Task Manager (`notes.rs`, `todo.rs`)
- **Commands:** `notes`, `todo`
- **Description:** Offline, privacy-first markdown notes and task management TUIs with instant search and status tracking.
