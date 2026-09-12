// =============================================================================
//  src/tools/video_player.rs — Video Filter & Background Player (`v`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::io::stderr;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};

const VIDEO_EXTENSIONS: &[&str] = &["mp4", "mkv", "avi", "mov", "webm", "flv", "m4v"];

fn find_media_player() -> Option<(String, Vec<String>)> {
    if let Ok(out) = Command::new("flatpak").arg("list").output() {
        let stdout = String::from_utf8_lossy(&out.stdout);
        if stdout.contains("org.videolan.VLC") {
            return Some(("flatpak".into(), vec!["run".into(), "org.videolan.VLC".into()]));
        }
    }
    if cmd_exists("vlc") {
        return Some(("vlc".into(), vec![]));
    }
    if cmd_exists("mpv") {
        return Some(("mpv".into(), vec!["--fs".into(), "--no-terminal".into()]));
    }
    if cmd_exists("celluloid") {
        return Some(("celluloid".into(), vec![]));
    }
    if cmd_exists("totem") {
        return Some(("totem".into(), vec![]));
    }
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

    let mut videos = Vec::new();
    collect_videos_recursive(&target_path, &mut videos);
    videos.sort();

    if videos.is_empty() {
        println!("\x1b[1;31m❌ No videos found in directory.\x1b[0m");
        return Ok(());
    }

    if videos.len() == 1 {
        println!("\x1b[1;92m▶ Playing:\x1b[0m {}", videos[0].display());
        spawn_player(&player_info, &videos[0])?;
        return Ok(());
    }

    let player_display_name = match player_info.0.as_str() {
        "vlc" => "VLC Media Player",
        "mpv" => "MPV Video Player",
        "flatpak" => "VLC (Flatpak)",
        "celluloid" => "Celluloid Player",
        "totem" => "GNOME Videos (Totem)",
        "xdg-open" => "Default OS Player",
        other => other,
    };

    let selected_idx = match run_video_tui(&videos, player_display_name)? {
        Some(idx) => idx,
        None => return Ok(()),
    };

    if selected_idx < videos.len() {
        let selected_file = &videos[selected_idx];
        println!("\x1b[1;92m▶ Playing:\x1b[0m {}", selected_file.display());
        spawn_player(&player_info, selected_file)?;
    }

    Ok(())
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

#[derive(Clone)]
struct VideoItem {
    path: PathBuf,
    folder: String,
    name: String,
    ext: String,
    size_str: String,
}

struct VideoApp {
    all: Vec<VideoItem>,
    query: String,
    filtered: Vec<usize>,
    cursor: usize,
}

impl VideoApp {
    fn new(videos: &[PathBuf]) -> Self {
        let all: Vec<VideoItem> = videos
            .iter()
            .map(|p| {
                let folder = p
                    .parent()
                    .and_then(|parent| parent.file_name())
                    .and_then(|s| s.to_str())
                    .unwrap_or(".")
                    .to_string();
                let name = p
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();
                let ext = p
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_uppercase();
                let size_bytes = fs::metadata(p).map(|m| m.len()).unwrap_or(0);
                let size_str = format_size(size_bytes);

                VideoItem {
                    path: p.clone(),
                    folder,
                    name,
                    ext,
                    size_str,
                }
            })
            .collect();

        let filtered: Vec<usize> = (0..all.len()).collect();
        Self {
            all,
            query: String::new(),
            filtered,
            cursor: 0,
        }
    }

    fn refilter(&mut self) {
        let q = self.query.to_lowercase();
        self.filtered = self
            .all
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                if q.is_empty()
                    || item.name.to_lowercase().contains(&q)
                    || item.folder.to_lowercase().contains(&q)
                    || item.ext.to_lowercase().contains(&q)
                {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        if self.cursor >= self.filtered.len() && !self.filtered.is_empty() {
            self.cursor = self.filtered.len() - 1;
        } else if self.filtered.is_empty() {
            self.cursor = 0;
        }
    }

    fn move_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn move_down(&mut self) {
        if !self.filtered.is_empty() && self.cursor < self.filtered.len() - 1 {
            self.cursor += 1;
        }
    }

    fn current_selected(&self) -> Option<&VideoItem> {
        if self.filtered.is_empty() || self.cursor >= self.filtered.len() {
            None
        } else {
            let real_idx = self.filtered[self.cursor];
            Some(&self.all[real_idx])
        }
    }
}

fn run_video_tui(videos: &[PathBuf], player_name: &str) -> Result<Option<usize>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr_handle = stderr();
    execute!(stderr_handle, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr_handle);
    let mut terminal = Terminal::new(backend)?;

    let mut app = VideoApp::new(videos);

    // Modern Vibrant Color Palette
    let c_violet = Color::Rgb(147, 112, 219);
    let c_cyan = Color::Rgb(0, 220, 230);
    let c_magenta = Color::Rgb(255, 105, 180);
    let c_green = Color::Rgb(50, 205, 50);
    let c_yellow = Color::Rgb(255, 215, 0);
    let c_card_bg = Color::Rgb(25, 28, 42);

    let res = loop {
        terminal.draw(|f| {
            let outer_block = Block::default()
                .title(Span::styled(
                    format!(" 🎬 FANCYBASH VIDEO VAULT & PLAYER  |  Engine: {} ", player_name),
                    Style::default().fg(c_magenta).add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(c_violet));

            let area = f.area();
            f.render_widget(outer_block.clone(), area);
            let inner_area = outer_block.inner(area);

            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // Top status bar (Search + Counts + Quick help)
                    Constraint::Min(6),    // Main dual pane body
                ])
                .split(inner_area);

            // 1. Search Bar & Status Header
            let count_str = format!("{}/{}", app.filtered.len(), app.all.len());
            let search_line = Line::from(vec![
                Span::styled("🔍 Filter: ", Style::default().fg(c_cyan).add_modifier(Modifier::BOLD)),
                Span::styled(&app.query, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled("█ ", Style::default().fg(c_yellow)),
                Span::styled(format!("({} matched)", count_str), Style::default().fg(c_green)),
                Span::raw("   "),
                Span::styled("[ENTER] Play  |  [ESC] Exit  |  [↑/↓] Select", Style::default().fg(Color::DarkGray)),
            ]);
            f.render_widget(Paragraph::new(search_line), main_chunks[0]);

            // 2. Dual Pane Body (Left: Video List 55%, Right: Video Info Card 45%)
            let body_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(55),
                    Constraint::Percentage(45),
                ])
                .split(main_chunks[1]);

            // Left Pane: Video List
            let list_block = Block::default()
                .title(Span::styled(" 📁 Available Videos ", Style::default().fg(c_cyan).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray));

            let items: Vec<ListItem> = app
                .filtered
                .iter()
                .enumerate()
                .map(|(filtered_idx, &real_idx)| {
                    let item = &app.all[real_idx];
                    let is_selected = filtered_idx == app.cursor;

                    let pointer = if is_selected { "▶ " } else { "  " };
                    let idx_str = format!("{:02} ", filtered_idx + 1);
                    let ext_badge = format!("[{}] ", item.ext);
                    let size_badge = format!("[{}]", item.size_str);

                    let style_base = if is_selected {
                        Style::default().fg(Color::White).bg(c_card_bg).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    };

                    let line = Line::from(vec![
                        Span::styled(pointer, if is_selected { Style::default().fg(c_green).add_modifier(Modifier::BOLD) } else { Style::default().fg(Color::DarkGray) }),
                        Span::styled(idx_str, Style::default().fg(c_cyan)),
                        Span::styled(&item.name, style_base),
                        Span::raw(" "),
                        Span::styled(ext_badge, Style::default().fg(c_yellow)),
                        Span::styled(size_badge, Style::default().fg(c_magenta)),
                    ]);

                    ListItem::new(line)
                })
                .collect();

            let list_widget = List::new(items).block(list_block);
            let mut state = ListState::default();
            if !app.filtered.is_empty() {
                state.select(Some(app.cursor));
            }
            f.render_stateful_widget(list_widget, body_chunks[0], &mut state);

            // Right Pane: Live Media Preview & Info Card
            let info_block = Block::default()
                .title(Span::styled(" ℹ️ Media Details ", Style::default().fg(c_yellow).add_modifier(Modifier::BOLD)))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray));

            let info_inner = info_block.inner(body_chunks[1]);
            f.render_widget(info_block, body_chunks[1]);

            if let Some(sel) = app.current_selected() {
                let info_lines = vec![
                    Line::from(vec![
                        Span::styled("🎬 Name: ", Style::default().fg(c_magenta).add_modifier(Modifier::BOLD)),
                        Span::styled(&sel.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(vec![
                        Span::styled("📁 Folder: ", Style::default().fg(c_cyan).add_modifier(Modifier::BOLD)),
                        Span::styled(&sel.folder, Style::default().fg(Color::Yellow)),
                    ]),
                    Line::from(vec![
                        Span::styled("💾 Size: ", Style::default().fg(c_green).add_modifier(Modifier::BOLD)),
                        Span::styled(&sel.size_str, Style::default().fg(Color::White)),
                    ]),
                    Line::from(vec![
                        Span::styled("🎞️ Format: ", Style::default().fg(c_yellow).add_modifier(Modifier::BOLD)),
                        Span::styled(&sel.ext, Style::default().fg(c_cyan)),
                    ]),
                    Line::from(vec![
                        Span::styled("📍 Path: ", Style::default().fg(Color::DarkGray)),
                        Span::styled(sel.path.display().to_string(), Style::default().fg(Color::DarkGray)),
                    ]),
                    Line::raw(""),
                    Line::from(Span::styled("┌────────────────────────────────────┐", Style::default().fg(c_violet))),
                    Line::from(Span::styled("│   🎬  FANCYBASH MEDIA PLAYER       │", Style::default().fg(c_cyan).add_modifier(Modifier::BOLD))),
                    Line::from(Span::styled("│                                    │", Style::default().fg(c_violet))),
                    Line::from(Span::styled("│     [▶] PRESS ENTER TO PLAY        │", Style::default().fg(c_green).add_modifier(Modifier::BOLD))),
                    Line::from(Span::styled("│         VIDEO IN BACKGROUND        │", Style::default().fg(c_yellow))),
                    Line::from(Span::styled("└────────────────────────────────────┘", Style::default().fg(c_violet))),
                ];
                let info_paragraph = Paragraph::new(info_lines).wrap(Wrap { trim: true });
                f.render_widget(info_paragraph, info_inner);
            } else {
                let empty_para = Paragraph::new(vec![
                    Line::from(Span::styled("❌ No video selected", Style::default().fg(Color::Red))),
                ]);
                f.render_widget(empty_para, info_inner);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                let is_ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

                match key.code {
                    KeyCode::Esc => break Ok(None),
                    KeyCode::Char('c') if is_ctrl => break Ok(None),
                    KeyCode::Up => app.move_up(),
                    KeyCode::Char('p') if is_ctrl => app.move_up(),
                    KeyCode::Down => app.move_down(),
                    KeyCode::Char('n') if is_ctrl => app.move_down(),
                    KeyCode::Enter => {
                        if !app.filtered.is_empty() && app.cursor < app.filtered.len() {
                            let real_idx = app.filtered[app.cursor];
                            break Ok(Some(real_idx));
                        }
                    }
                    KeyCode::Backspace => {
                        app.query.pop();
                        app.refilter();
                    }
                    KeyCode::Char(c) => {
                        app.query.push(c);
                        app.refilter();
                    }
                    _ => {}
                }
            }
        }
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn spawn_player((bin, base_args): &(String, Vec<String>), file: &Path) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new(bin);
    for arg in base_args {
        cmd.arg(arg);
    }
    cmd.arg(file);

    cmd.stdout(Stdio::null()).stderr(Stdio::null()).spawn()?;
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

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1048576), "1.0 MB");
    }
}
