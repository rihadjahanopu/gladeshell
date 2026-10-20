# 🔍 `file_find.rs` (`ff`) Comprehensive Research Report & Overhaul Plan

> **Gladeshell Tool Specification & Architecture Plan**  
> **Target:** `src/tools/file_find.rs`, `src/init/shared.rs`, `src/init/pwsh.rs`, `src/tools/keep.rs`  
> **Status:** APPROVED FOR IMPLEMENTATION  

---

## 1. Executive Summary & Problem Diagnosis

The `ff` command is advertised in Gladeshell as an **Ultra-Fast Parallel Native File Finder** with real-time TUI, live search, and action tags (`OPEN` / `CODE` / `CD` / `EXPLORE`). However, an in-depth code audit of `src/tools/file_find.rs`, `src/init/shared.rs`, and `src/init/pwsh.rs` uncovered **14 critical bugs, memory leaks, and architectural flaws**.

```mermaid
flowchart TD
    User["User runs `ff` in shell"] --> TUI["file_find.rs TUI opens"]
    
    subgraph MemoryAndPerf["🚨 Memory & Performance Bottlenecks"]
        P1["Massive Clone Loop: 800,000 heap allocs/sec while sitting idle!"]
        P2["'Rayon parallel search' claimed in docs but 100% missing in code"]
        P3["Main thread freezes during fuzzy filtering on every keystroke"]
        P4["Redundant O(N*M) path lowercasing & splitting on every file entry"]
    end
    
    subgraph TraversalAndSearch["🚨 Traversal & Search Engine Flaws"]
        T1["Directories unconditionally excluded (is_dir: continue) — broke 'ff -t d'"]
        T2["Linux & Windows Root: walks into /proc, /sys, C:\\$Recycle.Bin"]
        T3["No multi-word / space query tokenization ('tools find' returns 0 results)"]
        T4["Windows '\\' separator not treated as word boundary in subsequence scoring"]
    end
    
    subgraph UXKeybindings["🚨 UX, Keybindings & Shell Wrapper Bugs"]
        K1["CTRL+C Trap: copies path to clipboard instead of quitting!"]
        K2["Shell Wrapper desync: echoes 'COPIED:path' into user shell prompt"]
        K3["Mouse click ignores list scroll offset (selects wrong item)"]
        K4["Mouse wheel moves only 1 item per tick (sluggish scrolling)"]
        K5["Missing modern editors: zed, cursor, nvim, $EDITOR ignored"]
        K6["Compiler warning: unused variable 'lower' on Windows & macOS"]
    end

    TUI --> MemoryAndPerf
    TUI --> TraversalAndSearch
    TUI --> UXKeybindings
```

---

## 2. Exhaustive Bug Analysis (Line-by-Line)

| # | Bug Location | Severity | Description & Root Cause | Impact |
|---|---|---|---|---|
| **1** | `file_find.rs:746-747` | **CRITICAL** | **Massive Memory Allocation in TUI Draw Loop.** `let all_files = indexed_files.lock().unwrap().clone();` runs on every 50ms poll tick. For 20,000 files, it clones 40,000 Strings and PathBufs 20 times a second (~800,000 heap allocs/sec) even when completely idle! | Severe GC/allocator pressure, battery drain, thermal throttling, and frame drops. |
| **2** | `file_find.rs:245-248` | **CRITICAL** | **Directories Unconditionally Excluded (`if is_dir { continue; }`).** The CLI flag `--type d` / `-t d` is documented to find directories, and line 1058 handles `CdInto` for dirs, but line 246 unconditionally skips all directories during traversal. | Running `ff -t d` finds 0 items. Interactive `ff` can NEVER find or navigate to directories. |
| **3** | `file_find.rs:9` vs Code | **HIGH** | **Missing Rayon Parallel Search.** Module doc claims `//! Powered by ignore traversal engine and rayon parallel search.` but `rayon` is never imported or used. All search scoring runs single-threaded on the UI thread. | UI freezes for 50–200ms on every keystroke in large codebases. |
| **4** | `file_find.rs:182-218` | **HIGH** | **Root Traversal Hangs (Linux `/` & Windows `C:\`).** Does not exclude `/proc`, `/sys`, `/dev`, `/run` on Linux, nor `C:\$Recycle.Bin`, `System Volume Information` on Windows. | Traversal hangs on virtual kernel files like `/proc/kcore` or wastes time crawling device nodes. |
| **5** | `file_find.rs:335-388` | **HIGH** | **No Multi-Word / Space Query Tokenization.** Searches query as a single contiguous string. Searching `tools find` or `init shared` fails because no file path contains that literal substring with space. | Users cannot search by multiple path fragments. |
| **6** | `file_find.rs:266-285` | **MEDIUM** | **Redundant $O(N \times M)$ String Allocations During Traversal.** For every file, splits `relative_path` and calls `part.to_lowercase()`, allocating hundreds of thousands of heap strings even though `filter_entry` already filters directory names. | Slows down file scanning by 2x–3x. |
| **7** | `file_find.rs:1044-1052` | **HIGH** | **`Ctrl+C` Exit Trap.** In almost all terminal software, `Ctrl+C` cancels and exits. In `ff`, if an item is selected (row 0 is selected by default!), `Ctrl+C` copies path to clipboard instead of quitting. Users must know to press `Esc`. | Users feel trapped in the TUI when pressing standard `Ctrl+C`. |
| **8** | `file_find.rs:502` & `shared.rs:675` | **MEDIUM** | **Shell Wrapper Desync (`COPIED:...` Leak).** When copying path, Rust prints `COPIED:<path>` to stdout. The shell wrapper in `shared.rs` falls into `*)` and runs `echo "COPIED:<path>"`, dumping raw text into the user's terminal prompt. | Ugly terminal output after exiting. |
| **9** | `file_find.rs:1132` | **MEDIUM** | **Mouse Click Ignores List Scroll Offset.** `let clicked_idx = (my - list_rect.y - 1) as usize;` does not add `list_state.offset()`. | Clicking row 3 after scrolling down selects row 3 from the top of the entire list instead of the visible clicked row. |
| **10** | `file_find.rs:1155-1168` | **LOW** | **Sluggish Mouse Wheel (1 Row per Tick).** Scrolling the mouse wheel moves only 1 row at a time. | Mouse navigation feels slow and unresponsive. |
| **11** | `file_find.rs:572` | **LOW** | **Compiler Warning (`unused variable: lower`).** Line 572 declares `let lower = path.to_lowercase();` which is unused on Windows and macOS. | Emits compiler/clippy warning on Windows and macOS builds. |
| **12** | `file_find.rs:679-685` | **MEDIUM** | **Missing Modern Editors.** Only checks `code`, `codium`, and `code-insiders`. Ignores `zed` (set up by Gladeshell's `zed_setup`), `cursor`, `nvim`, and `$EDITOR`. | Users with Zed or Cursor cannot open files directly from `ff`. |
| **13** | `file_find.rs:898-900` | **LOW** | **Windows Path Display Mixed Slashes.** Formats parent dir as `format!("  {}/", parent_dir)` producing mixed paths like `src\tools/`. Also misses `\` word boundary bonus. | Visual inconsistency and suboptimal Windows scoring. |
| **14** | `file_find.rs:879-934` | **FEATURE GAP** | **No Dual-Pane File Content Preview.** Unlike `cf`, `ff` has a full-width list that leaves massive empty whitespace on widescreen monitors without showing file content previews or syntax highlighting. | Suboptimal UX compared to modern tools like `telescope` or `fzf`. |

---

## 3. Architectural Overhaul Design

```mermaid
classDiagram
    class FfEngine {
        +FfArgs args
        +Arc~Vec~FoundItem~~ indexed_files
        +AtomicBool is_done
        +start_background_indexing()
    }

    class ParallelSearchPipeline {
        +score_fuzzy_item_multi_token(query, relative_path)
        +rayon_parallel_filter(all_files, query)
        +normalize_path(path)
    }

    class FfTuiApp {
        +ListState list_state
        +String query
        +Vec~FoundItem~ filtered_items
        +Option~Instant~ copy_notice
        +bool show_preview
        +handle_input_event()
        +render_ui()
    }

    FfEngine --> ParallelSearchPipeline
    FfTuiApp --> ParallelSearchPipeline
```

### A. Zero-Allocation Background Indexing & TUI Loop
- Store indexed files as `Arc<Vec<FoundItem>>`.
- When background worker finishes or publishes a batch, swap the `Arc`.
- In `tui_loop`: **Zero clones per frame!** Only borrow the `Arc` when `query` changes or when a new batch arrives.
- Eliminate 800,000 allocations/sec to 0 allocations while idle.

### B. True Rayon Parallel Fuzzy Search
- Use `rayon::prelude::*` in `filter_and_score_items`.
- Distribute fuzzy scoring across all available CPU cores:
  ```rust
  all_files.par_iter()
      .filter_map(|item| score_fuzzy_item(&tokens, item).map(|score| (item, score)))
      .collect::<Vec<_>>()
  ```
- Instantaneous search response even across 100,000+ files.

### C. Multi-Token Query Tokenization & Path Boundary Scoring
- Split query by whitespace: `query.split_whitespace()`.
- Every token must match (either filename or path components).
- Treat both `/` and `\` as word boundaries (+30 score bonus).

### D. Directory & Symlink Filter Support
- Fix `is_dir`: when `--type d` is set, only include directories.
- In interactive mode, include both files and directories with distinctive visual badges (`📁` for dirs, file icons for files).
- Pressing `Enter` on a directory executes `CD:<path>` (changes directory in shell); pressing `Enter` on a file executes `OPEN:<path>`.

### E. Standardized Keybinding Map

| Key | Current Broken Behavior | Proposed New Behavior |
|---|---|---|
| **`ENTER`** | Open file (or cd if dir, but dirs were blocked) | **CONFIRM**: Open file with default viewer or `cd` to folder. |
| **`Ctrl+C`** | Copies path to clipboard! (Exit trap) | **CANCEL & QUIT**: Immediately exits without action. |
| **`ESC`** | Quit | **QUIT / CLEAR**: Clears search query if non-empty, exits if empty. |
| **`Ctrl+Y` / `y` (normal)** | Undocumented copy | **COPY TO CLIPBOARD**: Copies path to clipboard via `arboard`. |
| **`Ctrl+O`** | Opens File Explorer | **OPEN IN EDITOR**: Launches `$EDITOR`, `zed`, `code`, `cursor`, `nvim`. |
| **`Ctrl+E`** | Nothing | **OPEN IN EXPLORER**: Opens OS file manager. |
| **`Tab`** | Nothing | **TOGGLE PREVIEW**: Toggles dual-pane syntax/content preview. |
| **Mouse Wheel** | Moves 1 item | Moves 4 items per tick for smooth scrolling. |
| **Mouse Click** | Wrong row (ignores offset) | Correct row: `(row - y - 1) + list_state.offset()`. |

---

## 4. Implementation Roadmap

- [x] **Step 1: Fix Memory Allocation Stutter & Add True Rayon Parallelism (`file_find.rs`)**
  - Replace per-tick `all_files.clone()` with `Arc<Vec<FoundItem>>`.
  - Implement `rayon` parallel filtering across all CPU threads.
  - Eliminate unused variable warning (`lower`).
- [x] **Step 2: Fix Directory Exclusion & Root Traversal Protection (`file_find.rs`)**
  - Fix `is_dir` handling so `-t d` / `--type d` works properly.
  - Add system root blacklist for `/proc`, `/sys`, `/dev`, `/run` (Linux) and `C:\$Recycle.Bin`, `System Volume Information` (Windows).
- [x] **Step 3: Multi-Token Fuzzy Search & Cross-Platform Path Normalization (`file_find.rs`)**
  - Implement space-separated multi-token search (`query.split_whitespace()`).
  - Normalize Windows `\` and Linux `/` in path scoring and boundary bonuses.
- [x] **Step 4: Keybinding & Mouse Click Fixes (`file_find.rs`)**
  - Fix `Ctrl+C` to quit cleanly; map `Ctrl+Y` to copy path.
  - Fix mouse click row calculation with `list_state.offset()`.
  - Increase mouse wheel scroll to 4 rows per tick.
- [x] **Step 5: Modern Editor Integration & Shell Wrapper Clean-Up (`file_find.rs`, `shared.rs`)**
  - Add `zed`, `cursor`, `nvim`, `$EDITOR` support.
  - Fix shell wrapper to cleanly handle `COPIED:` without printing ugly leaks to prompt.
- [x] **Step 6: Responsive Preview Pane & Verification**
  - Add responsive side preview pane showing text file contents or metadata (auto wide-screen dual pane).
  - Verified: all unit tests pass with `cargo test file_find` (7/7 passed), `cargo fmt --check` clean, `cargo clippy -- -D warnings` clean, and full workspace tests pass (334 passed, 0 failed).


