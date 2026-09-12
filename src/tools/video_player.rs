// =============================================================================
//  src/tools/video_player.rs — Video Filter & Background Player (`v`)
// =============================================================================

use std::error::Error;
use std::fs;
use std::io::stdout;
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
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
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

    let selected_idx = match run_video_tui(&videos)? {
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

#[derive(Clone)]
struct VideoItem {
    folder: String,
    name: String,
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
                VideoItem {
                    folder,
                    name,
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
}

fn run_video_tui(videos: &[PathBuf]) -> Result<Option<usize>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = VideoApp::new(videos);

    let c_blue = Color::Rgb(100, 160, 255);
    let c_yellow = Color::Rgb(255, 215, 0);
    let c_magenta = Color::Rgb(255, 80, 220);
    let c_cyan = Color::Rgb(0, 220, 220);
    let c_green = Color::Rgb(80, 220, 120);

    let res = loop {
        terminal.draw(|f| {
            let outer_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));

            let inner_area = outer_block.inner(f.area());
            f.render_widget(outer_block, f.area());

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // Table Header (IDX   FOLDER        VIDEO NAME)
                    Constraint::Length(1), // Search Input (🔍 Search: | query)
                    Constraint::Length(1), // Count (9/9)
                    Constraint::Length(1), // Divider (────)
                    Constraint::Min(4),    // Items list
                ])
                .split(inner_area);

            // 1. Table Column Titles
            let header_line = Line::from(vec![
                Span::raw("    "),
                Span::styled("IDX", Style::default().fg(c_blue).add_modifier(Modifier::BOLD)),
                Span::raw("   "),
                Span::styled("FOLDER", Style::default().fg(c_yellow).add_modifier(Modifier::BOLD)),
                Span::raw("        "),
                Span::styled("VIDEO NAME", Style::default().fg(c_magenta).add_modifier(Modifier::BOLD)),
            ]);
            f.render_widget(Paragraph::new(header_line), chunks[0]);

            // 2. Search Input Prompt
            let search_line = Line::from(vec![
                Span::styled("🔍 Search: ", Style::default().fg(c_cyan).add_modifier(Modifier::BOLD)),
                Span::styled(&app.query, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled("│", Style::default().fg(c_yellow)),
            ]);
            f.render_widget(Paragraph::new(search_line), chunks[1]);

            // 3. Match Count (e.g., 9/9)
            let count_str = format!("{}/{}", app.filtered.len(), app.all.len());
            let count_line = Line::from(vec![
                Span::raw("   "),
                Span::styled(count_str, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            ]);
            f.render_widget(Paragraph::new(count_line), chunks[2]);

            // 4. Divider Line
            let width = chunks[3].width as usize;
            let divider_str = "─".repeat(width);
            let divider_line = Line::from(vec![
                Span::styled(divider_str, Style::default().fg(Color::DarkGray)),
            ]);
            f.render_widget(Paragraph::new(divider_line), chunks[3]);

            // 5. Video List Items
            let items: Vec<ListItem> = app
                .filtered
                .iter()
                .enumerate()
                .map(|(filtered_idx, &real_idx)| {
                    let item = &app.all[real_idx];
                    let is_selected = filtered_idx == app.cursor;

                    let pointer = if is_selected { "▶ " } else { "  " };
                    let idx_str = format!("{:<3}", filtered_idx + 1);
                    let folder_str = format!("{:<12}", item.folder);

                    let pointer_span = Span::styled(pointer, Style::default().fg(c_green).add_modifier(Modifier::BOLD));
                    let idx_span = Span::styled(idx_str, Style::default().fg(c_blue));
                    let icon_span = Span::styled("📁 ", Style::default().fg(c_yellow));
                    let folder_span = Span::styled(folder_str, Style::default().fg(c_yellow));
                    let name_span = Span::styled(
                        &item.name,
                        if is_selected {
                            Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(c_green)
                        },
                    );

                    let line = Line::from(vec![
                        pointer_span,
                        idx_span,
                        Span::raw(" "),
                        icon_span,
                        folder_span,
                        Span::raw(" "),
                        name_span,
                    ]);

                    ListItem::new(line)
                })
                .collect();

            let list_widget = List::new(items);
            let mut state = ListState::default();
            if !app.filtered.is_empty() {
                state.select(Some(app.cursor));
            }
            f.render_stateful_widget(list_widget, chunks[4], &mut state);
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
}
