# 🌿 Git Productivity Utilities (`gbranch` & `git_wip`)

- **Primary Sources:**
  - Branch Manager: [`src/tools/gbranch.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/gbranch.rs) (858 LoC)
  - Work In Progress: [`src/tools/git_wip.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/git_wip.rs) (1,351 LoC)
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Role

Developers interact with Git dozens of times a day. `gbranch` and `gwip` eliminate tedious command repetition:
- **`gbranch`:** Interactive Ratatui branch browser, switcher, searcher, and bulk cleaner.
- **`gwip`:** One-command emergency Work-in-Progress checkpoint with automatic remote branch creation and push.

---

## 2. 🎮 Interactive Branch Manager (`gbranch`)

```
┌─────────────────────────────────────────────────────────────┐
│                    gbranch Manager                          │
│  Search: [ main                                           ] │
│                                                             │
│  * main            (HEAD) 6a84c31 refactor(engines)...      │
│    dev             (Behind 2)     3a12f9e fix(theme)...     │
│    feat/vault      (Merged)       8b22c11 add aes-gcm...    │
│                                                             │
│  [Enter] Checkout  [d] Delete  [c] Create  [q] Quit         │
└─────────────────────────────────────────────────────────────┘
```
- Automatically identifies merged branches for safe batch deletion (`d`).
- Displays live commit logs and relative timestamps (e.g. `2 hours ago`).

---

## 3. 🚀 Smart WIP Pusher (`gwip`)

When switching contexts or ending a workday, developers often forget to stage and push unfinished changes.
`gwip` executes a 5-step automated sequence:
1. Detects untracked and modified files.
2. Stages all changes (`git add -A`).
3. Formats commit message: `WIP: <custom_message>` (defaults to `Work in progress (Save Point)` with timestamp).
4. Determines current upstream tracking branch.
5. Pushes to remote (`git push origin <branch>`), setting `-u` upstream automatically if the branch is new.

```bash
# Quick WIP push with default message
gwip

# WIP with custom description
gwip "refactor in-progress: parsing AST"
```
