// =============================================================================
//  src/tools/universal_clean.rs — Universal System Optimizer & Cleaner (`uc`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::process::Command;

use inquire::Confirm;

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

    let confirm = Confirm::new("Proceed with OS & Package Cache cleanup?")
        .with_default(false)
        .prompt()?;

    if !confirm {
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
            let _ = Command::new("sudo").args(["zypper", "clean"]).status();
        }
        "apk" => {
            let _ = Command::new("sudo").args(["apk", "cache", "clean"]).status();
        }
        _ => {
            println!("\x1b[0;33m⚠️ Standard package cleanup not configured for this distro.\x1b[0m");
        }
    }

    println!("\x1b[0;34m📋 Vacuuming systemd journal logs (older than 3 days)...\x1b[0m");
    let _ = Command::new("sudo")
        .args(["journalctl", "--vacuum-time=3d", "--quiet"])
        .status();

    println!("\n\x1b[0;32m✅ Universal Clean completed successfully!\x1b[0m");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_distro_does_not_panic() {
        let (mgr, name) = detect_distro();
        assert!(!mgr.is_empty());
        assert!(!name.is_empty());
    }
}
