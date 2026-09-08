// =============================================================================
//  src/tools/notes.rs — Plain-text notes manager (Phase 5)
//
//  Clipboard: native `arboard` crate (X11 + Wayland + macOS — zero external binary)
//  No xclip, xsel, wl-copy or any 3rd-party CLI tools.
// =============================================================================

use inquire::{Confirm, Select, Text};
use std::fs;
use std::path::PathBuf;

fn notes_dir_path() -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join(".my_notes")
}

/// Runs the interactive notes manager.
pub fn run(action_opt: Option<&str>, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = notes_dir_path();
    let general_dir = root_dir.join("General");
    fs::create_dir_all(&general_dir)?;

    let action = match action_opt {
        Some(a) => a,
        None => {
            let choice = Select::new(
                "📝 FANCYBASH NOTES MANAGER",
                vec![
                    "1. ➕ Add New Note",
                    "2. 📋 List / View Notes",
                    "3. 🔍 Search Notes",
                    "4. 📋 Copy Note to Clipboard",
                    "5. 🗑️ Delete Note",
                    "❌ Exit",
                ],
            )
            .prompt()?;

            if choice.contains("Exit") {
                return Ok(());
            }

            if choice.contains("Add") {
                "add"
            } else if choice.contains("List") {
                "list"
            } else if choice.contains("Search") {
                "search"
            } else if choice.contains("Copy") {
                "copy"
            } else if choice.contains("Delete") {
                "delete"
            } else {
                "list"
            }
        }
    };

    match action {
        "add" => {
            let categories = list_categories(&root_dir)?;
            let mut cat_options = vec!["➕ Create New Category".to_string()];
            cat_options.extend(categories);

            let selected_cat = Select::new("Select Category:", cat_options).prompt()?;

            let category = if selected_cat.contains("Create New Category") {
                let new_cat = Text::new("New Category Name:").prompt()?;
                let clean = new_cat.trim();
                if clean.is_empty() {
                    return Ok(());
                }
                clean.to_string()
            } else {
                selected_cat
            };

            let title = if !args.is_empty() {
                args.join(" ")
            } else {
                Text::new("Note Title:").prompt()?
            };

            let clean_title = title.trim();
            if clean_title.is_empty() {
                println!("❌ Note title cannot be empty!");
                return Ok(());
            }

            let cat_dir = root_dir.join(&category);
            fs::create_dir_all(&cat_dir)?;

            let file_path = cat_dir.join(format!("{}.txt", clean_title));

            if file_path.exists() {
                if !Confirm::new("Note already exists! Overwrite?")
                    .with_default(false)
                    .prompt()?
                {
                    return Ok(());
                }
            }

            let content = Text::new("Note Content:").prompt()?;
            let clean_content = content.trim();
            if clean_content.is_empty() {
                println!("❌ Note content cannot be empty!");
                return Ok(());
            }
            fs::write(&file_path, clean_content)?;
            println!("✅ Note saved: {}/{}", category, clean_title);
        }

        "list" | "ls" => {
            show_notes(&root_dir)?;
        }

        "search" => {
            let query = if !args.is_empty() {
                args.join(" ")
            } else {
                Text::new("Search query:").prompt()?
            };

            let clean_q = query.trim().to_lowercase();
            if clean_q.is_empty() {
                return Ok(());
            }

            println!("\n🔍 Searching for '{}' in notes...", clean_q);
            let mut matches = 0;
            for entry in walkdir::WalkDir::new(&root_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if content.to_lowercase().contains(&clean_q)
                            || entry.file_name().to_string_lossy().to_lowercase().contains(&clean_q)
                        {
                            println!("  📄 {}", entry.path().display());
                            // Show matching lines inline
                            for (lineno, line) in content.lines().enumerate() {
                                if line.to_lowercase().contains(&clean_q) {
                                    println!("    L{}: {}", lineno + 1, line.trim());
                                }
                            }
                            matches += 1;
                        }
                    }
                }
            }
            if matches == 0 {
                println!("  No matching notes found.");
            } else {
                println!("✅ Found {} matching note(s).", matches);
            }
        }

        // ── Native clipboard copy — arboard (X11 + Wayland + macOS, zero external binary) ──
        "copy" | "cp" => {
            let note_files = collect_all_notes(&root_dir)?;
            if note_files.is_empty() {
                println!("📋 No notes found! Create one first with: fancybash notes add");
                return Ok(());
            }

            let options: Vec<String> = note_files
                .iter()
                .map(|p| {
                    p.strip_prefix(&root_dir)
                        .unwrap_or(p)
                        .display()
                        .to_string()
                })
                .collect();

            let selected = Select::new("📋 Select Note to Copy to Clipboard:", options).prompt()?;
            let target_path = root_dir.join(&selected);

            let content = fs::read_to_string(&target_path)?;
            if content.is_empty() {
                println!("⚠️ Note is empty.");
                return Ok(());
            }

            match arboard::Clipboard::new() {
                Ok(mut clipboard) => {
                    clipboard.set_text(&content)?;
                    println!("✅ Copied '{}' to clipboard ({} chars).", selected, content.len());
                }
                Err(e) => {
                    eprintln!("❌ Clipboard unavailable: {e}");
                    eprintln!("   Printing note content instead:");
                    println!("{}", content);
                }
            }
        }

        "delete" | "rm" => {
            let note_files = collect_all_notes(&root_dir)?;
            if note_files.is_empty() {
                println!("📋 No notes to delete!");
                return Ok(());
            }

            let options: Vec<String> = note_files
                .iter()
                .map(|p| {
                    p.strip_prefix(&root_dir)
                        .unwrap_or(p)
                        .display()
                        .to_string()
                })
                .collect();

            let selected = Select::new("Select Note to Delete:", options).prompt()?;
            let target_path = root_dir.join(&selected);

            if Confirm::new(&format!("Delete '{}'?", selected))
                .with_default(false)
                .prompt()?
            {
                fs::remove_file(target_path)?;
                println!("🗑️ Note deleted.");
            }
        }

        _ => {
            println!("Usage: fancybash notes [add | list | search <query> | copy | delete]");
        }
    }

    Ok(())
}

fn list_categories(root_dir: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut cats = Vec::new();
    if let Ok(entries) = fs::read_dir(root_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                if let Some(name) = entry.file_name().to_str() {
                    cats.push(name.to_string());
                }
            }
        }
    }
    cats.sort();
    Ok(cats)
}

fn collect_all_notes(root_dir: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }
    files.sort();
    Ok(files)
}

fn show_notes(root_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let note_files = collect_all_notes(root_dir)?;
    if note_files.is_empty() {
        println!("📋 No notes found! Create one with: fancybash notes add");
        return Ok(());
    }

    let options: Vec<String> = note_files
        .iter()
        .map(|p| p.strip_prefix(root_dir).unwrap_or(p).display().to_string())
        .collect();

    println!("\n📝 ALL NOTES ({} total):", note_files.len());
    println!("──────────────────────────────────────");
    for label in &options {
        println!("  📄 {}", label);
    }
    println!("──────────────────────────────────────");

    // Offer to view a note inline
    if let Ok(selected) = Select::new("📖 View a note? (ESC to skip):", options).prompt() {
        let path = root_dir.join(&selected);
        let content = fs::read_to_string(&path).unwrap_or_else(|_| "(unreadable)".to_string());
        println!("\n── {} ──", selected);
        println!("{}", content);
        println!("────────────────────────────────────────\n");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_categories_empty() {
        let temp = std::env::temp_dir().join(format!("test_notes_{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();
        let cats = list_categories(&temp).unwrap();
        assert!(cats.is_empty());
        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_collect_all_notes_empty_dir() {
        let temp = std::env::temp_dir().join(format!("test_notes_collect_{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();
        let notes = collect_all_notes(&temp).unwrap();
        assert!(notes.is_empty());
        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_note_write_and_read() {
        let temp = std::env::temp_dir().join(format!("test_note_rw_{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();
        let file = temp.join("test.txt");
        fs::write(&file, "hello fancybash notes").unwrap();
        let content = fs::read_to_string(&file).unwrap();
        assert_eq!(content, "hello fancybash notes");
        let _ = fs::remove_dir_all(&temp);
    }
}
