// ============================================================================
//  src/buffer_engine.rs — Active Line Buffer & Cursor State Machine
//
//  SAFETY GUARANTEES:
//   • Zero `.unwrap()` / `.expect()` / raw index slices.
//   • All string slicing uses `.get()` with safe fallbacks.
//   • `insert_char` is bounded by MAX_BUFFER_CHARS to prevent unbounded growth.
//   • Unicode-correct: cursor positions are character counts, not byte offsets.
//   • All `char_to_byte_index` mappings use the standard `.char_indices()` API.
// ============================================================================

use crate::input_parser::KeyModifiers;

/// Hard limit on typed buffer length in Unicode scalar values.
/// Prevents memory exhaustion if the PTY floods input.
const MAX_BUFFER_CHARS: usize = 65_536;

#[derive(Debug, Clone, Default)]
pub struct BufferEngine {
    typed_buffer: String,
    cursor_position: usize, // Character index in typed_buffer (0 ..= char_count())
    active_suggestion: Option<String>,
}

impl BufferEngine {
    pub fn new() -> Self {
        Self {
            typed_buffer: String::new(),
            cursor_position: 0,
            active_suggestion: None,
        }
    }

    // ── Accessors ─────────────────────────────────────────────────────────────

    /// Current typed buffer content.
    #[inline]
    pub fn typed_buffer(&self) -> &str {
        &self.typed_buffer
    }

    /// Current cursor position as a character (Unicode scalar) index.
    #[inline]
    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    /// Active suggestion string, if any.
    #[inline]
    pub fn active_suggestion(&self) -> Option<&str> {
        self.active_suggestion.as_deref()
    }

    /// Total number of Unicode scalar values in the typed buffer.
    #[inline]
    pub fn char_count(&self) -> usize {
        self.typed_buffer.chars().count()
    }

    // ── Mutation ──────────────────────────────────────────────────────────────

    /// Insert a character at the current cursor position.
    /// Silently no-ops if the buffer would exceed MAX_BUFFER_CHARS.
    pub fn insert_char(&mut self, c: char) {
        if self.char_count() >= MAX_BUFFER_CHARS {
            return;
        }
        let byte_pos = self.char_to_byte_index(self.cursor_position);
        self.typed_buffer.insert(byte_pos, c);
        self.cursor_position += 1;
        self.validate_suggestion();
    }

    /// Delete the character immediately before the cursor (Backspace).
    /// Safe for all Unicode including multi-byte sequences; no-ops at position 0.
    pub fn backspace(&mut self) {
        if self.cursor_position == 0 {
            return;
        }
        let target_char_idx = self.cursor_position - 1;
        let byte_pos = self.char_to_byte_index(target_char_idx);
        self.typed_buffer.remove(byte_pos);
        self.cursor_position -= 1;
        self.validate_suggestion();
    }

    /// Delete the character at the cursor position (Delete key).
    pub fn delete(&mut self) {
        if self.cursor_position < self.char_count() {
            let byte_pos = self.char_to_byte_index(self.cursor_position);
            self.typed_buffer.remove(byte_pos);
            self.validate_suggestion();
        }
    }

    /// Move cursor left by one character, or by one word with modifier.
    pub fn move_left(&mut self, modifiers: KeyModifiers) {
        if self.cursor_position == 0 {
            return;
        }

        match modifiers {
            KeyModifiers::Ctrl | KeyModifiers::Alt | KeyModifiers::CtrlAlt => {
                // Word-backward: skip trailing whitespace, then skip the word
                let chars: Vec<char> = self.typed_buffer.chars().collect();
                let mut idx = self.cursor_position;
                while idx > 0
                    && chars
                        .get(idx - 1)
                        .map(|c| c.is_whitespace())
                        .unwrap_or(false)
                {
                    idx -= 1;
                }
                while idx > 0
                    && !chars
                        .get(idx - 1)
                        .map(|c| c.is_whitespace())
                        .unwrap_or(true)
                {
                    idx -= 1;
                }
                self.cursor_position = idx;
            }
            _ => {
                self.cursor_position -= 1;
            }
        }
    }

    /// Move cursor right by one character, accept suggestion, or jump a word.
    pub fn move_right(&mut self, modifiers: KeyModifiers) {
        let total_chars = self.char_count();

        // At end of typed buffer with an active suggestion → accept it
        if self.cursor_position == total_chars && self.active_suggestion.is_some() {
            match modifiers {
                KeyModifiers::Ctrl | KeyModifiers::Alt | KeyModifiers::CtrlAlt => {
                    self.accept_suggestion_word();
                }
                _ => {
                    self.accept_suggestion_full();
                }
            }
            return;
        }

        if self.cursor_position < total_chars {
            match modifiers {
                KeyModifiers::Ctrl | KeyModifiers::Alt | KeyModifiers::CtrlAlt => {
                    // Word-forward: skip the current word, then skip whitespace
                    let chars: Vec<char> = self.typed_buffer.chars().collect();
                    let mut idx = self.cursor_position;
                    while idx < total_chars
                        && !chars.get(idx).map(|c| c.is_whitespace()).unwrap_or(true)
                    {
                        idx += 1;
                    }
                    while idx < total_chars
                        && chars.get(idx).map(|c| c.is_whitespace()).unwrap_or(false)
                    {
                        idx += 1;
                    }
                    self.cursor_position = idx;
                }
                _ => {
                    self.cursor_position += 1;
                }
            }
        }
    }

    /// Move cursor to the beginning of the buffer.
    pub fn move_home(&mut self) {
        self.cursor_position = 0;
    }

    /// Move cursor to the end of the buffer, or accept the full suggestion.
    pub fn move_end(&mut self) {
        if self.cursor_position == self.char_count() && self.active_suggestion.is_some() {
            self.accept_suggestion_full();
        } else {
            self.cursor_position = self.char_count();
        }
    }

    /// Accept the full active suggestion, replacing the typed buffer entirely.
    pub fn accept_suggestion_full(&mut self) {
        if let Some(sug) = self.active_suggestion.take() {
            self.typed_buffer = sug;
            self.cursor_position = self.char_count();
        }
    }

    /// Accept the suggestion one word at a time (Ctrl/Alt + Right).
    ///
    /// Uses character-safe slicing throughout — no raw byte indexing.
    pub fn accept_suggestion_word(&mut self) {
        // Clone to avoid borrow conflict with &mut self
        if let Some(sug) = self.active_suggestion.clone() {
            // Safety: typed_buffer.len() is a byte length; we must not use it
            // as a char index. Use starts_with for prefix validation, then
            // skip ahead using char counting.
            if !sug.starts_with(self.typed_buffer.as_str()) {
                return;
            }

            // Safe character-level tail extraction
            let typed_char_count = self.char_count();
            let tail: String = sug.chars().skip(typed_char_count).collect();
            if tail.is_empty() {
                return;
            }

            let mut added = String::new();
            let mut chars = tail.chars().peekable();

            // Consume any leading whitespace
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() {
                    added.push(c);
                    chars.next();
                } else {
                    break;
                }
            }

            // Consume the next word (stop at whitespace or path delimiter)
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() || c == '/' || c == '-' || c == '_' {
                    added.push(c);
                    chars.next();
                    break;
                }
                added.push(c);
                chars.next();
            }

            if !added.is_empty() {
                self.typed_buffer.push_str(&added);
                self.cursor_position = self.char_count();
                self.validate_suggestion();
            }
        }
    }

    /// Set the active suggestion. Validates it immediately.
    pub fn set_active_suggestion(&mut self, sug: Option<String>) {
        self.active_suggestion = sug;
        self.validate_suggestion();
    }

    /// Replace the typed buffer entirely and move cursor to end.
    pub fn set_typed_buffer(&mut self, buffer: &str) {
        self.typed_buffer = buffer.to_string();
        self.cursor_position = self.char_count();
        self.validate_suggestion();
    }

    /// Clear the entire line state.
    pub fn clear(&mut self) {
        self.typed_buffer.clear();
        self.cursor_position = 0;
        self.active_suggestion = None;
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Drop the suggestion if it no longer starts with the typed buffer,
    /// or if it equals the typed buffer (no ghost text to show).
    fn validate_suggestion(&mut self) {
        if let Some(ref sug) = self.active_suggestion {
            let typed = self.typed_buffer.as_str();
            if !sug.starts_with(typed) || sug == typed {
                self.active_suggestion = None;
            }
        }
    }

    /// Map a character index to its byte offset in `typed_buffer`.
    ///
    /// Returns `typed_buffer.len()` (append position) when `char_idx` is
    /// at or beyond the end — never panics, never returns an out-of-bounds offset.
    fn char_to_byte_index(&self, char_idx: usize) -> usize {
        // char_indices() yields (byte_offset, char) pairs; taking the n-th entry
        // gives us the byte position of the n-th character.
        self.typed_buffer
            .char_indices()
            .nth(char_idx)
            .map(|(byte_off, _)| byte_off)
            .unwrap_or(self.typed_buffer.len())
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_backspace_ascii() {
        let mut e = BufferEngine::new();
        e.insert_char('g');
        e.insert_char('i');
        e.insert_char('t');
        assert_eq!(e.typed_buffer(), "git");
        assert_eq!(e.cursor_position(), 3);
        e.backspace();
        assert_eq!(e.typed_buffer(), "gi");
        assert_eq!(e.cursor_position(), 2);
    }

    #[test]
    fn test_backspace_at_start_is_noop() {
        let mut e = BufferEngine::new();
        e.backspace(); // must not panic
        assert_eq!(e.typed_buffer(), "");
        assert_eq!(e.cursor_position(), 0);
    }

    #[test]
    fn test_delete_at_end_is_noop() {
        let mut e = BufferEngine::new();
        e.insert_char('a');
        e.delete(); // cursor at end → no-op
        assert_eq!(e.typed_buffer(), "a");
    }

    #[test]
    fn test_unicode_emoji_insert_and_backspace() {
        let mut e = BufferEngine::new();
        e.insert_char('⚡'); // 3 bytes, 1 char
        e.insert_char(' ');
        e.insert_char('a');
        assert_eq!(e.char_count(), 3);
        assert_eq!(e.cursor_position(), 3);
        e.move_left(KeyModifiers::None);
        assert_eq!(e.cursor_position(), 2);
        e.backspace(); // removes ' '
        assert_eq!(e.typed_buffer(), "⚡a");
        assert_eq!(e.char_count(), 2);
    }

    #[test]
    fn test_cjk_characters() {
        let mut e = BufferEngine::new();
        for c in "日本語".chars() {
            e.insert_char(c);
        }
        assert_eq!(e.char_count(), 3);
        e.backspace();
        assert_eq!(e.typed_buffer(), "日本");
        assert_eq!(e.char_count(), 2);
    }

    #[test]
    fn test_suggestion_acceptance_full() {
        let mut e = BufferEngine::new();
        e.set_typed_buffer("git ");
        e.set_active_suggestion(Some("git status".to_string()));
        assert_eq!(e.active_suggestion(), Some("git status"));
        e.move_right(KeyModifiers::None); // accepts full suggestion
        assert_eq!(e.typed_buffer(), "git status");
        assert_eq!(e.active_suggestion(), None);
    }

    #[test]
    fn test_suggestion_word_accept_unicode() {
        let mut e = BufferEngine::new();
        e.set_typed_buffer("git ");
        e.set_active_suggestion(Some("git checkout main".to_string()));
        e.accept_suggestion_word(); // should accept "checkout"
                                    // After accepting: "git checkout" (or "git checkout ")
        assert!(
            e.typed_buffer().starts_with("git checkout"),
            "word accept should append next word, got: {}",
            e.typed_buffer()
        );
    }

    #[test]
    fn test_backspace_invalidates_mismatched_suggestion() {
        let mut e = BufferEngine::new();
        e.set_typed_buffer("git status");
        e.set_active_suggestion(Some("git status --short".to_string()));
        e.backspace(); // "git statu" — suggestion still matches
        assert_eq!(e.active_suggestion(), Some("git status --short"));
        e.set_active_suggestion(Some("git commit".to_string()));
        assert_eq!(e.active_suggestion(), None); // "git commit" ≠ prefix of "git statu"
    }

    #[test]
    fn test_buffer_max_length_guard() {
        let mut e = BufferEngine::new();
        // Insert MAX_BUFFER_CHARS characters
        for _ in 0..MAX_BUFFER_CHARS {
            e.insert_char('a');
        }
        assert_eq!(e.char_count(), MAX_BUFFER_CHARS);
        // One more insert should be silently ignored
        e.insert_char('x');
        assert_eq!(e.char_count(), MAX_BUFFER_CHARS);
    }

    #[test]
    fn test_move_home_and_end() {
        let mut e = BufferEngine::new();
        e.set_typed_buffer("hello world");
        assert_eq!(e.cursor_position(), 11);
        e.move_home();
        assert_eq!(e.cursor_position(), 0);
        e.move_end();
        assert_eq!(e.cursor_position(), 11);
    }

    #[test]
    fn test_insert_mid_buffer_unicode() {
        let mut e = BufferEngine::new();
        e.set_typed_buffer("abc");
        e.move_home();
        e.move_right(KeyModifiers::None); // cursor at 1
        e.insert_char('⚡'); // insert between 'a' and 'b'
        assert_eq!(e.typed_buffer(), "a⚡bc");
        assert_eq!(e.cursor_position(), 2);
    }

    #[test]
    fn test_clear_resets_all_state() {
        let mut e = BufferEngine::new();
        e.set_typed_buffer("something");
        e.set_active_suggestion(Some("something else".to_string()));
        e.clear();
        assert_eq!(e.typed_buffer(), "");
        assert_eq!(e.cursor_position(), 0);
        assert_eq!(e.active_suggestion(), None);
    }
}
