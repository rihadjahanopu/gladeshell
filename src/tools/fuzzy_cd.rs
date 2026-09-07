// =============================================================================
//  src/tools/fuzzy_cd.rs — Interactive Fuzzy Directory Navigator (`cf`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use inquire::Select;

fn collect_dirs_recursive(dir: &Path, acc: &mut Vec<PathBuf>, depth: usize) {
    if depth > 4 || acc.len() > 500 {
        return;
    }
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    if !name.starts_with('.') && name != "node_modules" && name != "target" {
                        acc.push(path.clone());
                        collect_dirs_recursive(&path, acc, depth + 1);
                    }
                }
            }
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    let mut dirs = vec![current_dir.clone()];

    collect_dirs_recursive(&current_dir, &mut dirs, 1);

    if dirs.is_empty() {
        println!("\x1b[0;33m⚠️ No subdirectories found.\x1b[0m");
        return Ok(());
    }

    let items: Vec<String> = dirs
        .iter()
        .map(|p| {
            if let Ok(rel) = p.strip_prefix(&current_dir) {
                if rel.as_os_str().is_empty() {
                    ". (current)".to_string()
                } else {
                    format!("📁 {}", rel.display())
                }
            } else {
                format!("📁 {}", p.display())
            }
        })
        .collect();

    let chosen = match Select::new("📂 Navigate Directory:", items).prompt() {
        Ok(v) => v,
        Err(_) => {
            println!("👋 Cancelled.");
            return Ok(());
        }
    };

    let target_dir = if chosen == ". (current)" {
        current_dir
    } else {
        let path_str = chosen.trim_start_matches("📁 ");
        current_dir.join(path_str)
    };

    println!("{}", target_dir.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_dirs_current() {
        let mut dirs = Vec::new();
        let cur = std::env::current_dir().unwrap();
        collect_dirs_recursive(&cur, &mut dirs, 1);
        assert!(!dirs.is_empty());
    }
}
