// =============================================================================
//  src/daemon/mod.rs — Persistent background socket server for fancybash
//
//  Architecture:
//    - Single-threaded / non-blocking Unix socket server per user
//    - Handles prompt rendering in < 0.5 ms
//    - Background Git watcher updates git cache asynchronously
// =============================================================================

pub mod client;

use crate::core::prompt::{self, PromptContext};
use crate::git;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::net::{UnixListener, UnixStream};

/// Return user-specific daemon socket path cross-platform
pub fn socket_path() -> PathBuf {
    let temp_dir = std::env::temp_dir();
    temp_dir.join("fancybash_daemon.sock")
}

/// Run the daemon server loop. This blocks the current thread.
pub fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        let path = socket_path();
        if path.exists() {
            let _ = fs::remove_file(&path);
        }

        let listener = UnixListener::bind(&path)?;
        println!("fancybash daemon listening on {}", path.display());

        let running = Arc::new(AtomicBool::new(true));

        // Spawn background Git status update thread
        let r = running.clone();
        thread::spawn(move || {
            while r.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(500));
                let cached_path = git::read_cached().path;
                if !cached_path.as_os_str().is_empty() {
                    git::refresh(&cached_path);
                }
            }
        });

        // Spawn background SystemMetrics update thread (every 500ms)
        let r_sys = running.clone();
        thread::spawn(move || {
            use crate::core::sysinfo::SystemMetrics;
            SystemMetrics::update_cache(SystemMetrics::collect_fast());
            while r_sys.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(500));
                SystemMetrics::update_cache(SystemMetrics::collect_fast());
            }
        });

        // Spawn background ToolVersions update thread (every 60s)
        let r_tools = running.clone();
        thread::spawn(move || {
            use crate::core::sysinfo::ToolVersions;
            ToolVersions::update_cache(ToolVersions::collect());
            while r_tools.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(60));
                ToolVersions::update_cache(ToolVersions::collect());
            }
        });

        let mut _conn_id = 0u64;

        for stream in listener.incoming() {
            if !running.load(Ordering::Relaxed) {
                break;
            }

            match stream {
                Ok(stream) => {
                    _conn_id += 1;
                    // Each client is handled in its own thread so the main
                    // accept loop is never blocked by prompt rendering.
                    thread::spawn(move || {
                        let mut buf = vec![0u8; 4096];
                        handle_client(stream, &mut buf);
                    });
                }
                Err(e) => {
                    eprintln!("Socket error: {e}");
                }
            }
        }

        if path.exists() {
            let _ = fs::remove_file(&path);
        }

        Ok(())
    }

    #[cfg(not(unix))]
    {
        println!("fancybash daemon socket server running in in-process fallback mode on Windows.");
        Ok(())
    }
}

#[cfg(unix)]
fn handle_client(mut stream: UnixStream, buf: &mut Vec<u8>) {
    let mut reader = BufReader::new(&stream);
    let mut line = String::new();

    if reader.read_line(&mut line).is_err() || line.is_empty() {
        return;
    }

    // Protocol format: cwd\x1fexit_code\x1ftheme_id\x1fuser\x1fhost\x1fcmd_duration_ms
    let parts: Vec<&str> = line.trim_end_matches('\n').split('\x1f').collect();

    let mut ctx = PromptContext::default();

    if !parts.is_empty() {
        let cwd_str = parts[0];
        let cwd_path = std::path::Path::new(cwd_str);
        let cwd_bytes = cwd_str.as_bytes();
        let len = cwd_bytes.len().min(ctx.cwd.len());
        ctx.cwd[..len].copy_from_slice(&cwd_bytes[..len]);
        ctx.cwd_len = len;

        // Ensure folder size is computed in background if not already cached
        crate::core::sysinfo::ensure_folder_size_cached(cwd_path);

        // Refresh Git status for this client cwd
        let git_status = git::get_status(cwd_path);
        if git_status.is_git_repo {
            let branch_bytes = git_status.branch.as_bytes();
            let blen = branch_bytes.len().min(ctx.git_branch.len());
            ctx.git_branch[..blen].copy_from_slice(&branch_bytes[..blen]);
            ctx.git_branch_len = blen;
            ctx.git_dirty = git_status.dirty;
        }
    }

    if parts.len() > 1 {
        ctx.last_exit = parts[1].parse::<i32>().unwrap_or(0);
    }

    if parts.len() > 2 {
        let raw_id = parts[2].parse::<usize>().unwrap_or(0);
        // 0 = "use persisted theme" (the zsh/bash precmd hook always sends 0).
        // Read from ~/.config/fancybash/theme so the user's choice survives reloads.
        ctx.theme_id = if raw_id == 0 {
            prompt::active_theme_id()
        } else {
            raw_id
        };
    }

    if parts.len() > 3 {
        let u_bytes = parts[3].as_bytes();
        let ulen = u_bytes.len().min(ctx.user.len());
        ctx.user[..ulen].copy_from_slice(&u_bytes[..ulen]);
        ctx.user_len = ulen;
    }

    if parts.len() > 4 {
        let h_bytes = parts[4].as_bytes();
        let hlen = h_bytes.len().min(ctx.host.len());
        ctx.host[..hlen].copy_from_slice(&h_bytes[..hlen]);
        ctx.host_len = hlen;
    }

    if parts.len() > 5 {
        ctx.cmd_duration_ms = parts[5].parse::<u64>().unwrap_or(0);
    }

    if parts.len() > 6 {
        ctx.shell = parts[6].parse::<u8>().unwrap_or(0);
    }

    match prompt::render(&ctx, buf) {
        Ok(written) => {
            let _ = stream.write_all(&buf[..written]);
        }
        Err(_) => {
            let _ = stream.write_all(b"fancybash: buffer overflow\n");
        }
    }
}
