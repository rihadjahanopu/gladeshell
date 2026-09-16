// =============================================================================
//  src/tools/project_setup/vite.rs — Interactive Vite Project Generator (React/Vue)
// =============================================================================

use std::error::Error;
use std::process::Command;
use super::tailwind::run_css;
use super::utils::{cmd_ok, prompt_confirm, prompt_select, resolve_cmd};

/// `fancybash vite` — Interactive Vite Project Generator with optional Tailwind v4.
pub fn run_vite() -> Result<(), Box<dyn Error>> {
    let pm     = prompt_select("⚡ Setup Vite with:", &["1) Bun", "2) NPM"])?;
    let add_tw = prompt_confirm("Add Tailwind CSS v4?", true)?;

    if pm.contains("Bun") {
        Command::new(resolve_cmd("bunx")).arg("create-vite@latest").arg(".").status()?;
        if add_tw {
            let ok = cmd_ok("bun", &["add", "tailwindcss", "@tailwindcss/vite"]);
            if !ok {
                let retry = prompt_confirm("Install failed. Retry with --force?", false).unwrap_or(false);
                if retry {
                    cmd_ok("bun", &["add", "tailwindcss", "@tailwindcss/vite", "--force"]);
                }
            }
        }
    } else {
        Command::new(resolve_cmd("npx")).arg("create-vite@latest").arg(".").status()?;
        if add_tw {
            let ok = cmd_ok("npm", &["install", "tailwindcss", "@tailwindcss/vite"]);
            if !ok {
                let retry = prompt_confirm("Install failed (peer deps?). Retry with --legacy-peer-deps?", false).unwrap_or(false);
                if retry {
                    cmd_ok("npm", &["install", "tailwindcss", "@tailwindcss/vite", "--legacy-peer-deps"]);
                }
            }
        }
    }

    if add_tw {
        run_css()?;
    }

    Ok(())
}
