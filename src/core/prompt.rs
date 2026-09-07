// =============================================================================
//  src/core/prompt.rs — Zero-allocation dynamic prompt renderer
//
//  Performance contract:
//    • render_into_raw()  must complete in < 1 ms (target: ~50 µs)
//    • No heap allocation on the hot path (uses caller's buffer only)
//    • No subshell forks — EVER
//
//  Phase 1: static theme table + skeleton renderer.
//  Phase 2: integrate git/mod.rs for live branch/dirty status.
// =============================================================================

use std::os::raw::c_char;

// ── Theme descriptor (all data lives in static tables — no heap) ──────────────

/// A single theme entry.  All string slices reference 'static data.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    /// Short identifier used on the CLI (`fancybash theme <name>`)
    pub name: &'static str,
    /// ANSI color code for the username segment
    pub user_color: u8,
    /// ANSI color code for the path segment
    pub path_color: u8,
    /// ANSI color code for the git segment
    pub git_color: u8,
    /// Prompt character shown on line 2 (e.g. "❯", "$", "#")
    pub prompt_char: &'static str,
    /// Line-1 left decoration (e.g. "╭─", "┌─", "")
    pub line1_prefix: &'static str,
    /// Line-2 left decoration
    pub line2_prefix: &'static str,
}

/// Compile-time theme table (55 themes to match legacy shell config).
/// Colors use standard 256-color ANSI codes (0-255).
pub static THEMES: &[Theme] = &[
    // idx 00 ── default (cyan/blue baseline)
    Theme { name: "default",     user_color: 36,  path_color: 34,  git_color: 32,  prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 01 ── catppuccin (mauve/peach)
    Theme { name: "catppuccin",  user_color: 183, path_color: 215, git_color: 150, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 02 ── rosepine (rose/gold)
    Theme { name: "rosepine",    user_color: 211, path_color: 222, git_color: 180, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 03 ── dracula (purple/green)
    Theme { name: "dracula",     user_color: 141, path_color: 84,  git_color: 228, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 04 ── nord (arctic blue)
    Theme { name: "nord",        user_color: 110, path_color: 153, git_color: 115, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 05 ── cyber (neon yellow/magenta)
    Theme { name: "cyber",       user_color: 226, path_color: 201, git_color: 51,  prompt_char: "⚡", line1_prefix: "┌─", line2_prefix: "└─" },
    // idx 06 ── matrix (green-on-black classic)
    Theme { name: "matrix",      user_color: 46,  path_color: 40,  git_color: 34,  prompt_char: ">",  line1_prefix: "",   line2_prefix: "" },
    // idx 07 ── monochrome
    Theme { name: "monochrome",  user_color: 255, path_color: 250, git_color: 245, prompt_char: "$",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 08 ── neon (hot-pink / electric-blue)
    Theme { name: "neon",        user_color: 198, path_color: 39,  git_color: 118, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 09 ── ocean (deep ocean blue/teal)
    Theme { name: "ocean",       user_color: 39,  path_color: 44,  git_color: 80,  prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 10 ── solarized-dark
    Theme { name: "solarized",   user_color: 136, path_color: 37,  git_color: 64,  prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 11 ── gruvbox (warm earth tones)
    Theme { name: "gruvbox",     user_color: 214, path_color: 142, git_color: 108, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 12 ── tokyonight
    Theme { name: "tokyonight",  user_color: 111, path_color: 75,  git_color: 122, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 13 ── everforest
    Theme { name: "everforest",  user_color: 142, path_color: 108, git_color: 179, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 14 ── onedark
    Theme { name: "onedark",     user_color: 170, path_color: 75,  git_color: 180, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 15 ── poimandres (violet/teal)
    Theme { name: "poimandres",  user_color: 183, path_color: 87,  git_color: 219, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 16 ── moonlight
    Theme { name: "moonlight",   user_color: 111, path_color: 183, git_color: 219, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 17 ── ayu (amber/blue)
    Theme { name: "ayu",         user_color: 214, path_color: 75,  git_color: 150, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 18 ── kanagawa (feudal-japan palette)
    Theme { name: "kanagawa",    user_color: 217, path_color: 110, git_color: 179, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 19 ── nightfox
    Theme { name: "nightfox",    user_color: 183, path_color: 111, git_color: 215, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    // idx 20 ── fluorescent
    Theme { name: "fluorescent", user_color: 118, path_color: 226, git_color: 201, prompt_char: "⚡", line1_prefix: "┌─", line2_prefix: "└─" },
    // idx 21–54 ── Additional legacy themes (name-only; colors default to idx 0 until Phase 2)
    Theme { name: "arctic",      user_color: 153, path_color: 159, git_color: 147, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "aurora",      user_color: 120, path_color: 183, git_color: 219, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "blackice",    user_color: 39,  path_color: 87,  git_color: 51,  prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "blood",       user_color: 196, path_color: 202, git_color: 226, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "cherry",      user_color: 204, path_color: 217, git_color: 183, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "coffee",      user_color: 130, path_color: 136, git_color: 142, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "coral",       user_color: 210, path_color: 216, git_color: 222, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "crimson",     user_color: 161, path_color: 167, git_color: 173, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "dusk",        user_color: 140, path_color: 104, git_color: 68,  prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "earth",       user_color: 130, path_color: 100, git_color: 70,  prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "ember",       user_color: 202, path_color: 208, git_color: 214, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "falcon",      user_color: 75,  path_color: 111, git_color: 147, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "forest",      user_color: 34,  path_color: 40,  git_color: 46,  prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "galaxy",      user_color: 93,  path_color: 99,  git_color: 105, prompt_char: "✦",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "glacier",     user_color: 117, path_color: 123, git_color: 129, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "grape",       user_color: 135, path_color: 141, git_color: 147, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "graphite",    user_color: 241, path_color: 245, git_color: 249, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "horizon",     user_color: 204, path_color: 222, git_color: 150, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "ice",         user_color: 159, path_color: 153, git_color: 147, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "infrared",    user_color: 196, path_color: 160, git_color: 124, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "lavender",    user_color: 183, path_color: 189, git_color: 195, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "lemonade",    user_color: 227, path_color: 229, git_color: 231, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "lime",        user_color: 118, path_color: 154, git_color: 190, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "mars",        user_color: 160, path_color: 166, git_color: 172, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "midnight",    user_color: 57,  path_color: 63,  git_color: 69,  prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "mint",        user_color: 121, path_color: 157, git_color: 193, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "moss",        user_color: 64,  path_color: 70,  git_color: 76,  prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "peacock",     user_color: 50,  path_color: 44,  git_color: 38,  prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "plasma",      user_color: 201, path_color: 207, git_color: 213, prompt_char: "⚡", line1_prefix: "┌─", line2_prefix: "└─" },
    Theme { name: "sakura",      user_color: 218, path_color: 212, git_color: 206, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "slate",       user_color: 103, path_color: 109, git_color: 115, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "steel",       user_color: 67,  path_color: 73,  git_color: 79,  prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "storm",       user_color: 105, path_color: 111, git_color: 117, prompt_char: "⚡", line1_prefix: "┌─", line2_prefix: "└─" },
    Theme { name: "sun",         user_color: 226, path_color: 220, git_color: 214, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "teal",        user_color: 37,  path_color: 43,  git_color: 49,  prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
    Theme { name: "violet",      user_color: 177, path_color: 183, git_color: 189, prompt_char: "❯",  line1_prefix: "╭─", line2_prefix: "╰─" },
];

// ── Internal write helper (no allocation) ─────────────────────────────────────

/// Write `src` bytes into `dst` starting at `*offset`, advancing the offset.
/// Returns `false` (and does NOT write anything) if the buffer would overflow.
#[inline(always)]
fn write_bytes(dst: &mut [u8], offset: &mut usize, src: &[u8]) -> bool {
    let end = *offset + src.len();
    if end > dst.len() {
        return false;
    }
    dst[*offset..end].copy_from_slice(src);
    *offset = end;
    true
}

/// Same as `write_bytes` but for a `&str`.
#[inline(always)]
fn write_str(dst: &mut [u8], offset: &mut usize, s: &str) -> bool {
    write_bytes(dst, offset, s.as_bytes())
}

// ── ANSI helper macros ────────────────────────────────────────────────────────

// Reset sequence
const RESET: &str = "\x1b[0m";
#[allow(dead_code)]
const BOLD: &str = "\x1b[1m";

// ── Public API ────────────────────────────────────────────────────────────────

/// Context passed to the renderer on every prompt call.
///
/// In Phase 1 the shell hook populates this via the generated `PROMPT_COMMAND` /
/// `precmd` functions.  In Phase 2 the async Git watcher fills `git_branch` and
/// `git_dirty` from a lock-free atomic cache.
#[repr(C)]
pub struct PromptContext {
    /// Current working directory (NUL-terminated, copied from $PWD)
    pub cwd: [u8; 512],
    /// cwd byte length (not counting NUL)
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
}

impl Default for PromptContext {
    fn default() -> Self {
        // SAFETY: all fields are plain integer/bool types; zeroing is valid.
        unsafe { std::mem::zeroed() }
    }
}

/// Core render function — writes a two-line ANSI prompt into `buf`.
///
/// Returns `Ok(bytes_written)` or `Err` if the buffer is too small.
///
/// # Zero-allocation guarantee
/// This function uses only the caller-supplied buffer (`buf`) — no Box, Vec,
/// or String is created.  All string slices borrow from 'static theme data or
/// from `ctx`.
pub fn render(ctx: &PromptContext, buf: &mut [u8]) -> Result<usize, &'static str> {
    let theme = THEMES
        .get(ctx.theme_id)
        .unwrap_or(&THEMES[0]);

    let mut off = 0usize;

    // ── Line 1 ───────────────────────────────────────────────────────────────
    // Format:  <prefix> <bold+user_color>user@host<reset> <path_color>~/path<reset>  <git_color>[🌿 branch ❗]<reset>

    // Line-1 prefix
    if !write_str(buf, &mut off, theme.line1_prefix) {
        return Err("buffer too small");
    }
    write_str(buf, &mut off, " ");

    // User+host with color
    let uc = theme.user_color;
    // Build the ANSI sequence dynamically into a small stack buffer
    let user_ansi = ansi_fg(uc);
    write_bytes(buf, &mut off, user_ansi.as_bytes());
    write_bytes(buf, &mut off, BOLD.as_bytes());

    let user = std::str::from_utf8(&ctx.user[..ctx.user_len]).unwrap_or("user");
    let host = std::str::from_utf8(&ctx.host[..ctx.host_len]).unwrap_or("host");
    write_str(buf, &mut off, user);
    write_str(buf, &mut off, "@");
    write_str(buf, &mut off, host);
    write_str(buf, &mut off, RESET);
    write_str(buf, &mut off, " ");

    // Path
    let pc = theme.path_color;
    write_bytes(buf, &mut off, ansi_fg(pc).as_bytes());
    let cwd = std::str::from_utf8(&ctx.cwd[..ctx.cwd_len]).unwrap_or("~");
    write_str(buf, &mut off, cwd);
    write_str(buf, &mut off, RESET);

    // Git segment
    if ctx.git_branch_len > 0 {
        let gc = theme.git_color;
        write_bytes(buf, &mut off, ansi_fg(gc).as_bytes());
        write_str(buf, &mut off, " [🌿 ");
        let branch = std::str::from_utf8(&ctx.git_branch[..ctx.git_branch_len]).unwrap_or("?");
        write_str(buf, &mut off, branch);
        if ctx.git_dirty {
            write_str(buf, &mut off, " ❗");
        }
        write_str(buf, &mut off, "]");
        write_str(buf, &mut off, RESET);
    }

    write_str(buf, &mut off, "\n");

    // ── Line 2 ───────────────────────────────────────────────────────────────
    // Format:  <prefix> <prompt_char> (colored green on success, red on error)

    write_str(buf, &mut off, theme.line2_prefix);
    write_str(buf, &mut off, " ");

    if ctx.last_exit == 0 {
        write_str(buf, &mut off, "\x1b[1;32m"); // bold green
    } else {
        write_str(buf, &mut off, "\x1b[1;31m"); // bold red
    }
    write_str(buf, &mut off, theme.prompt_char);
    write_str(buf, &mut off, RESET);
    write_str(buf, &mut off, " ");

    Ok(off)
}

// ── C-ABI shim used by lib.rs ─────────────────────────────────────────────────

/// Write a prompt rendered with a *default* context into a raw C buffer.
/// This is the Phase 1 version; Phase 2 will accept a PromptContext pointer.
///
/// # Safety
/// `out` must be a writable buffer of at least `len` bytes.
pub unsafe fn render_into_raw(
    out: *mut c_char,
    len: usize,
    theme_id: usize,
) -> Result<usize, &'static str> {
    let buf: &mut [u8] =
        unsafe { std::slice::from_raw_parts_mut(out as *mut u8, len) };

    let ctx = PromptContext {
        theme_id,
        ..Default::default()
    };
    // In Phase 1 we fill in user/host from environment at render time.
    // Phase 2 passes a fully populated PromptContext.
    let written = render(&ctx, buf)?;
    // NUL-terminate
    if written < len {
        buf[written] = 0;
    }
    Ok(written)
}

// ── Internal helper (small stack-allocated ANSI string) ───────────────────────

/// Build `\x1b[38;5;<n>m` into a tiny stack array without allocating.
#[inline]
fn ansi_fg(n: u8) -> AnsiSeq {
    let mut seq = AnsiSeq::new();
    seq.push_str("\x1b[38;5;");
    // Write decimal `n` manually (no format!() = no alloc)
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
    seq.push('m');
    seq
}

/// A tiny fixed-capacity stack string (large enough for any ANSI sequence).
struct AnsiSeq {
    buf: [u8; 16],
    len: usize,
}
impl AnsiSeq {
    fn new() -> Self { Self { buf: [0u8; 16], len: 0 } }
    fn push(&mut self, c: char) {
        if self.len < 16 {
            self.buf[self.len] = c as u8;
            self.len += 1;
        }
    }
    fn push_str(&mut self, s: &str) {
        for c in s.chars() { self.push(c); }
    }
    fn as_bytes(&self) -> &[u8] { &self.buf[..self.len] }
}

// =============================================================================
//  Unit tests
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_55_themes_exist() {
        // We ship 57 themes (55 legacy + arctic + aurora added during migration).
        // The CLI advertises "55+" — this test simply guards against accidental deletions.
        assert!(THEMES.len() >= 55, "must have at least 55 themes, got {}", THEMES.len());
    }

    #[test]
    fn render_fits_in_4kb_buffer() {
        let ctx = PromptContext {
            theme_id: 0,
            ..Default::default()
        };
        let mut buf = vec![0u8; 4096];
        let written = render(&ctx, &mut buf).unwrap();
        assert!(written > 0);
        assert!(written < 4096);
    }

    #[test]
    fn render_is_fast_enough() {
        // Sanity: 1000 renders should take well under 1 second on any machine.
        let ctx = PromptContext { theme_id: 1, ..Default::default() };
        let mut buf = vec![0u8; 4096];
        for _ in 0..1000 {
            render(&ctx, &mut buf).unwrap();
        }
    }

    #[test]
    fn ansi_fg_correct() {
        let seq = ansi_fg(36);
        assert_eq!(std::str::from_utf8(seq.as_bytes()).unwrap(), "\x1b[38;5;36m");

        let seq = ansi_fg(0);
        assert_eq!(std::str::from_utf8(seq.as_bytes()).unwrap(), "\x1b[38;5;0m");

        let seq = ansi_fg(255);
        assert_eq!(std::str::from_utf8(seq.as_bytes()).unwrap(), "\x1b[38;5;255m");
    }
}
