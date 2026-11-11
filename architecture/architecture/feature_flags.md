# 🚩 Feature Flags & Dependency Matrix

- **Primary Source:** [`Cargo.toml`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/Cargo.toml), [`src/lib.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/lib.rs), [`src/engines/mod.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/engines/mod.rs)
- **Minimum Supported Rust Version (MSRV):** `1.95`

---

## 1. 🎯 Feature Matrix Overview

Gladeshell supports modular builds to suit diverse operating environments, from embedded and server containers (where binary size must be minimal) to full desktop developer workstations.

```
┌───────────────────────────────────────────────────────────────┐
│                     Feature Flag Tree                         │
│                                                               │
│                     [default]                                 │
│                    /         \                                │
│       [prompt-engine]       [tools]                           │
│        /          \            |                              │
│     (gix)       (rayon)      (walkdir, ignore, ratatui...)    │
└───────────────────────────────────────────────────────────────┘
```

| Feature Name | Intended Use Case | Dependencies Activated |
| :--- | :--- | :--- |
| **`default`** | Standard developer installation | Activates `prompt-engine` + `tools` |
| **`prompt-engine`** | Minimal prompt with full Git & parallel metrics | `gix` (gitoxide), `rayon` |
| **`tools`** | CLI subcommands and Ratatui TUI utilities | `walkdir`, `ignore`, `globset`, `ratatui`, `crossterm`, `sysinfo`, `vault` crates |
| **`full`** | Comprehensive production distribution | `prompt-engine` + `tools` |
| **`--no-default-features`** | Ultra-lightweight core shell library | Pure Rust stdlib + `mimalloc` + `unicode-width` |

---

## 2. 🧩 Minimal Core Mode (`--no-default-features`)

When compiling with `cargo check --lib --no-default-features`:
- **Active Modules:**
  - `src/lib.rs` (C-ABI Dynamic Library exports)
  - `src/engines/terminal/` (Buffer, Parser, Renderer, Plugin)
  - `src/engines/intelligence/` (TypoEngine)
  - `src/core/prompt.rs` (Static prompt themes)
  - `src/core/aliases/` (Alias generator)
- **Deactivated Modules:**
  - `src/engines/system/` (Gated with `#[cfg(feature = "tools")]`)
  - `src/tools/` (Gated with `#[cfg(feature = "tools")]`)
  - Heavy Git status parsing (Gitoxide is disabled; prompt displays basic prompt without git badges)

### Conditional Compilation Guard Example
In `src/engines/mod.rs`:
```rust
pub mod intelligence;
#[cfg(feature = "tools")]
pub mod system;
pub mod terminal;

pub use intelligence::typo::TypoEngine;
#[cfg(feature = "tools")]
pub use system::delete::DeleteEngine;
```

---

## 3. 📦 Dependency Isolation & Licensing

All external dependencies in Gladeshell are vetted for zero C-library linkage and permissive open-source licenses (MIT, Apache-2.0, or BSD):
- **Git Operations:** Uses pure-Rust `gix` (`gitoxide`), eliminating `libgit2` or C compiler requirements.
- **Cryptography:** Uses pure-Rust `aes-gcm` and `pbkdf2` from the RustCrypto project, eliminating OpenSSL linkage.
- **Terminal UI:** Uses `ratatui` with pure-Rust `crossterm` backend.
