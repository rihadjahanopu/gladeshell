// =============================================================================
//  src/tools/dman.rs — Interactive Docker TUI Manager (Phase 5)
// =============================================================================

use inquire::Select;
use std::process::Command;

const DMAN_MENU: &[&str] = &[
    "1. 📦 Containers (List, Start, Stop, Logs, Exec)",
    "2. 🖼️ Images (List, Pull, Remove, Prune)",
    "3. 💾 Volumes (List, Create, Remove, Prune)",
    "4. 🌐 Networks (List, Create, Remove, Inspect)",
    "5. 🐙 Docker Compose (Up, Down, Logs, PS)",
    "6. 📊 System Stats & Resource Monitor",
    "7. 🧹 System Clean & Prune",
    "❌ Exit",
];

/// Runs the interactive Docker TUI manager.
pub fn run(action_opt: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    if !is_docker_installed() {
        return Err("docker command not found. Please install Docker to use dman.".into());
    }

    let action = match action_opt {
        Some(a) => a.to_string(),
        None => match Select::new("🐳 DOCKER INTERACTIVE MANAGER (dman)", DMAN_MENU.to_vec()).prompt() {
            Ok(selected) => selected.to_string(),
            Err(_) => return Ok(()),
        },
    };

    if action.contains("Exit") {
        return Ok(());
    }

    if action.contains("Containers") || action.starts_with("1.") {
        manage_containers()?;
    } else if action.contains("Images") || action.starts_with("2.") {
        manage_images()?;
    } else if action.contains("Volumes") || action.starts_with("3.") {
        manage_volumes()?;
    } else if action.contains("Networks") || action.starts_with("4.") {
        manage_networks()?;
    } else if action.contains("Compose") || action.starts_with("5.") {
        manage_compose()?;
    } else if action.contains("Stats") || action.starts_with("6.") {
        Command::new("docker").arg("stats").status()?;
    } else if action.contains("Clean") || action.starts_with("7.") {
        Command::new("docker").arg("system").arg("prune").arg("-a").arg("-f").status()?;
        println!("✅ Docker system pruned cleanly!");
    }

    Ok(())
}

fn manage_containers() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("docker")
        .arg("ps")
        .arg("-a")
        .arg("--format")
        .arg("{{.ID}}\t{{.Names}}\t{{.Status}}\t{{.Image}}")
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.is_empty() {
        println!("📦 No containers found.");
        return Ok(());
    }

    let selected = Select::new("Select Container:", lines).prompt()?;
    let container_id = selected.split_whitespace().next().unwrap_or("");

    let act = Select::new(
        &format!("Action for container '{}':", container_id),
        vec!["Start", "Stop", "Restart", "Logs", "Exec Bash", "Remove"],
    )
    .prompt()?;

    match act {
        "Start" => { Command::new("docker").arg("start").arg(container_id).status()?; }
        "Stop" => { Command::new("docker").arg("stop").arg(container_id).status()?; }
        "Restart" => { Command::new("docker").arg("restart").arg(container_id).status()?; }
        "Logs" => { Command::new("docker").arg("logs").arg("-f").arg(container_id).status()?; }
        "Exec Bash" => { Command::new("docker").arg("exec").arg("-it").arg(container_id).arg("bash").status()?; }
        "Remove" => { Command::new("docker").arg("rm").arg("-f").arg(container_id).status()?; }
        _ => {}
    }

    Ok(())
}

fn manage_images() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("docker")
        .arg("images")
        .arg("--format")
        .arg("{{.Repository}}:{{.Tag}}\t{{.ID}}\t{{.Size}}")
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.is_empty() {
        println!("🖼️ No images found.");
        return Ok(());
    }

    let selected = Select::new("Select Image:", lines).prompt()?;
    let image_id = selected.split_whitespace().nth(1).unwrap_or("");

    let act = Select::new(&format!("Action for image '{}':", image_id), vec!["Remove Image", "Inspect", "History"]).prompt()?;

    match act {
        "Remove Image" => { Command::new("docker").arg("rmi").arg("-f").arg(image_id).status()?; }
        "Inspect" => { Command::new("docker").arg("image").arg("inspect").arg(image_id).status()?; }
        "History" => { Command::new("docker").arg("history").arg(image_id).status()?; }
        _ => {}
    }

    Ok(())
}

fn manage_volumes() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("docker").arg("volume").arg("ls").arg("--format").arg("{{.Name}}\t{{.Driver}}").output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.is_empty() {
        println!("💾 No volumes found.");
        return Ok(());
    }

    let selected = Select::new("Select Volume:", lines).prompt()?;
    let vol_name = selected.split_whitespace().next().unwrap_or("");

    let act = Select::new(&format!("Action for volume '{}':", vol_name), vec!["Inspect", "Remove Volume"]).prompt()?;
    match act {
        "Inspect" => { Command::new("docker").arg("volume").arg("inspect").arg(vol_name).status()?; }
        "Remove Volume" => { Command::new("docker").arg("volume").arg("rm").arg(vol_name).status()?; }
        _ => {}
    }

    Ok(())
}

fn manage_networks() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("docker").arg("network").arg("ls").arg("--format").arg("{{.ID}}\t{{.Name}}\t{{.Driver}}").output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.is_empty() {
        println!("🌐 No networks found.");
        return Ok(());
    }

    let selected = Select::new("Select Network:", lines).prompt()?;
    let net_id = selected.split_whitespace().next().unwrap_or("");

    let act = Select::new(&format!("Action for network '{}':", net_id), vec!["Inspect", "Remove Network"]).prompt()?;
    match act {
        "Inspect" => { Command::new("docker").arg("network").arg("inspect").arg(net_id).status()?; }
        "Remove Network" => { Command::new("docker").arg("network").arg("rm").arg(net_id).status()?; }
        _ => {}
    }

    Ok(())
}

fn manage_compose() -> Result<(), Box<dyn std::error::Error>> {
    let act = Select::new("Docker Compose Action:", vec!["docker compose up -d", "docker compose down", "docker compose ps", "docker compose logs -f"]).prompt()?;
    let parts: Vec<&str> = act.split_whitespace().collect();
    if parts.len() >= 3 {
        Command::new(parts[0]).args(&parts[1..]).status()?;
    }
    Ok(())
}

fn is_docker_installed() -> bool {
    crate::core::utils::cmd_exists("docker")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dman_menu_non_empty() {
        assert!(!DMAN_MENU.is_empty());
    }
}
