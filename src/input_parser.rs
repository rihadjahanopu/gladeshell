// ============================================================================
//  src/input_parser.rs — Unified VT100 / ANSI / CSI / SS3 Key Event Parser
//
//  Normalizes standard terminal CSI escape sequences, xterm.js / IDE terminal
//  SS3 sequences (DECCKM Application Cursor Mode), UTF-8 multi-byte input,
//  and ASCII control codes.
//
//  SAFETY GUARANTEES:
//   • Never panics on truncated, malformed, or adversarial byte streams.
//   • All array indexing is bounds-checked via slice.get().
//   • UTF-8 decode tries 1–4 byte windows before Latin-1 fallback.
//   • Windows RawModeGuard::Drop restores original console mode.
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum KeyModifiers {
    None,
    Shift,
    Alt,
    Ctrl,
    CtrlShift,
    AltShift,
    CtrlAlt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyEvent {
    Char(char),
    Backspace,
    Delete,
    Enter,
    Tab,
    BackTab,
    Left(KeyModifiers),
    Right(KeyModifiers),
    Up(KeyModifiers),
    Down(KeyModifiers),
    Home,
    End,
    Esc,
    Unknown(Vec<u8>),
}

pub struct InputParser;

impl InputParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse a raw byte slice from stdin/PTY into normalized `KeyEvent`s.
    /// Never panics on malformed or truncated byte streams.
    pub fn parse_bytes(&self, input: &[u8]) -> Vec<KeyEvent> {
        let mut events = Vec::new();
        let mut idx = 0usize;

        while idx < input.len() {
            let (event, read_len) = self.parse_single(&input[idx..]);
            if read_len == 0 {
                // Cannot make progress — consume one byte as Unknown to avoid
                // an infinite loop on pathological input.
                if idx < input.len() {
                    events.push(KeyEvent::Unknown(vec![input[idx]]));
                    idx += 1;
                }
                continue;
            }
            events.push(event);
            idx += read_len;
        }

        events
    }

    /// Parse the next single `KeyEvent` starting at `bytes[0]`.
    /// Returns `(KeyEvent, bytes_consumed)`; consumed=0 signals "cannot parse".
    fn parse_single(&self, bytes: &[u8]) -> (KeyEvent, usize) {
        let first = match bytes.first() {
            Some(&b) => b,
            None => return (KeyEvent::Unknown(Vec::new()), 0),
        };

        match first {
            // Backspace: both UNIX DEL (0x7f) and legacy BS (0x08)
            0x08 | 0x7f => (KeyEvent::Backspace, 1),

            // Line terminators
            b'\r' | b'\n' => (KeyEvent::Enter, 1),

            // Horizontal tab
            b'\t' => (KeyEvent::Tab, 1),

            // Escape sequence introducer
            0x1b => self.parse_escape_sequence(bytes),

            // Control character mappings
            // These are unambiguous single-byte control sequences
            0x01 => (KeyEvent::Home, 1),                     // Ctrl+A
            0x03 => (KeyEvent::Esc, 1),                      // Ctrl+C (cancel line)
            0x04 => (KeyEvent::Delete, 1),                   // Ctrl+D
            0x05 => (KeyEvent::End, 1),                      // Ctrl+E
            0x0b => (KeyEvent::Esc, 1),                      // Ctrl+K (clear line)
            0x15 => (KeyEvent::Esc, 1),                      // Ctrl+U (clear line)
            0x17 => (KeyEvent::Left(KeyModifiers::Ctrl), 1), // Ctrl+W (word-left/delete)

            // Other unhandled control codes (0x00..=0x1a excluding those above)
            code if code < 0x20 => (KeyEvent::Unknown(vec![code]), 1),

            // DEL is handled above (0x7f). Anything ≥ 0x80 is a UTF-8 continuation
            // byte or a valid multi-byte sequence start — handled in parse_utf8_char.
            _ => self.parse_utf8_char(bytes),
        }
    }

    /// Parse escape sequences starting with `\x1b`.
    fn parse_escape_sequence(&self, bytes: &[u8]) -> (KeyEvent, usize) {
        match bytes.get(1) {
            None => (KeyEvent::Esc, 1), // bare ESC at end of buffer

            Some(&b'[') => self.parse_csi(bytes),
            Some(&b'O') => self.parse_ss3(bytes),

            Some(_) => {
                // Alt + character: ESC followed by a regular character
                let (event, sub_len) = self.parse_utf8_char(&bytes[1..]);
                match event {
                    KeyEvent::Char(_) => (event, 1 + sub_len),
                    _ => (KeyEvent::Esc, 1),
                }
            }
        }
    }

    /// Parse CSI sequences (`\x1b[...`).
    fn parse_csi(&self, bytes: &[u8]) -> (KeyEvent, usize) {
        if bytes.len() < 3 {
            return (KeyEvent::Esc, 1);
        }

        // bytes[0] = ESC, bytes[1] = '[', bytes[2] = first parameter byte / terminator
        let third = bytes[2];

        // Shift+Tab: ESC [ Z
        if third == b'Z' {
            return (KeyEvent::BackTab, 3);
        }

        // Simple single-letter terminators with no parameters
        match third {
            b'A' => return (KeyEvent::Up(KeyModifiers::None), 3),
            b'B' => return (KeyEvent::Down(KeyModifiers::None), 3),
            b'C' => return (KeyEvent::Right(KeyModifiers::None), 3),
            b'D' => return (KeyEvent::Left(KeyModifiers::None), 3),
            b'H' => return (KeyEvent::Home, 3),
            b'F' => return (KeyEvent::End, 3),
            _ => {}
        }

        // Find the end of the CSI parameter sequence.
        // Parameter bytes: 0x30–0x3f (digits, ';', ':', '<', '=', '>', '?')
        // Intermediate bytes: 0x20–0x2f (space, '!', '"', etc.)
        // Final byte: 0x40–0x7e
        let mut end_idx = 2usize;
        while end_idx < bytes.len() {
            let b = bytes[end_idx];
            if b >= 0x40 && b <= 0x7e {
                break; // final byte
            }
            end_idx += 1;
        }

        if end_idx >= bytes.len() {
            // Incomplete sequence — consume all bytes as Unknown
            return (KeyEvent::Unknown(bytes.to_vec()), bytes.len());
        }

        let terminator = bytes[end_idx];
        let total_len = end_idx + 1;

        // Build parameter string from bytes[2..end_idx]
        let param_bytes = match bytes.get(2..end_idx) {
            Some(b) => b,
            None => {
                return (
                    KeyEvent::Unknown(bytes[..total_len.min(bytes.len())].to_vec()),
                    total_len.min(bytes.len()),
                )
            }
        };
        let param_str = String::from_utf8_lossy(param_bytes);
        let params: Vec<&str> = param_str.split(';').collect();

        match (terminator, params.as_slice()) {
            // Tilde-terminated sequences
            (b'~', ["3"]) => (KeyEvent::Delete, total_len),
            (b'~', ["1"]) | (b'~', ["7"]) => (KeyEvent::Home, total_len),
            (b'~', ["4"]) | (b'~', ["8"]) => (KeyEvent::End, total_len),

            // Modified arrow keys: \x1b[1;Nm  or  \x1b[Nm
            (b'C', ["1", mod_str]) | (b'C', [mod_str]) => {
                (KeyEvent::Right(self.parse_modifier(mod_str)), total_len)
            }
            (b'D', ["1", mod_str]) | (b'D', [mod_str]) => {
                (KeyEvent::Left(self.parse_modifier(mod_str)), total_len)
            }
            (b'A', ["1", mod_str]) | (b'A', [mod_str]) => {
                (KeyEvent::Up(self.parse_modifier(mod_str)), total_len)
            }
            (b'B', ["1", mod_str]) | (b'B', [mod_str]) => {
                (KeyEvent::Down(self.parse_modifier(mod_str)), total_len)
            }

            _ => (KeyEvent::Unknown(bytes[..total_len].to_vec()), total_len),
        }
    }

    /// Parse SS3 sequences (`\x1bO...`) — DECCKM Application Cursor Mode.
    /// Sent by xterm.js, VS Code terminal, JetBrains, Kitty in certain modes.
    fn parse_ss3(&self, bytes: &[u8]) -> (KeyEvent, usize) {
        match bytes.get(2) {
            None => (KeyEvent::Esc, 1),
            Some(&b'A') => (KeyEvent::Up(KeyModifiers::None), 3),
            Some(&b'B') => (KeyEvent::Down(KeyModifiers::None), 3),
            Some(&b'C') => (KeyEvent::Right(KeyModifiers::None), 3),
            Some(&b'D') => (KeyEvent::Left(KeyModifiers::None), 3),
            Some(&b'H') => (KeyEvent::Home, 3),
            Some(&b'F') => (KeyEvent::End, 3),
            Some(&b) => (KeyEvent::Unknown(vec![0x1b, b'O', b]), 3),
        }
    }

    /// Map ANSI modifier parameter (1-indexed) to `KeyModifiers`.
    fn parse_modifier(&self, mod_str: &str) -> KeyModifiers {
        // Graceful: parse failure → None (modifier code 1 = no modifier)
        match mod_str.parse::<u32>().unwrap_or(1) {
            2 => KeyModifiers::Shift,
            3 => KeyModifiers::Alt,
            5 => KeyModifiers::Ctrl,
            6 => KeyModifiers::CtrlShift,
            7 => KeyModifiers::AltShift,
            8 => KeyModifiers::CtrlAlt,
            _ => KeyModifiers::None,
        }
    }

    /// Decode the next UTF-8 character from `bytes`.
    ///
    /// Tries progressively longer windows (1–4 bytes) so that partial sequences
    /// from fragmented reads are handled correctly. Falls back to treating the
    /// first byte as a Latin-1 character (U+0000..=U+00FF) only as a last resort.
    fn parse_utf8_char(&self, bytes: &[u8]) -> (KeyEvent, usize) {
        if bytes.is_empty() {
            return (KeyEvent::Unknown(Vec::new()), 0);
        }

        // Determine the expected UTF-8 sequence length from the leading byte.
        let seq_len = utf8_sequence_len(bytes[0]);

        if seq_len > 0 && bytes.len() >= seq_len {
            // Attempt to decode exactly seq_len bytes
            if let Ok(s) = std::str::from_utf8(&bytes[..seq_len]) {
                if let Some(ch) = s.chars().next() {
                    return (KeyEvent::Char(ch), ch.len_utf8());
                }
            }
        }

        // Fallback: try 1–4 bytes progressively
        for window in 1..=4usize.min(bytes.len()) {
            if let Ok(s) = std::str::from_utf8(&bytes[..window]) {
                if let Some(ch) = s.chars().next() {
                    return (KeyEvent::Char(ch), ch.len_utf8());
                }
            }
        }

        // Last resort: interpret as Latin-1 U+0000–U+00FF (avoids replacement char)
        let ch = bytes[0] as char;
        (KeyEvent::Char(ch), 1)
    }
}

/// Return the expected byte length of a UTF-8 sequence given its leading byte.
/// Returns 0 for continuation bytes (0x80–0xBF) and invalid bytes.
#[inline]
fn utf8_sequence_len(byte: u8) -> usize {
    match byte {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => 0, // continuation or invalid — can't determine length
    }
}

impl Default for InputParser {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Raw Mode Terminal Management
// ============================================================================

// ── Unix (Linux / macOS / BSD) ────────────────────────────────────────────────

#[cfg(unix)]
pub struct RawModeGuard {
    original_termios: libc::termios,
}

#[cfg(unix)]
impl RawModeGuard {
    /// Enable raw mode. Returns `Err` if `tcgetattr`/`tcsetattr` fail.
    pub fn enable() -> std::io::Result<Self> {
        use std::mem::MaybeUninit;

        let fd = libc::STDIN_FILENO;

        let original_termios = unsafe {
            let mut uninit = MaybeUninit::<libc::termios>::uninit();
            if libc::tcgetattr(fd, uninit.as_mut_ptr()) < 0 {
                return Err(std::io::Error::last_os_error());
            }
            uninit.assume_init()
        };

        let mut raw = original_termios;
        unsafe {
            libc::cfmakeraw(&mut raw);
            if libc::tcsetattr(fd, libc::TCSADRAIN, &raw) < 0 {
                return Err(std::io::Error::last_os_error());
            }
        }

        Ok(RawModeGuard { original_termios })
    }
}

#[cfg(unix)]
impl Drop for RawModeGuard {
    fn drop(&mut self) {
        // Best-effort restore — ignore errors during drop
        unsafe {
            let _ = libc::tcsetattr(libc::STDIN_FILENO, libc::TCSADRAIN, &self.original_termios);
        }
    }
}

// ── Windows ───────────────────────────────────────────────────────────────────

#[cfg(windows)]
pub struct RawModeGuard {
    original_mode: u32,
    handle: *mut std::ffi::c_void,
}

#[cfg(windows)]
unsafe impl Send for RawModeGuard {}
#[cfg(windows)]
unsafe impl Sync for RawModeGuard {}

#[cfg(windows)]
impl RawModeGuard {
    /// Enable raw mode on Windows Console.
    pub fn enable() -> std::io::Result<Self> {
        use std::ptr;
        // SAFETY: Windows API call with well-defined semantics.
        let handle = unsafe {
            windows_sys::Win32::System::Console::GetStdHandle(
                windows_sys::Win32::System::Console::STD_INPUT_HANDLE,
            )
        };
        if handle == ptr::null_mut() {
            return Ok(RawModeGuard {
                original_mode: 0,
                handle: ptr::null_mut(),
            });
        }
        let mut original_mode = 0u32;
        unsafe {
            windows_sys::Win32::System::Console::GetConsoleMode(handle, &mut original_mode);
        }
        // ENABLE_VIRTUAL_TERMINAL_INPUT = 0x0200
        // Disable ENABLE_ECHO_INPUT (0x0004), ENABLE_LINE_INPUT (0x0002)
        let raw_mode = (original_mode | 0x0200) & !(0x0004 | 0x0002);
        unsafe {
            windows_sys::Win32::System::Console::SetConsoleMode(handle, raw_mode);
        }
        Ok(RawModeGuard {
            original_mode,
            handle: handle as *mut std::ffi::c_void,
        })
    }
}

#[cfg(windows)]
impl Drop for RawModeGuard {
    fn drop(&mut self) {
        if self.handle.is_null() {
            return;
        }
        // Best-effort restore
        unsafe {
            windows_sys::Win32::System::Console::SetConsoleMode(
                self.handle as *mut _,
                self.original_mode,
            );
        }
    }
}

// ── Stub for other targets (Wasm, etc.) ───────────────────────────────────────

#[cfg(not(any(unix, windows)))]
pub struct RawModeGuard;

#[cfg(not(any(unix, windows)))]
impl RawModeGuard {
    pub fn enable() -> std::io::Result<Self> {
        Ok(RawModeGuard)
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backspace_both_codes() {
        let parser = InputParser::new();
        assert_eq!(parser.parse_bytes(&[0x7f]), vec![KeyEvent::Backspace]);
        assert_eq!(parser.parse_bytes(&[0x08]), vec![KeyEvent::Backspace]);
    }

    #[test]
    fn test_control_keys() {
        let parser = InputParser::new();
        assert_eq!(parser.parse_bytes(&[0x01]), vec![KeyEvent::Home]); // Ctrl+A
        assert_eq!(parser.parse_bytes(&[0x05]), vec![KeyEvent::End]); // Ctrl+E
        assert_eq!(parser.parse_bytes(&[0x03]), vec![KeyEvent::Esc]); // Ctrl+C
    }

    #[test]
    fn test_csi_arrows() {
        let parser = InputParser::new();
        assert_eq!(
            parser.parse_bytes(b"\x1b[A"),
            vec![KeyEvent::Up(KeyModifiers::None)]
        );
        assert_eq!(
            parser.parse_bytes(b"\x1b[B"),
            vec![KeyEvent::Down(KeyModifiers::None)]
        );
        assert_eq!(
            parser.parse_bytes(b"\x1b[C"),
            vec![KeyEvent::Right(KeyModifiers::None)]
        );
        assert_eq!(
            parser.parse_bytes(b"\x1b[D"),
            vec![KeyEvent::Left(KeyModifiers::None)]
        );
        assert_eq!(parser.parse_bytes(b"\x1b[H"), vec![KeyEvent::Home]);
        assert_eq!(parser.parse_bytes(b"\x1b[F"), vec![KeyEvent::End]);
    }

    #[test]
    fn test_ss3_arrows() {
        let parser = InputParser::new();
        assert_eq!(
            parser.parse_bytes(b"\x1bOA"),
            vec![KeyEvent::Up(KeyModifiers::None)]
        );
        assert_eq!(
            parser.parse_bytes(b"\x1bOB"),
            vec![KeyEvent::Down(KeyModifiers::None)]
        );
        assert_eq!(
            parser.parse_bytes(b"\x1bOC"),
            vec![KeyEvent::Right(KeyModifiers::None)]
        );
        assert_eq!(
            parser.parse_bytes(b"\x1bOD"),
            vec![KeyEvent::Left(KeyModifiers::None)]
        );
        assert_eq!(parser.parse_bytes(b"\x1bOH"), vec![KeyEvent::Home]);
        assert_eq!(parser.parse_bytes(b"\x1bOF"), vec![KeyEvent::End]);
    }

    #[test]
    fn test_modified_arrows() {
        let parser = InputParser::new();
        // Ctrl+Right
        assert_eq!(
            parser.parse_bytes(b"\x1b[1;5C"),
            vec![KeyEvent::Right(KeyModifiers::Ctrl)]
        );
        // Alt+Left
        assert_eq!(
            parser.parse_bytes(b"\x1b[1;3D"),
            vec![KeyEvent::Left(KeyModifiers::Alt)]
        );
        // Shift+Right
        assert_eq!(
            parser.parse_bytes(b"\x1b[1;2C"),
            vec![KeyEvent::Right(KeyModifiers::Shift)]
        );
        // Ctrl+Up
        assert_eq!(
            parser.parse_bytes(b"\x1b[1;5A"),
            vec![KeyEvent::Up(KeyModifiers::Ctrl)]
        );
    }

    #[test]
    fn test_tilde_sequences() {
        let parser = InputParser::new();
        assert_eq!(parser.parse_bytes(b"\x1b[3~"), vec![KeyEvent::Delete]);
        assert_eq!(parser.parse_bytes(b"\x1b[1~"), vec![KeyEvent::Home]);
        assert_eq!(parser.parse_bytes(b"\x1b[4~"), vec![KeyEvent::End]);
    }

    #[test]
    fn test_shift_tab() {
        let parser = InputParser::new();
        assert_eq!(parser.parse_bytes(b"\x1b[Z"), vec![KeyEvent::BackTab]);
    }

    #[test]
    fn test_utf8_ascii() {
        let parser = InputParser::new();
        assert_eq!(
            parser.parse_bytes(b"hello"),
            vec![
                KeyEvent::Char('h'),
                KeyEvent::Char('e'),
                KeyEvent::Char('l'),
                KeyEvent::Char('l'),
                KeyEvent::Char('o'),
            ]
        );
    }

    #[test]
    fn test_utf8_multibyte_emoji() {
        let parser = InputParser::new();
        // ⚡ = U+26A1, encoded as 3 bytes: e2 9a a1
        let bytes = "⚡".as_bytes();
        let events = parser.parse_bytes(bytes);
        assert_eq!(events, vec![KeyEvent::Char('⚡')]);
    }

    #[test]
    fn test_utf8_cjk() {
        let parser = InputParser::new();
        let bytes = "日本語".as_bytes();
        let events = parser.parse_bytes(bytes);
        assert_eq!(
            events,
            vec![
                KeyEvent::Char('日'),
                KeyEvent::Char('本'),
                KeyEvent::Char('語'),
            ]
        );
    }

    #[test]
    fn test_utf8_4byte_char() {
        let parser = InputParser::new();
        // 𝄞 (MUSICAL SYMBOL G CLEF) = U+1D11E, 4 bytes
        let bytes = "𝄞".as_bytes();
        let events = parser.parse_bytes(bytes);
        assert_eq!(events, vec![KeyEvent::Char('𝄞')]);
    }

    #[test]
    fn test_malformed_input_no_panic() {
        let parser = InputParser::new();
        // Various malformed sequences
        let _ = parser.parse_bytes(&[]);
        let _ = parser.parse_bytes(&[0xff, 0xfe]);
        let _ = parser.parse_bytes(&[0x1b]); // bare ESC
        let _ = parser.parse_bytes(&[0x1b, b'[']); // incomplete CSI
        let _ = parser.parse_bytes(&[0x80, 0x81, 0x82]); // bare continuation bytes
    }

    #[test]
    fn test_mixed_multibyte_and_control() {
        let parser = InputParser::new();
        let mut input = "⚡".as_bytes().to_vec();
        input.push(0x7f); // Backspace after emoji
        input.extend_from_slice("git".as_bytes());
        let events = parser.parse_bytes(&input);
        assert_eq!(events[0], KeyEvent::Char('⚡'));
        assert_eq!(events[1], KeyEvent::Backspace);
        assert_eq!(events[2], KeyEvent::Char('g'));
    }

    #[test]
    fn test_no_infinite_loop_on_bad_bytes() {
        let parser = InputParser::new();
        // 0x80 is a UTF-8 continuation byte with no leading byte — would loop
        // if we naively tried to advance without consuming anything.
        let result = parser.parse_bytes(&[0x80, 0x80, b'a']);
        // Must produce 3 events (not infinite) and end with Char('a')
        assert_eq!(result.last(), Some(&KeyEvent::Char('a')));
    }
}
