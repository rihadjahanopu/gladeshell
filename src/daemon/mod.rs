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
use std::os::unix::fs::MetadataExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Return user-specific Unix socket path: `/tmp/fancybash_<UID>.sock`
pub fn socket_path() -> PathBuf {
    let uid = fs::metadata("/proc/self")
        .map(|m| m.uid())
        .unwrap_or(1000);
    PathBuf::from(format!("/tmp/fancybash_{uid}.sock"))
}

/// Run the daemon server loop. This blocks the current thread.
pub fn run_server() -> Result<(), Box<dyn std::error::Error>> {
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
        let mut last_cwd = PathBuf::new();
        while r.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(500));
            let current_cwd = std::env::current_dir().unwrap_or_default();
            if current_cwd != last_cwd {
                git::refresh(&current_cwd);
                last_cwd = current_cwd;
            }
        }
    });

    let mut buf = vec![0u8; 4096];

    for stream in listener.incoming() {
        if !running.load(Ordering::Relaxed) {
            break;
        }

        match stream {
            Ok(stream) => {
                handle_client(stream, &mut buf);
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

fn handle_client(mut stream: UnixStream, buf: &mut [u8]) {
    let mut reader = BufReader::new(&stream);
    let mut line = String::new();

    if reader.read_line(&mut line).is_err() || line.is_empty() {
        return;
    }

    // Protocol format: cwd\x1fexit_code\x1ftheme_id\x1fuser\x1fhost\x1fcmd_duration_ms
    let parts: Vec<&str> = line.trim_end_matches('\n').split('\x1f').collect();

    let mut ctx = PromptContext::default();

    if !parts.is_empty() {
        let cwd_bytes = parts[0].as_bytes();
        let len = cwd_bytes.len().min(ctx.cwd.len());
        ctx.cwd[..len].copy_from_slice(&cwd_bytes[..len]);
        ctx.cwd_len = len;

        // Refresh Git status for this cwd asynchronously / check cache
        let git_status = git::read_cached();
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
        ctx.theme_id = parts[2].parse::<usize>().unwrap_or(0);
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

    match prompt::render(&ctx, buf) {
        Ok(written) => {
            let _ = stream.write_all(&buf[..written]);
        }
        Err(_) => {
            let _ = stream.write_all(b"fancybash: buffer overflow\n");
        }
    }
}
