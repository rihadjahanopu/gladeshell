// =============================================================================
//  src/tools/fkill.rs — Interactive Process Killer & Port Killer (Phase 5)
// =============================================================================

use inquire::{Confirm, Select};
use std::fs;
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

    if let Ok(pid_num) = pid_str.parse::<usize>() {
        let target_pid = sysinfo::Pid::from(pid_num);
        if Confirm::new(&format!("Kill process PID {} ({})?", pid_num, selected.trim()))
            .with_default(false)
            .prompt()?
        {
            if let Some(proc) = sys.process(target_pid) {
                proc.kill();
                println!("✅ Process {} terminated.", pid_num);
            } else {
                eprintln!("❌ Failed to locate process {}.", pid_num);
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

    let port_num: u16 = port.parse().map_err(|_| "Invalid port number")?;

    // Native: parse /proc/net/tcp and /proc/net/tcp6 instead of lsof
    let pids = find_pids_by_port(port_num);

    if pids.is_empty() {
        println!("❌ Port {} is not in use.", port);
        return Ok(());
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    println!("⚡ Found process(es) on port {}: {}", port, pids.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", "));
    for pid_num in pids {
        let target_pid = sysinfo::Pid::from(pid_num);
        if let Some(proc) = sys.process(target_pid) {
            proc.kill();
        }
    }
    println!("✅ Successfully killed process(es) on port {}", port);

    Ok(())
}

/// Parse /proc/net/tcp and /proc/net/tcp6 to find PIDs listening on a port.
/// Returns a list of PIDs. No external tools needed.
fn find_pids_by_port(port: u16) -> Vec<usize> {
    let hex_port = format!("{:04X}", port);
    let mut inodes: Vec<u64> = Vec::new();

    for path in &["/proc/net/tcp", "/proc/net/tcp6"] {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines().skip(1) {
                let cols: Vec<&str> = line.split_whitespace().collect();
                // col[1] = local_address "0100007F:1F90", col[9] = inode
                if cols.len() > 9 {
                    let local = cols[1];
                    if local.ends_with(&format!(":{}", hex_port)) {
                        if let Ok(inode) = cols[9].parse::<u64>() {
                            inodes.push(inode);
                        }
                    }
                }
            }
        }
    }

    if inodes.is_empty() {
        return vec![];
    }

    // Walk /proc/<pid>/fd to match inodes to PIDs
    let mut pids = Vec::new();
    if let Ok(proc_dir) = fs::read_dir("/proc") {
        for entry in proc_dir.flatten() {
            let name = entry.file_name();
            let pid_str = name.to_string_lossy();
            if !pid_str.chars().all(|c| c.is_ascii_digit()) { continue; }
            let Ok(pid): Result<usize, _> = pid_str.parse() else { continue };
            let fd_dir = entry.path().join("fd");
            if let Ok(fds) = fs::read_dir(&fd_dir) {
                for fd in fds.flatten() {
                    if let Ok(link) = fs::read_link(fd.path()) {
                        let link_str = link.to_string_lossy();
                        // Link looks like "socket:[12345]"
                        if link_str.starts_with("socket:[") {
                            let inner = &link_str[8..link_str.len()-1];
                            if let Ok(inode) = inner.parse::<u64>() {
                                if inodes.contains(&inode) {
                                    pids.push(pid);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pids
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
