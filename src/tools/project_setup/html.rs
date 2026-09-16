// =============================================================================
//  src/tools/project_setup/html.rs — Serve or run index.html (`html`)
// =============================================================================

use std::error::Error;
use std::path::Path;
use std::process::Command;
use super::utils::{cmd_ok, resolve_cmd};

/// `fancybash html` — Serve or run `index.html` with Bun, or open in default browser.
pub fn run_html() -> Result<(), Box<dyn Error>> {
    let html_file = if Path::new("index.html").exists() {
        "index.html"
    } else if Path::new("public/index.html").exists() {
        "public/index.html"
    } else {
        "index.html"
    };

    if cmd_ok("bun", &["--version"]) {
        println!("🚀 Serving {html_file} with Bun...");
        Command::new(resolve_cmd("bun")).arg("run").arg(html_file).status()?;
    } else {
        println!("🌐 Opening {html_file} in default browser...");
        let opener = if cfg!(target_os = "macos") {
            "open"
        } else if cfg!(target_os = "windows") {
            "explorer"
        } else {
            "xdg-open"
        };
        let _ = Command::new(opener).arg(html_file).status();
    }
    Ok(())
}
