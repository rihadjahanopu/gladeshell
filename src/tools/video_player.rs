// =============================================================================
//  src/tools/video_player.rs — Video Filter & Background Player (`v`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use inquire::Select;

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "mov", "webm", "flv", "m4v"];

fn find_media_player() -> Option<(String, Vec<String>)> {
    // 1. Flatpak VLC
    if let Ok(out) = Command::new("flatpak").arg("list").output() {
        let stdout = String::from_utf8_lossy(&out.stdout);
        if stdout.contains("org.videolan.VLC") {
            return Some((
                "flatpak".into(),
                vec!["run".into(), "org.videolan.VLC".into()],
            ));
        }
    }
    // 2. Native VLC
    if cmd_exists("vlc") {
        return Some(("vlc".into(), vec![]));
    }
    // 3. MPV
    if cmd_exists("mpv") {
        return Some((
            "mpv".into(),
            vec!["--fs".into(), "--no-terminal".into()],
        ));
    }
    // 4. Celluloid
    if cmd_exists("celluloid") {
        return Some(("celluloid".into(), vec![]));
    }
    // 5. Totem
    if cmd_exists("totem") {
        return Some(("totem".into(), vec![]));
    }
    // 6. xdg-open
    if cmd_exists("xdg-open") {
        return Some(("xdg-open".into(), vec![]));
    }

    None
}

fn cmd_exists(name: &str) -> bool {
    crate::core::utils::cmd_exists(name)
}

fn collect_videos_recursive(dir: &Path, acc: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Ignore hidden directories like .git
                if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    if !name.starts_with('.') && name != "node_modules" {
                        collect_videos_recursive(&path, acc);
                    }
                }
            } else if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if VIDEO_EXTENSIONS.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                        acc.push(path);
                    }
                }
            }
        }
    }
}

pub fn run(target: Option<&str>) -> Result<(), Box<dyn Error>> {
    let player_info = match find_media_player() {
        Some(info) => info,
        None => {
            return Err("❌ No video player found. Please install VLC or mpv.".into());
        }
    };

    let target_path = PathBuf::from(target.unwrap_or("."));

    if target_path.is_file() {
        println!("\x1b[1;35m🎬 Playing video:\x1b[0m {}", target_path.display());
        spawn_player(&player_info, &target_path)?;
        return Ok(());
    }

    // Recursively find videos
    let mut videos = Vec::new();
    collect_videos_recursive(&target_path, &mut videos);
    videos.sort();

    if videos.is_empty() {
        println!("\x1b[1;31m❌ No videos found in directory.\x1b[0m");
        return Ok(());
    }

    // Format choices for inquire
    let items: Vec<String> = videos
        .iter()
        .enumerate()
        .map(|(idx, path)| {
            let parent = path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|s| s.to_str())
                .unwrap_or("");
            let file_name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("");

            format!("[{:2}] 📁 {:<16} │ {}", idx + 1, parent, file_name)
        })
        .collect();

    let chosen_item = match Select::new("🔍 Search & Select Video:", items).prompt() {
        Ok(v) => v,
        Err(_) => {
            println!("👋 Exit");
            return Ok(());
        }
    };

    // Find corresponding path index
    let selected_idx: usize = chosen_item
        .split(']')
        .next()
        .and_then(|s| s.trim_start_matches('[').trim().parse().ok())
        .map(|n: usize| n.saturating_sub(1))
        .unwrap_or(0);

    if selected_idx < videos.len() {
        let selected_file = &videos[selected_idx];
        println!("\x1b[1;92m▶ Playing:\x1b[0m {}", selected_file.display());
        spawn_player(&player_info, selected_file)?;
    }

    Ok(())
}

fn spawn_player((bin, base_args): &(String, Vec<String>), file: &Path) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new(bin);
    for arg in base_args {
        cmd.arg(arg);
    }
    cmd.arg(file);

    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_extensions_list() {
        assert!(VIDEO_EXTENSIONS.contains(&"mp4"));
        assert!(VIDEO_EXTENSIONS.contains(&"mkv"));
    }
}
