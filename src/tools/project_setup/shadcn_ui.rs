// =============================================================================
//  src/tools/project_setup/shadcn_ui.rs — Interactive Shadcn UI Setup
// =============================================================================

use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;
use super::utils::{patch_tsconfig, patch_viteconfig, prompt_select, prompt_text, resolve_cmd};

/// `fancybash ui` — Interactive Shadcn UI Setup.
pub fn run_ui() -> Result<(), Box<dyn Error>> {
    let project_type = if Path::new("tsconfig.app.json").exists() {
        println!("  Detected: Vite project");
        "vite"
    } else if Path::new("next.config.js").exists()
        || Path::new("next.config.ts").exists()
        || Path::new("next.config.mjs").exists()
        || fs::read_to_string("package.json")
            .map(|c| c.contains("\"next\""))
            .unwrap_or(false)
    {
        println!("  Detected: Next.js project");
        "nextjs"
    } else {
        println!("  Could not auto-detect project type.");
        let choice = prompt_select("Choose manually:", &["1) Vite (React)", "2) Next.js"])?;
        if choice.contains("Vite") { "vite" } else { "nextjs" }
    };

    let pm         = prompt_select("Package manager:", &["1) Bun", "2) NPM"])?;
    let components = prompt_text("Add components (e.g. button card input, or empty for default):")?;

    println!("\nPre-configuring path aliases before shadcn init...");
    patch_tsconfig();

    let use_bun = pm.contains("Bun");

    let runner = resolve_cmd(if use_bun { "bunx" } else { "npx" });
    let init_prefix: &[&str] = if use_bun { &["--bun", "shadcn@latest", "init"] } else { &["shadcn@latest", "init"] };
    let add_prefix:  &[&str] = if use_bun { &["--bun", "shadcn@latest", "add"] } else { &["shadcn@latest", "add"] };
    let vite_flag:   &[&str] = if project_type == "vite" { &["-t", "vite"] } else { &[] };

    let _ = Command::new(&runner).args(init_prefix).args(vite_flag).status();

    let mut add_cmd = Command::new(&runner);
    add_cmd.args(add_prefix);
    if components.trim().is_empty() {
        add_cmd.arg("button");
    } else {
        for comp in components.split_whitespace() {
            add_cmd.arg(comp);
        }
    }
    add_cmd.status()?;

    if project_type == "vite" {
        println!("\nPatching vite.config with alias & tailwind...");
        patch_viteconfig();
    } else {
        println!("  Next.js detected — vite.config patch skipped.");
    }

    println!("\n---------------------------------------------------");
    println!("✅ Shadcn UI setup complete! Happy coding!");
    println!("---------------------------------------------------");
    Ok(())
}
