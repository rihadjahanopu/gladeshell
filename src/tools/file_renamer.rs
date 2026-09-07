// =============================================================================
//  src/tools/file_renamer.rs — Smart Batch File Renamer (`rn`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::path::Path;

pub fn run(target: Option<&str>) -> Result<(), Box<dyn Error>> {
    let dir_str = target.unwrap_or(".");
    let dir_path = Path::new(dir_str);

    if !dir_path.exists() || !dir_path.is_dir() {
        return Err(format!("Directory '{dir_str}' does not exist.").into());
    }

    println!("\x1b[1;36m🧹 Cleaning and renaming files in:\x1b[0m {}", dir_path.display());

    let entries = fs::read_dir(dir_path)?;
    let mut renamed_count = 0;
    let mut skipped_count = 0;

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let old_filename = match path.file_name().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => continue,
        };

        // Separate stem and extension
        let (stem, ext) = match old_filename.rfind('.') {
            Some(idx) if idx > 0 => (&old_filename[..idx], &old_filename[idx..]),
            _ => (old_filename, ""),
        };

        // Clean stem: lowercase, spaces to '-', remove non-alphanumeric except '_' and '-'
        let clean_stem: String = stem
            .to_lowercase()
            .chars()
            .map(|c| if c.is_whitespace() { '-' } else { c })
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
            .collect();

        let clean_ext = ext.to_lowercase();
        let new_filename = format!("{clean_stem}{clean_ext}");

        if old_filename == new_filename {
            continue;
        }

        let new_path = path.with_file_name(&new_filename);

        if new_path.exists() {
            println!("\x1b[0;33m⚠️ Skipped:\x1b[0m '{new_filename}' already exists.");
            skipped_count += 1;
        } else {
            fs::rename(&path, &new_path)?;
            println!("\x1b[0;32m✨ Renamed:\x1b[0m '{old_filename}' -> '{new_filename}'");
            renamed_count += 1;
        }
    }

    println!(
        "\x1b[1;32m✅ Batch rename complete:\x1b[0m {renamed_count} renamed, {skipped_count} skipped."
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_renamer_non_existent_dir() {
        assert!(run(Some("/non_existent_directory_xyz")).is_err());
    }
}
