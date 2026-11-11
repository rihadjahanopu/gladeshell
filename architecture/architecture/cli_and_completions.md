# 🚀 CLI Fast-Path Dispatcher & Shell Completions

- **Primary Sources:**
  - CLI Schema: [`src/cli.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/cli.rs) (582 LoC)
  - Entrypoint & Fast-Path Router: [`src/main.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/main.rs) (942 LoC)
- **Crates:** `clap` (derive v4), `clap_complete` (v4)

---

## 1. 🎯 Architectural Role

Gladeshell serves as both an ultra-responsive prompt renderer (where every millisecond counts) and a comprehensive multi-tool hub with dozens of subcommands.

Running full Clap argument parsing on every prompt render would waste CPU cycles parsing schemas. Gladeshell implements a **Dual-Layer Dispatching Strategy**:

```
[Raw CLI Invocations]
          │
          ▼
Is raw_args[1] a hot-path command? (`prompt`, `version`, `auto-ls`)
          │
   ┌──────┴──────┐
   │             │
  YES            NO
   │             │
   ▼             ▼
Fast-Path     Full Clap Parser
Dispatcher    Schema Validation &
(< 50µs)      Subcommand Routing
```

---

## 2. ⚡ The Fast-Path Dispatcher (`src/main.rs`)

If the user runs:
```bash
gladeshell prompt --shell zsh --cwd /tmp
```
`main.rs` inspects `std::env::args()` directly before initializing Clap:
1. Detects `raw_args[1] == "prompt"`.
2. Extracts flags (`--shell`, `--cwd`, `--exit-code`, `--cmd-duration`) using high-speed positional matching.
3. Invokes `gladeshell_core::prompt::render()` directly.
4. Bypasses schema reflection, allocator churn, and regex validations, executing in **under 50 microseconds**.

---

## 3. ⌨️ Dynamic Shell Completion Generator (`clap_complete`)

Gladeshell dynamically compiles tab-completion scripts for all major shells via `clap_complete`:
```bash
gladeshell completion bash > ~/.local/share/bash-completion/completions/gladeshell
gladeshell completion zsh  > ~/.zfunc/_gladeshell
gladeshell completion fish > ~/.config/fish/completions/gladeshell.fish
gladeshell completion pwsh > ~/Documents/PowerShell/Completions/gladeshell.ps1
```

Supported shells:
- **Bash:** Programmable completion functions (`_gladeshell`).
- **Zsh:** Compdef definitions with flag description previews.
- **Fish:** Native fish completion commands with contextual summaries.
- **PowerShell:** `Register-ArgumentCompleter` scripts.
- **Elvish:** Elvish map completion tables.
