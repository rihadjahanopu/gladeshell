# ADR 0003: Domain-Driven Engine Layer with Zero-Breaking Module Aliases

- **Status:** Accepted
- **Deciders:** Core Engineering Team
- **Date:** 2026-11-09

---

## 1. 🎯 Context
Prior to v1.2.16, Gladeshell's 16 engine subsystems (`buffer_engine.rs`, `input_parser.rs`, `renderer.rs`, `plugin_engine.rs`, `delete_engine.rs`, `typo_engine.rs`) were dispersed across root `src/`, `src/core/`, and `src/tools/`.
As the codebase grew to 107 files and 10,000+ active users, this created:
1. Namespace collisions.
2. Unclear domain ownership between terminal mechanics, intelligence, and system deletion.
3. Code clutter.

However, moving files naively would cause breaking changes for external plugins and downstream library consumers importing legacy module paths (e.g. `gladeshell_core::buffer_engine::BufferEngine`).

---

## 2. 💡 Decision
We consolidated all engines under a structured domain architecture:
- `src/engines/terminal/` (buffer, parser, renderer, plugin)
- `src/engines/intelligence/` (typo engine)
- `src/engines/system/` (delete engine, gated behind `#[cfg(feature = "tools")]`)

To guarantee **zero breaking changes** and avoid leaving physical 1-line dummy files on disk, we introduced **Rust module aliasing** directly in `src/lib.rs`, `src/core/mod.rs`, and `src/tools/mod.rs`:
```rust
pub use engines::terminal::buffer as buffer_engine;
pub use engines::terminal::parser as input_parser;
pub use engines::terminal::plugin as plugin_engine;
pub use engines::terminal::renderer;
```

---

## 3. 🚀 Consequences

### Positive:
- **Clean Architecture:** Domain separation is clear, idiomatic, and highly maintainable.
- **Zero Breaking Changes:** 100% backward-compatible for existing code, unit tests, and shell hooks.
- **Zero Disk Clutter:** No 1-line dummy files left on the filesystem.
- **All 357 Tests Pass:** Full regression safety verified across Ubuntu, macOS, and Windows.

### Trade-offs:
- Required adding `#[cfg(feature = "tools")]` to `engines::system` to maintain clean minimal builds under `--no-default-features`.
