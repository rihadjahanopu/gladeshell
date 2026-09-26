// ============================================================================
//  src/renderer.rs — Atomic Double-Buffered Redraw Engine
//
//  SAFETY GUARANTEES:
//   • Zero raw byte-index slicing — ghost tail extracted via char-count skip.
//   • ANSI width calculator handles CSI, SS3, OSC, DCS, and PM sequences.
//   • Ghost text is capped at MAX_GHOST_LEN to prevent megabyte output.
//   • No stdout/stderr pollution on failure.
// ============================================================================

use std::io::{self, Write};
use unicode_width::UnicodeWidthChar;
use crate::buffer_engine::BufferEngine;

/// Maximum number of characters to render in ghost text (suggestion tail).
/// Prevents runaway output if a corrupt/very-long suggestion is in the cache.
const MAX_GHOST_LEN: usize = 512;

pub struct Renderer {
    ghost_color_ansi: String,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            ghost_color_ansi: "\x1b[90m".to_string(),
        }
    }

    /// Override the ghost text ANSI escape code (default: ANSI 90, dark gray).
    pub fn with_ghost_color(mut self, ansi_code: &str) -> Self {
        self.ghost_color_ansi = ansi_code.to_string();
        self
    }

    /// Build the complete atomic redraw escape sequence into a `String`.
    ///
    /// Sequence:
    ///  1. `\x1b[G`         — Move to column 1 (beginning of current line)
    ///  2. `\x1b[K`         — Erase from cursor to end of line
    ///  3. prompt_prefix    — Prompt string (e.g. `user@host:~$ `)
    ///  4. `\x1b[K`         — Erase again after prefix (handles prefix ANSI codes)
    ///  5. Syntax-highlighted typed text
    ///  6. Ghost suggestion  — dim gray tail, only when cursor is at EOL
    ///  7. `\x1b[0m`        — Reset attributes
    ///  8. `\x1b[{col}G`    — Reposition cursor to user's edit position
    pub fn render_line_to_string(&self, buffer: &BufferEngine, prompt_prefix: &str) -> String {
        let mut out = String::with_capacity(512);

        // 1+2. Move to column 1 and erase to end-of-line
        out.push_str("\x1b[G\x1b[K");

        // 3+4. Prompt prefix
        if !prompt_prefix.is_empty() {
            out.push_str(prompt_prefix);
            out.push_str("\x1b[K");
        }

        // 5. Highlighted typed content
        let typed = buffer.typed_buffer();
        out.push_str(&self.render_syntax_highlighted(typed));

        // 6. Ghost text — only when cursor is exactly at the end of typed buffer
        if buffer.cursor_position() == buffer.char_count() {
            if let Some(sug) = buffer.active_suggestion() {
                // Safe char-count-based tail extraction — no raw byte slicing
                let typed_char_count = typed.chars().count();
                let sug_char_count = sug.chars().count();

                if sug.starts_with(typed) && sug_char_count > typed_char_count {
                    // Collect the tail as characters, capped at MAX_GHOST_LEN
                    let ghost_tail: String = sug
                        .chars()
                        .skip(typed_char_count)
                        .take(MAX_GHOST_LEN)
                        .collect();

                    if !ghost_tail.is_empty() {
                        out.push_str(&self.ghost_color_ansi);
                        out.push_str(&ghost_tail);
                        out.push_str("\x1b[0m");
                    }
                }
            }
        }

        // 7. Reposition cursor to the user's actual edit position
        // We must strip ANSI escapes from the prompt prefix before measuring
        // its visible width, otherwise the column calculation is wrong.
        let prompt_cols = self.visible_width(prompt_prefix);
        let cursor_text: String = typed.chars().take(buffer.cursor_position()).collect();
        let cursor_cols = self.visible_width(&cursor_text);
        // Terminals use 1-based column numbers
        let target_col = prompt_cols + cursor_cols + 1;
        out.push_str(&format!("\x1b[{}G", target_col));

        out
    }

    /// Write the full atomic redraw sequence to `writer` in a single flush.
    /// Single-syscall write minimizes flicker in xterm.js / IDE terminals.
    pub fn redraw<W: Write>(
        &self,
        writer: &mut W,
        buffer: &BufferEngine,
        prompt_prefix: &str,
    ) -> io::Result<()> {
        let render_str = self.render_line_to_string(buffer, prompt_prefix);
        writer.write_all(render_str.as_bytes())?;
        writer.flush()
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Lightweight built-in token syntax highlighter for the render path.
    ///
    /// Applies color to: command word (green), flags (cyan), quoted strings (yellow).
    /// Falls back to no color for plain arguments.
    fn render_syntax_highlighted(&self, text: &str) -> String {
        if text.trim().is_empty() {
            return text.to_string();
        }

        let mut highlighted = String::with_capacity(text.len() + 64);
        let mut token_index = 0usize;

        for token in text.split_inclusive(|c: char| c.is_whitespace()) {
            let trimmed = token.trim();
            if trimmed.is_empty() {
                highlighted.push_str(token);
                continue;
            }

            // Separate the trailing whitespace from the word
            let word_end = token.len() - (token.len() - token.trim_end().len());
            let (word, ws) = token.split_at(word_end);
            // Note: split_at on a &str requires a byte boundary; since we
            // computed word_end by trimming (which respects char boundaries),
            // this is safe.

            if word == "|" || word == "&&" || word == "||" || word == ";" {
                // Pipe / Operator → bold yellow
                highlighted.push_str("\x1b[1;33m");
                highlighted.push_str(word);
                highlighted.push_str("\x1b[0m");
                token_index = 0; // Next token after pipe/operator is a new command word
                highlighted.push_str(ws);
                continue;
            }

            if token_index == 0 {
                // First token → command word, bold green
                highlighted.push_str("\x1b[1;32m");
                highlighted.push_str(word);
                highlighted.push_str("\x1b[0m");
            } else if word.starts_with('-') {
                // Flag / option → magenta
                highlighted.push_str("\x1b[35m");
                highlighted.push_str(word);
                highlighted.push_str("\x1b[0m");
            } else if word.starts_with('"') || word.starts_with('\'') {
                // String literal → cyan
                highlighted.push_str("\x1b[36m");
                highlighted.push_str(word);
                highlighted.push_str("\x1b[0m");
            } else if word.starts_with('$') {
                // Variable → magenta bold
                highlighted.push_str("\x1b[1;35m");
                highlighted.push_str(word);
                highlighted.push_str("\x1b[0m");
            } else if word == ">" || word == ">>" || word == "<" {
                // Redirection → yellow
                highlighted.push_str("\x1b[33m");
                highlighted.push_str(word);
                highlighted.push_str("\x1b[0m");
            } else {
                highlighted.push_str(word);
            }

            highlighted.push_str(ws);
            token_index += 1;
        }

        highlighted
    }

    /// Calculate the visible terminal column width of a string.
    ///
    /// Correctly strips:
    ///  - CSI sequences: `ESC [ ... <letter>`
    ///  - SS3 sequences: `ESC O <letter>`
    ///  - OSC sequences: `ESC ] ... (BEL or ESC \\)` (used by hyperlinks, iTerm2)
    ///  - DCS/PM/APC:    `ESC [PX^_] ... ESC \\`
    ///  - Simple 2-byte: `ESC <single-char>`
    ///
    /// Uses `unicode_width` for accurate CJK / emoji column widths.
    fn visible_width(&self, s: &str) -> usize {
        let mut width = 0usize;
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c != '\x1b' {
                // Regular character — measure its display width
                width += UnicodeWidthChar::width(c).unwrap_or(1);
                continue;
            }

            // ESC seen — determine escape sequence type
            match chars.peek() {
                None => break, // bare ESC at end

                Some(&'[') => {
                    // CSI: ESC [ ... <final byte in 0x40-0x7E>
                    chars.next(); // consume '['
                    for c2 in chars.by_ref() {
                        if ('\x40'..='\x7e').contains(&c2) {
                            break; // final byte consumed
                        }
                    }
                }

                Some(&'O') => {
                    // SS3: ESC O <single char>
                    chars.next(); // consume 'O'
                    chars.next(); // consume the SS3 final byte
                }

                Some(&']') => {
                    // OSC: ESC ] ... BEL  or  ESC ] ... ESC \\
                    chars.next(); // consume ']'
                    loop {
                        match chars.next() {
                            None | Some('\x07') => break, // BEL terminates
                            Some('\x1b') => {
                                if chars.peek() == Some(&'\\') {
                                    chars.next(); // consume '\\' (ST)
                                }
                                break;
                            }
                            _ => {} // consume body
                        }
                    }
                }

                Some(&c2) if matches!(c2, 'P' | 'X' | '^' | '_') => {
                    // DCS / SOS / PM / APC: same termination as OSC
                    chars.next(); // consume the introducer
                    loop {
                        match chars.next() {
                            None => break,
                            Some('\x1b') => {
                                if chars.peek() == Some(&'\\') {
                                    chars.next();
                                }
                                break;
                            }
                            _ => {}
                        }
                    }
                }

                Some(_) => {
                    // 2-byte escape (e.g. ESC c = RIS, ESC = = DECKPAM)
                    chars.next(); // consume the single-char sequence
                }
            }
        }

        width
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_buffer(text: &str, suggestion: Option<&str>) -> BufferEngine {
        let mut b = BufferEngine::new();
        b.set_typed_buffer(text);
        b.set_active_suggestion(suggestion.map(str::to_string));
        b
    }

    #[test]
    fn test_render_contains_mandatory_escape_sequences() {
        let renderer = Renderer::new();
        let buffer = make_buffer("a", None);
        let out = renderer.render_line_to_string(&buffer, "");
        assert!(out.contains("\x1b[K"), "must contain Erase-In-Line");
        assert!(out.contains("\x1b[G"), "must contain move-to-col-1");
    }

    #[test]
    fn test_ghost_text_renders_suggestion_tail() {
        let renderer = Renderer::new();
        let buffer = make_buffer("git ", Some("git status"));
        let out = renderer.render_line_to_string(&buffer, "$ ");
        assert!(out.contains("status"), "ghost tail must appear");
        assert!(out.contains("\x1b[90m"), "ghost must use gray ANSI code");
    }

    #[test]
    fn test_ghost_text_not_rendered_when_cursor_not_at_end() {
        let renderer = Renderer::new();
        let mut buffer = BufferEngine::new();
        buffer.set_typed_buffer("git ");
        buffer.set_active_suggestion(Some("git status".to_string()));
        buffer.move_home(); // cursor at 0, not at end
        let out = renderer.render_line_to_string(&buffer, "");
        assert!(!out.contains("status"), "ghost must not render when cursor is not at EOL");
    }

    #[test]
    fn test_backspace_clears_stale_ghost_text() {
        let renderer = Renderer::new();
        let mut buffer = BufferEngine::new();
        buffer.set_typed_buffer("git checkout");
        buffer.set_active_suggestion(Some("git checkout main".to_string()));
        let out1 = renderer.render_line_to_string(&buffer, "$ ");
        assert!(out1.contains(" main"));

        buffer.set_typed_buffer("git check");
        buffer.set_active_suggestion(None);
        let out2 = renderer.render_line_to_string(&buffer, "$ ");
        assert!(out2.starts_with("\x1b[G\x1b[K"), "must start with move+erase");
        assert!(!out2.contains(" main"), "stale ghost must be gone");
    }

    #[test]
    fn test_cursor_column_with_emoji() {
        let renderer = Renderer::new();
        let mut buffer = BufferEngine::new();
        buffer.set_typed_buffer("⚡ git");
        buffer.move_home(); // cursor at 0
        let out = renderer.render_line_to_string(&buffer, "$ ");
        // prompt "$ " = 2 cols, cursor at char 0 → target col = 2 + 0 + 1 = 3
        assert!(out.contains("\x1b[3G"), "cursor position must be column 3");
    }

    #[test]
    fn test_cursor_column_cjk_wide_chars() {
        let renderer = Renderer::new();
        let mut buffer = BufferEngine::new();
        // Each CJK char is 2 columns wide; "日本" = 4 columns
        buffer.set_typed_buffer("日本語");
        buffer.move_home();
        buffer.move_right(KeyModifiers::None); // cursor after '日' (char 1)
        let out = renderer.render_line_to_string(&buffer, "");
        // 0 prompt cols + 2 cols for '日' + 1 = col 3
        assert!(out.contains("\x1b[3G"), "CJK cursor must account for 2-col width");
    }

    #[test]
    fn test_visible_width_strips_csi_sequences() {
        let renderer = Renderer::new();
        // "\x1b[1;32m" is 7 bytes but 0 visible columns
        let s = "\x1b[1;32mhello\x1b[0m";
        assert_eq!(renderer.visible_width(s), 5, "ANSI CSI must not count toward width");
    }

    #[test]
    fn test_visible_width_strips_osc_hyperlink() {
        let renderer = Renderer::new();
        // OSC 8 hyperlink: ESC ] 8 ; ; url ST  (simplified)
        let s = "\x1b]8;;https://example.com\x07link\x1b]8;;\x07";
        // "link" = 4 columns
        assert_eq!(renderer.visible_width(s), 4, "OSC sequences must not count toward width");
    }

    #[test]
    fn test_visible_width_ss3_sequence() {
        let renderer = Renderer::new();
        // ESC O C = SS3 Right Arrow, 0 visible cols
        let s = "\x1bOCtext";
        assert_eq!(renderer.visible_width(s), 4, "SS3 must not count toward width");
    }

    #[test]
    fn test_ghost_text_cap() {
        let renderer = Renderer::new();
        // Create a suggestion longer than MAX_GHOST_LEN
        let long_sug = format!("a{}", "x".repeat(MAX_GHOST_LEN + 100));
        let buffer = make_buffer("a", Some(&long_sug));
        let out = renderer.render_line_to_string(&buffer, "");
        // Ghost portion must be capped
        let ghost_portion: String = out
            .chars()
            .skip_while(|&c| c != 'm') // skip past ANSI code
            .skip(1)
            .take_while(|&c| c != '\x1b') // until reset
            .collect();
        assert!(
            ghost_portion.chars().count() <= MAX_GHOST_LEN,
            "ghost text must be capped at MAX_GHOST_LEN"
        );
    }

    #[test]
    fn test_unicode_does_not_panic() {
        let renderer = Renderer::new();
        let _ = renderer.render_line_to_string(&make_buffer("⚡ git テスト --verbose", None), "❯ ");
        let _ = renderer.render_line_to_string(&make_buffer("日本語コマンド", None), "$ ");
        let _ = renderer.render_line_to_string(&make_buffer("café --option", None), "❯❯❯ ");
    }
}

// Re-export KeyModifiers for use in tests above
#[cfg(test)]
use crate::input_parser::KeyModifiers;
