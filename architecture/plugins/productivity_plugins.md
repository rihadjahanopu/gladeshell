# 🔌 Productivity Plugins (Autosuggest, Autocomplete, Highlight)

- **Primary Sources:**
  - [`src/plugins/autosuggest.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/plugins/autosuggest.rs)
  - [`src/plugins/autocomplete.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/plugins/autocomplete.rs)
  - [`src/plugins/highlight.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/plugins/highlight.rs)
- **Architectural Domain:** Real-Time Terminal Line Productivity

---

## 1. 🎯 Architectural Role

Gladeshell provides modern, fish-like interactive shell capabilities natively written in Rust:
1. **Autosuggest:** Displays dimmed ghost text suggestions as you type, fetched from history and command templates.
2. **Autocomplete:** Dynamic candidate generation when pressing `Tab` (commands, subcommands, files, git branches).
3. **Highlight:** Real-time syntax highlighting in the terminal buffer (valid executables colored green, invalid commands colored red, flags colored cyan).

---

## 2. 🧠 Plugin Mechanics

### A. Real-Time Autosuggest (`autosuggest.rs`)
- **Prefix History Matching:** Maintains a fast in-memory index of recent shell history.
- **Ghost Text Rendering:** Computes the common prefix and renders only the remaining tail in ANSI dark gray (`\x1b[90m`).
- **One-Key Acceptance:** Pressing `Right Arrow` or `Ctrl+F` instantly commits the suggestion to the active buffer.

### B. Trie & Dynamic Autocomplete (`autocomplete.rs`)
- **Multi-Source Completion:**
  1. Internal Gladeshell subcommands and flags.
  2. Executables found in `$PATH`.
  3. Filesystem paths in the current working directory.
  4. Git branches (if inside a repository).
- **Tab Cycle State:** Pressing `Tab` repeatedly cycles through matches alphabetically.

### C. Fast Syntax Highlighter (`highlight.rs`)
- **Token Classification:**
  - **Executable:** Green if found in `$PATH` or built-in aliases; Red if unknown.
  - **Options & Flags (`--verbose`, `-f`):** Cyan styling.
  - **Quoted Strings (`"..."` / `'...'`):** Yellow styling.
  - **Paths & Files:** Underlined or magenta if the file exists on disk.
- **Zero-Flicker Execution:** Highlighting operates directly on character indices without creating temporary String allocations for every token.
