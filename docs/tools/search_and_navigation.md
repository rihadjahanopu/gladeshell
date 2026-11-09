# 🔍 Search & Smart Navigation Suite

- **Primary Sources:**
  - File Finder: [`src/tools/file_find.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/file_find.rs) (1,866 LoC)
  - Fast Grep: [`src/tools/fast_grep.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/fast_grep.rs) (146 LoC)
  - Fuzzy CD: [`src/tools/fuzzy_cd.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/fuzzy_cd.rs) (1,596 LoC)
  - Frecency Jumper: [`src/tools/z_jumper.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/z_jumper.rs) (300 LoC)
  - Drive Jumper: [`src/tools/drive_jumper.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/drive_jumper.rs) (67 LoC)
  - History Search: [`src/tools/history_search.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/history_search.rs) (408 LoC)
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Overview

High-speed discovery and navigation are vital for developer velocity. This suite integrates fuzzy search, frecency directory ranking, and ripgrep parallel regex matching into interactive terminal experiences.

---

## 2. 🧩 Component Breakdown

### A. Interactive File Finder (`file_find.rs` & `fast_grep.rs`)
- Full-screen Ratatui TUI for searching filenames and regex file contents simultaneously.
- Split-pane layout: Left pane displays search results; right pane shows real-time syntax-highlighted file preview.
- Powered by `grep-searcher` and `grep-regex` crates for gigabyte-per-second search speeds.

### B. Smart Frecency Jumper (`z_jumper.rs` & `fuzzy_cd.rs`)
- Implements Mozilla's **Frecency Algorithm** (Frequency + Recency):
  $$\text{Score} = \text{Visit Count} \times \text{Recency Decay Factor}$$
- As days pass, unused paths decay while active project folders remain at the top.
- UNC path normalizer ensures identical behavior across Windows drive letters (`C:\`, `D:\`) and Unix mounts (`/mnt/`, `/Volumes/`).

### C. Fuzzy History Search (`history_search.rs`)
- Intercepts `Ctrl+R` to provide an instant, fuzzy-ranked search through shell history across all terminal tabs.

---

## 3. 💻 CLI Usage

```bash
# Jump to directory matching query (e.g., ~/Development/web-app)
z web

# Interactive fuzzy directory browser
fcd

# Search file contents with Ripgrep engine
fast-grep "TODO:" src/

# Launch full-screen interactive file finder with live preview
file-find
```
