use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub fn clean_rc_file() -> std::io::Result<()> {
    let home = match std::env::var("HOME") {
        Ok(val) => val,
        Err(_) => return Ok(()),
    };

    let rc_path = PathBuf::from(&home).join(".zshrc");
    if !rc_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&rc_path)?;
    let start_marker = "# >>> fancy-zshrc >>>";
    let end_marker = "# <<< fancy-zshrc <<<";
    let bad_strings = ["nvm.sh", "bash_completion", "_bun", "bun completions"];

    let mut block = Vec::new();
    let mut others = Vec::new();
    let mut in_block = false;

    for line in content.lines() {
        if line.contains(start_marker) {
            in_block = true;
        }
        
        if in_block {
            block.push(line);
        } else {
            let remove = bad_strings.iter().any(|b| line.contains(b));
            if !remove {
                others.push(line);
            }
        }

        if line.contains(end_marker) {
            in_block = false;
        }
    }

    // Trim trailing empty lines from others
    while let Some(last) = others.last() {
        if last.trim().is_empty() {
            others.pop();
        } else {
            break;
        }
    }

    let mut final_content = others.join("\n");
    if !block.is_empty() {
        if !final_content.is_empty() {
            final_content.push('\n');
            final_content.push('\n');
        }
        final_content.push_str(&block.join("\n"));
    }
    final_content.push('\n'); // Ensure trailing newline

    // Write back atomically using a temp file
    let tmp_path = rc_path.with_extension("tmp");
    fs::write(&tmp_path, final_content)?;
    fs::rename(&tmp_path, &rc_path)?;

    Ok(())
}

/// Inject `eval "$(fancybash init <shell>)"` (or equivalent) into the
/// appropriate shell RC file — idempotent, appends only if not already present.
///
/// Returns `Ok(true)` when the line was freshly injected, `Ok(false)` when it
/// was already there (or the shell is unsupported for auto-inject).
pub fn ensure_init_in_rc(shell: &str) -> std::io::Result<bool> {
    let home = match std::env::var("HOME") {
        Ok(val) => val,
        Err(_) => return Ok(false),
    };

    let (rc_path, eval_line): (PathBuf, String) = match shell.to_ascii_lowercase().as_str() {
        "zsh" => (
            PathBuf::from(&home).join(".zshrc"),
            r#"eval "$(fancybash init zsh)""#.to_string(),
        ),
        "bash" => (
            PathBuf::from(&home).join(".bashrc"),
            r#"eval "$(fancybash init bash)""#.to_string(),
        ),
        "fish" => (
            PathBuf::from(&home).join(".config/fish/config.fish"),
            "fancybash init fish | source".to_string(),
        ),
        // PowerShell profile path varies per platform — skip auto-inject
        _ => return Ok(false),
    };

    // Read existing content (treat missing file as empty)
    let existing = if rc_path.exists() {
        fs::read_to_string(&rc_path)?
    } else {
        String::new()
    };

    // Idempotent check — bail if any fancybash init line is already present
    if existing.contains("fancybash init") {
        return Ok(false);
    }

    // Ensure parent directory exists (e.g. ~/.config/fish/)
    if let Some(parent) = rc_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Append the eval line with a comment so uninstall can strip it
    let injection = format!(
        "\n# fancybash shell initialization\n{eval_line}\n"
    );

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&rc_path)?;

    file.write_all(injection.as_bytes())?;

    Ok(true)
}
