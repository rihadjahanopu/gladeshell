// =============================================================================
//  src/tools/fuzzy_cd.rs — Interactive Fuzzy Directory Navigator (`cf`)
//
//  Pure Rust Ratatui + Crossterm dual-pane directory navigator with live item
//  preview, mode switching (Dev Walk / Recent Dirs), fast search & match counter.
// =============================================================================

use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use walkdir::WalkDir;

// ── colour palette (modern dark theme matching fkill.rs) ───────────────────────
const C_BG: Color = Color::Rgb(10, 10, 18);
const C_BORDER: Color = Color::Rgb(80, 220, 140); // vibrant mint green
const C_ACCENT: Color = Color::Rgb(100, 210, 255); // neon cyan
const C_SELECTED: Color = Color::Rgb(255, 85, 140); // hot pink / magenta accent
const C_DIM: Color = Color::Rgb(120, 120, 140);
const C_TEXT: Color = Color::Rgb(220, 220, 230);
const C_GREEN: Color = Color::Rgb(80, 220, 120);
const C_YELLOW: Color = Color::Rgb(255, 200, 80);
const C_WHITE: Color = Color::Rgb(255, 255, 255);
const C_CYAN: Color = Color::Rgb(80, 220, 255);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchMode {
    DevWalk,
    RecentDirs,
}

/// Action the shell wrapper should take on the selected item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CfAction {
    CdInto,   // cd into directory
    OpenCode, // open in VS Code (F2)
    OpenFile, // open file with type-appropriate handler
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub rel_path: String,
    pub is_dir: bool,
    pub size: u64,
}

pub struct FuzzyCdApp {
    pub current_dir: PathBuf,
    pub mode: SearchMode,
    pub all_items: Vec<FileEntry>,
    pub filtered_indices: Vec<usize>,
    pub list_state: ListState,
    pub query: String,
}

impl FuzzyCdApp {
    pub fn new(current_dir: PathBuf) -> Self {
        let mut app = Self {
            current_dir,
            mode: SearchMode::DevWalk,
            all_items: Vec::new(),
            filtered_indices: Vec::new(),
            list_state: ListState::default(),
            query: String::new(),
        };
        app.load_items();
        app
    }

    pub fn load_items(&mut self) {
        self.all_items.clear();

        match self.mode {
            SearchMode::DevWalk => self.load_dev_walk(),
            SearchMode::RecentDirs => self.load_recent_dirs(),
        }

        self.filter_items();
    }

    fn load_dev_walk(&mut self) {
        // Always include current directory
        self.all_items.push(FileEntry {
            path: self.current_dir.clone(),
            rel_path: "./".to_string(),
            is_dir: true,
            size: 4096,
        });

        let max_entries = 15000;
        let walker = WalkDir::new(&self.current_dir)
            .max_depth(7)
            .into_iter()
            .filter_entry(|e| {
                if let Some(name) = e.file_name().to_str() {
                    if name.starts_with('.') && name != "." && name != ".." {
                        return false;
                    }
                    if name == "node_modules"
                        || name == "target"
                        || name == "vendor"
                        || name == "dist"
                        || name == "build"
                        || name == ".git"
                        || name == ".cargo"
                        || name == ".rustup"
                        || name == ".cache"
                        || name == "__pycache__"
                        || name == ".next"
                        || name == ".nuxt"
                        || name == "venv"
                        || name == ".venv"
                    {
                        return false;
                    }
                }
                true
            });

        for entry in walker.flatten() {
            if self.all_items.len() >= max_entries {
                break;
            }

            let path = entry.path().to_path_buf();
            if path == self.current_dir {
                continue;
            }

            let is_dir = entry.file_type().is_dir();
            let size = 0;

            let rel_path = match path.strip_prefix(&self.current_dir) {
                Ok(rel) => {
                    let s = rel.to_string_lossy();
                    if is_dir {
                        format!("./{}/", s)
                    } else {
                        format!("./{}", s)
                    }
                }
                Err(_) => path.to_string_lossy().to_string(),
            };

            self.all_items.push(FileEntry {
                path,
                rel_path,
                is_dir,
                size,
            });
        }
    }

    fn load_recent_dirs(&mut self) {
        let mut seen = std::collections::HashSet::new();

        // 1. Zoxide history if available (~/.local/share/zoxide/db.zo)
        if let Some(home) = dirs_home() {
            let zoxide_db = home.join(".local/share/zoxide/db.zo");
            if zoxide_db.exists() {
                if let Ok(file) = fs::File::open(&zoxide_db) {
                    let reader = BufReader::new(file);
                    for line in reader.lines().flatten() {
                        let path_str = line.split('|').next().unwrap_or("").trim();
                        if !path_str.is_empty() {
                            let p = PathBuf::from(path_str);
                            if p.is_dir() && seen.insert(p.clone()) {
                                let rel_path = if let Ok(rel) = p.strip_prefix(&self.current_dir) {
                                    format!("./{}/", rel.display())
                                } else {
                                    p.to_string_lossy().to_string()
                                };
                                self.all_items.push(FileEntry {
                                    path: p,
                                    rel_path,
                                    is_dir: true,
                                    size: 4096,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Fallback: Add common developer dirs in HOME if recent list is empty
        if self.all_items.is_empty() {
            if let Some(home) = dirs_home() {
                let candidates = ["Developer", "Projects", "Desktop", "Downloads", "Documents"];
                for c in &candidates {
                    let p = home.join(c);
                    if p.is_dir() && seen.insert(p.clone()) {
                        self.all_items.push(FileEntry {
                            path: p.clone(),
                            rel_path: p.to_string_lossy().to_string(),
                            is_dir: true,
                            size: 4096,
                        });
                    }
                }
            }
        }
    }

    pub fn filter_items(&mut self) {
        let q = self.query.to_lowercase();
        if q.is_empty() {
            self.filtered_indices = (0..self.all_items.len()).collect();
        } else {
            self.filtered_indices = self
                .all_items
                .iter()
                .enumerate()
                .filter(|(_, item)| item.rel_path.to_lowercase().contains(&q))
                .map(|(idx, _)| idx)
                .collect();
        }

        if self.filtered_indices.is_empty() {
            self.list_state.select(None);
        } else {
            let selected = self.list_state.selected().unwrap_or(0);
            let next_sel = selected.min(self.filtered_indices.len().saturating_sub(1));
            self.list_state.select(Some(next_sel));
        }
    }

    pub fn run_loop<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<Option<(String, CfAction)>, Box<dyn Error>> {
        loop {
            terminal.draw(|f| self.render_ui(f))?;

            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        return Ok(None);
                    }
                    (KeyCode::Char('z'), KeyModifiers::CONTROL) | (KeyCode::Tab, _) => {
                        self.mode = match self.mode {
                            SearchMode::DevWalk => SearchMode::RecentDirs,
                            SearchMode::RecentDirs => SearchMode::DevWalk,
                        };
                        self.query.clear();
                        self.load_items();
                    }
                    (KeyCode::Enter, _) => {
                        if let Some(idx) = self.list_state.selected() {
                            if let Some(&orig_idx) = self.filtered_indices.get(idx) {
                                let item = &self.all_items[orig_idx];
                                let path = item.path.to_string_lossy().to_string();
                                let action = if item.is_dir {
                                    CfAction::CdInto
                                } else {
                                    CfAction::OpenFile
                                };
                                return Ok(Some((path, action)));
                            }
                        }
                        return Ok(None);
                    }
                    // [v] → Open selected item in VS Code
                    (KeyCode::Char('v'), KeyModifiers::NONE) | (KeyCode::F(2), _) => {
                        if let Some(idx) = self.list_state.selected() {
                            if let Some(&orig_idx) = self.filtered_indices.get(idx) {
                                let item = &self.all_items[orig_idx];
                                let path = item.path.to_string_lossy().to_string();
                                return Ok(Some((path, CfAction::OpenCode)));
                            }
                        }
                    }
                    (KeyCode::Up, _) | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                        self.move_select(-1);
                    }
                    (KeyCode::Down, _) | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                        self.move_select(1);
                    }
                    (KeyCode::PageUp, _) => {
                        self.move_select(-10);
                    }
                    (KeyCode::PageDown, _) => {
                        self.move_select(10);
                    }
                    (KeyCode::Backspace, _) => {
                        self.query.pop();
                        self.filter_items();
                    }
                    (KeyCode::Char(c), KeyModifiers::NONE)
                    | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                        self.query.push(c);
                        self.filter_items();
                    }
                    _ => {}
                }
            }
        }
    }

    fn move_select(&mut self, delta: i32) {
        if self.filtered_indices.is_empty() {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0) as i32;
        let len = self.filtered_indices.len() as i32;
        let next = (current + delta).clamp(0, len - 1);
        self.list_state.select(Some(next as usize));
    }

    fn render_ui(&mut self, frame: &mut Frame) {
        let area = frame.area();

        // Dark background
        frame.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

        // 4-Tier Vertical Layout (Banner, Search, Dual Pane Body, Status Bar)
        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Top Banner
                Constraint::Length(3), // Search Bar
                Constraint::Min(6),    // Dual Pane Body
                Constraint::Length(3), // Bottom Status Bar
            ])
            .split(area);

        // ── 1. Top Banner ───────────────────────────────────────────────────────
        let mode_name = match self.mode {
            SearchMode::DevWalk => "Dev Walk (Current Directory Tree)",
            SearchMode::RecentDirs => "Recent Dirs (Zoxide / History)",
        };

        let banner_text = Line::from(vec![
            Span::styled("⚡  ", Style::default().fg(C_YELLOW)),
            Span::styled(
                "CF",
                Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " — Interactive Fuzzy Directory Navigator",
                Style::default().fg(C_TEXT).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  [{}]  ({} items)", mode_name, self.all_items.len()),
                Style::default().fg(C_DIM),
            ),
        ]);

        let banner = Paragraph::new(banner_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(C_BORDER))
                    .style(Style::default().bg(C_BG)),
            );
        frame.render_widget(banner, outer[0]);

        // ── 2. Search Bar ───────────────────────────────────────────────────────
        let match_count = self.filtered_indices.len();
        let total_count = self.all_items.len();

        let search_text = Line::from(vec![
            Span::styled(" 🔍 ", Style::default().fg(C_ACCENT)),
            Span::styled(
                &self.query,
                Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD),
            ),
            Span::styled("█", Style::default().fg(C_BORDER)),
            Span::styled(
                format!("   ({}/{} matches)", match_count, total_count),
                Style::default().fg(C_DIM),
            ),
        ]);

        let search_bar = Paragraph::new(search_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_ACCENT))
                .title(Span::styled(
                    " Search Filter ",
                    Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().bg(C_BG)),
        );
        frame.render_widget(search_bar, outer[1]);

        // ── 3. Dual Pane Body ───────────────────────────────────────────────────
        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(52), // Left File List
                Constraint::Percentage(48), // Right Live Preview
            ])
            .split(outer[2]);

        // Left File List Panel
        let list_items: Vec<ListItem> = self
            .filtered_indices
            .iter()
            .enumerate()
            .map(|(i, &orig_idx)| {
                let is_selected = self.list_state.selected() == Some(i);
                let item = &self.all_items[orig_idx];

                let icon = if item.is_dir {
                    "📁 "
                } else {
                    let ext = item.path.extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_lowercase();
                    match ext.as_str() {
                        "mp4"|"mkv"|"avi"|"mov"|"webm"|"flv"|"wmv"|"m4v"|"ogv"|"m2ts"|"rmvb"|"3gp" => "🎬 ",
                        "jpg"|"jpeg"|"png"|"gif"|"bmp"|"webp"|"svg"|"avif"|"heic"|"tiff"|"ico" => "🖼️  ",
                        "mp3"|"flac"|"ogg"|"wav"|"aac"|"m4a"|"opus"|"wma" => "🎵 ",
                        "pdf" => "📕 ",
                        "docx"|"doc"|"odt" => "📝 ",
                        "xlsx"|"xls"|"ods"|"csv" => "📊 ",
                        "pptx"|"ppt"|"odp" => "📊 ",
                        "zip"|"tar"|"gz"|"bz2"|"xz"|"7z"|"rar"|"zst" => "🗜️  ",
                        "rs"|"py"|"js"|"ts"|"go"|"c"|"cpp"|"java"|"rb"|"php"|"swift"|"kt" => "⚡ ",
                        "sh"|"bash"|"zsh"|"fish" => "🖥️  ",
                        "md"|"txt"|"rst" => "📄 ",
                        "json"|"yaml"|"yml"|"toml"|"xml" => "🔧 ",
                        _ => "📄 ",
                    }
                };

                if is_selected {
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            "❯ ",
                            Style::default().fg(C_SELECTED).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(icon, Style::default().fg(C_YELLOW)),
                        Span::styled(
                            &item.rel_path,
                            Style::default()
                                .fg(C_WHITE)
                                .bg(Color::Rgb(40, 20, 60))
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]))
                } else {
                    let color = if item.is_dir { C_CYAN } else { C_GREEN };
                    ListItem::new(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(icon, Style::default().fg(C_DIM)),
                        Span::styled(&item.rel_path, Style::default().fg(color)),
                    ]))
                }
            })
            .collect();

        let list_title = match self.mode {
            SearchMode::DevWalk => " 📂 Dev Directory Tree ",
            SearchMode::RecentDirs => " 🕒 Recent Directories ",
        };

        let list_widget = List::new(list_items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .title(Span::styled(
                    list_title,
                    Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().bg(C_BG)),
        );

        frame.render_stateful_widget(list_widget, body_chunks[0], &mut self.list_state);

        // Right Preview Panel
        let selected_entry = self
            .list_state
            .selected()
            .and_then(|idx| self.filtered_indices.get(idx))
            .map(|&orig_idx| &self.all_items[orig_idx]);

        render_preview_panel(frame, body_chunks[1], selected_entry);

        // ── 4. Bottom Status Bar ────────────────────────────────────────────────
        let status_line = Line::from(vec![
            Span::styled(" ↑↓ ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled("Navigate", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_DIM)),
            Span::styled("↵ ", Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled("Open / Cd", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_DIM)),
            Span::styled("v ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled("Code", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_DIM)),
            Span::styled("⇥ ", Style::default().fg(C_YELLOW).add_modifier(Modifier::BOLD)),
            Span::styled("Switch Mode", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_DIM)),
            Span::styled("⎋ ", Style::default().fg(C_SELECTED).add_modifier(Modifier::BOLD)),
            Span::styled("Quit ", Style::default().fg(C_DIM)),
        ]);

        let status_bar = Paragraph::new(status_line)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(C_DIM))
                    .style(Style::default().bg(C_BG)),
            );
        frame.render_widget(status_bar, outer[3]);
    }
}

fn render_preview_panel(frame: &mut Frame, area: Rect, entry: Option<&FileEntry>) {
    let title = match entry {
        Some(e) if e.is_dir => format!(" 👁️ Directory Contents: '{}' ", e.rel_path),
        Some(e) => format!(" 👁️ File Preview: '{}' ", e.rel_path),
        None => " 👁️ Preview ".to_string(),
    };

    let preview_block = Block::default()
        .title(Span::styled(
            title,
            Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(C_ACCENT))
        .style(Style::default().bg(C_BG));

    let inner = preview_block.inner(area);
    frame.render_widget(preview_block, area);

    if inner.height < 2 || inner.width < 5 {
        return;
    }

    let mut lines = Vec::new();

    if let Some(item) = entry {
        if item.is_dir {
            let (children, size_str) = get_dir_preview(&item.path);
            lines.push(Line::from(vec![
                Span::styled("📊 Total Size: ", Style::default().fg(C_YELLOW).add_modifier(Modifier::BOLD)),
                Span::styled(size_str, Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
            ]));
            lines.push(Line::from(Span::styled(
                "─".repeat(inner.width as usize),
                Style::default().fg(C_DIM),
            )));
            if children.is_empty() {
                lines.push(Line::from(Span::styled("  (empty directory)", Style::default().fg(C_DIM))));
            } else {
                for child in children
                    .into_iter()
                    .take((inner.height as usize).saturating_sub(3))
                {
                    let color = if child.ends_with('/') || child.ends_with('@') {
                        C_CYAN
                    } else {
                        C_GREEN
                    };
                    lines.push(Line::from(vec![Span::styled(
                        child,
                        Style::default().fg(color),
                    )]));
                }
            }
        } else {
            let (file_lines, size_str) = get_file_preview(&item.path);
            lines.push(Line::from(vec![
                Span::styled("📊 File Size: ", Style::default().fg(C_YELLOW).add_modifier(Modifier::BOLD)),
                Span::styled(size_str, Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
            ]));
            lines.push(Line::from(Span::styled(
                "─".repeat(inner.width as usize),
                Style::default().fg(C_DIM),
            )));
            for l in file_lines
                .into_iter()
                .take((inner.height as usize).saturating_sub(3))
            {
                lines.push(Line::from(Span::styled(l, Style::default().fg(C_TEXT))));
            }
        }
    } else {
        lines.push(Line::from(Span::styled("No item selected", Style::default().fg(C_DIM))));
    }

    frame.render_widget(Paragraph::new(lines), inner);
}

fn get_dir_preview(dir_path: &Path) -> (Vec<String>, String) {
    let mut items = Vec::new();
    let mut total_size: u64 = 0;

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                total_size += meta.len();
            }
            let name = entry.file_name().to_string_lossy().to_string();
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let is_symlink = entry.file_type().map(|t| t.is_symlink()).unwrap_or(false);

            if is_dir {
                items.push(format!("📁 {}/", name));
            } else if is_symlink {
                items.push(format!("🔗 {}@", name));
            } else {
                items.push(format!("📄 {}", name));
            }
        }
    }

    items.sort();
    let size_str = format_size(total_size);
    (items, size_str)
}

fn get_file_preview(file_path: &Path) -> (Vec<String>, String) {
    let mut lines = Vec::new();
    let mut size_str = "0B".to_string();

    if let Ok(meta) = fs::metadata(file_path) {
        size_str = format_size(meta.len());
    }

    if let Ok(file) = fs::File::open(file_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().take(30) {
            if let Ok(l) = line {
                lines.push(l);
            } else {
                lines.push("[Binary file or non-UTF8 text preview]".to_string());
                break;
            }
        }
    }
    (lines, size_str)
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}K", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1}M", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1}G", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn dirs_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

fn open_tty() -> Box<dyn io::Write + Send> {
    #[cfg(unix)]
    {
        if let Ok(file) = fs::OpenOptions::new().read(true).write(true).open("/dev/tty") {
            return Box::new(file);
        }
    }
    #[cfg(windows)]
    {
        if let Ok(file) = fs::OpenOptions::new().read(true).write(true).open("CONOUT$") {
            return Box::new(file);
        }
    }
    Box::new(io::stderr())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;

    enable_raw_mode()?;
    let mut tty = open_tty();
    execute!(tty, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(tty);
    let mut terminal = Terminal::new(backend)?;

    let mut app = FuzzyCdApp::new(current_dir);

    let res = app.run_loop(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Ok(Some((selected_path, action))) = res {
        // Output tagged path so the shell wrapper can dispatch correctly
        let tag = match action {
            CfAction::CdInto   => "CD",
            CfAction::OpenCode => "CODE",
            CfAction::OpenFile => "OPEN",
        };
        println!("{}:{}", tag, selected_path);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500B");
        assert_eq!(format_size(4096), "4.0K");
        assert_eq!(format_size(1048576), "1.0M");
    }

    #[test]
    fn test_fuzzy_cd_app_init() {
        let cur = std::env::current_dir().unwrap();
        let app = FuzzyCdApp::new(cur);
        assert!(!app.all_items.is_empty());
    }
}
