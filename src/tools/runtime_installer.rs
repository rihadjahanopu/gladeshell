// =============================================================================
//  src/tools/runtime_installer.rs — Interactive JS Runtime & NVM Installer (`rt`)
// =============================================================================

use std::error::Error;
use std::process::Command;

use inquire::Select;

pub fn run() -> Result<(), Box<dyn Error>> {
    let options = vec![
        "NVM (Node Version Manager)",
        "Node.js (LTS Version)",
        "Bun (Fast JS Runtime)",
        "Deno (Secure JS Runtime)",
    ];

    let chosen = match Select::new("🚀 Ultimate Runtime & Tool Installer:", options).prompt() {
        Ok(opt) => opt,
        Err(_) => {
            println!("👋 Exit");
            return Ok(());
        }
    };

    match chosen {
        "NVM (Node Version Manager)" => {
            let home = std::env::var("HOME").unwrap_or_default();
            let nvm_path = format!("{home}/.nvm");
            if std::path::Path::new(&nvm_path).exists() {
                println!("\x1b[1;32m✅ NVM is already installed at {nvm_path}\x1b[0m");
            } else {
                println!("\x1b[1;36m📥 Installing NVM...\x1b[0m");
                let status = Command::new("bash")
                    .arg("-c")
                    .arg("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash")
                    .status()?;
                if status.success() {
                    println!("\x1b[1;32m✨ NVM installed successfully!\x1b[0m");
                }
            }
        }
        "Node.js (LTS Version)" => {
            println!("\x1b[1;36m📦 Installing Node.js LTS via NVM...\x1b[0m");
            let status = Command::new("bash")
                .arg("-c")
                .arg("export NVM_DIR=\"$HOME/.nvm\" && [ -s \"$NVM_DIR/nvm.sh\" ] && \\. \"$NVM_DIR/nvm.sh\" && nvm install --lts && nvm use --lts")
                .status()?;
            if status.success() {
                println!("\x1b[1;32m✨ Node.js LTS installed!\x1b[0m");
            } else {
                eprintln!("\x1b[1;31m❌ Failed to install Node.js. Please install NVM first.\x1b[0m");
            }
        }
        "Bun (Fast JS Runtime)" => {
            if cmd_exists("bun") {
                println!("\x1b[1;32m✅ Bun is already installed.\x1b[0m");
            } else {
                println!("\x1b[1;36m🥐 Installing Bun...\x1b[0m");
                let status = Command::new("bash")
                    .arg("-c")
                    .arg("curl -fsSL https://bun.sh/install | bash")
                    .status()?;
                if status.success() {
                    println!("\x1b[1;32m✨ Bun installed successfully!\x1b[0m");
                }
            }
        }
        "Deno (Secure JS Runtime)" => {
            if cmd_exists("deno") {
                println!("\x1b[1;32m✅ Deno is already installed.\x1b[0m");
            } else {
                println!("\x1b[1;36m🦕 Installing Deno...\x1b[0m");
                let status = Command::new("bash")
                    .arg("-c")
                    .arg("curl -fsSL https://deno.land/x/install/install.sh | sh")
                    .status()?;
                if status.success() {
                    println!("\x1b[1;32m✨ Deno installed successfully!\x1b[0m");
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn cmd_exists(cmd: &str) -> bool {
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
    fn test_cmd_exists_fn() {
        assert!(cmd_exists("bash") || cmd_exists("sh"));
    }
}
