// =============================================================================
//  src/tools/system_clean.rs — Non-interactive System Maintenance Cache Cleaner (`clean`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::process::Command;

fn cmd_exists(name: &str) -> bool {
    crate::core::utils::cmd_exists(name)
}

/// Cleans cross-platform temporary and cache directories using pure Rust std::fs
fn clean_temp_directories() -> usize {
    let mut cleaned_bytes = 0;
    let temp_dir = std::env::temp_dir();
    
    if let Ok(entries) = fs::read_dir(&temp_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(meta) = entry.metadata() {
                cleaned_bytes += meta.len() as usize;
                if meta.is_file() {
                    let _ = fs::remove_file(path);
                } else if meta.is_dir() {
                    let _ = fs::remove_dir_all(path);
                }
            }
        }
    }
    cleaned_bytes
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("\x1b[1;33m🧹 Cleaning system caches...\x1b[0m");

    // Pure Rust temporary directory cleanup
    let bytes_freed = clean_temp_directories();
    println!("\x1b[1;36m💾 Cleared ~{} KB of temporary file caches\x1b[0m", bytes_freed / 1024);

    if cmd_exists("apt-get") || cmd_exists("pacman") || cmd_exists("dnf") {
        let is_cached = Command::new("sudo")
            .args(["-n", "true"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        if !is_cached {
            println!("\x1b[1;36m🔐 Sudo authentication required for package cache cleanup...\x1b[0m");
            let _ = Command::new("sudo").arg("-v").status();
        }
    }

    if cmd_exists("apt-get") {
        let _ = Command::new("sudo").args(["apt-get", "autoclean"]).status();
    } else if cmd_exists("pacman") {
        let _ = Command::new("sudo").args(["pacman", "-Sc", "--noconfirm"]).status();
    } else if cmd_exists("dnf") {
        let _ = Command::new("sudo").args(["dnf", "clean", "all"]).status();
    } else if cmd_exists("brew") {
        let _ = Command::new("brew").arg("cleanup").status();
    }

    if cmd_exists("flatpak") {
        println!("\x1b[1;34m💎 Cleaning Flatpak unused data...\x1b[0m");
        let _ = Command::new("flatpak").args(["uninstall", "--unused", "-y"]).status();
    }

    println!("\x1b[1;32m✨ System cache cleanup completed!\x1b[0m");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmd_exists_clean_fn() {
        assert!(cmd_exists("cargo") || cmd_exists("git") || cmd_exists("sh") || cmd_exists("cmd"));
    }
}
