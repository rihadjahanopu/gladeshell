# 🗑️ Modern Deletion Explorer (`rmd` & `rmf`)

- **Primary Sources:**
  - [`src/tools/rmd.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/rmd.rs)
  - [`src/tools/rmf.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/rmf.rs)
  - Engine Core: [`src/engines/system/delete.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/engines/system/delete.rs)
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Overview

`rmd` (Remove Directory) and `rmf` (Remove Files) replace Unix `rm -rf` and Windows `rmdir /s /q` with modern, safe, dual-mode tools.

### Key Capabilities:
- **Dual-Mode Operation:** Seamless CLI batch mode + Interactive Ratatui Terminal UI.
- **Safety First (`--trash` by default or flag):** Moves items to the native OS Recycle Bin / Trash rather than permanent disk wipe.
- **Ripgrep Parallel Search Bar:** In `rmf`, pressing `/` launches a multithreaded directory explorer powered by `ignore::WalkBuilder`.
- **Dry-Run Capability (`--dry-run`):** Calculates exact size and file count that would be deleted without touching the disk.

```
┌─────────────────────────────────────────────────────────────┐
│                    rmf Cleanup Explorer                     │
│  Search: [ node_modules                  ] 12 targets found │
│                                                             │
│  [x] 📁 node_modules/            (145.2 MB)                 │
│  [x] 📄 package-lock.json.bak    (240.1 KB)                 │
│  [ ] 📁 target/debug/build/      (1.2 GB)                   │
│                                                             │
│  [Space] Toggle  [t] Trash  [Enter] Wipe  [/] Search  [q] Quit
└─────────────────────────────────────────────────────────────┘
```

---

## 2. 🎮 Interactive Keybindings (`rmf`)

| Keybinding | Action |
| :---: | :--- |
| `↑` / `k` | Move cursor selection up |
| `↓` / `j` | Move cursor selection down |
| `Space` | Toggle selection checkmark for current item |
| `a` | Select all matching targets |
| `/` | Focus search bar to type Ripgrep glob/filter |
| `t` | Safely send all selected items to OS Recycle Bin / Trash |
| `Enter` | Permanently wipe selected items (requires confirmation prompt) |
| `q` / `Esc` | Safely exit without modifying disk |

---

## 3. 🧪 CLI Interface Examples

```bash
# Safe directory removal with progress bar
rmd my_old_folder

# Move directory to Recycle Bin / Trash
rmd --trash build/

# Interactive directory wipe confirmation
rmd -i large_data/

# Multi-file wildcard removal
rmf "*.log" "*.tmp"

# Launch full-screen Cleanup Explorer TUI
rmf
```
