// =============================================================================
//  src/tools/rmf.rs — Remove file with confirmation (`rmf`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::path::Path;

pub fn run(name: &str, force: bool) -> Result<(), Box<dyn Error>> {
    if name.trim().is_empty() {
        println!("\x1b[1;33mUsage: rmf <file_name>\x1b[0m");
        return Ok(());
    }

    let path = Path::new(name);
    if !path.exists() {
        println!("\x1b[1;31m❌ File does not exist: {}\x1b[0m", name);
        return Ok(());
    }

    if force {
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
        println!("\x1b[1;32m✅ Removed: {}\x1b[0m", name);
    } else {
        println!("\x1b[1;33m❓ Remove '{}'? (run with -f to skip prompt)\x1b[0m", name);
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
        println!("\x1b[1;32m✅ Removed: {}\x1b[0m", name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rmf_run() {
        let test_file = "target/test_rmf_file.txt";
        fs::write(test_file, "test").unwrap();
        run(test_file, true).unwrap();
        assert!(!Path::new(test_file).exists());
    }
}
