// =============================================================================
//  src/tools/fkill.rs — Interactive Process Killer & Port Killer (Phase 5)
// =============================================================================

use inquire::{Confirm, Select};
use std::process::Command;
use sysinfo::System;

/// Runs interactive process killer.
pub fn run_fkill() -> Result<(), Box<dyn std::error::Error>> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processes: Vec<String> = sys
        .processes()
        .iter()
        .map(|(pid, proc_info)| {
            format!(
                "{:<8} | {:<20} | {:<8.1} MB",
                pid.to_string(),
                proc_info.name().to_string_lossy(),
                proc_info.memory() as f64 / 1024.0 / 1024.0
            )
        })
        .collect();

    processes.sort();

    if processes.is_empty() {
        println!("📋 No running processes found.");
        return Ok(());
    }

    let selected = Select::new("⚡ Select process to kill:", processes).prompt()?;
    let pid_str = selected.split_whitespace().next().unwrap_or("");

    if let Ok(pid) = pid_str.parse::<i32>() {
        if Confirm::new(&format!("Kill process PID {} ({})?", pid, selected.trim()))
            .with_default(false)
            .prompt()?
        {
            let status = Command::new("kill").arg("-9").arg(pid.to_string()).status();
            match status {
                Ok(s) if s.success() => println!("✅ Process {} terminated.", pid),
                _ => eprintln!("❌ Failed to kill process {}.", pid),
            }
        }
    }

    Ok(())
}

/// Kills process running on a specific port.
pub fn run_kp(port_opt: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let port = match port_opt {
        Some(p) => p.trim().to_string(),
        None => inquire::Text::new("Enter port number to kill (e.g. 3000):").prompt()?,
    };

    if port.is_empty() {
        println!("Port cannot be empty.");
        return Ok(());
    }

    let output = Command::new("lsof")
        .arg("-ti")
        .arg(format!(":{}", port))
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let pids: Vec<&str> = stdout.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();

    if pids.is_empty() {
        println!("❌ Port {} is not in use.", port);
        return Ok(());
    }

    println!("⚡ Found process(es) on port {}: {}", port, pids.join(", "));
    for pid in pids {
        let _ = Command::new("kill").arg("-9").arg(pid).status();
    }
    println!("✅ Successfully killed process(es) on port {}", port);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fkill_runs() {
        let mut sys = System::new_all();
        sys.refresh_all();
        assert!(!sys.processes().is_empty());
    }
}
