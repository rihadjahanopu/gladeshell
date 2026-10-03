// ============================================================================
//  src/daemon/mod.rs — Persistent background socket server for gladeshell
//
//  Architecture:
//    • Multi-threaded Unix IPC socket server (one socket per user)
//    • Concurrent client handlers via worker threads (<0.05ms / 50µs per prompt)
//    • Background threads update Git cache & SystemMetrics asynchronously
//
//  SAFETY GUARANTEES:
//    • All array indexing uses .get() — no direct slice[n] indexing.
//    • Background threads are wrapped in catch_unwind to prevent silent death.
//    • Daemon never writes to stdout — all informational output → stderr.
//    • Buffer overflow in prompt render is caught and replaced with an error message.
//    • Socket path is computed dynamically and exposed via socket_path_str().
//    • IPC payload length is validated (0 < len ≤ MAX_PAYLOAD_BYTES).
// ============================================================================

pub mod client;

use crate::core::prompt::{self, PromptContext};
use crate::git;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::panic;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::net::{UnixListener, UnixStream};

/// Maximum accepted IPC payload size (bytes). Rejects obviously corrupt requests.
const MAX_PAYLOAD_BYTES: usize = 16_384;

// ── Socket path ───────────────────────────────────────────────────────────────

/// Return the user-specific daemon socket path.
///
/// Path format: `<tmpdir>/gladeshell_<user_tag>.sock`
///
/// On Linux: uses UID from /proc/self/status for uniqueness.
/// On macOS/BSD: falls back to $USER / $LOGNAME.
/// On Windows: named pipe would be used, but the daemon is Unix-only.
pub fn socket_path() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let user_tag = resolve_user_tag();
    temp_dir.join(format!("gladeshell_{user_tag}.sock"))
}

/// Return the socket path as a UTF-8 string (for embedding in shell scripts).
pub fn socket_path_str() -> String {
    socket_path().to_string_lossy().into_owned()
}

/// Determine a unique, stable user identifier for the socket file name.
fn resolve_user_tag() -> String {
    // Linux: read real UID from /proc/self/status (most robust, no external crate)
    if let Ok(status) = fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if let Some(rest) = line.strip_prefix("Uid:") {
                if let Some(uid) = rest.split_whitespace().next() {
                    if !uid.is_empty() {
                        return uid.to_string();
                    }
                }
            }
        }
    }

    // Environment fallback chain (macOS, BSD, other Unix, Windows)
    std::env::var("UID")
        .or_else(|_| std::env::var("USER"))
        .or_else(|_| std::env::var("LOGNAME"))
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "default".to_string())
}

// ── Server ────────────────────────────────────────────────────────────────────

/// Run the daemon server loop. Blocks the calling thread until shutdown.
pub fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        let path = socket_path();

        // Remove stale socket file from a previous crash
        if path.exists() {
            let _ = fs::remove_file(&path);
        }

        let listener = UnixListener::bind(&path)?;
        // Use stderr — the daemon must never pollute stdout
        eprintln!("gladeshell daemon listening on {}", path.display());

        let running = Arc::new(AtomicBool::new(true));

        // ── Background: Git cache refresh ─────────────────────────────────────
        let r_git = running.clone();
        let _git_thread = thread::Builder::new()
            .name("fb-git-refresh".to_string())
            .spawn(move || {
                while r_git.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_millis(500));

                    // Wrap in catch_unwind so a panic here does NOT kill the thread
                    let result = panic::catch_unwind(|| {
                        let cached = git::read_cached();
                        if !cached.path.as_os_str().is_empty() {
                            let needs_refresh = cached
                                .last_updated
                                .map(|t| t.elapsed() >= Duration::from_millis(1500))
                                .unwrap_or(true);
                            if needs_refresh {
                                git::refresh(&cached.path);
                            }
                        }
                    });

                    if result.is_err() {
                        // Thread survived the panic; continue running
                    }
                }
            });

        // ── Background: SystemMetrics refresh ────────────────────────────────
        let r_sys = running.clone();
        let _sys_thread = thread::Builder::new()
            .name("fb-sysmetrics".to_string())
            .spawn(move || {
                use crate::core::sysinfo::SystemMetrics;
                // Initial collection before entering the loop
                let _ = panic::catch_unwind(|| {
                    SystemMetrics::update_cache(SystemMetrics::collect_fast());
                });
                while r_sys.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_millis(500));
                    let _ = panic::catch_unwind(|| {
                        SystemMetrics::update_cache(SystemMetrics::collect_fast());
                    });
                }
            });

        // ── Background: ToolVersions refresh (every 60 s) ─────────────────────
        let r_tools = running.clone();
        let _tools_thread = thread::Builder::new()
            .name("fb-toolversions".to_string())
            .spawn(move || {
                use crate::core::sysinfo::ToolVersions;
                let _ = panic::catch_unwind(|| {
                    ToolVersions::update_cache(ToolVersions::collect());
                });
                while r_tools.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_secs(60));
                    let _ = panic::catch_unwind(|| {
                        ToolVersions::update_cache(ToolVersions::collect());
                    });
                }
            });

        // ── Accept loop ───────────────────────────────────────────────────────
        for stream_result in listener.incoming() {
            if !running.load(Ordering::Relaxed) {
                break;
            }

            match stream_result {
                Ok(stream) => {
                    let _ = thread::Builder::new()
                        .name("fb-client".to_string())
                        .spawn(move || {
                            // Wrap the entire handler in catch_unwind so a bug
                            // in prompt rendering does not kill the accept loop.
                            let _ = panic::catch_unwind(|| {
                                let mut buf = vec![0u8; 4096];
                                handle_client(stream, &mut buf);
                            });
                        });
                }
                Err(_e) => {
                    // Log to stderr only; do not crash the server
                    // Uncomment for debug: eprintln!("gladeshell daemon: accept error: {_e}");
                }
            }
        }

        // Cleanup socket file on graceful exit
        if path.exists() {
            let _ = fs::remove_file(&path);
        }

        Ok(())
    }

    #[cfg(not(unix))]
    {
        // Windows / Wasm: in-process fallback mode (daemon not functional)
        eprintln!("gladeshell daemon: Unix socket not available; running in no-op mode.");
        Ok(())
    }
}

// ── Client handler ────────────────────────────────────────────────────────────

/// Handle a single connected client.
///
/// Wire protocol (binary, preferred):
///   Request:  [u32-LE payload_len] [payload bytes, fields joined by '\x1f']
///   Response: [u32-LE resp_len]    [prompt bytes]
///
/// Legacy text fallback (for old shell hooks):
///   Request:  fields joined by '\x1f', newline-terminated plain text
///   Response: plain text
#[cfg(unix)]
fn handle_client(mut stream: UnixStream, buf: &mut Vec<u8>) {
    use std::io::Read;

    let mut len_buf = [0u8; 4];

    // Attempt to read the 4-byte binary length prefix
    if stream.read_exact(&mut len_buf).is_err() {
        return; // EOF or broken pipe — nothing to do
    }

    let payload_len = u32::from_le_bytes(len_buf) as usize;

    if payload_len > 0 && payload_len <= MAX_PAYLOAD_BYTES {
        // ── Binary protocol ───────────────────────────────────────────────────
        let mut payload = vec![0u8; payload_len];
        if stream.read_exact(&mut payload).is_err() {
            return;
        }
        let parts_storage = String::from_utf8_lossy(&payload).into_owned();
        let parts: Vec<&str> = parts_storage.split('\x1f').collect();

        handle_prompt_parts(&parts, buf);

        let written = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
        let resp_len = (written as u32).to_le_bytes();
        let _ = stream.write_all(&resp_len);
        let _ = stream.write_all(&buf[..written]);
        return;
    }

    // ── Legacy text-protocol fallback ─────────────────────────────────────────
    // The 4 bytes we already read may be the start of a text line;
    // re-join them with the rest of the line.
    let prefix = String::from_utf8_lossy(&len_buf).into_owned();
    let mut rest = String::new();
    {
        let mut reader = BufReader::new(&stream);
        let _ = reader.read_line(&mut rest);
    }

    let full_line = format!("{}{}", prefix, rest);
    let trimmed = full_line.trim_end_matches('\n').trim_end_matches('\r');
    let legacy_parts: Vec<&str> = trimmed.split('\x1f').collect();

    handle_prompt_parts(&legacy_parts, buf);

    let written = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    let _ = stream.write_all(&buf[..written]);
}

// ── Prompt request dispatcher ─────────────────────────────────────────────────

#[cfg(unix)]
fn handle_prompt_parts(parts: &[&str], buf: &mut Vec<u8>) {
    // ── Special IPC commands ──────────────────────────────────────────────────

    match parts.first().copied() {
        Some("SUGGEST") => {
            let input = parts.get(1).copied().unwrap_or("");
            let suggestion = crate::plugins::autosuggest::suggest(input).unwrap_or_default();
            buf.clear();
            buf.extend_from_slice(suggestion.as_bytes());
            return;
        }

        Some("HIGHLIGHT") => {
            let input = parts.get(1).copied().unwrap_or("");
            let hl = crate::plugins::highlight::highlight(input);
            buf.clear();
            buf.extend_from_slice(hl.as_bytes());
            return;
        }

        Some("COMPLETE") => {
            let input = parts.get(1).copied().unwrap_or("");
            let completions = crate::plugins::autocomplete::complete(input);
            buf.clear();
            buf.extend_from_slice(completions.as_bytes());
            return;
        }

        Some("SOCKET-PATH") => {
            // Shell scripts can ask for the canonical socket path
            let path = socket_path_str();
            buf.clear();
            buf.extend_from_slice(path.as_bytes());
            return;
        }

        _ => {} // fall through to prompt rendering
    }

    // ── Prompt render request ─────────────────────────────────────────────────

    let mut ctx = PromptContext::default();

    // Field 0: cwd
    if let Some(&cwd_str) = parts.first() {
        let cwd_bytes = cwd_str.as_bytes();
        let len = cwd_bytes.len().min(ctx.cwd.len());
        ctx.cwd[..len].copy_from_slice(&cwd_bytes[..len]);
        ctx.cwd_len = len;

        let cwd_path = std::path::Path::new(cwd_str);
        crate::core::sysinfo::ensure_folder_size_cached(cwd_path);

        let git_status = git::get_status(cwd_path);
        if git_status.is_git_repo {
            let branch_bytes = git_status.branch.as_bytes();
            let blen = branch_bytes.len().min(ctx.git_branch.len());
            ctx.git_branch[..blen].copy_from_slice(&branch_bytes[..blen]);
            ctx.git_branch_len = blen;
            ctx.git_dirty = git_status.dirty;
        }
    }

    // Field 1: last exit code
    if let Some(&exit_str) = parts.get(1) {
        ctx.last_exit = exit_str.parse::<i32>().unwrap_or(0);
    }

    // Field 2: theme id (0 = read persisted preference)
    if let Some(&theme_str) = parts.get(2) {
        let raw_id = theme_str.parse::<usize>().unwrap_or(0);
        ctx.theme_id = if raw_id == 0 {
            prompt::active_theme_id()
        } else {
            raw_id
        };
    }

    // Field 3: username
    if let Some(&user_str) = parts.get(3) {
        let u_bytes = user_str.as_bytes();
        let ulen = u_bytes.len().min(ctx.user.len());
        ctx.user[..ulen].copy_from_slice(&u_bytes[..ulen]);
        ctx.user_len = ulen;
    }

    // Field 4: hostname
    if let Some(&host_str) = parts.get(4) {
        let h_bytes = host_str.as_bytes();
        let hlen = h_bytes.len().min(ctx.host.len());
        ctx.host[..hlen].copy_from_slice(&h_bytes[..hlen]);
        ctx.host_len = hlen;
    }

    // Field 5: command duration (ms)
    if let Some(&dur_str) = parts.get(5) {
        ctx.cmd_duration_ms = dur_str.parse::<u64>().unwrap_or(0);
    }

    // Field 6: shell identifier
    if let Some(&shell_str) = parts.get(6) {
        ctx.shell = shell_str.parse::<u8>().unwrap_or(0);
    }

    // Render into the shared buffer
    buf.resize(4096, 0);
    if prompt::render(&ctx, buf).is_err() {
        // Write a safe, bounded error message — never overflow the buffer
        const ERR_MSG: &[u8] = b"gladeshell: prompt render error\n";
        let safe_len = ERR_MSG.len().min(buf.len());
        buf[..safe_len].copy_from_slice(&ERR_MSG[..safe_len]);
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_path_is_not_empty() {
        let path = socket_path();
        assert!(!path.as_os_str().is_empty());
    }

    #[test]
    fn test_socket_path_str_matches_socket_path() {
        let expected = socket_path().to_string_lossy().into_owned();
        assert_eq!(socket_path_str(), expected);
    }

    #[test]
    fn test_resolve_user_tag_returns_non_empty() {
        let tag = resolve_user_tag();
        assert!(!tag.is_empty(), "user tag must not be empty");
    }

    #[test]
    fn test_socket_path_in_temp_dir() {
        let path = socket_path();
        let temp = std::env::temp_dir();
        assert!(
            path.starts_with(&temp),
            "socket path {path:?} must be inside temp dir {temp:?}"
        );
    }

    #[test]
    fn test_socket_path_contains_gladeshell() {
        let path = socket_path_str();
        assert!(
            path.contains("gladeshell"),
            "socket filename must contain 'gladeshell'"
        );
    }
}
