# 📄 File & Content Utilities

- **Primary Sources:**
  - Bulk File Renamer: [`src/tools/file_renamer.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/file_renamer.rs) (1,365 LoC)
  - Auto LS: [`src/tools/auto_ls.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/auto_ls.rs) (335 LoC)
  - Backup Utility: [`src/tools/bak.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/bak.rs) (71 LoC)
  - Interactive Bun Runner: [`src/tools/bun_runner.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/bun_runner.rs) (780 LoC)
  - Help & Cheat-Sheet: [`src/tools/keep.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/keep.rs) (1,595 LoC)
  - Directory & File Creators: [`src/tools/mkd.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/mkd.rs) (39 LoC), [`src/tools/touch_tool.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/touch_tool.rs) (44 LoC)
  - Secret Generator CLI: [`src/tools/secret_gen_tool.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/secret_gen_tool.rs) (35 LoC)
  - Trash CLI: [`src/tools/trash.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/trash.rs) (86 LoC)
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Role

A collection of high-frequency micro-utilities designed to replace shell scripts with fast, type-safe Rust commands.

---

## 2. 🧰 Component Overview

### A. Bulk File Renamer (`file_renamer.rs`)
- Batch renames hundreds of files using regex find-and-replace, numeric sequence insertion, and case conversion.
- Shows side-by-side interactive diff preview before writing to disk.

### B. Auto LS (`auto_ls.rs`)
- Automatically lists directory contents whenever the user navigates into a folder (`cd`).
- Displays file count, directory count, size, and active git branch badge.
- Automatically throttles if the directory contains more than 1,000 files to prevent scrolling lag.

### C. Fast Backup & Restore (`bak.rs`)
- Creates an instant timestamped `.bak` copy of any file or directory (`bak config.json`).
- Running `bak config.json` a second time prompts to restore from the backup.

### D. Command Cheat-Sheet (`keep.rs`)
- Interactive fuzzy-searchable documentation viewer for all Gladeshell commands, shortcuts, and aliases directly in the shell.

---

## 3. 💻 CLI Usage

```bash
# Launch interactive batch file renamer
renamer

# Create backup of a file
bak .env

# Restore file from backup
bak .env

# Generate instant cryptographically secure password
gen --len 32

# View Gladeshell interactive cheat-sheet
keep

# Create directory and jump into it instantly
mkd my_new_feature_dir
```
