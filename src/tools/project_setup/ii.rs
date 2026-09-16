// =============================================================================
//  src/tools/project_setup/ii.rs — Interactive Project Initializer (`ii`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;
use super::utils::{prompt_select, resolve_cmd};

/// `fancybash ii` — Interactive Project Initializer (Bun/NPM/PNPM/Yarn + .gitignore).
pub fn run_ii() -> Result<(), Box<dyn Error>> {
    let pm = prompt_select(
        "🚀 Select Package Manager:",
        &[
            "1) 🥐 Bun (Fast)",
            "2) 📦 NPM (Standard)",
            "3) 🟡 PNPM (Strict)",
            "4) 🧶 Yarn (Classic)",
        ],
    )?;

    if pm.contains("Bun") {
        Command::new(resolve_cmd("bun")).arg("init").arg("-y").status()?;
    } else if pm.contains("NPM") {
        Command::new(resolve_cmd("npm")).arg("init").arg("-y").status()?;
    } else if pm.contains("PNPM") {
        Command::new(resolve_cmd("pnpm")).arg("init").status()?;
    } else {
        Command::new(resolve_cmd("yarn")).arg("init").arg("-y").status()?;
    }

    if !Path::new(".gitignore").exists() {
        let gitignore = "node_modules/\n.env\n.env*.local\ndist/\nbuild/\n.next/\n.cache/\n*.log\n.DS_Store\n";
        fs::write(".gitignore", gitignore)?;
        println!("✅ .gitignore created.");
    }

    println!("✅ Project initialized successfully!");
    Ok(())
}
