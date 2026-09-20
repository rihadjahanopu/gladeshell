// =============================================================================
//  src/tools/self_upgrade.rs — Self-upgrader for fancybash (`upgrade`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::io::Read;
use std::process::Command;

pub fn run() -> Result<(), Box<dyn Error>> {
    const GH_RS_URL: &str =
        "https://raw.githubusercontent.com/rihadjahanopu/fancybash-rs/main/install.sh";
    const GH_URL: &str =
        "https://raw.githubusercontent.com/rihadjahanopu/fancybash/main/install.sh";
    const CDN_URL: &str = "https://fancybash.netlify.app/install.sh";

    println!("\x1b[1;35m⚡ Upgrading fancybash to latest version...\x1b[0m");

    let temp_dir = std::env::temp_dir();
    let script_path = temp_dir.join("fancybash_install.sh");

    // Tier 1: ureq HTTP download
    for url in [GH_RS_URL, GH_URL, CDN_URL] {
        if let Ok(response) = ureq::get(url).call() {
            let mut reader = response.into_reader();
            let mut script_content = String::new();
            if reader.read_to_string(&mut script_content).is_ok() && !script_content.is_empty() {
                if fs::write(&script_path, &script_content).is_ok() {
                    println!("\x1b[1;32m✨ fancybash upgrade script fetched successfully!\x1b[0m");
                    if run_installer(&script_path) {
                        return Ok(());
                    }
                }
            }
        }
    }

    // Tier 2: System curl fallback
    println!("\x1b[1;33m⚠️ ureq fetch failed, attempting system curl fallback...\x1b[0m");
    for url in [GH_RS_URL, GH_URL, CDN_URL] {
        let status = Command::new("curl")
            .args(["-fsSL", url, "-o"])
            .arg(&script_path)
            .status();

        if let Ok(st) = status {
            if st.success() && script_path.exists() {
                println!("\x1b[1;32m✨ fancybash upgrade script downloaded via curl!\x1b[0m");
                if run_installer(&script_path) {
                    return Ok(());
                }
            }
        }
    }

    // Tier 3: System wget fallback
    for url in [GH_RS_URL, GH_URL, CDN_URL] {
        let status = Command::new("wget")
            .args(["-qO"])
            .arg(&script_path)
            .arg(url)
            .status();

        if let Ok(st) = status {
            if st.success() && script_path.exists() {
                println!("\x1b[1;32m✨ fancybash upgrade script downloaded via wget!\x1b[0m");
                if run_installer(&script_path) {
                    return Ok(());
                }
            }
        }
    }

    // Tier 4: Check for local install.sh script in current directory or parent
    for candidate in ["./install.sh", "../install.sh"] {
        let p = std::path::Path::new(candidate);
        if p.exists() {
            println!("\x1b[1;36m💡 Found local install.sh script — executing...\x1b[0m");
            if run_installer(p) {
                return Ok(());
            }
        }
    }

    // Tier 5: Cargo install fallback
    if Command::new("cargo").arg("--version").status().map(|s| s.success()).unwrap_or(false) {
        println!("\x1b[1;35m⚡ Attempting upgrade via `cargo install --git`...\x1b[0m");
        let cargo_st = Command::new("cargo")
            .args(["install", "--git", "https://github.com/rihadjahanopu/fancybash-rs"])
            .status();

        if let Ok(st) = cargo_st {
            if st.success() {
                println!("\x1b[1;32m✨ fancybash upgraded successfully via Cargo!\x1b[0m");
                return Ok(());
            }
        }
    }

    Err("❌ Upgrade failed. Could not fetch installer or build binary.".into())
}

fn run_installer(path: &std::path::Path) -> bool {
    println!("\x1b[1;35m🚀 Executing upgrade script...\x1b[0m");
    let status = Command::new("bash").arg(path).status();

    match status {
        Ok(st) if st.success() => {
            println!("\x1b[1;32m✨ fancybash upgraded successfully!\x1b[0m");
            println!(
                "\x1b[1;36m💡 Restart your shell or run: eval \"$(fancybash init <shell>)\"\x1b[0m"
            );
            true
        }
        _ => {
            println!("\x1b[1;33m⚠️ Upgrade script executed, but returned non-zero status.\x1b[0m");
            println!("\x1b[1;36m💡 You can manually run: bash {}\x1b[0m", path.display());
            false
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_self_upgrade_cdn_url_valid() {
        const CDN_URL: &str = "https://fancybash.netlify.app/install.sh";
        assert!(CDN_URL.starts_with("https://"));
    }
}


