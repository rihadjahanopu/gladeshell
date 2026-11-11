# 🎨 Prompt Engine & 55 Themes Architecture

- **Primary Source:** [`src/core/prompt.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/core/prompt.rs)
- **Architectural Domain:** Zero-Allocation Dynamic Prompt Renderer
- **Latency Guarantee:** Sub-millisecond (Empirical: ~75µs)

---

## 1. 🎯 Architectural Role

The Prompt Engine is the central visual heartbeat of Gladeshell. Every time a prompt is rendered in Zsh, Bash, Fish, or PowerShell, `src/core/prompt.rs` compiles context variables (current directory, git status, exit code, command execution duration, system resources, and active theme) into an ANSI-escaped terminal string.

---

## 2. 💎 Bit-31 TrueColor Packing

Gladeshell stores all theme color palettes in compact `u32` integers:
- **Bit 31 = 0:** ANSI 256-color palette index (`0..=255`). Emits `\x1b[38;5;Nm`.
- **Bit 31 = 1:** 24-bit TrueColor RGB (`0x80RRGGBB`). Emits `\x1b[38;2;R;G;Bm`.

```rust
pub const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    0x8000_0000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
```
This zero-allocation bitpacking representation allows storing entire 55-theme palettes directly in static binary data without heap strings.

---

## 3. 📋 All 55 Built-In Themes

Themes are loaded via `~/.gladeshell_theme` or dynamically overridden via `--theme <name>`:

| Group | Notable Themes | Design Style |
| :--- | :--- | :--- |
| **Minimalist** | `minimal`, `pure`, `slick`, `lambda` | 1-line ultra-clean indicators, quiet git status |
| **Developer Staples** | `robbyrussell`, `agnoster`, `p10k`, `starship`, `spaceship` | Arrow chevrons, Powerline glyphs, multi-line layouts |
| **Popular Editor Palettes**| `catppuccin`, `tokyonight`, `dracula`, `nord`, `gruvbox`, `rosepine`, `kanagawa`, `monokai`, `onedark` | TrueColor calibrated palettes matching popular code editors |
| **Futuristic & Cyber** | `cyberpunk`, `synthwave`, `matrix`, `outrun`, `neon_pulse`, `cyber_samurai` | Vivid neon, dual-tone gradient accents |

---

## 4. 🧹 Transient Prompt Support

For users who want clean terminal history without cluttered prompts after command execution, Gladeshell supports **Transient Prompts** (`test_render_transient`):
- While typing: Full multi-line prompt with git branch, system status, and virtual environment.
- After pressing `Enter`: Condenses into a simple single character (`❯ ` or `λ `), preserving vertical terminal real estate.
