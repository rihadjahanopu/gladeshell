// =============================================================================
//  src/core/prompt.rs — Zero-allocation dynamic prompt renderer
//
//  Performance contract:
//    • render()           must complete in < 1 ms (target: ~50 µs)
//    • No heap allocation on the hot path (uses caller's buffer only)
//    • No subshell forks — EVER
//
//  Color encoding (u32 per channel):
//    • value <= 0x00_00_00_FF  → ANSI 256-color index (bits 0-7)
//    • value has bit-31 set    → 24-bit true color (bits 0-23 = 0xRRGGBB)
//
//  55 themes mirror config.zsh fb_theme_* functions exactly:
//    same name, same hex color, same emoji, same prompt_char, same line structure.
// =============================================================================

use std::os::raw::c_char;

// ── Color encoding helpers ────────────────────────────────────────────────────

/// Encode a 24-bit hex color (0xRRGGBB) as a true-color u32 (bit 31 set).
const fn tc(hex: u32) -> u32 { 0x8000_0000 | hex }

/// Encode an ANSI 256-color index as a plain u32.
const fn a8(n: u8) -> u32 { n as u32 }

#[inline(always)]
fn is_true_color(c: u32) -> bool { c & 0x8000_0000 != 0 }
#[inline(always)]
fn tc_r(c: u32) -> u8 { ((c >> 16) & 0xFF) as u8 }
#[inline(always)]
fn tc_g(c: u32) -> u8 { ((c >> 8) & 0xFF) as u8 }
#[inline(always)]
fn tc_b(c: u32) -> u8 { (c & 0xFF) as u8 }
#[inline(always)]
fn a8_idx(c: u32) -> u8 { (c & 0xFF) as u8 }

// ── Theme descriptor ──────────────────────────────────────────────────────────

/// A single theme entry — all data is 'static, zero heap.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    /// CLI name: `fancybash theme <name>` / saved in ~/.fancybash_theme
    pub name: &'static str,
    /// Decorative emoji shown at start of line 1 (empty string = none)
    pub emoji: &'static str,
    /// User@host color: bit-31 = true-color (0x80RRGGBB), else ANSI256 index
    pub user_color: u32,
    /// CWD path color (same encoding)
    pub path_color: u32,
    /// Git branch/status color (same encoding)
    pub git_color: u32,
    /// Prompt character on line 2 (e.g. "❯", "❯❯❯", "▶▶", "$")
    pub prompt_char: &'static str,
    /// Line-1 structural prefix (before emoji+user, e.g. "╭─" or "")
    pub line1_prefix: &'static str,
    /// Line-2 structural prefix (before prompt_char, e.g. "╰─" or "")
    pub line2_prefix: &'static str,
}

// ── THEMES — 55 entries matching config.zsh fb_theme_* exactly ───────────────
//
//  Order matches config.zsh theme numbers 1–55.
//  Hex colors extracted verbatim from each fb_theme_* PROMPT string.
//  Names with underscores (catppuccin_frappe, dracula_pro, cyber_samurai,
//  neon_pulse) are kept as-is to match the Zsh function suffix.

pub static THEMES: &[Theme] = &[
    // 00 ── minimal — ultra-clean compact path prompt (user_color=0 hides user@host)
    Theme { name: "minimal",          emoji: "",   user_color: 0,               path_color: a8(51),          git_color: a8(46),          prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 01 ── full — rich detailed theme with cyan/green/yellow accents
    Theme { name: "full",             emoji: "⚡", user_color: a8(51),          path_color: a8(82),          git_color: a8(226),         prompt_char: "❯❯❯",   line1_prefix: "",   line2_prefix: ""   },
    // 02 ── robbyrussell — %F{green}➜ cyan path green ❯
    Theme { name: "robbyrussell",     emoji: "➜",  user_color: a8(46),          path_color: a8(51),          git_color: a8(46),          prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 03 ── p10k — blue ╭─/╰─, 🐧 green@host cyan path
    Theme { name: "p10k",             emoji: "🐧", user_color: a8(46),          path_color: a8(51),          git_color: a8(46),          prompt_char: "❯",     line1_prefix: "╭─", line2_prefix: "╰─" },
    // 04 ── agnoster — powerline blue/green/yellow
    Theme { name: "agnoster",         emoji: "💻", user_color: a8(75),          path_color: a8(34),          git_color: a8(226),         prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 05 ── catppuccin — #ca9ee6 / #89b4fa / #f5c2e7
    Theme { name: "catppuccin",       emoji: "🐱", user_color: tc(0xca9ee6),    path_color: tc(0x89b4fa),    git_color: tc(0xf5c2e7),    prompt_char: "❯❯❯",  line1_prefix: "",   line2_prefix: ""   },
    // 06 ── tokyonight — #bb9af7 / #7dcfff / #7aa2f7 ⚡
    Theme { name: "tokyonight",       emoji: "🌌", user_color: tc(0xbb9af7),    path_color: tc(0x7dcfff),    git_color: tc(0x7aa2f7),    prompt_char: "⚡",    line1_prefix: "",   line2_prefix: ""   },
    // 07 ── dracula — #bd93f9 / #ff79c6 / #8be9fd
    Theme { name: "dracula",          emoji: "🧛", user_color: tc(0xbd93f9),    path_color: tc(0xff79c6),    git_color: tc(0x8be9fd),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 08 ── nord — #88c0d0 / #81a1c1 / #8fbcbb ❄️
    Theme { name: "nord",             emoji: "❄️", user_color: tc(0x88c0d0),    path_color: tc(0x81a1c1),    git_color: tc(0x8fbcbb),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 09 ── bira — yellow ╭─/╰─ green@host blue path magenta git $
    Theme { name: "bira",             emoji: "",   user_color: a8(46),          path_color: a8(75),          git_color: a8(201),         prompt_char: "$",     line1_prefix: "╭─", line2_prefix: "╰─" },
    // 10 ── pure — blue path 240-git green ❯
    Theme { name: "pure",             emoji: "",   user_color: a8(75),          path_color: a8(240),         git_color: a8(240),         prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 11 ── starship — 🚀 magenta/green/cyan
    Theme { name: "starship",         emoji: "🚀", user_color: a8(201),         path_color: a8(51),          git_color: a8(226),         prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 12 ── cyberpunk — ⚡ #ffee00 / #00f0ff / #ff0055 ▶▶
    Theme { name: "cyberpunk",        emoji: "⚡", user_color: tc(0xffee00),    path_color: tc(0x00f0ff),    git_color: tc(0xff0055),    prompt_char: "▶▶",    line1_prefix: "",   line2_prefix: ""   },
    // 13 ── synthwave — 🌅 #ff007f / #00ffff / #9d00ff ❯❯
    Theme { name: "synthwave",        emoji: "🌅", user_color: tc(0xff007f),    path_color: tc(0x00ffff),    git_color: tc(0x9d00ff),    prompt_char: "❯❯",   line1_prefix: "",   line2_prefix: ""   },
    // 14 ── gruvbox — 🌴 #fabd2f / #8ec07c / #fe8019
    Theme { name: "gruvbox",          emoji: "🌴", user_color: tc(0xfabd2f),    path_color: tc(0x8ec07c),    git_color: tc(0xfe8019),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 15 ── onedark — 🌐 #61afef / #c678dd / #98c379
    Theme { name: "onedark",          emoji: "🌐", user_color: tc(0x61afef),    path_color: tc(0xc678dd),    git_color: tc(0x98c379),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 16 ── sorin — blue@host cyan path magenta git
    Theme { name: "sorin",            emoji: "",   user_color: a8(75),          path_color: a8(51),          git_color: a8(201),         prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 17 ── spaceship — 🚀 blue@host cyan path green ➜
    Theme { name: "spaceship",        emoji: "🚀", user_color: a8(75),          path_color: a8(51),          git_color: a8(46),          prompt_char: "➜",     line1_prefix: "",   line2_prefix: ""   },
    // 18 ── halflife — λ #ff9800 / #ffeb3b ▶
    Theme { name: "halflife",         emoji: "λ",  user_color: tc(0xff9800),    path_color: tc(0xffeb3b),    git_color: tc(0xff9800),    prompt_char: "▶",     line1_prefix: "",   line2_prefix: ""   },
    // 19 ── paradox — ⚡ blue/magenta powerline ❯
    Theme { name: "paradox",          emoji: "⚡", user_color: a8(75),          path_color: a8(201),         git_color: a8(51),          prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 20 ── bureau — 240-gray / yellow@host / cyan path / green ❯
    Theme { name: "bureau",           emoji: "",   user_color: a8(226),         path_color: a8(51),          git_color: a8(46),          prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 21 ── gallifrey — ⏳ #ffd700 / #00bfff
    Theme { name: "gallifrey",        emoji: "⏳", user_color: tc(0xffd700),    path_color: tc(0x00bfff),    git_color: tc(0xffd700),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 22 ── material — 💎 #00e676 / #00e5ff / #d500f9
    Theme { name: "material",         emoji: "💎", user_color: tc(0x00e676),    path_color: tc(0x00e5ff),    git_color: tc(0xd500f9),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 23 ── monokai — 🔥 #a6e22e / #f92672 / #e6db74
    Theme { name: "monokai",          emoji: "🔥", user_color: tc(0xa6e22e),    path_color: tc(0xf92672),    git_color: tc(0xe6db74),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 24 ── palenight — 🍇 #c792ea / #89ddff / #ff5370
    Theme { name: "palenight",        emoji: "🍇", user_color: tc(0xc792ea),    path_color: tc(0x89ddff),    git_color: tc(0xff5370),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 25 ── powerlineclassic — blue@host green path cyan git
    Theme { name: "powerlineclassic", emoji: "",   user_color: a8(75),          path_color: a8(34),          git_color: a8(51),          prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 26 ── lambda — λ green / blue ❯
    Theme { name: "lambda",           emoji: "λ",  user_color: a8(46),          path_color: a8(75),          git_color: a8(46),          prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 27 ── hyper — ⚡ #ff00ff / #00ffff / #ffff00
    Theme { name: "hyper",            emoji: "⚡", user_color: tc(0xff00ff),    path_color: tc(0x00ffff),    git_color: tc(0xffff00),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 28 ── slick — ● green / cyan ❯
    Theme { name: "slick",            emoji: "●",  user_color: a8(46),          path_color: a8(51),          git_color: a8(46),          prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 29 ── matrix — 📟 #00ff00 [matrix]❯
    Theme { name: "matrix",           emoji: "📟", user_color: tc(0x00ff00),    path_color: tc(0x00ff00),    git_color: tc(0x00ff00),    prompt_char: "[matrix]❯", line1_prefix: "", line2_prefix: "" },
    // 30 ── sunset — 🌇 #ff6b6b / #feca57 / #5f27cd
    Theme { name: "sunset",           emoji: "🌇", user_color: tc(0xff6b6b),    path_color: tc(0xfeca57),    git_color: tc(0x5f27cd),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 31 ── solarized — ☀️ #b58900 / #2aa198 / #cb4b16
    Theme { name: "solarized",        emoji: "☀️", user_color: tc(0xb58900),    path_color: tc(0x2aa198),    git_color: tc(0xcb4b16),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 32 ── rosepine — 🌹 #eb6f92 / #c4a7e7 / #f6c177
    Theme { name: "rosepine",         emoji: "🌹", user_color: tc(0xeb6f92),    path_color: tc(0xc4a7e7),    git_color: tc(0xf6c177),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 33 ── everforest — 🌲 #a7c080 / #e2b76e / #7fbbb3
    Theme { name: "everforest",       emoji: "🌲", user_color: tc(0xa7c080),    path_color: tc(0xe2b76e),    git_color: tc(0x7fbbb3),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 34 ── kanagawa — 🌊 #7e9cd8 / #e06d76 / #e09e72
    Theme { name: "kanagawa",         emoji: "🌊", user_color: tc(0x7e9cd8),    path_color: tc(0xe06d76),    git_color: tc(0xe09e72),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 35 ── nightowl — 🦉 #82aaff / #7fdbca / #c792ea
    Theme { name: "nightowl",         emoji: "🦉", user_color: tc(0x82aaff),    path_color: tc(0x7fdbca),    git_color: tc(0xc792ea),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 36 ── cobalt2 — ⚡ #ffc400 / #0088ff / #00e5ff
    Theme { name: "cobalt2",          emoji: "⚡", user_color: tc(0xffc400),    path_color: tc(0x0088ff),    git_color: tc(0x00e5ff),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 37 ── shadesofpurple — 🍇 #bd93f9 / #ffd600 / #ff8000 ❯❯
    Theme { name: "shadesofpurple",   emoji: "🍇", user_color: tc(0xbd93f9),    path_color: tc(0xffd600),    git_color: tc(0xff8000),    prompt_char: "❯❯",   line1_prefix: "",   line2_prefix: ""   },
    // 38 ── ayu — 🎨 #ff8f40 / #95e6cb / #ffd700
    Theme { name: "ayu",              emoji: "🎨", user_color: tc(0xff8f40),    path_color: tc(0x95e6cb),    git_color: tc(0xffd700),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 39 ── snazzy — ✨ #ff5c57 / #5ffaef / #ff6ac1
    Theme { name: "snazzy",           emoji: "✨", user_color: tc(0xff5c57),    path_color: tc(0x5ffaef),    git_color: tc(0xff6ac1),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 40 ── outrun — 🌆 #ff007f / #00f0ff / #ffc800 ▶▶
    Theme { name: "outrun",           emoji: "🌆", user_color: tc(0xff007f),    path_color: tc(0x00f0ff),    git_color: tc(0xffc800),    prompt_char: "▶▶",    line1_prefix: "",   line2_prefix: ""   },
    // 41 ── oceanic — 🌊 #6699cc / #ec5f67 / #5fb3b3
    Theme { name: "oceanic",          emoji: "🌊", user_color: tc(0x6699cc),    path_color: tc(0xec5f67),    git_color: tc(0x5fb3b3),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 42 ── moonlight — 🌙 #82aaff / #b4a0ff / #87ddff
    Theme { name: "moonlight",        emoji: "🌙", user_color: tc(0x82aaff),    path_color: tc(0xb4a0ff),    git_color: tc(0x87ddff),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 43 ── papercolor — 📜 #5faf87 / #d78700 / #af005f
    Theme { name: "papercolor",       emoji: "📜", user_color: tc(0x5faf87),    path_color: tc(0xd78700),    git_color: tc(0xaf005f),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 44 ── horizon — 🌄 #e95c72 / #f0907a / #fac591
    Theme { name: "horizon",          emoji: "🌄", user_color: tc(0xe95c72),    path_color: tc(0xf0907a),    git_color: tc(0xfac591),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 45 ── catppuccin_frappe — ☕ #ca9ee6 / #bac2de / #99d1db ❯❯
    Theme { name: "catppuccin_frappe",emoji: "☕", user_color: tc(0xca9ee6),    path_color: tc(0xbac2de),    git_color: tc(0x99d1db),    prompt_char: "❯❯",   line1_prefix: "",   line2_prefix: ""   },
    // 46 ── dracula_pro — 🗡️ #a27aff / #ff80bf / #80ffea ▶
    Theme { name: "dracula_pro",      emoji: "🗡️", user_color: tc(0xa27aff),    path_color: tc(0xff80bf),    git_color: tc(0x80ffea),    prompt_char: "▶",     line1_prefix: "",   line2_prefix: ""   },
    // 47 ── cyber_samurai — 🥷 #ff2a6d / #05d9e8 / #fff200 ❯❯❯
    Theme { name: "cyber_samurai",    emoji: "🥷", user_color: tc(0xff2a6d),    path_color: tc(0x05d9e8),    git_color: tc(0xfff200),    prompt_char: "❯❯❯",  line1_prefix: "",   line2_prefix: ""   },
    // 48 ── evergreen — 🍃 #2ecc71 / #1abc9c / #3498db
    Theme { name: "evergreen",        emoji: "🍃", user_color: tc(0x2ecc71),    path_color: tc(0x1abc9c),    git_color: tc(0x3498db),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 49 ── ghost — 👻 #bdc3c7 / #95a5a6 / #ecf0f1
    Theme { name: "ghost",            emoji: "👻", user_color: tc(0xbdc3c7),    path_color: tc(0x95a5a6),    git_color: tc(0xecf0f1),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 50 ── oxide — ⚙️ #d35400 / #e67e22 / #f1c40f
    Theme { name: "oxide",            emoji: "⚙️", user_color: tc(0xd35400),    path_color: tc(0xe67e22),    git_color: tc(0xf1c40f),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 51 ── neon_pulse — 🔮 #39ff14 / #ff1493 / #00ffff ⚡
    Theme { name: "neon_pulse",       emoji: "🔮", user_color: tc(0x39ff14),    path_color: tc(0xff1493),    git_color: tc(0x00ffff),    prompt_char: "⚡",    line1_prefix: "",   line2_prefix: ""   },
    // 52 ── volcano — 🌋 #e74c3c / #c0392b / #f39c12 ▶
    Theme { name: "volcano",          emoji: "🌋", user_color: tc(0xe74c3c),    path_color: tc(0xc0392b),    git_color: tc(0xf39c12),    prompt_char: "▶",     line1_prefix: "",   line2_prefix: ""   },
    // 53 ── sakura — 🌸 #ffb7b2 / #ffda09 / #e2f0cb
    Theme { name: "sakura",           emoji: "🌸", user_color: tc(0xffb7b2),    path_color: tc(0xffda09),    git_color: tc(0xe2f0cb),    prompt_char: "❯",     line1_prefix: "",   line2_prefix: ""   },
    // 54 ── galaxy — 🌌 #9b59b6 / #8e44ad / #3498db ✨❯
    Theme { name: "galaxy",           emoji: "🌌", user_color: tc(0x9b59b6),    path_color: tc(0x8e44ad),    git_color: tc(0x3498db),    prompt_char: "✨ ❯",  line1_prefix: "",   line2_prefix: ""   },
];

// ── Internal write helpers (no allocation) ────────────────────────────────────

#[inline(always)]
fn write_bytes(dst: &mut [u8], offset: &mut usize, src: &[u8]) -> bool {
    let end = *offset + src.len();
    if end > dst.len() { return false; }
    dst[*offset..end].copy_from_slice(src);
    *offset = end;
    true
}

#[inline(always)]
fn write_str(dst: &mut [u8], offset: &mut usize, s: &str) -> bool {
    write_bytes(dst, offset, s.as_bytes())
}

// ── ANSI helpers ──────────────────────────────────────────────────────────────

const RESET: &str = "\x1b[0m";
#[allow(dead_code)]
const BOLD: &str = "\x1b[1m";

/// Tiny fixed-capacity stack string — large enough for any ANSI sequence.
/// True-color: `\x1b[38;2;255;255;255m` = 19 bytes → 24-byte buffer is safe.
struct AnsiSeq {
    buf: [u8; 24],
    len: usize,
}
impl AnsiSeq {
    fn new() -> Self { Self { buf: [0u8; 24], len: 0 } }
    fn push(&mut self, c: char) {
        if self.len < 24 { self.buf[self.len] = c as u8; self.len += 1; }
    }
    fn push_str(&mut self, s: &str) {
        for c in s.chars() { self.push(c); }
    }
    #[allow(dead_code)]
    fn as_bytes(&self) -> &[u8] { &self.buf[..self.len] }
    fn as_str(&self) -> &str { std::str::from_utf8(&self.buf[..self.len]).unwrap_or("") }
}

/// Write decimal digits of n into seq (no alloc).
#[inline]
fn push_decimal(seq: &mut AnsiSeq, n: u8) {
    if n >= 100 {
        seq.push((b'0' + n / 100) as char);
        seq.push((b'0' + (n / 10) % 10) as char);
        seq.push((b'0' + n % 10) as char);
    } else if n >= 10 {
        seq.push((b'0' + n / 10) as char);
        seq.push((b'0' + n % 10) as char);
    } else {
        seq.push((b'0' + n) as char);
    }
}

/// Build `\x1b[38;5;Nm` (ANSI 256-color foreground).
#[inline]
fn ansi_fg(n: u8) -> AnsiSeq {
    let mut seq = AnsiSeq::new();
    seq.push_str("\x1b[38;5;");
    push_decimal(&mut seq, n);
    seq.push('m');
    seq
}

/// Build `\x1b[38;2;R;G;Bm` (24-bit true-color foreground).
#[inline]
fn ansi_fg_true(r: u8, g: u8, b: u8) -> AnsiSeq {
    let mut seq = AnsiSeq::new();
    seq.push_str("\x1b[38;2;");
    push_decimal(&mut seq, r);
    seq.push(';');
    push_decimal(&mut seq, g);
    seq.push(';');
    push_decimal(&mut seq, b);
    seq.push('m');
    seq
}

/// Wrap an ANSI escape in shell non-printing markers, then write into buf.
#[inline(always)]
fn write_ansi(dst: &mut [u8], offset: &mut usize, ansi: &str, shell: u8) -> bool {
    match shell {
        0 => write_str(dst, offset, "%{") && write_str(dst, offset, ansi) && write_str(dst, offset, "%}"),
        1 => write_str(dst, offset, "\\[") && write_str(dst, offset, ansi) && write_str(dst, offset, "\\]"),
        _ => write_str(dst, offset, ansi),
    }
}

/// Write a theme color (ANSI256 or true-color) as a foreground escape.
#[inline]
fn write_theme_color(dst: &mut [u8], off: &mut usize, color: u32, shell: u8) -> bool {
    if is_true_color(color) {
        let seq = ansi_fg_true(tc_r(color), tc_g(color), tc_b(color));
        write_ansi(dst, off, seq.as_str(), shell)
    } else {
        let seq = ansi_fg(a8_idx(color));
        write_ansi(dst, off, seq.as_str(), shell)
    }
}

// ── Active Theme Persistence ──────────────────────────────────────────────────

/// Returns path to active theme config file (~/.config/fancybash/theme).
pub fn theme_config_path() -> std::path::PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        std::path::PathBuf::from(home).join(".config").join("fancybash").join("theme")
    } else {
        std::path::PathBuf::from(".fancybash_theme")
    }
}

/// Reads the currently active theme index. Falls back to 0 (minimal).
pub fn active_theme_id() -> usize {
    let path = theme_config_path();
    if let Ok(content) = std::fs::read_to_string(&path) {
        let trimmed = content.trim();
        if let Ok(id) = trimmed.parse::<usize>() {
            if id < THEMES.len() { return id; }
        }
        if let Some((i, _)) = THEMES.iter().enumerate().find(|(_, t)| t.name.eq_ignore_ascii_case(trimmed)) {
            return i;
        }
    }
    // Fallback: check legacy ~/.fancybash_theme (written by fancy_theme Zsh fn)
    if let Ok(home) = std::env::var("HOME") {
        let legacy = std::path::PathBuf::from(home).join(".fancybash_theme");
        if let Ok(content) = std::fs::read_to_string(&legacy) {
            let trimmed = content.trim();
            if let Some((i, _)) = THEMES.iter().enumerate().find(|(_, t)| t.name.eq_ignore_ascii_case(trimmed)) {
                return i;
            }
        }
    }
    0
}

/// Saves the selected theme name to ~/.config/fancybash/theme.
/// Saves the selected theme name to ~/.config/fancybash/theme.
pub fn set_active_theme(name: &str) -> Result<usize, String> {
    let name_trim = name.trim();
    if let Ok(num) = name_trim.parse::<usize>() {
        let idx = if num > 0 { num - 1 } else { 0 };
        if idx < THEMES.len() {
            let theme = &THEMES[idx];
            let path = theme_config_path();
            if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); }
            if std::fs::write(&path, theme.name).is_ok() {
                return Ok(idx);
            }
        }
    }

    if let Some((i, theme)) = THEMES.iter().enumerate().find(|(_, t)| t.name.eq_ignore_ascii_case(name_trim)) {
        let path = theme_config_path();
        if let Some(parent) = path.parent() { let _ = std::fs::create_dir_all(parent); }
        if std::fs::write(&path, theme.name).is_ok() {
            Ok(i)
        } else {
            Err(format!("failed to write theme config to {}", path.display()))
        }
    } else {
        Err(format!("unknown theme '{name}'"))
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Context passed to the renderer on every prompt call.
#[repr(C)]
pub struct PromptContext {
    /// Current working directory (NUL-terminated, from $PWD)
    pub cwd: [u8; 512],
    pub cwd_len: usize,
    /// Username (NUL-terminated, from $USER)
    pub user: [u8; 64],
    pub user_len: usize,
    /// Hostname (NUL-terminated, from $HOSTNAME)
    pub host: [u8; 64],
    pub host_len: usize,
    /// Git branch name (empty → not in a git repo)
    pub git_branch: [u8; 128],
    pub git_branch_len: usize,
    /// true = working tree has uncommitted changes
    pub git_dirty: bool,
    /// Last command exit status (0 = success)
    pub last_exit: i32,
    /// Theme index (into THEMES static table)
    pub theme_id: usize,
    /// Target shell: 0 = Zsh (%{...%}), 1 = Bash (\[...\]), 2 = Fish/Pwsh (raw)
    pub shell: u8,
    /// Command duration in milliseconds (for future use)
    pub cmd_duration_ms: u64,
}

impl Default for PromptContext {
    fn default() -> Self {
        // SAFETY: all fields are plain integer/bool/array types; zeroing is valid.
        unsafe { std::mem::zeroed() }
    }
}

/// Core render — writes a two-line ANSI prompt into `buf`.
///
/// Line 1: `{line1_prefix} {emoji} {user_color}user@host{reset} {path_color}~/path{reset}  {git_color}[🌿 branch]{reset}`
/// Line 2: `{line2_prefix} {green|red}{prompt_char}{reset} `
///
/// # Zero-allocation guarantee
/// Uses only the caller-supplied buffer. No Box, Vec, or String is created.
pub fn render(ctx: &PromptContext, buf: &mut [u8]) -> Result<usize, &'static str> {
    let theme = THEMES.get(ctx.theme_id).unwrap_or(&THEMES[0]);
    let mut off = 0usize;
    let s = ctx.shell;

    // ── Line 1 ───────────────────────────────────────────────────────────────

    // Structural prefix (e.g. "╭─" for p10k/bira, "" for most)
    if !theme.line1_prefix.is_empty() {
        if !write_str(buf, &mut off, theme.line1_prefix) { return Err("buffer too small"); }
        write_str(buf, &mut off, " ");
    }

    // Decorative emoji (e.g. "🌌" for tokyonight, "🧛" for dracula)
    if !theme.emoji.is_empty() {
        write_str(buf, &mut off, theme.emoji);
        write_str(buf, &mut off, " ");
    }

    // User@host in user_color (bold) — skipped if user_color == 0 (e.g. minimal theme)
    if theme.user_color != 0 {
        if !write_theme_color(buf, &mut off, theme.user_color, s) { return Err("buffer too small"); }
        if !write_ansi(buf, &mut off, BOLD, s) { return Err("buffer too small"); }

        let user = std::str::from_utf8(&ctx.user[..ctx.user_len]).unwrap_or("user");
        let host = std::str::from_utf8(&ctx.host[..ctx.host_len]).unwrap_or("host");
        write_str(buf, &mut off, user);
        write_str(buf, &mut off, "@");
        write_str(buf, &mut off, host);
        write_ansi(buf, &mut off, RESET, s);
        write_str(buf, &mut off, " ");
    }

    // CWD in path_color
    write_theme_color(buf, &mut off, theme.path_color, s);
    let cwd = std::str::from_utf8(&ctx.cwd[..ctx.cwd_len]).unwrap_or("~");
    write_str(buf, &mut off, cwd);
    write_ansi(buf, &mut off, RESET, s);

    // Git segment (only when inside a git repo)
    if ctx.git_branch_len > 0 {
        write_str(buf, &mut off, "  ");
        write_theme_color(buf, &mut off, theme.git_color, s);
        write_str(buf, &mut off, "[🌿 ");
        let branch = std::str::from_utf8(&ctx.git_branch[..ctx.git_branch_len]).unwrap_or("?");
        write_str(buf, &mut off, branch);
        if ctx.git_dirty { write_str(buf, &mut off, " ❗"); }
        write_str(buf, &mut off, "]");
        write_ansi(buf, &mut off, RESET, s);
    }

    write_str(buf, &mut off, "\n");

    // ── Line 2 ───────────────────────────────────────────────────────────────

    if !theme.line2_prefix.is_empty() {
        write_str(buf, &mut off, theme.line2_prefix);
        write_str(buf, &mut off, " ");
    }

    // Prompt character: green on success, red on error
    if ctx.last_exit == 0 {
        write_ansi(buf, &mut off, "\x1b[1;32m", s);
    } else {
        write_ansi(buf, &mut off, "\x1b[1;31m", s);
    }
    write_str(buf, &mut off, theme.prompt_char);
    write_ansi(buf, &mut off, RESET, s);
    write_str(buf, &mut off, " ");

    Ok(off)
}

// ── C-ABI shim used by lib.rs ─────────────────────────────────────────────────

/// Write a prompt rendered with a default context into a raw C buffer.
///
/// # Safety
/// `out` must be a writable buffer of at least `len` bytes.
pub unsafe fn render_into_raw(
    out: *mut c_char,
    len: usize,
    theme_id: usize,
) -> Result<usize, &'static str> {
    let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(out as *mut u8, len) };
    let ctx = PromptContext { theme_id, ..Default::default() };
    let written = render(&ctx, buf)?;
    if written < len { buf[written] = 0; }
    Ok(written)
}

// =============================================================================
//  Unit tests
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_55_themes_exist() {
        // Exactly 55 themes to mirror config.zsh fb_theme_* functions.
        assert_eq!(THEMES.len(), 55,
            "expected 55 themes (to match config.zsh), got {}", THEMES.len());
    }

    #[test]
    fn theme_names_match_config_zsh() {
        let expected = [
            "minimal", "full", "robbyrussell", "p10k", "agnoster",
            "catppuccin", "tokyonight", "dracula", "nord", "bira",
            "pure", "starship", "cyberpunk", "synthwave", "gruvbox",
            "onedark", "sorin", "spaceship", "halflife", "paradox",
            "bureau", "gallifrey", "material", "monokai", "palenight",
            "powerlineclassic", "lambda", "hyper", "slick", "matrix",
            "sunset", "solarized", "rosepine", "everforest", "kanagawa",
            "nightowl", "cobalt2", "shadesofpurple", "ayu", "snazzy",
            "outrun", "oceanic", "moonlight", "papercolor", "horizon",
            "catppuccin_frappe", "dracula_pro", "cyber_samurai", "evergreen",
            "ghost", "oxide", "neon_pulse", "volcano", "sakura", "galaxy",
        ];
        assert_eq!(THEMES.len(), expected.len());
        for (i, (theme, &exp_name)) in THEMES.iter().zip(expected.iter()).enumerate() {
            assert_eq!(theme.name, exp_name,
                "theme[{i}]: expected name '{exp_name}', got '{}'", theme.name);
        }
    }

    #[test]
    fn true_color_themes_have_bit31_set() {
        // Themes using hex colors from config.zsh must encode as true-color.
        let tc_themes = ["catppuccin", "tokyonight", "dracula", "nord",
                         "gruvbox", "onedark", "synthwave", "cyberpunk",
                         "galaxy", "sakura", "volcano"];
        for name in tc_themes {
            let t = THEMES.iter().find(|t| t.name == name)
                .unwrap_or_else(|| panic!("theme '{}' not found", name));
            assert!(is_true_color(t.user_color),
                "theme '{}' user_color should be true-color", name);
        }
    }

    #[test]
    fn ansi256_themes_have_no_true_color_bit() {
        // ANSI256-only themes must NOT have bit 31 set.
        let a256_themes = ["minimal", "full", "bira", "pure", "sorin",
                           "bureau", "paradox", "powerlineclassic", "lambda", "slick"];
        for name in a256_themes {
            let t = THEMES.iter().find(|t| t.name == name)
                .unwrap_or_else(|| panic!("theme '{}' not found", name));
            assert!(!is_true_color(t.user_color),
                "theme '{}' user_color should be ANSI256", name);
        }
    }

    #[test]
    fn render_fits_in_4kb_buffer() {
        let ctx = PromptContext { theme_id: 0, ..Default::default() };
        let mut buf = vec![0u8; 4096];
        let written = render(&ctx, &mut buf).unwrap();
        assert!(written > 0);
        assert!(written < 4096);
    }

    #[test]
    fn render_is_fast_enough() {
        // 1000 renders across all 55 themes should take well under 1 second.
        let mut buf = vec![0u8; 4096];
        for i in 0..1000 {
            let ctx = PromptContext { theme_id: i % THEMES.len(), ..Default::default() };
            render(&ctx, &mut buf).unwrap();
        }
    }

    #[test]
    fn ansi_fg_correct() {
        let seq = ansi_fg(36);
        assert_eq!(seq.as_str(), "\x1b[38;5;36m");

        let seq = ansi_fg(0);
        assert_eq!(seq.as_str(), "\x1b[38;5;0m");

        let seq = ansi_fg(255);
        assert_eq!(seq.as_str(), "\x1b[38;5;255m");
    }

    #[test]
    fn ansi_fg_true_correct() {
        // Test #ca9ee6 → R=202, G=158, B=230
        let seq = ansi_fg_true(202, 158, 230);
        assert_eq!(seq.as_str(), "\x1b[38;2;202;158;230m");

        // Test #00ff00 → R=0, G=255, B=0
        let seq = ansi_fg_true(0, 255, 0);
        assert_eq!(seq.as_str(), "\x1b[38;2;0;255;0m");
    }

    #[test]
    fn color_encoding_roundtrip() {
        // True color roundtrip: tc(0xRRGGBB)
        let c = tc(0xca9ee6);
        assert!(is_true_color(c));
        assert_eq!(tc_r(c), 0xca);
        assert_eq!(tc_g(c), 0x9e);
        assert_eq!(tc_b(c), 0xe6);

        // ANSI256 roundtrip
        let c = a8(36);
        assert!(!is_true_color(c));
        assert_eq!(a8_idx(c), 36);
    }

    #[test]
    fn all_themes_render_without_panic() {
        let mut buf = vec![0u8; 4096];
        for (i, theme) in THEMES.iter().enumerate() {
            let ctx = PromptContext {
                theme_id: i,
                git_branch_len: 4,
                git_dirty: true,
                last_exit: if i % 2 == 0 { 0 } else { 1 },
                ..Default::default()
            };
            let result = render(&ctx, &mut buf);
            assert!(result.is_ok(), "theme '{}' (idx {}) failed to render", theme.name, i);
        }
    }
}
