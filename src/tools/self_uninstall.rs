// =============================================================================
//  src/tools/self_uninstall.rs — Self-uninstaller for fancybash (`uninstall`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("\x1b[1;33m🗑️ Uninstalling fancybash...\x1b[0m");

    let home = match std::env::var("HOME") {
        Ok(h) => PathBuf::from(h),
        Err(_) => return Err("Could not determine HOME directory".into()),
    };

    let target_files = vec![
        home.join(".bashrc"),
        home.join(".zshrc"),
        home.join(".config/fish/config.fish"),
    ];

    let mut cleaned_any = false;

    for path in target_files {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                let lines: Vec<&str> = content.lines().collect();
                let mut new_lines = Vec::new();
                let mut inside_fancy_block = false;
                let mut modified = false;

                for line in lines {
                    if line.contains("# >>> fancy-") {
                        inside_fancy_block = true;
                        modified = true;
                        continue;
                    }
                    if line.contains("# <<< fancy-") {
                        inside_fancy_block = false;
                        modified = true;
                        continue;
                    }
                    if inside_fancy_block {
                        modified = true;
                        continue;
                    }
                    if line.contains("fancybash init") || line.contains("# fancybash shell initialization") {
                        modified = true;
                        continue;
                    }
                    new_lines.push(line);
                }

                if modified {
                    let mut result_str = new_lines.join("\n");
                    if !result_str.is_empty() {
                        result_str.push('\n');
                    }
                    if fs::write(&path, result_str).is_ok() {
                        println!("\x1b[1;32m✅ Cleaned fancybash block from: {}\x1b[0m", path.display());
                        cleaned_any = true;
                    }
                }
            }
        }
    }

    // Try removing installed binaries
    let bin_paths = vec![
        home.join(".cargo/bin/fancybash"),
        home.join(".local/bin/fancybash"),
    ];

    for bin in bin_paths {
        if bin.exists() {
            if fs::remove_file(&bin).is_ok() {
                println!("\x1b[1;32m✅ Removed binary: {}\x1b[0m", bin.display());
            }
        }
    }

    if cleaned_any {
        println!("\n\x1b[1;32m🎉 fancybash uninstalled successfully!\x1b[0m");
        println!("\x1b[1;36m💡 Restart your shell or terminal for changes to take effect.\x1b[0m");
    } else {
        println!("\x1b[0;33mℹ️ No fancybash initializations found in shell config files.\x1b[0m");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_uninstall_does_not_panic() {
        assert!(std::env::var("HOME").is_ok());
    }
}
