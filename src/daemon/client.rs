// =============================================================================
//  src/daemon/client.rs — Unix socket client & prompt fallback
// =============================================================================

use super::socket_path;
use crate::core::prompt::{self, PromptContext};
use crate::git;
use std::io::{BufReader, Write};
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::net::UnixStream;

/// Request a prompt from the running daemon.
/// Returns `Ok(rendered_string)` or `Err` if the daemon is unreachable.
pub fn request_prompt(
    cwd: &str,
    exit_code: i32,
    theme_id: usize,
    user: &str,
    host: &str,
    cmd_duration_ms: u64,
    shell: u8,
) -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        let path = socket_path();
        let mut stream = UnixStream::connect(&path)?;
        stream.set_read_timeout(Some(Duration::from_millis(50)))?;
        stream.set_write_timeout(Some(Duration::from_millis(50)))?;

        let req = format!("{cwd}\x1f{exit_code}\x1f{theme_id}\x1f{user}\x1f{host}\x1f{cmd_duration_ms}\x1f{shell}\n");
        stream.write_all(req.as_bytes())?;

        use std::io::Read;
        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader.read_to_string(&mut response)?;

        return Ok(response);
    }

    #[cfg(not(unix))]
    {
        let _ = (cmd_duration_ms,);
        Err("Daemon sockets not supported on non-unix OS; falling back to in-process rendering".into())
    }
}

/// Fallback renderer when daemon is offline: renders prompt in-process synchronously.
pub fn render_fallback(
    cwd: &str,
    exit_code: i32,
    theme_id: usize,
    user: &str,
    host: &str,
    shell: u8,
) -> String {
    let mut ctx = PromptContext::default();

    let cwd_bytes = cwd.as_bytes();
    let clen = cwd_bytes.len().min(ctx.cwd.len());
    ctx.cwd[..clen].copy_from_slice(&cwd_bytes[..clen]);
    ctx.cwd_len = clen;

    ctx.last_exit = exit_code;
    ctx.theme_id = theme_id;
    ctx.shell = shell;

    let u_bytes = user.as_bytes();
    let ulen = u_bytes.len().min(ctx.user.len());
    ctx.user[..ulen].copy_from_slice(&u_bytes[..ulen]);
    ctx.user_len = ulen;

    let h_bytes = host.as_bytes();
    let hlen = h_bytes.len().min(ctx.host.len());
    ctx.host[..hlen].copy_from_slice(&h_bytes[..hlen]);
    ctx.host_len = hlen;

    // Check git status synchronously
    let cwd_path = std::path::Path::new(cwd);
    let status = git::get_status(cwd_path);
    if status.is_git_repo {
        let branch_bytes = status.branch.as_bytes();
        let blen = branch_bytes.len().min(ctx.git_branch.len());
        ctx.git_branch[..blen].copy_from_slice(&branch_bytes[..blen]);
        ctx.git_branch_len = blen;
        ctx.git_dirty = status.dirty;
    }

    let mut buf = vec![0u8; 4096];
    match prompt::render(&ctx, &mut buf) {
        Ok(written) => String::from_utf8_lossy(&buf[..written]).to_string(),
        Err(_) => String::from("fancybash> "),
    }
}
