// =============================================================================
//  src/tools/system_clean.rs — Non-interactive System Maintenance Cache Cleaner (`clean`)
// =============================================================================

use std::error::Error;
use std::process::Command;

fn cmd_exists(name: &str) -> bool {
    crate::core::utils::cmd_exists(name)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("\x1b[1;33m🧹 Cleaning system caches...\x1b[0m");

    if cmd_exists("apt-get") {
        let _ = Command::new("sh")
            .args(["-c", "sudo apt-get autoremove --purge -y && sudo apt-get autoclean && sudo apt-get clean -y"])
            .status();
    } else if cmd_exists("pacman") {
        let _ = Command::new("sh")
            .args(["-c", "orphans=$(pacman -Qtdq 2>/dev/null); [ -n \"$orphans\" ] && sudo pacman -Rns --noconfirm $orphans 2>/dev/null || true; sudo pacman -Sc --noconfirm"])
            .status();
    } else if cmd_exists("dnf") {
        let _ = Command::new("sh")
            .args(["-c", "sudo dnf autoremove -y && sudo dnf clean all"])
            .status();
    } else if cmd_exists("brew") {
        let _ = Command::new("sh")
            .args(["-c", "brew cleanup"])
            .status();
    }

    if cmd_exists("flatpak") {
        println!("\x1b[1;34m💎 Cleaning Flatpak unused data...\x1b[0m");
        let _ = Command::new("sh")
            .args(["-c", "flatpak uninstall --unused -y && flatpak repair"])
            .status();
    }

    println!("\x1b[1;32m✨ System cache cleanup completed!\x1b[0m");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmd_exists_clean_fn() {
        assert!(cmd_exists("sh"));
    }
}
