// =============================================================================
//  src/tools/universal_clean.rs — Universal System Optimizer & Cleaner (`uc`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn detect_distro() -> (String, String) {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        let mut id = String::new();
        let mut name = String::from("Linux");
        for line in content.lines() {
            if let Some(val) = line.strip_prefix("ID=") {
                id = val.trim_matches('"').to_string();
            } else if let Some(val) = line.strip_prefix("NAME=") {
                name = val.trim_matches('"').to_string();
            }
        }
        let mgr = match id.as_str() {
            "ubuntu" | "debian" | "pop" | "mint" | "kali" | "deepin" => "apt",
            "fedora" | "rhel" | "centos" | "rocky" | "nobara" => "dnf",
            "arch" | "manjaro" | "endeavouros" | "cachyos" => "pacman",
            "opensuse-tumbleweed" | "opensuse-leap" | "suse" => "zypper",
            "apk" | "alpine" => "apk",
            _ => "unknown",
        };
        (mgr.to_string(), name)
    } else {
        ("unknown".to_string(), "Linux System".to_string())
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let (pkg_mgr, distro_name) = detect_distro();

    println!("\x1b[0;36m╔════════════════════════════════════╗\x1b[0m");
    println!("\x1b[0;36m║\x1b[0m   🚀 Universal System Cleaner v3.0  \x1b[0;36m║\x1b[0m");
    println!("\x1b[0;36m╚════════════════════════════════════╝\x1b[0m");
    println!("\x1b[0;36mOS:\x1b[0m {distro_name}");
    println!("\x1b[0;36mPackage Manager:\x1b[0m {pkg_mgr}\n");

    print!("Proceed with OS & Package Cache cleanup? [y/N]: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if !input.trim().eq_ignore_ascii_case("y") {
        println!("👋 Cleanup cancelled.");
        return Ok(());
    }

    println!("\n\x1b[0;34m🗑️ Cleaning package manager caches...\x1b[0m");

    match pkg_mgr.as_str() {
        "apt" => {
            let _ = Command::new("sudo").args(["apt-get", "autoremove", "-y"]).status();
            let _ = Command::new("sudo").args(["apt-get", "autoclean"]).status();
        }
        "dnf" => {
            let _ = Command::new("sudo").args(["dnf", "autoremove", "-y"]).status();
            let _ = Command::new("sudo").args(["dnf", "clean", "all"]).status();
        }
        "pacman" => {
            let _ = Command::new("sudo").args(["pacman", "-Sc", "--noconfirm"]).status();
        }
        "zypper" => {
            let _ = Command::new("sudo").args(["zypper", "clean", "--all"]).status();
        }
        "apk" => {
            let _ = Command::new("sudo").args(["apk", "cache", "clean"]).status();
        }
        _ => {
            println!("⚠️ Unsupported package manager for automated cache cleaning.");
        }
    }

    // Flatpak cleanup
    if crate::core::utils::cmd_exists("flatpak") {
        println!("\n\x1b[0;34m📦 Cleaning unused Flatpak runtimes...\x1b[0m");
        let _ = Command::new("flatpak").args(["uninstall", "--unused", "-y"]).status();
    }

    // Snap cleanup
    if crate::core::utils::cmd_exists("snap") {
        println!("\n\x1b[0;34m⚡ Cleaning old Snap revisions...\x1b[0m");
        let _ = Command::new("bash")
            .arg("-c")
            .arg("snap list --all | awk '/disabled/{print $1, $3}' | while read snapname revision; do sudo snap remove \"$snapname\" --revision=\"$revision\"; done")
            .status();
    }

    // Systemd Journal logs cleanup
    if crate::core::utils::cmd_exists("journalctl") {
        println!("\n\x1b[0;34m📜 Vacuuming system logs older than 7 days...\x1b[0m");
        let _ = Command::new("sudo").args(["journalctl", "--vacuum-time=7d"]).status();
    }

    // Docker cleanup if available
    if crate::core::utils::cmd_exists("docker") {
        println!("\n\x1b[0;34m🐳 Pruning unused Docker images & containers...\x1b[0m");
        let _ = Command::new("docker").args(["system", "prune", "-f"]).status();
    }

    println!("\n\x1b[1;32m✨ Universal system cleanup completed successfully!\x1b[0m");
    Ok(())
}
