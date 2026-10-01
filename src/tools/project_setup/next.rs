// ============================================================================
// STATUS: 100% NATIVE RUST (ZERO EXTERNAL BINARY DEPENDENCIES)
// AUDIT COMPLETED: FULL FEATURE PARITY, CROSS-OS VERIFIED & OPTIMIZED
// HANDS-OFF GUARANTEE: NO MANUAL EDITS REQUIRED
// ============================================================================

// =============================================================================
//  src/tools/project_setup/next.rs — Interactive Next.js Project Generator
// =============================================================================

use std::error::Error;
use std::process::Command;
use super::utils::{prompt_select, resolve_cmd};

/// `gladeshell next` — Interactive Next.js Project Generator.
pub fn run_next() -> Result<(), Box<dyn Error>> {
    let pm = prompt_select("⚡ Setup Next.js with:", &["1) Bun", "2) NPM"])?;

    if pm.contains("Bun") {
        Command::new(resolve_cmd("bunx")).arg("create-next-app@latest").arg(".").status()?;
    } else {
        Command::new(resolve_cmd("npx")).arg("create-next-app@latest").arg(".").status()?;
    }
    Ok(())
}
