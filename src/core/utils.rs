// =============================================================================
//  src/core/utils.rs — Shared Native Rust utilities
//
//  Zero external binary dependencies. Uses only std.
// =============================================================================

use std::path::Path;

/// Returns true if `name` exists as an executable in any directory on PATH.
/// Fully native — no `which` subprocess, no external tools.
pub fn cmd_exists(name: &str) -> bool {
    let p = Path::new(name);
    if p.is_absolute() {
        return p.is_file();
    }
    std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .map(|dir| Path::new(dir).join(name))
        .any(|full| full.is_file())
}

/// Read a file line by line, returning all lines as a Vec<String>.
/// Returns empty vec if file doesn't exist or cannot be read.
pub fn read_lines(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .map(|l| l.to_string())
        .collect()
}
