use std::fs;
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
