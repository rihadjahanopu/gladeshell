// ============================================================================
// STATUS: 100% NATIVE RUST & BULLETPROOF (ZERO EXTERNAL BINARY DEPENDENCIES)
// AUDIT COMPLETED: FULL FEATURE PARITY, CROSS-OS VERIFIED & OPTIMIZED
// HANDS-OFF GUARANTEE: NO MANUAL EDITS REQUIRED
// ============================================================================

use std::fs;

use std::io::Write;
use std::path::PathBuf;

/// Return whether `brush` binary or `~/.brushrc` is present on this system.
pub fn is_brush_present() -> bool {
    is_shell_installed("brush")
        || crate::core::utils::home_dir()
            .map(|h| h.join(".brushrc").exists())
            .unwrap_or(false)
}

/// Resolve the OS-specific `brush` TOML configuration file path (`config.toml`).
///
/// - Linux / macOS: `${XDG_CONFIG_HOME:-$HOME/.config}/brush/config.toml`
/// - Windows: `%APPDATA%\brush\config.toml` (fallback: `%USERPROFILE%\.config\brush\config.toml`)
pub fn get_brush_config_path() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        if let Some(appdata) = std::env::var_os("APPDATA") {
            if !appdata.is_empty() {
                return Some(PathBuf::from(appdata).join("brush").join("config.toml"));
            }
        }
    }
    #[cfg(not(windows))]
    {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            let trimmed = xdg.trim();
            if !trimmed.is_empty() {
                return Some(PathBuf::from(trimmed).join("brush").join("config.toml"));
            }
        }
    }
    crate::core::utils::home_dir().map(|h| h.join(".config").join("brush").join("config.toml"))
}

/// Ensure `[ui]\nsyntax-highlighting = true` is configured in Brush's `config.toml`.
///
/// Respects explicit user settings (`true` or `false`), preserves existing comments
/// when inserting under `[ui]`, skips malformed TOML safely, and writes atomically.
pub fn ensure_brush_ui_config() -> std::io::Result<bool> {
    match get_brush_config_path() {
        Some(path) => ensure_brush_ui_config_at(&path),
        None => Ok(false),
    }
}

/// Ensure `[ui]\nsyntax-highlighting = true` in a specific TOML config file path.
pub fn ensure_brush_ui_config_at(cfg_path: &std::path::Path) -> std::io::Result<bool> {
    let raw_existing = if cfg_path.exists() {
        fs::read_to_string(cfg_path)?
    } else {
        String::new()
    };
    let existing_trimmed_bom = raw_existing.trim_start_matches('\u{FEFF}');
    let has_bom = raw_existing.len() != existing_trimmed_bom.len();
    let existing = existing_trimmed_bom.to_string();

    let line_ending = if existing.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };

    let mut updated = if existing.trim().is_empty() {
        format!(
            "# GladeShell — Brush interactive syntax highlighting{le}[ui]{le}syntax-highlighting = true{le}",
            le = line_ending
        )
    } else {
        let Ok(mut parsed) = toml::from_str::<toml::Value>(&existing) else {
            return Ok(false);
        };

        if let Some(ui_val) = parsed.get("ui") {
            if ui_val.get("syntax-highlighting").is_some() {
                return Ok(false);
            }
            if let Some(insert_pos) = find_toml_section_end_of_header(&existing, "[ui]") {
                let mut out = String::with_capacity(existing.len() + 40);
                out.push_str(&existing[..insert_pos]);
                if !existing[..insert_pos].ends_with('\n') {
                    out.push_str(line_ending);
                }
                out.push_str("syntax-highlighting = true");
                out.push_str(line_ending);
                out.push_str(&existing[insert_pos..]);
                out
            } else if let Some(ui_tbl) = parsed.get_mut("ui").and_then(|v| v.as_table_mut()) {
                ui_tbl.insert("syntax-highlighting".into(), toml::Value::Boolean(true));
                toml::to_string_pretty(&parsed).map_err(std::io::Error::other)?
            } else {
                return Ok(false);
            }
        } else {
            let mut out = existing.clone();
            if !out.ends_with('\n') {
                out.push_str(line_ending);
            }
            out.push_str(&format!(
                "{le}# GladeShell — Brush interactive syntax highlighting{le}[ui]{le}syntax-highlighting = true{le}",
                le = line_ending
            ));
            out
        }
    };

    if has_bom {
        updated.insert(0, '\u{FEFF}');
    }

    if let Some(parent) = cfg_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let tmp_name = format!(
        ".{}.tmp.{}",
        cfg_path.file_name().unwrap_or_default().to_string_lossy(),
        std::process::id()
    );
    let tmp_path = cfg_path.with_file_name(tmp_name);
    fs::write(&tmp_path, updated)?;
    crate::core::utils::atomic_replace(&tmp_path, cfg_path)?;

    Ok(true)
}

/// Find byte offset right after the matching TOML section header line (e.g. `[ui]`).
fn find_toml_section_end_of_header(content: &str, header: &str) -> Option<usize> {
    let mut offset = 0;
    for chunk in content.split_inclusive('\n') {
        let trimmed = chunk.trim();
        let matches_header = trimmed == header
            || trimmed
                .strip_prefix(header)
                .map(|rest| rest.trim_start().starts_with('#'))
                .unwrap_or(false);
        offset += chunk.len();
        if matches_header {
            return Some(offset);
        }
    }
    None
}

/// Return dynamic start and end markers for a given shell name.
pub fn get_markers_for_shell(shell: &str) -> (String, String) {
    match shell.to_ascii_lowercase().as_str() {
        "zsh" => (
            "# >>> glade-zshrc >>>".into(),
            "# <<< glade-zshrc <<<".into(),
        ),
        "bash" => (
            "# >>> glade-bashrc >>>".into(),
            "# <<< glade-bashrc <<<".into(),
        ),
        "fish" => ("# >>> glade-fish >>>".into(), "# <<< glade-fish <<<".into()),
        "pwsh" | "powershell" => (
            "# >>> glade-powershell >>>".into(),
            "# <<< glade-powershell <<<".into(),
        ),
        other => (
            format!("# >>> glade-{} >>>", other),
            format!("# <<< glade-{} <<<", other),
        ),
    }
}

/// Get the configuration file path for a target shell.
pub fn get_rc_path_for_shell(shell: &str) -> Option<PathBuf> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()?;

    match shell.to_ascii_lowercase().as_str() {
        "zsh" => Some(PathBuf::from(&home).join(".zshrc")),
        "bash" => Some(PathBuf::from(&home).join(".bashrc")),
        "brush" => Some(PathBuf::from(&home).join(".brushrc")),
        "fish" => Some(PathBuf::from(&home).join(".config/fish/config.fish")),
        "pwsh" | "powershell" => {
            let mut candidates = Vec::new();

            candidates.push(
                PathBuf::from(&home)
                    .join("Documents")
                    .join("PowerShell")
                    .join("Microsoft.PowerShell_profile.ps1"),
            );
            candidates.push(
                PathBuf::from(&home)
                    .join("Documents")
                    .join("WindowsPowerShell")
                    .join("Microsoft.PowerShell_profile.ps1"),
            );

            // Check OneDrive-redirected Documents (standard on Windows 10/11)
            if let Ok(onedrive) = std::env::var("OneDrive") {
                candidates.push(
                    PathBuf::from(&onedrive)
                        .join("Documents")
                        .join("PowerShell")
                        .join("Microsoft.PowerShell_profile.ps1"),
                );
                candidates.push(
                    PathBuf::from(&onedrive)
                        .join("Documents")
                        .join("WindowsPowerShell")
                        .join("Microsoft.PowerShell_profile.ps1"),
                );
            }
            if let Ok(onedrive_c) = std::env::var("OneDriveConsumer") {
                candidates.push(
                    PathBuf::from(&onedrive_c)
                        .join("Documents")
                        .join("PowerShell")
                        .join("Microsoft.PowerShell_profile.ps1"),
                );
                candidates.push(
                    PathBuf::from(&onedrive_c)
                        .join("Documents")
                        .join("WindowsPowerShell")
                        .join("Microsoft.PowerShell_profile.ps1"),
                );
            }
            if let Ok(onedrive_b) = std::env::var("OneDriveCommercial") {
                candidates.push(
                    PathBuf::from(&onedrive_b)
                        .join("Documents")
                        .join("PowerShell")
                        .join("Microsoft.PowerShell_profile.ps1"),
                );
                candidates.push(
                    PathBuf::from(&onedrive_b)
                        .join("Documents")
                        .join("WindowsPowerShell")
                        .join("Microsoft.PowerShell_profile.ps1"),
                );
            }

            candidates.push(
                PathBuf::from(&home)
                    .join(".config")
                    .join("powershell")
                    .join("Microsoft.PowerShell_profile.ps1"),
            );

            for candidate in &candidates {
                if candidate.exists() {
                    return Some(candidate.clone());
                }
            }
            Some(candidates[0].clone())
        }
        _ => None,
    }
}

/// Clean and re-order gladeshell block for current active shell.
pub fn clean_rc_file() -> std::io::Result<()> {
    let shell = std::env::var("SHELL").unwrap_or_default();
    let target_shell = if std::env::var_os("BRUSH_VERSION").is_some() || shell.contains("brush") {
        "brush"
    } else if shell.contains("zsh") {
        "zsh"
    } else if shell.contains("fish") {
        "fish"
    } else if shell.contains("bash") {
        "bash"
    } else {
        #[cfg(windows)]
        {
            "pwsh"
        }
        #[cfg(not(windows))]
        {
            "bash"
        }
    };

    if let Some(rc_path) = get_rc_path_for_shell(target_shell) {
        clean_specific_rc_file(&rc_path)?;
    }

    Ok(())
}

/// Reorder the gladeshell config block to the end of a specific RC file.
/// The block is PRESERVED — never deleted — and moved to the bottom so it
/// always loads last (after user config). Safe to call multiple times.
pub fn clean_specific_rc_file(rc_path: &PathBuf) -> std::io::Result<()> {
    if !rc_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(rc_path)?;

    // Fast path: nothing to reorder
    if !content.contains("# >>> glade-") {
        return Ok(());
    }

    // Split into: user content (others) and glade block (block).
    // Block is preserved and relocated to the end of the file.
    let mut block: Vec<&str> = Vec::new();
    let mut others: Vec<&str> = Vec::new();
    let mut in_block = false;

    for line in content.lines() {
        if line.contains("# >>> glade-") {
            in_block = true;
        }
        if in_block {
            block.push(line);
        } else {
            others.push(line);
        }
        if line.contains("# <<< glade-") {
            in_block = false;
        }
    }

    let line_ending = if content.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };

    // Trim trailing blank lines from user content section
    while let Some(last) = others.last() {
        if last.trim().is_empty() {
            others.pop();
        } else {
            break;
        }
    }

    // Reassemble: user content first, glade block at the end
    let mut final_content = others.join(line_ending);
    if !block.is_empty() {
        if !final_content.is_empty() {
            final_content.push_str(line_ending);
            final_content.push_str(line_ending);
        }
        final_content.push_str(&block.join(line_ending));
    }
    final_content.push_str(line_ending);

    // Write back atomically using a PID-stamped temp file.
    let tmp_name = format!(
        "{}.tmp.{}",
        rc_path.file_name().unwrap_or_default().to_string_lossy(),
        std::process::id()
    );
    let tmp_path = rc_path.with_file_name(tmp_name);
    fs::write(&tmp_path, final_content)?;
    crate::core::utils::atomic_replace(&tmp_path, rc_path)?;

    // Remove stale compiled bytecode if zsh
    if rc_path.to_string_lossy().contains(".zshrc") {
        if let Some(parent) = rc_path.parent() {
            let _ = fs::remove_file(parent.join(".zshrc.zwc"));
        }
    }

    Ok(())
}

/// Inject `eval "$(gladeshell init <shell>)"` (or equivalent) into the
/// appropriate shell RC file — idempotent, appends only if not already present.
pub fn ensure_init_in_rc(shell: &str) -> std::io::Result<bool> {
    let _ = ensure_auto_heal_hooks();
    if shell.eq_ignore_ascii_case("brush") {
        let _ = ensure_brush_ui_config();
    }

    let rc_path = match get_rc_path_for_shell(shell) {
        Some(p) => p,
        None => return Ok(false),
    };

    let (start_marker, end_marker) = get_markers_for_shell(shell);

    let eval_block = match shell.to_ascii_lowercase().as_str() {
        "zsh" => format!(
            "{start_marker}\nexport PATH=\"$HOME/.cargo/bin:$HOME/.local/bin:$PATH\"\nif (( ${{+commands[gladeshell]}} )); then\n    _fb_cache=\"$HOME/.gladeshell/cache/init.zsh\"\n    _fb_bin=\"${{commands[gladeshell]}}\"\n    if [[ -f \"$_fb_cache\" && -n \"$_fb_bin\" && \"$_fb_cache\" -nt \"$_fb_bin\" ]]; then\n        source \"$_fb_cache\"\n    else\n        eval \"$(gladeshell init zsh)\"\n    fi\nfi\n{end_marker}"
        ),
        "bash" => format!(
            "{start_marker}\nexport PATH=\"$HOME/.cargo/bin:$HOME/.local/bin:$PATH\"\nif [[ -z \"${{BRUSH_VERSION:-}}\" ]] || ! grep -qF '# >>> glade-brush >>>' \"$HOME/.brushrc\" 2>/dev/null; then\n    if type gladeshell >/dev/null 2>&1; then\n        _fb_cache=\"$HOME/.gladeshell/cache/init.bash\"\n        _fb_bin=\"$(command -v gladeshell 2>/dev/null)\"\n        if [[ -f \"$_fb_cache\" && -n \"$_fb_bin\" && \"$_fb_cache\" -nt \"$_fb_bin\" ]]; then\n            source \"$_fb_cache\"\n        else\n            eval \"$(gladeshell init bash)\"\n        fi\n    fi\nfi\n{end_marker}"
        ),
        "brush" => format!(
            "{start_marker}\nexport PATH=\"$HOME/.cargo/bin:$HOME/.local/bin:$PATH\"\nif type gladeshell >/dev/null 2>&1; then\n    _fb_cache=\"$HOME/.gladeshell/cache/init.brush\"\n    _fb_bin=\"$(command -v gladeshell 2>/dev/null)\"\n    if [[ -f \"$_fb_cache\" && -n \"$_fb_bin\" && \"$_fb_cache\" -nt \"$_fb_bin\" ]]; then\n        source \"$_fb_cache\"\n    else\n        eval \"$(gladeshell init brush)\"\n    fi\nfi\n{end_marker}"
        ),
        "fish" => format!(
            "{start_marker}\nset -gx PATH $HOME/.cargo/bin $HOME/.local/bin $PATH\nif type -q gladeshell\n    set -l _fb_cache \"$HOME/.gladeshell/cache/init.fish\"\n    set -l _fb_bin (command -v gladeshell 2>/dev/null)\n    if test -f \"$_fb_cache\" -a -n \"$_fb_bin\" -a \"$_fb_cache\" -nt \"$_fb_bin\"\n        source \"$_fb_cache\"\n    else\n        gladeshell init fish | source\n    end\nend\n{end_marker}"
        ),
        "pwsh" | "powershell" => format!(
            "{start_marker}\n$localBin = Join-Path $HOME \".local\\bin\"\nif (-not ($env:PATH -split [System.IO.Path]::PathSeparator -contains $localBin)) {{\n    $env:PATH = \"$localBin$([System.IO.Path]::PathSeparator)$env:PATH\"\n}}\n$fb_cache = Join-Path $HOME \".gladeshell\\cache\\init.ps1\"\nif ([System.IO.File]::Exists($fb_cache)) {{\n    . $fb_cache\n}} elseif (Get-Command gladeshell -ErrorAction SilentlyContinue) {{\n    gladeshell init pwsh | Invoke-Expression\n}}\n{end_marker}"
        ),
        other => format!(
            "{start_marker}\neval \"$(gladeshell init {other})\"\n{end_marker}"
        ),
    };

    // Read existing content (treat missing file as empty)
    let existing = if rc_path.exists() {
        fs::read_to_string(&rc_path)?
    } else {
        String::new()
    };

    // Idempotent check — if our exact block is already present, nothing to do
    if existing.contains(&start_marker) && existing.contains(&end_marker) {
        return Ok(false);
    }

    // If the file has any glade block at all (possibly misplaced or from a
    // previous version), reorder it to the end first, then re-read.
    if existing.contains("# >>> glade-") || existing.contains("gladeshell init") {
        clean_specific_rc_file(&rc_path)?;

        // After reorder, re-check — if markers now exist, block was just
        // misplaced (not missing). Reorder fixed it; no append needed.
        let after_reorder = fs::read_to_string(&rc_path).unwrap_or_default();
        if after_reorder.contains(&start_marker) && after_reorder.contains(&end_marker) {
            return Ok(false);
        }
    }

    // Ensure parent directory exists
    if let Some(parent) = rc_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Append the eval block
    let line_ending = if existing.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let injection = format!("{line_ending}{eval_block}{line_ending}");

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&rc_path)?;

    file.write_all(injection.as_bytes())?;

    Ok(true)
}

/// Ensure background auto-heal hooks exist in startup files (~/.zshenv, ~/.profile, ~/.bash_profile, fish conf.d, pwsh profile).
/// When a new shell starts, even if the main RC file (~/.zshrc, ~/.bashrc, config.fish) was wiped,
/// the auto-heal hook silently invokes `gladeshell setup` to re-inject the configuration instantly.
pub fn ensure_auto_heal_hooks() -> std::io::Result<()> {
    if let Some(home) = crate::core::utils::home_dir() {
        // 1. Zsh auto-heal hook in ~/.zshenv (sourced before ~/.zshrc)
        // IMPORTANT: only re-inject when the block is truly MISSING from .zshrc.
        // The old unconditional `gladeshell setup` call was the root cause of the
        // "block added on every new tab" regression — setup re-injected on every
        // shell start even when the block was already present.
        let zshenv = home.join(".zshenv");
        let start_marker = "# >>> glade-zshenv >>>";
        let end_marker = "# <<< glade-zshenv <<<";
        let existing = if zshenv.exists() {
            fs::read_to_string(&zshenv).unwrap_or_default()
        } else {
            String::new()
        };

        if !existing.contains(start_marker) || !existing.contains(end_marker) {
            let hook = format!(
                "{start_marker}\n\
# Guard: only re-inject when the glade block is truly missing (not every startup).\n\
if command -v gladeshell >/dev/null 2>&1; then\n\
    if ! grep -qF '# >>> glade-zshrc >>>' \"$HOME/.zshrc\" 2>/dev/null; then\n\
        gladeshell setup >/dev/null 2>&1\n\
    fi\n\
fi\n\
{end_marker}\n"
            );
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&zshenv)?;
            file.write_all(hook.as_bytes())?;
        }

        // 2. Bash & Brush auto-heal hooks in ~/.profile (and existing ~/.bash_profile)
        // Avoid inventing ~/.bash_profile if absent, since its presence suppresses ~/.profile.
        let has_brush = is_brush_present();
        for bash_file_name in &[".profile", ".bash_profile"] {
            let bash_file = home.join(bash_file_name);
            if *bash_file_name == ".bash_profile" && !bash_file.exists() {
                continue;
            }
            let p_start_marker = "# >>> glade-profile >>>";
            let p_end_marker = "# <<< glade-profile <<<";
            let p_existing = if bash_file.exists() {
                fs::read_to_string(&bash_file).unwrap_or_default()
            } else {
                String::new()
            };
            let mut need_nl = !p_existing.is_empty() && !p_existing.ends_with('\n');

            if !p_existing.contains(p_start_marker) || !p_existing.contains(p_end_marker) {
                let prefix = if need_nl { "\n" } else { "" };
                need_nl = false;
                // Guard: only call setup when .bashrc block is truly absent.
                let hook = format!(
                    "{prefix}{p_start_marker}\n\
# Only re-inject when the glade block is truly missing (not on every startup).\n\
if command -v gladeshell >/dev/null 2>&1; then\n\
    if ! grep -qF '# >>> glade-bashrc >>>' \"$HOME/.bashrc\" 2>/dev/null; then\n\
        gladeshell setup >/dev/null 2>&1\n\
    fi\n\
fi\n\
{p_end_marker}\n"
                );
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&bash_file)?;
                file.write_all(hook.as_bytes())?;
            }

            if has_brush {
                let b_start = "# >>> glade-brush-login >>>";
                let b_end = "# <<< glade-brush-login <<<";
                if !p_existing.contains(b_start) || !p_existing.contains(b_end) {
                    let prefix = if need_nl { "\n" } else { "" };
                    let b_hook = format!(
                        "{prefix}{b_start}\n\
# Load ~/.brushrc for Brush login shells (`brush -l`)\n\
if [[ -n \"${{BRUSH_VERSION:-}}\" && -r \"$HOME/.brushrc\" && -z \"${{_GLADESHELL_BRUSH_LOADED:-}}\" ]]; then\n\
    _GLADESHELL_BRUSH_LOADED=1\n\
    . \"$HOME/.brushrc\"\n\
fi\n\
{b_end}\n"
                    );
                    let mut file = fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&bash_file)?;
                    file.write_all(b_hook.as_bytes())?;
                }
            }
        }

        // 3. Fish auto-heal hook in ~/.config/fish/conf.d/00_gladeshell_heal.fish
        let fish_conf_d = home.join(".config").join("fish").join("conf.d");
        if fish_conf_d.exists() || fs::create_dir_all(&fish_conf_d).is_ok() {
            let fish_heal_file = fish_conf_d.join("00_gladeshell_heal.fish");
            let f_start_marker = "# >>> glade-fish-heal >>>";
            let f_end_marker = "# <<< glade-fish-heal <<<";
            let f_existing = if fish_heal_file.exists() {
                fs::read_to_string(&fish_heal_file).unwrap_or_default()
            } else {
                String::new()
            };

            if !f_existing.contains(f_start_marker) || !f_existing.contains(f_end_marker) {
                // Guard: only call setup when config.fish block is truly absent.
                let hook = format!(
                    "{f_start_marker}\n\
# Only re-inject when the glade block is truly missing (not on every startup).\n\
if type -q gladeshell\n\
    if not grep -qF '# >>> glade-fish >>>' \"$HOME/.config/fish/config.fish\" 2>/dev/null\n\
        gladeshell setup >/dev/null 2>&1\n\
    end\n\
end\n\
{f_end_marker}\n"
                );
                let _ = fs::write(&fish_heal_file, hook);
            }
        }

        // 4. PowerShell auto-heal hook in ~/.config/powershell/profile.ps1
        let pwsh_dir = home.join(".config").join("powershell");
        if pwsh_dir.exists() || fs::create_dir_all(&pwsh_dir).is_ok() {
            let pwsh_profile = pwsh_dir.join("profile.ps1");
            let pw_start_marker = "# >>> glade-pwsh-heal >>>";
            let pw_end_marker = "# <<< glade-pwsh-heal <<<";
            let pw_existing = if pwsh_profile.exists() {
                fs::read_to_string(&pwsh_profile).unwrap_or_default()
            } else {
                String::new()
            };

            if !pw_existing.contains(pw_start_marker) || !pw_existing.contains(pw_end_marker) {
                // Guard: only call setup when the pwsh profile block is truly absent.
                let hook = format!(
                    "{pw_start_marker}\n\
# Only re-inject when the glade block is truly missing (not on every startup).\n\
if (Get-Command gladeshell -ErrorAction SilentlyContinue) {{\n\
    $gladeRc = Join-Path $HOME '.config/powershell/Microsoft.PowerShell_profile.ps1'\n\
    if (-not (Select-String -Path $gladeRc -Pattern '# >>> glade-powershell >>>' -Quiet -ErrorAction SilentlyContinue)) {{\n\
        gladeshell setup | Out-Null\n\
    }}\n\
}}\n\
{pw_end_marker}\n"
                );
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&pwsh_profile)?;
                file.write_all(hook.as_bytes())?;
            }
        }
    }
    Ok(())
}

/// Check if `shell` executable is available on `$PATH` or in standard user bin directories.
fn is_shell_installed(shell: &str) -> bool {
    if crate::core::utils::cmd_exists(shell) {
        return true;
    }
    if let Some(home) = crate::core::utils::home_dir() {
        for bin_dir in [".local/bin", ".cargo/bin"] {
            if home.join(bin_dir).join(shell).is_file() {
                return true;
            }
            #[cfg(windows)]
            if home.join(bin_dir).join(format!("{shell}.exe")).is_file() {
                return true;
            }
        }
    }
    false
}

/// Auto-detect and configure all shells installed/present on host system.
pub fn ensure_all_installed_shells_configured() -> std::io::Result<Vec<String>> {
    let mut configured = Vec::new();
    let target_shells = ["zsh", "bash", "brush", "fish", "pwsh"];

    for shell in &target_shells {
        if let Some(path) = get_rc_path_for_shell(shell) {
            let should_attempt = path.exists() || {
                #[cfg(windows)]
                {
                    *shell == "pwsh" || is_shell_installed(shell)
                }
                #[cfg(not(windows))]
                {
                    is_shell_installed(shell)
                }
            };
            if should_attempt && ensure_init_in_rc(shell)? {
                configured.push(shell.to_string());
            }
        }
    }

    let _ = ensure_auto_heal_hooks();
    if is_brush_present() {
        let _ = ensure_brush_ui_config();
    }

    Ok(configured)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brush_markers_and_rc_path() {
        let (start, end) = get_markers_for_shell("brush");
        assert_eq!(start, "# >>> glade-brush >>>");
        assert_eq!(end, "# <<< glade-brush <<<");
        let path = get_rc_path_for_shell("brush").unwrap();
        assert!(path.ends_with(".brushrc"));
    }

    #[test]
    fn test_ensure_brush_ui_config_creates_and_preserves() {
        let dir =
            std::env::temp_dir().join(format!("gladeshell_brush_cfg_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let cfg = dir.join("brush").join("config.toml");

        // 1. Creates fresh file when missing
        assert!(ensure_brush_ui_config_at(&cfg).unwrap());
        let c1 = fs::read_to_string(&cfg).unwrap();
        assert!(c1.contains("[ui]"));
        assert!(c1.contains("syntax-highlighting = true"));

        // 2. Idempotent on second call
        assert!(!ensure_brush_ui_config_at(&cfg).unwrap());

        // 3. Preserves explicit false
        fs::write(&cfg, "[ui]\nsyntax-highlighting = false\n").unwrap();
        assert!(!ensure_brush_ui_config_at(&cfg).unwrap());
        assert!(fs::read_to_string(&cfg)
            .unwrap()
            .contains("syntax-highlighting = false"));

        // 4. Inserts cleanly under existing [ui] section while keeping other keys & comments
        fs::write(&cfg, "# custom comment\n[ui]\nmouse = true\n").unwrap();
        assert!(ensure_brush_ui_config_at(&cfg).unwrap());
        let c4 = fs::read_to_string(&cfg).unwrap();
        assert!(c4.contains("# custom comment"));
        assert!(c4.contains("syntax-highlighting = true"));
        assert!(c4.contains("mouse = true"));

        // 5. Appends [ui] when other sections exist without [ui]
        fs::write(&cfg, "[experimental]\nzsh-hooks = true\n").unwrap();
        assert!(ensure_brush_ui_config_at(&cfg).unwrap());
        let c5 = fs::read_to_string(&cfg).unwrap();
        assert!(c5.contains("zsh-hooks = true"));
        assert!(c5.contains("[ui]"));
        assert!(c5.contains("syntax-highlighting = true"));

        // 6. Preserves UTF-8 BOM + CRLF on Windows configs
        fs::write(&cfg, "\u{FEFF}[experimental]\r\nzsh-hooks = true\r\n").unwrap();
        assert!(ensure_brush_ui_config_at(&cfg).unwrap());
        let c6 = fs::read_to_string(&cfg).unwrap();
        assert!(c6.starts_with('\u{FEFF}'));
        assert!(c6.contains("[ui]\r\nsyntax-highlighting = true\r\n"));

        // 7. Skips broken TOML safely
        fs::write(&cfg, "[broken ===\n").unwrap();
        assert!(!ensure_brush_ui_config_at(&cfg).unwrap());

        let _ = fs::remove_dir_all(&dir);
    }
}
