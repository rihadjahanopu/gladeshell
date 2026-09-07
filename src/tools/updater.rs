// =============================================================================
//  src/tools/updater.rs — `uup` mega system updater (Phase 4 implementation)
//
//  Detects available package managers & development runtime updaters:
//    • System: apt, pacman, dnf, brew, snap, flatpak
//    • Runtimes: rustup, bun, npm, pnpm, yarn, pipx
// =============================================================================

use inquire::MultiSelect;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct UpdaterTool {
    pub name: &'static str,
    pub command: &'static str,
    pub check_bin: &'static str,
    pub args: &'static [&'static str],
}

const UPDATER_REGISTRY: &[UpdaterTool] = &[
    UpdaterTool {
        name: "APT (Debian/Ubuntu)",
        command: "sudo",
        check_bin: "apt",
        args: &["apt", "update", "&&", "sudo", "apt", "upgrade", "-y"],
    },
    UpdaterTool {
        name: "Pacman (Arch Linux)",
        command: "sudo",
        check_bin: "pacman",
        args: &["pacman", "-Syu", "--noconfirm"],
    },
    UpdaterTool {
        name: "DNF (Fedora/RHEL)",
        command: "sudo",
        check_bin: "dnf",
        args: &["dnf", "upgrade", "-y"],
    },
    UpdaterTool {
        name: "Homebrew (macOS/Linux)",
        command: "brew",
        check_bin: "brew",
        args: &["update"],
    },
    UpdaterTool {
        name: "Snap packages",
        command: "sudo",
        check_bin: "snap",
        args: &["snap", "refresh"],
    },
    UpdaterTool {
        name: "Flatpak packages",
        command: "flatpak",
        check_bin: "flatpak",
        args: &["update", "-y"],
    },
    UpdaterTool {
        name: "Rustup toolchains",
        command: "rustup",
        check_bin: "rustup",
        args: &["update"],
    },
    UpdaterTool {
        name: "Bun JavaScript Runtime",
        command: "bun",
        check_bin: "bun",
        args: &["upgrade"],
    },
    UpdaterTool {
        name: "NPM Global Packages",
        command: "npm",
        check_bin: "npm",
        args: &["update", "-g"],
    },
    UpdaterTool {
        name: "PNPM Package Manager",
        command: "pnpm",
        check_bin: "pnpm",
        args: &["self-update"],
    },
    UpdaterTool {
        name: "Yarn Package Manager",
        command: "yarn",
        check_bin: "yarn",
        args: &["set", "version", "latest"],
    },
    UpdaterTool {
        name: "Pipx Python Applications",
        command: "pipx",
        check_bin: "pipx",
        args: &["upgrade-all"],
    },
];

/// Probes for installed package managers and runs interactive upgrade workflow.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Scanning for installed package managers and runtime updaters...");

    let available: Vec<&UpdaterTool> = UPDATER_REGISTRY
        .iter()
        .filter(|t| is_cmd_available(t.check_bin))
        .collect();

    if available.is_empty() {
        println!("⚠️ No supported package managers or runtime updaters detected.");
        return Ok(());
    }

    println!("✅ Found {} available updater tool(s).", available.len());

    let options: Vec<String> = available
        .iter()
        .map(|t| format!("{} ({})", t.name, t.check_bin))
        .collect();

    let ans = MultiSelect::new("Select tools to run updates for:", options)
        .with_default(&[0])
        .prompt();

    let selected_indices = match ans {
        Ok(indices) => indices,
        Err(_) => {
            println!("Cancelled.");
            return Ok(());
        }
    };

    if selected_indices.is_empty() {
        println!("No tools selected.");
        return Ok(());
    }

    println!("\n🚀 Starting update suite...\n");

    for choice in selected_indices {
        if let Some(tool) = available.iter().find(|t| choice.starts_with(t.name)) {
            println!("--------------------------------------------------");
            println!("🔄 Running update for: {}", tool.name);
            println!("--------------------------------------------------");

            let mut cmd = Command::new(tool.command);

            // Special case for APT shell chain
            if tool.check_bin == "apt" {
                cmd = Command::new("sh");
                cmd.args(&["-c", "sudo apt update && sudo apt upgrade -y"]);
            } else {
                cmd.args(tool.args);
            }

            let status = cmd.status();
            match status {
                Ok(s) if s.success() => println!("✅ Finished {}", tool.name),
                Ok(s) => eprintln!("❌ {} failed with status: {}", tool.name, s),
                Err(e) => eprintln!("❌ Failed to execute {}: {}", tool.name, e),
            }
            println!();
        }
    }

    println!("✨ Mega update process finished!");
    Ok(())
}

fn is_cmd_available(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_updater_registry_non_empty() {
        assert!(!UPDATER_REGISTRY.is_empty());
    }

    #[test]
    fn test_is_cmd_available_cargo() {
        assert!(is_cmd_available("cargo"));
    }
}
