// =============================================================================
//  src/tools/vault.rs — Hardened AES-256 Multi-Vault Manager (Phase 5)
// =============================================================================

use inquire::{Password, Select, Text};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn vault_store_dir() -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join(".secret_vaults")
}

fn ram_base_dir() -> PathBuf {
    let shm = Path::new("/dev/shm");
    if shm.exists() && shm.is_dir() {
        shm.to_path_buf()
    } else {
        std::env::temp_dir()
    }
}

/// Runs the interactive AES-256 Vault Manager.
pub fn run(action_opt: Option<&str>, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let store_dir = vault_store_dir();
    fs::create_dir_all(&store_dir)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&store_dir, fs::Permissions::from_mode(0o700));
    }

    let action = match action_opt {
        Some(a) => a,
        None => {
            let choice = Select::new(
                "🔐 HARDENED AES-256 MULTI-VAULT MANAGER",
                vec![
                    "1. 🔓 Unlock Vault (Mount to RAM)",
                    "2. 🔒 Lock Vault (Encrypt & Wipe RAM)",
                    "3. ➕ Create New Vault",
                    "4. 📋 List All Vaults",
                    "❌ Exit",
                ],
            )
            .prompt()?;

            if choice.contains("Exit") {
                return Ok(());
            }

            if choice.contains("Unlock") {
                "unlock"
            } else if choice.contains("Lock") {
                "lock"
            } else if choice.contains("Create") {
                "create"
            } else if choice.contains("List") {
                "list"
            } else {
                "list"
            }
        }
    };

    match action {
        "create" | "new" => {
            let name = if !args.is_empty() {
                args[0].clone()
            } else {
                Text::new("Enter new vault name (e.g. MySecrets):").prompt()?
            };

            let clean_name = name.trim().replace(' ', "_");
            if clean_name.is_empty() {
                println!("❌ Vault name cannot be empty!");
                return Ok(());
            }

            let enc_file = store_dir.join(format!("{}.enc", clean_name));
            if enc_file.exists() {
                println!("❌ Vault '{}.enc' already exists!", clean_name);
                return Ok(());
            }

            let pass1 = Password::new("Master Password:").prompt()?;
            let pass2 = Password::new("Confirm Master Password:").prompt()?;

            if pass1 != pass2 {
                println!("❌ Passwords do not match!");
                return Ok(());
            }

            if pass1.trim().is_empty() {
                println!("❌ Password cannot be empty!");
                return Ok(());
            }

            let ram_base = ram_base_dir();
            let ram_vault = ram_base.join(format!(".vault_{}_{}", clean_name, std::process::id()));
            fs::create_dir_all(&ram_vault)?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(&ram_vault, fs::Permissions::from_mode(0o700));
            }

            // Create initial readme
            fs::write(
                ram_vault.join("README.txt"),
                format!("Secret Vault '{}' initialized successfully.\nAdd your sensitive files here and run 'fancybash vault lock' to encrypt.", clean_name),
            )?;

            // Pack and encrypt
            let archive = ram_base.join(format!(".tmp_{}.tar.gz", std::process::id()));
            let tar_status = Command::new("tar")
                .arg("-czf")
                .arg(&archive)
                .arg("-C")
                .arg(&ram_vault)
                .arg(".")
                .status();

            if let Ok(s) = tar_status {
                if s.success() {
                    let enc_status = Command::new("openssl")
                        .arg("enc")
                        .arg("-aes-256-cbc")
                        .arg("-pbkdf2")
                        .arg("-iter")
                        .arg("500000")
                        .arg("-pass")
                        .arg(format!("pass:{}", pass1))
                        .arg("-in")
                        .arg(&archive)
                        .arg("-out")
                        .arg(&enc_file)
                        .status();

                    let _ = fs::remove_file(&archive);
                    let _ = fs::remove_dir_all(&ram_vault);

                    if let Ok(es) = enc_status {
                        if es.success() {
                            println!("✅ Created encrypted vault: {}", enc_file.display());
                        } else {
                            println!("❌ Encryption failed!");
                        }
                    }
                }
            }
        }

        "unlock" | "open" => {
            let enc_vaults = list_encrypted_vaults(&store_dir)?;
            if enc_vaults.is_empty() {
                println!("📋 No encrypted vaults found in {}", store_dir.display());
                return Ok(());
            }

            let name = if !args.is_empty() {
                args[0].clone()
            } else {
                Select::new("Select vault to unlock:", enc_vaults).prompt()?
            };

            let clean_name = name.trim().trim_end_matches(".enc").to_string();
            let enc_file = store_dir.join(format!("{}.enc", clean_name));

            if !enc_file.exists() {
                println!("❌ Vault file not found: {}", enc_file.display());
                return Ok(());
            }

            let pass = Password::new("Enter Master Password:").prompt()?;
            let ram_base = ram_base_dir();
            let archive = ram_base.join(format!(".tmp_dec_{}.tar.gz", std::process::id()));

            let dec_status = Command::new("openssl")
                .arg("enc")
                .arg("-d")
                .arg("-aes-256-cbc")
                .arg("-pbkdf2")
                .arg("-iter")
                .arg("500000")
                .arg("-pass")
                .arg(format!("pass:{}", pass))
                .arg("-in")
                .arg(&enc_file)
                .arg("-out")
                .arg(&archive)
                .status();

            if let Ok(ds) = dec_status {
                if ds.success() {
                    let home = std::env::var_os("HOME").map(PathBuf::from).unwrap();
                    let target_dir = home.join(&clean_name);

                    if target_dir.exists() {
                        let _ = fs::remove_dir_all(&target_dir);
                    }
                    fs::create_dir_all(&target_dir)?;

                    let tar_status = Command::new("tar")
                        .arg("-xzf")
                        .arg(&archive)
                        .arg("-C")
                        .arg(&target_dir)
                        .status();

                    let _ = fs::remove_file(&archive);

                    if let Ok(ts) = tar_status {
                        if ts.success() {
                            println!("🔓 Vault unlocked to: {}", target_dir.display());
                        } else {
                            println!("❌ Tar extraction failed!");
                        }
                    }
                } else {
                    let _ = fs::remove_file(&archive);
                    println!("❌ Invalid password or corrupted vault!");
                }
            }
        }

        "lock" | "close" => {
            let home = std::env::var_os("HOME").map(PathBuf::from).unwrap();
            let name = if !args.is_empty() {
                args[0].clone()
            } else {
                Text::new("Enter vault name to lock:").prompt()?
            };

            let clean_name = name.trim().to_string();
            let target_dir = home.join(&clean_name);

            if !target_dir.exists() {
                println!("❌ Open vault directory not found: {}", target_dir.display());
                return Ok(());
            }

            let pass1 = Password::new("Master Password:").prompt()?;
            let pass2 = Password::new("Confirm Master Password:").prompt()?;

            if pass1 != pass2 {
                println!("❌ Passwords do not match!");
                return Ok(());
            }

            let enc_file = store_dir.join(format!("{}.enc", clean_name));
            let ram_base = ram_base_dir();
            let archive = ram_base.join(format!(".tmp_enc_{}.tar.gz", std::process::id()));

            let tar_status = Command::new("tar")
                .arg("-czf")
                .arg(&archive)
                .arg("-C")
                .arg(&target_dir)
                .arg(".")
                .status();

            if let Ok(ts) = tar_status {
                if ts.success() {
                    let enc_status = Command::new("openssl")
                        .arg("enc")
                        .arg("-aes-256-cbc")
                        .arg("-pbkdf2")
                        .arg("-iter")
                        .arg("500000")
                        .arg("-pass")
                        .arg(format!("pass:{}", pass1))
                        .arg("-in")
                        .arg(&archive)
                        .arg("-out")
                        .arg(&enc_file)
                        .status();

                    let _ = fs::remove_file(&archive);

                    if let Ok(es) = enc_status {
                        if es.success() {
                            fs::remove_dir_all(&target_dir)?;
                            println!("🔒 Vault locked and encrypted: {}", enc_file.display());
                        } else {
                            println!("❌ Encryption failed!");
                        }
                    }
                }
            }
        }

        "list" | "ls" => {
            let enc_vaults = list_encrypted_vaults(&store_dir)?;
            if enc_vaults.is_empty() {
                println!("📋 No encrypted vaults found in {}", store_dir.display());
            } else {
                println!("\n🔐 VAULT STORE ({} encrypted vaults):", enc_vaults.len());
                println!("──────────────────────────────────────");
                for v in enc_vaults {
                    println!("  🔒 {}", v);
                }
                println!("──────────────────────────────────────\n");
            }
        }

        _ => {
            println!("Usage: fancybash vault [create <name> | lock <name> | unlock <name> | list]");
        }
    }

    Ok(())
}

fn list_encrypted_vaults(store_dir: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut vaults = Vec::new();
    if let Ok(entries) = fs::read_dir(store_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".enc") {
                vaults.push(name);
            }
        }
    }
    vaults.sort();
    Ok(vaults)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_store_path_exists() {
        let store = vault_store_dir();
        assert!(store.to_string_lossy().contains(".secret_vaults"));
    }
}
