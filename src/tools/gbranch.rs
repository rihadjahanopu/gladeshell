// =============================================================================
//  src/tools/gbranch.rs — Interactive Git Branch Manager & Switcher (Phase 5)
// =============================================================================

use inquire::{Confirm, Select, Text};
use std::process::Command;

/// Runs the interactive Git Branch Manager.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    if !is_git_repo() {
        return Err("Not inside a git repository!".into());
    }

    let output = Command::new("git")
        .arg("branch")
        .arg("-a")
        .arg("--format")
        .arg("%(refname:short)")
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let branches: Vec<String> = stdout
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    if branches.is_empty() {
        println!("🌿 No git branches found.");
        return Ok(());
    }

    let mut options = vec!["➕ Create New Branch".to_string()];
    options.extend(branches);

    let selected = Select::new("🌿 Git Branch Manager:", options).prompt()?;

    if selected.contains("Create New Branch") {
        let new_branch = Text::new("New Branch Name:").prompt()?;
        let clean = new_branch.trim();
        if !clean.is_empty() {
            let status = Command::new("git")
                .arg("checkout")
                .arg("-b")
                .arg(clean)
                .status()?;
            if status.success() {
                println!("✅ Switched to new branch: {}", clean);
            }
        }
    } else {
        let branch_name = selected.trim();
        let act = Select::new(
            &format!("Action for branch '{}':", branch_name),
            vec!["Checkout / Switch", "Delete Branch", "Pull Latest"],
        )
        .prompt()?;

        match act {
            "Checkout / Switch" => {
                let status = Command::new("git").arg("checkout").arg(branch_name).status()?;
                if status.success() {
                    println!("✅ Switched to branch: {}", branch_name);
                }
            }
            "Delete Branch" => {
                if Confirm::new(&format!("Delete branch '{}'?", branch_name))
                    .with_default(false)
                    .prompt()?
                {
                    let status = Command::new("git").arg("branch").arg("-D").arg(branch_name).status()?;
                    if status.success() {
                        println!("🗑️ Deleted branch: {}", branch_name);
                    }
                }
            }
            "Pull Latest" => {
                let _ = Command::new("git").arg("checkout").arg(branch_name).status();
                let status = Command::new("git").arg("pull").status()?;
                if status.success() {
                    println!("✅ Branch '{}' is up to date.", branch_name);
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn is_git_repo() -> bool {
    Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
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
    fn test_is_git_repo_fn() {
        let _ = is_git_repo();
    }
}
