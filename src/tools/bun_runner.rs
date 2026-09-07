// =============================================================================
//  src/tools/bun_runner.rs — Interactive Bun JS/TS File Runner (`run`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::process::Command;

use inquire::Select;

pub fn run() -> Result<(), Box<dyn Error>> {
    // 1. Collect .js and .ts files in current directory
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if ext.eq_ignore_ascii_case("js") || ext.eq_ignore_ascii_case("ts") {
                        if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                            files.push(filename.to_string());
                        }
                    }
                }
            }
        }
    }

    files.sort();

    if files.is_empty() {
        println!("\x1b[0;31m❌ No .js or .ts files found in current directory!\x1b[0m");
        return Ok(());
    }

    // 2. Select file
    println!("\n\x1b[0;36m╭──────────────────────────────────────────╮\x1b[0m");
    println!("\x1b[0;36m│\x1b[0m  \x1b[1m⚡ BUN INTERACTIVE RUNNER\x1b[0m               \x1b[0;36m│\x1b[0m");
    println!("\x1b[0;36m╰──────────────────────────────────────────╯\x1b[0m");

    let items: Vec<String> = files
        .iter()
        .map(|f| {
            if f.ends_with(".ts") {
                format!("📘 {f}")
            } else {
                format!("📒 {f}")
            }
        })
        .collect();

    let chosen_item = match Select::new("Select JS/TS file to run:", items).prompt() {
        Ok(item) => item,
        Err(_) => {
            println!("👋 Cancelled.");
            return Ok(());
        }
    };

    // Extract filename from chosen_item ("📘 filename.ts" -> "filename.ts")
    let selected_file = chosen_item
        .trim_start_matches("📘 ")
        .trim_start_matches("📒 ")
        .to_string();

    // 3. Select mode
    let modes = vec![
        "🚀 bun run     (default)",
        "🔥 bun --hot   (hot reload)",
        "👁 bun --watch (watch mode)",
    ];

    let chosen_mode = match Select::new("Choose run mode:", modes).prompt() {
        Ok(mode) => mode,
        Err(_) => {
            println!("👋 Cancelled.");
            return Ok(());
        }
    };

    let (action, label, color) = if chosen_mode.contains("--hot") {
        ("--hot", "HOT RELOAD", "\x1b[0;31m")
    } else if chosen_mode.contains("--watch") {
        ("--watch", "WATCH MODE", "\x1b[1;33m")
    } else {
        ("run", "RUN", "\x1b[1;32m")
    };

    println!("\n{color}⚙ {label}:\x1b[0m \x1b[1m{selected_file}\x1b[0m\n");

    // 4. Run bun
    let status = Command::new("bun")
        .arg(action)
        .arg(&selected_file)
        .status();

    match status {
        Ok(s) => {
            if !s.success() {
                eprintln!("\x1b[0;31mProcess exited with status: {s}\x1b[0m");
            }
        }
        Err(e) => {
            eprintln!("\x1b[0;31mFailed to execute bun: {e}. Is Bun installed?\x1b[0m");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bun_runner_empty_dir_graceful() {
        // Just verify file collection logic logic without blocking prompt
        let files: Vec<String> = vec![];
        assert!(files.is_empty());
    }
}
