// =============================================================================
//  src/tools/system_update.rs — Non-interactive System Maintenance Updater (`update`)
// =============================================================================

use std::error::Error;
use std::process::Command;

fn cmd_exists(name: &str) -> bool {
    crate::core::utils::cmd_exists(name)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("\x1b[1;36m🔄 Updating system packages...\x1b[0m");

    if cmd_exists("apt-get") {
        let _ = Command::new("sh")
            .args(["-c", "sudo apt-get update && sudo apt-get upgrade -y && sudo apt-get dist-upgrade -y && sudo apt-get install -f"])
            .status();
    } else if cmd_exists("pacman") {
        let _ = Command::new("sudo")
            .args(["pacman", "-Syu", "--noconfirm"])
            .status();
    } else if cmd_exists("dnf") {
        let _ = Command::new("sudo")
            .args(["dnf", "upgrade", "--refresh", "-y"])
            .status();
    } else if cmd_exists("brew") {
        let _ = Command::new("sh")
            .args(["-c", "brew update && brew upgrade"])
            .status();
    }

    if cmd_exists("flatpak") {
        println!("\x1b[1;34m📦 Updating Flatpaks...\x1b[0m");
        let _ = Command::new("flatpak")
            .args(["update", "-y"])
            .status();
    }

    if cmd_exists("snap") {
        println!("\x1b[1;35m⚡ Refreshing Snaps...\x1b[0m");
        let _ = Command::new("sh")
            .args(["-c", "sudo snap refresh 2>/dev/null || true"])
            .status();
    }

    println!("\x1b[1;32m✨ System update completed!\x1b[0m");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmd_exists_fn() {
        assert!(cmd_exists("sh"));
    }
}
