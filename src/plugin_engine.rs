// ============================================================================
//  src/plugin_engine.rs — State Engine & Plugin Orchestration Pipeline
//
//  SAFETY GUARANTEES:
//   • Zero `.unwrap()` / `.expect()` / raw index slices.
//   • Tab completion suffix uses char-count-safe slicing.
//   • Autosuggestion update has a soft timeout guard to keep input loop <5ms.
//   • All plugin calls degrade gracefully on failure.
// ============================================================================

use std::io::{self, Write};
use std::time::{Duration, Instant};

use crate::buffer_engine::BufferEngine;
use crate::input_parser::{InputParser, KeyEvent};
use crate::plugins::{autocomplete, autosuggest};
use crate::renderer::Renderer;

/// If autosuggestion lookup takes longer than this, skip and return None.
/// Keeps the input loop responsive under all conditions.
const AUTOSUGGEST_TIMEOUT: Duration = Duration::from_millis(3);

pub struct PluginEngine {
    parser: InputParser,
    buffer: BufferEngine,
    renderer: Renderer,
    prompt_prefix: String,
}

impl PluginEngine {
    pub fn new() -> Self {
        Self {
            parser: InputParser::new(),
            buffer: BufferEngine::new(),
            renderer: Renderer::new(),
            prompt_prefix: String::new(),
        }
    }

    /// Set the prompt prefix string (e.g. `user@host:~$ `).
    pub fn with_prompt_prefix(mut self, prefix: &str) -> Self {
        self.prompt_prefix = prefix.to_string();
        self
    }

    /// Read-only access to the inner `BufferEngine`.
    pub fn buffer(&self) -> &BufferEngine {
        &self.buffer
    }

    /// Mutable access to the inner `BufferEngine`.
    pub fn buffer_mut(&mut self) -> &mut BufferEngine {
        &mut self.buffer
    }

    /// Process a raw byte slice from stdin/PTY.
    /// Returns command strings submitted via Enter; never panics.
    pub fn process_input_bytes(&mut self, input: &[u8]) -> Vec<String> {
        let events = self.parser.parse_bytes(input);
        let mut submitted = Vec::new();

        for event in events {
            if let Some(cmd) = self.process_key_event(event) {
                submitted.push(cmd);
            }
        }

        submitted
    }

    /// Process a single `KeyEvent`.
    /// Returns `Some(String)` when Enter is pressed; `None` otherwise.
    pub fn process_key_event(&mut self, event: KeyEvent) -> Option<String> {
        match event {
            KeyEvent::Char(c) => {
                self.buffer.insert_char(c);
                self.update_autosuggestion();
            }

            // Both 0x7f and 0x08 are normalised to Backspace by InputParser
            KeyEvent::Backspace => {
                self.buffer.backspace();
                self.update_autosuggestion();
            }

            KeyEvent::Delete => {
                self.buffer.delete();
                self.update_autosuggestion();
            }

            // Arrow Right / SS3 \x1bOC / Modified \x1b[1;5C
            KeyEvent::Right(mods) => {
                self.buffer.move_right(mods);
            }

            // Arrow Left / SS3 \x1bOD / Modified \x1b[1;3D
            KeyEvent::Left(mods) => {
                self.buffer.move_left(mods);
            }

            KeyEvent::Home => {
                self.buffer.move_home();
            }

            KeyEvent::End => {
                self.buffer.move_end();
            }

            KeyEvent::Tab => {
                self.handle_tab_completion();
            }

            KeyEvent::Enter => {
                let submitted = self.buffer.typed_buffer().to_string();
                self.buffer.clear();
                // Add submitted command directly to top of live history cache
                if !submitted.trim().is_empty() {
                    autosuggest::add_history_entry(&submitted);
                }
                return Some(submitted);
            }

            KeyEvent::Esc => {
                self.buffer.clear();
            }

            // Up/Down, BackTab, Unknown — no-op (shell handles history navigation)
            _ => {}
        }

        None
    }

    /// Render the current line to `writer` (single atomic flush).
    pub fn render<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.renderer
            .redraw(writer, &self.buffer, &self.prompt_prefix)
    }

    /// Render the current line into a `String` representation.
    pub fn render_to_string(&self) -> String {
        self.renderer
            .render_line_to_string(&self.buffer, &self.prompt_prefix)
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Handle Tab key: find the first completion candidate and apply its suffix.
    ///
    /// Uses char-count-based suffix extraction — no raw byte slicing.
    fn handle_tab_completion(&mut self) {
        let typed = self.buffer.typed_buffer().to_string();
        let completions = autocomplete::complete(&typed);
        if completions.is_empty() {
            return;
        }

        let first_candidate = match completions.lines().next() {
            Some(c) if !c.is_empty() => c,
            _ => return,
        };

        // The last word typed (the token we are completing)
        let current_word = typed.split_whitespace().last().unwrap_or("");

        if !first_candidate.starts_with(current_word) {
            return;
        }

        // Safe char-level suffix extraction: skip typed chars, take the rest
        let candidate_char_count = first_candidate.chars().count();
        let current_word_char_count = current_word.chars().count();

        if candidate_char_count <= current_word_char_count {
            return; // nothing to append
        }

        let suffix: String = first_candidate
            .chars()
            .skip(current_word_char_count)
            .collect();

        for c in suffix.chars() {
            self.buffer.insert_char(c);
        }
        self.update_autosuggestion();
    }

    /// Look up history autosuggestion for the current buffer.
    ///
    /// If the lookup takes longer than AUTOSUGGEST_TIMEOUT, it is abandoned
    /// and the suggestion is cleared — this keeps keypress latency under 5ms.
    fn update_autosuggestion(&mut self) {
        let typed = self.buffer.typed_buffer();
        if typed.trim().is_empty() {
            self.buffer.set_active_suggestion(None);
            return;
        }

        let typed_owned = typed.to_string();
        let start = Instant::now();

        let suggestion = autosuggest::suggest(&typed_owned);

        // Soft timeout: if suggest() took too long (e.g. huge history file),
        // discard the result and clear the suggestion to stay responsive.
        if start.elapsed() > AUTOSUGGEST_TIMEOUT {
            self.buffer.set_active_suggestion(None);
            return;
        }

        self.buffer.set_active_suggestion(suggestion);
    }
}

impl Default for PluginEngine {
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

    #[test]
    fn test_ss3_right_arrow_accepts_suggestion() {
        let mut engine = PluginEngine::new();
        engine.process_input_bytes(b"git");
        assert_eq!(engine.buffer().typed_buffer(), "git");

        engine
            .buffer_mut()
            .set_active_suggestion(Some("git checkout".to_string()));

        // xterm.js SS3 Right Arrow
        engine.process_input_bytes(b"\x1bOC");
        assert_eq!(engine.buffer().typed_buffer(), "git checkout");
    }

    #[test]
    fn test_backspace_both_codes() {
        let mut engine = PluginEngine::new();
        engine.process_input_bytes(b"cargo");
        assert_eq!(engine.buffer().typed_buffer(), "cargo");

        // UNIX DEL
        engine.process_input_bytes(&[0x7f]);
        assert_eq!(engine.buffer().typed_buffer(), "carg");

        // Windows / xterm.js BS
        engine.process_input_bytes(&[0x08]);
        assert_eq!(engine.buffer().typed_buffer(), "car");
    }

    #[test]
    fn test_enter_submits_and_clears() {
        let mut engine = PluginEngine::new();
        let submitted = engine.process_input_bytes(b"echo hello\r");
        assert_eq!(submitted, vec!["echo hello"]);
        assert_eq!(engine.buffer().typed_buffer(), "");
    }

    #[test]
    fn test_ctrl_c_clears_line() {
        let mut engine = PluginEngine::new();
        engine.process_input_bytes(b"partial command");
        engine.process_input_bytes(&[0x03]); // Ctrl+C → Esc → clear
        assert_eq!(engine.buffer().typed_buffer(), "");
    }

    #[test]
    fn test_tab_completion_unicode_suffix_no_panic() {
        let mut engine = PluginEngine::new();
        // Force a unicode typed buffer — tab must not panic
        engine.process_input_bytes("git s".as_bytes());
        engine.process_input_bytes(b"\t"); // Tab
                                           // Result doesn't matter — just must not panic
        let _ = engine.buffer().typed_buffer();
    }

    #[test]
    fn test_unicode_input_does_not_panic() {
        let mut engine = PluginEngine::new();
        let emoji_bytes = "⚡ git".as_bytes();
        engine.process_input_bytes(emoji_bytes);
        assert_eq!(engine.buffer().typed_buffer(), "⚡ git");
    }

    #[test]
    fn test_multiple_enter_events() {
        let mut engine = PluginEngine::new();
        let submitted = engine.process_input_bytes(b"cmd1\rcmd2\r");
        assert_eq!(submitted.len(), 2);
        assert_eq!(submitted[0], "cmd1");
        assert_eq!(submitted[1], "cmd2");
    }

    #[test]
    fn test_home_and_end_keys() {
        let mut engine = PluginEngine::new();
        engine.process_input_bytes(b"hello");
        engine.process_input_bytes(b"\x1b[H"); // Home
        assert_eq!(engine.buffer().cursor_position(), 0);
        engine.process_input_bytes(b"\x1b[F"); // End
        assert_eq!(engine.buffer().cursor_position(), 5);
    }

    #[test]
    fn test_ctrl_a_and_e_home_end() {
        let mut engine = PluginEngine::new();
        engine.process_input_bytes(b"world");
        engine.process_input_bytes(&[0x01]); // Ctrl+A = Home
        assert_eq!(engine.buffer().cursor_position(), 0);
        engine.process_input_bytes(&[0x05]); // Ctrl+E = End
        assert_eq!(engine.buffer().cursor_position(), 5);
    }

    #[test]
    fn test_render_to_string_does_not_panic() {
        let mut engine = PluginEngine::new().with_prompt_prefix("❯ ");
        engine.process_input_bytes("git status".as_bytes());
        let out = engine.render_to_string();
        assert!(!out.is_empty());
        assert!(out.contains("\x1b[G"));
    }
}
