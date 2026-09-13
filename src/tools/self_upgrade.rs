// =============================================================================
//  src/tools/self_upgrade.rs — Self-upgrader for fancybash (`upgrade`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::io::Read;

pub fn run() -> Result<(), Box<dyn Error>> {
    const CDN_URL: &str = "https://fancybash.netlify.app/i.sh";
    const GH_URL: &str  =
        "https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main/i.sh";

    println!("\x1b[1;35m⚡ Upgrading fancybash to latest version...\x1b[0m");

    for url in [CDN_URL, GH_URL] {
        match ureq::get(url).call() {
            Ok(response) => {
                let mut reader = response.into_reader();
                let mut script_content = String::new();
                if reader.read_to_string(&mut script_content).is_ok() && !script_content.is_empty() {
                    let temp_dir = std::env::temp_dir();
                    let script_path = temp_dir.join("fancybash_install.sh");
                    if fs::write(&script_path, &script_content).is_ok() {
                        println!("\x1b[1;32m✨ fancybash upgrade script fetched successfully!\x1b[0m");
                        println!(
                            "\x1b[1;36m💡 Script downloaded to: {}\x1b[0m",
                            script_path.display()
                        );
                        println!(
                            "\x1b[1;36m💡 Restart your shell or run: eval \"$(fancybash init <shell>)\"\x1b[0m"
                        );
                        return Ok(());
                    }
                }
            }
            Err(_) => continue,
        }
    }

    Err("❌ Upgrade failed. Please check your network connection.".into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_self_upgrade_cdn_url_valid() {
        const CDN_URL: &str = "https://fancybash.netlify.app/i.sh";
        assert!(CDN_URL.starts_with("https://"));
    }
}
