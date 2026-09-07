// =============================================================================
//  src/tools/self_upgrade.rs — Self-upgrader for fancybash (`upgrade`)
// =============================================================================

use std::error::Error;
use std::process::Command;

pub fn run() -> Result<(), Box<dyn Error>> {
    const CDN_URL: &str = "https://fancybash.netlify.app/i.sh";
    const GH_URL: &str  =
        "https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main/i.sh";

    println!("\x1b[1;35m⚡ Upgrading fancybash to latest version...\x1b[0m");

    for url in [CDN_URL, GH_URL] {
        let curl = Command::new("curl")
            .args(["-fsSL", url])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn();

        if let Ok(mut curl_child) = curl {
            let curl_stdout = curl_child.stdout.take().expect("curl stdout");
            let bash = Command::new("bash")
                .stdin(curl_stdout)
                .status();

            let _ = curl_child.wait();

            match bash {
                Ok(s) if s.success() => {
                    println!("\x1b[1;32m✨ fancybash upgraded successfully!\x1b[0m");
                    println!(
                        "\x1b[1;36m💡 Restart your shell or run: eval \"$(fancybash init <shell>)\"\x1b[0m"
                    );
                    return Ok(());
                }
                _ => {
                    continue;
                }
            }
        }
    }

    Err("❌ Upgrade failed. Please check your network connection.".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_upgrade_cdn_url_valid() {
        const CDN_URL: &str = "https://fancybash.netlify.app/i.sh";
        assert!(CDN_URL.starts_with("https://"));
    }
}
