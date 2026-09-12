// =============================================================================
//  src/tools/uninstaller.rs — `uu` interactive app uninstaller (Phase 4)
// =============================================================================

use inquire::{Confirm, Select, Text};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct UninstallManager {
    pub name: &'static str,
    pub check_bin: &'static str,
    pub remove_cmd: &'static str,
    pub args_prefix: &'static [&'static str],
}

const MANAGERS: &[UninstallManager] = &[
    UninstallManager {
        name: "APT (Debian/Ubuntu)",
        check_bin: "apt",
        remove_cmd: "sudo",
        args_prefix: &["apt", "remove", "-y"],
    },
    UninstallManager {
        name: "Pacman (Arch Linux)",
        check_bin: "pacman",
        remove_cmd: "sudo",
        args_prefix: &["pacman", "-R"],
    },
    UninstallManager {
        name: "DNF (Fedora/RHEL)",
        check_bin: "dnf",
        remove_cmd: "sudo",
        args_prefix: &["dnf", "remove", "-y"],
    },
    UninstallManager {
        name: "Snap",
        check_bin: "snap",
        remove_cmd: "sudo",
        args_prefix: &["snap", "remove"],
    },
    UninstallManager {
        name: "Flatpak",
        check_bin: "flatpak",
        remove_cmd: "flatpak",
        args_prefix: &["uninstall", "-y"],
    },
    UninstallManager {
        name: "Homebrew",
        check_bin: "brew",
        remove_cmd: "brew",
        args_prefix: &["uninstall"],
    },
    UninstallManager {
        name: "Cargo (Rust binaries)",
        check_bin: "cargo",
        remove_cmd: "cargo",
        args_prefix: &["uninstall"],
    },
    UninstallManager {
        name: "Pipx (Python apps)",
        check_bin: "pipx",
        remove_cmd: "pipx",
        args_prefix: &["uninstall"],
    },
];

/// Interactive uninstaller workflow.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let available: Vec<&UninstallManager> = MANAGERS
        .iter()
        .filter(|m| is_cmd_available(m.check_bin))
        .collect();

    if available.is_empty() {
        println!("⚠️ No supported package managers detected on this system.");
        return Ok(());
    }

    let pkg_name = match Text::new("Enter package/application name to uninstall:").prompt() {
        Ok(val) => val.trim().to_string(),
        Err(_) => {
            println!("Cancelled.");
            return Ok(());
        }
    };

    if pkg_name.is_empty() {
        println!("Package name cannot be empty.");
        return Ok(());
    }

    let options: Vec<String> = available.iter().map(|m| m.name.to_string()).collect();

    let selected_manager_name = match Select::new("Select package manager:", options).prompt() {
        Ok(m) => m,
        Err(_) => {
            println!("Cancelled.");
            return Ok(());
        }
    };

    let manager = available
        .into_iter()
        .find(|m| m.name == selected_manager_name)
        .ok_or("Selected manager not found")?;

    let confirmed = Confirm::new(&format!(
        "Are you sure you want to uninstall '{}' using {}?",
        pkg_name, manager.name
    ))
    .with_default(false)
    .prompt()?;

    if !confirmed {
        println!("Uninstall cancelled.");
        return Ok(());
    }

    println!(
        "\n🗑️ Uninstalling '{}' using {}...\n",
        pkg_name, manager.name
    );

    let mut cmd = Command::new(manager.remove_cmd);
    cmd.args(manager.args_prefix);
    cmd.arg(&pkg_name);

    let status = cmd.status();

    match status {
        Ok(s) if s.success() => {
            println!("✅ Successfully uninstalled '{}'", pkg_name);
        }
        Ok(s) => eprintln!("❌ Uninstall failed with status: {}", s),
        Err(e) => eprintln!("❌ Failed to execute uninstaller: {}", e),
    }

    Ok(())
}

fn is_cmd_available(cmd: &str) -> bool {
    crate::core::utils::cmd_exists(cmd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uninstaller_managers_non_empty() {
        assert!(!MANAGERS.is_empty());
    }
}
