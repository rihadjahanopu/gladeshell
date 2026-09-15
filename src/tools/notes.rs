// =============================================================================
//  src/tools/notes.rs — FANCYBASH Notes Manager (Modern fkill-style TUI)
// =============================================================================

use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

// ── Colour Palette (fkill & gwip consistent dark violet/teal) ───────────────
const C_BG: Color = Color::Rgb(8, 12, 22);
const C_BORDER: Color = Color::Rgb(0, 210, 180);       // teal accent
const C_ACCENT: Color = Color::Rgb(0, 240, 200);       // bright teal
const C_SELECTED_BG: Color = Color::Rgb(0, 45, 40);    // dark teal row bg
const C_SELECTED_FG: Color = Color::Rgb(0, 255, 200);  // selected text
const C_DIM: Color = Color::Rgb(90, 110, 120);
const C_TEXT: Color = Color::Rgb(210, 225, 235);
const C_GREEN: Color = Color::Rgb(80, 220, 140);
const C_YELLOW: Color = Color::Rgb(255, 200, 80);
const C_PINK: Color = Color::Rgb(255, 80, 160);
const C_WHITE: Color = Color::Rgb(255, 255, 255);
const C_CAT: Color = Color::Rgb(130, 200, 255);        // category label blue
const C_CONTENT: Color = Color::Rgb(195, 230, 215);    // note content text
const C_VIOLET: Color = Color::Rgb(180, 100, 255);     // gwip-style violet
const C_CYAN: Color = Color::Rgb(0, 229, 255);       // neon cyan
const C_CARD: Color = Color::Rgb(16, 22, 34);         // card background

fn notes_dir_path() -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join(".my_notes")
}

// ── Data Model ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct NoteItem {
    pub category: String,
    pub title: String,
    pub file_path: PathBuf,
    pub created_time: String,
    pub content: String,
}

// ── App Pages & Gwip-style Actions ──────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum AppPage {
    List,   // Page 1: Note search & catalog
    Detail, // Page 2: gwip-style Note Actions & Edit view
}

#[derive(Debug, Clone)]
pub struct ActionItem {
    pub label: &'static str,
    pub desc: &'static str,
    pub emoji: &'static str,
    pub shortcut: &'static str,
}

pub const ACTION_ITEMS: &[ActionItem] = &[
    ActionItem { label: "Edit Content",    desc: "Edit note text in interactive buffer",  emoji: "✏️", shortcut: "Enter / e" },
    ActionItem { label: "Copy Content",    desc: "Copy note text strictly to clipboard", emoji: "📋", shortcut: "c"         },
    ActionItem { label: "Open VS Code",    desc: "Open note file in VS Code editor",      emoji: "💻", shortcut: "Ctrl+V"    },
    ActionItem { label: "Open Folder",     desc: "Open containing folder in file manager",emoji: "📂", shortcut: "o"         },
    ActionItem { label: "Note Statistics", desc: "View word, line, and character stats",  emoji: "📊", shortcut: "s"         },
    ActionItem { label: "Delete Note",     desc: "Delete note file permanently",          emoji: "🗑️", shortcut: "d"         },
    ActionItem { label: "Back to Catalog", desc: "Return to notes search catalog",        emoji: "🔙", shortcut: "Esc"       },
];

// ── Modal Mode ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Modal {
    None,
    ConfirmDelete(usize),                    // index into filtered_indices
    NewNoteField(u8, String, String, String),// active_field (0=cat,1=title,2=content), cat, title, content
}

// ── App State ────────────────────────────────────────────────────────────────

pub struct NotesApp {
    pub root_dir: PathBuf,
    pub items: Vec<NoteItem>,
    pub filtered_indices: Vec<usize>,
    pub list_state: ListState,
    pub query: String,
    pub status_msg: Option<(String, bool)>, // (msg, is_error)
    pub scroll_offset: u16,

    // Page 2 & GWIP-style UX State
    pub page: AppPage,
    pub action_cursor: usize,
    pub is_editing_content: bool,
    pub edit_buffer: String,

    modal: Modal,
}

impl NotesApp {
    pub fn new(root_dir: PathBuf) -> Self {
        let mut app = Self {
            root_dir,
            items: Vec::new(),
            filtered_indices: Vec::new(),
            list_state: ListState::default(),
            query: String::new(),
            status_msg: None,
            scroll_offset: 0,
            page: AppPage::List,
            action_cursor: 0,
            is_editing_content: false,
            edit_buffer: String::new(),
            modal: Modal::None,
        };
        app.load_notes();
        app
    }

    pub fn load_notes(&mut self) {
        self.items.clear();
        seed_sample_notes_if_empty(&self.root_dir);

        if let Ok(cat_entries) = fs::read_dir(&self.root_dir) {
            for cat_entry in cat_entries.flatten() {
                let cat_path = cat_entry.path();
                if cat_path.is_dir() {
                    let cat_name = cat_entry.file_name().to_string_lossy().to_string();
                    if let Ok(file_entries) = fs::read_dir(&cat_path) {
                        for file_entry in file_entries.flatten() {
                            let file_path = file_entry.path();
                            if file_path.is_file() {
                                let stem = file_path
                                    .file_stem()
                                    .map(|s| s.to_string_lossy().to_string())
                                    .unwrap_or_default();
                                if !stem.is_empty() && !stem.starts_with('.') {
                                    let created_time = fs::metadata(&file_path)
                                        .and_then(|m| m.modified())
                                        .map(format_system_time)
                                        .unwrap_or_else(|_| "—".to_string());
                                    let content = fs::read_to_string(&file_path).unwrap_or_default();
                                    self.items.push(NoteItem {
                                        category: cat_name.clone(),
                                        title: stem,
                                        file_path,
                                        created_time,
                                        content,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        self.items.sort_by(|a, b| a.category.cmp(&b.category).then_with(|| a.title.cmp(&b.title)));
        self.filter_items();
    }

    pub fn filter_items(&mut self) {
        let q = self.query.to_lowercase();
        if q.is_empty() {
            self.filtered_indices = (0..self.items.len()).collect();
        } else {
            self.filtered_indices = self
                .items
                .iter()
                .enumerate()
                .filter(|(_, item)| {
                    item.title.to_lowercase().contains(&q)
                        || item.category.to_lowercase().contains(&q)
                        || item.content.to_lowercase().contains(&q)
                })
                .map(|(i, _)| i)
                .collect();
        }
        if self.filtered_indices.is_empty() {
            self.list_state.select(None);
        } else {
            let cur = self.list_state.selected().unwrap_or(0);
            self.list_state.select(Some(cur.min(self.filtered_indices.len() - 1)));
        }
        self.scroll_offset = 0;
    }

    fn selected_note(&self) -> Option<&NoteItem> {
        let idx = self.list_state.selected()?;
        let orig = *self.filtered_indices.get(idx)?;
        self.items.get(orig)
    }

    fn move_select(&mut self, delta: i32) {
        if self.filtered_indices.is_empty() { return; }
        let cur = self.list_state.selected().unwrap_or(0) as i32;
        let len = self.filtered_indices.len() as i32;
        let next = (cur + delta).rem_euclid(len);
        self.list_state.select(Some(next as usize));
        self.scroll_offset = 0;
    }

    fn delete_selected(&mut self) {
        if let Some(sel) = self.list_state.selected() {
            if sel < self.filtered_indices.len() {
                let orig = self.filtered_indices[sel];
                let path = self.items[orig].file_path.clone();
                let title = self.items[orig].title.clone();
                if let Err(e) = fs::remove_file(&path) {
                    self.status_msg = Some((format!("❌ Delete failed: {e}"), true));
                } else {
                    self.status_msg = Some((format!("🗑  '{}' deleted", title), false));
                    self.load_notes();
                }
            }
        }
        self.modal = Modal::None;
    }

    fn create_note(&mut self, category: &str, title: &str, content: &str) {
        let cat = if category.trim().is_empty() { "General" } else { category.trim() };
        let ttl = if title.trim().is_empty() { "Untitled" } else { title.trim() };
        let cat_dir = self.root_dir.join(cat);
        if let Err(e) = fs::create_dir_all(&cat_dir) {
            self.status_msg = Some((format!("❌ {e}"), true));
            self.modal = Modal::None;
            return;
        }
        let file_path = cat_dir.join(format!("{ttl}.txt"));
        if file_path.exists() {
            self.status_msg = Some((format!("⚠  Note '{ttl}' already exists"), true));
            self.modal = Modal::None;
            return;
        }
        if let Err(e) = fs::write(&file_path, content) {
            self.status_msg = Some((format!("❌ {e}"), true));
        } else {
            self.status_msg = Some((format!("✅ Created '{cat}/{ttl}'"), false));
            self.load_notes();
        }
        self.modal = Modal::None;
    }

    pub fn open_detail_page(&mut self) {
        if let Some(note) = self.selected_note() {
            self.edit_buffer = note.content.clone();
            self.page = AppPage::Detail;
            self.action_cursor = 0;
            self.is_editing_content = false;
        }
    }

    pub fn open_in_vscode(&mut self) {
        if let Some(note) = self.selected_note() {
            let path = &note.file_path;
            match Command::new("code").arg(path).spawn() {
                Ok(_) => {
                    self.status_msg = Some((format!("💻 Opened '{}' in VS Code", note.title), false));
                }
                Err(_) => {
                    if let Ok(editor) = std::env::var("EDITOR") {
                        let _ = Command::new(editor).arg(path).spawn();
                        self.status_msg = Some((format!("💻 Opened '{}' in $EDITOR", note.title), false));
                    } else {
                        self.status_msg = Some(("❌ Could not launch VS Code ('code' not found)".to_string(), true));
                    }
                }
            }
        }
    }

    pub fn open_folder(&mut self) {
        if let Some(note) = self.selected_note() {
            if let Some(parent) = note.file_path.parent() {
                #[cfg(target_os = "linux")]
                let res = Command::new("xdg-open").arg(parent).spawn();
                #[cfg(target_os = "macos")]
                let res = Command::new("open").arg(parent).spawn();
                #[cfg(target_os = "windows")]
                let res = Command::new("explorer").arg(parent).spawn();
                #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
                let res = Command::new("xdg-open").arg(parent).spawn();

                match res {
                    Ok(_) => {
                        self.status_msg = Some((format!("📂 Opened folder: {}", parent.display()), false));
                    }
                    Err(e) => {
                        self.status_msg = Some((format!("❌ Failed to open folder: {e}"), true));
                    }
                }
            }
        }
    }

    pub fn save_current_detail_edit(&mut self) {
        if let Some(sel) = self.list_state.selected() {
            if sel < self.filtered_indices.len() {
                let orig = self.filtered_indices[sel];
                let path = self.items[orig].file_path.clone();
                let title = self.items[orig].title.clone();
                let new_content = self.edit_buffer.clone();
                if let Err(e) = fs::write(&path, &new_content) {
                    self.status_msg = Some((format!("❌ Save failed: {e}"), true));
                } else {
                    self.items[orig].content = new_content;
                    self.status_msg = Some((format!("✅ Note '{title}' updated successfully"), false));
                }
            }
        }
        self.is_editing_content = false;
    }

    pub fn save_edited_note(&mut self, sel: usize, new_content: String) {
        if sel < self.filtered_indices.len() {
            let orig = self.filtered_indices[sel];
            let path = self.items[orig].file_path.clone();
            let title = self.items[orig].title.clone();
            if let Err(e) = fs::write(&path, &new_content) {
                self.status_msg = Some((format!("❌ Save failed: {e}"), true));
            } else {
                self.items[orig].content = new_content;
                self.status_msg = Some((format!("✅ Note '{title}' updated"), false));
            }
        }
        self.modal = Modal::None;
    }

    pub fn copy_selected_note_content(&mut self) {
        if let Some(note) = self.selected_note() {
            let content = if self.page == AppPage::Detail {
                self.edit_buffer.clone()
            } else {
                note.content.clone()
            };
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                if clipboard.set_text(&content).is_ok() {
                    self.status_msg = Some(("📋 Copied note content to clipboard".to_string(), false));
                } else {
                    self.status_msg = Some(("❌ Failed to copy to clipboard".to_string(), true));
                }
            } else {
                self.status_msg = Some(("❌ Clipboard unavailable".to_string(), true));
            }
        }
    }

    // ── Main TUI event loop ───────────────────────────────────────────────────
    pub fn run_loop<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<Option<NoteItem>> {
        loop {
            terminal.draw(|f| self.render_ui(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }

                // ── Modal handling ────────────────────────────────────────────
                match &self.modal.clone() {
                    Modal::ConfirmDelete(_) => {
                        match key.code {
                            KeyCode::Char('y') | KeyCode::Char('Y') => self.delete_selected(),
                            _ => self.modal = Modal::None,
                        }
                        continue;
                    }
                    Modal::NewNoteField(active, cat, title, content) => {
                        let (mut active, mut cat, mut title, mut content) = (*active, cat.clone(), title.clone(), content.clone());
                        match (key.code, key.modifiers) {
                            (KeyCode::Esc, _) => { self.modal = Modal::None; }
                            (KeyCode::Tab, _) => {
                                active = (active + 1) % 3;
                                self.modal = Modal::NewNoteField(active, cat, title, content);
                            }
                            (KeyCode::Char('s'), KeyModifiers::CONTROL)
                            | (KeyCode::Enter, KeyModifiers::CONTROL) => {
                                let c = cat.clone();
                                let t = title.clone();
                                let cnt = content.clone();
                                self.create_note(&c, &t, &cnt);
                            }
                            (KeyCode::Enter, _) => {
                                if active == 0 {
                                    active = 1;
                                    self.modal = Modal::NewNoteField(active, cat, title, content);
                                } else if active == 1 {
                                    active = 2;
                                    self.modal = Modal::NewNoteField(active, cat, title, content);
                                } else {
                                    content.push('\n');
                                    self.modal = Modal::NewNoteField(active, cat, title, content);
                                }
                            }
                            (KeyCode::Backspace, _) => {
                                if active == 0 { cat.pop(); }
                                else if active == 1 { title.pop(); }
                                else { content.pop(); }
                                self.modal = Modal::NewNoteField(active, cat, title, content);
                            }
                            (KeyCode::Char(c), _) => {
                                if active == 0 { cat.push(c); }
                                else if active == 1 { title.push(c); }
                                else { content.push(c); }
                                self.modal = Modal::NewNoteField(active, cat, title, content);
                            }
                            _ => {}
                        }
                        continue;
                    }
                    Modal::None => {}
                }

                // ── Page 2 GWIP-style UX Handling ─────────────────────────────
                if self.page == AppPage::Detail {
                    if self.is_editing_content {
                        // Right Pane Note Content Editor
                        match (key.code, key.modifiers) {
                            (KeyCode::Esc, _) | (KeyCode::Left, KeyModifiers::NONE) => {
                                self.is_editing_content = false;
                            }
                            (KeyCode::Char('s'), KeyModifiers::CONTROL)
                            | (KeyCode::Enter, KeyModifiers::CONTROL) => {
                                self.save_current_detail_edit();
                            }
                            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                                self.copy_selected_note_content();
                            }
                            (KeyCode::Enter, _) => {
                                self.edit_buffer.push('\n');
                            }
                            (KeyCode::Tab, _) => {
                                self.edit_buffer.push_str("    ");
                            }
                            (KeyCode::Backspace, _) => {
                                self.edit_buffer.pop();
                            }
                            (KeyCode::Char(c), _) => {
                                self.edit_buffer.push(c);
                            }
                            _ => {}
                        }
                    } else {
                        // Left Pane Action Menu Navigation
                        match (key.code, key.modifiers) {
                            (KeyCode::Esc, _) | (KeyCode::Char('q'), KeyModifiers::NONE) => {
                                self.page = AppPage::List;
                            }
                            (KeyCode::Up, _) | (KeyCode::Char('k'), KeyModifiers::NONE) => {
                                if self.action_cursor > 0 { self.action_cursor -= 1; }
                            }
                            (KeyCode::Down, _) | (KeyCode::Char('j'), KeyModifiers::NONE) => {
                                if self.action_cursor < ACTION_ITEMS.len() - 1 { self.action_cursor += 1; }
                            }
                            (KeyCode::Tab, _) | (KeyCode::Right, _) => {
                                self.is_editing_content = true;
                            }
                            (KeyCode::Enter, _) => {
                                match self.action_cursor {
                                    0 => self.is_editing_content = true,
                                    1 => self.copy_selected_note_content(),
                                    2 => self.open_in_vscode(),
                                    3 => self.open_folder(),
                                    4 => {
                                        let words = self.edit_buffer.split_whitespace().count();
                                        let chars = self.edit_buffer.chars().count();
                                        self.status_msg = Some((format!("📊 Note stats: {words} words, {chars} characters"), false));
                                    }
                                    5 => {
                                        if let Some(sel) = self.list_state.selected() {
                                            self.modal = Modal::ConfirmDelete(sel);
                                        }
                                    }
                                    6 => self.page = AppPage::List,
                                    _ => {}
                                }
                            }
                            (KeyCode::Char('e'), KeyModifiers::NONE) => self.is_editing_content = true,
                            (KeyCode::Char('c'), KeyModifiers::NONE) | (KeyCode::Char('y'), KeyModifiers::NONE) => self.copy_selected_note_content(),
                            (KeyCode::Char('v'), KeyModifiers::CONTROL) => self.open_in_vscode(),
                            (KeyCode::Char('o'), KeyModifiers::NONE) => self.open_folder(),
                            (KeyCode::Char('d'), KeyModifiers::NONE) => {
                                if let Some(sel) = self.list_state.selected() {
                                    self.modal = Modal::ConfirmDelete(sel);
                                }
                            }
                            _ => {}
                        }
                    }
                    continue;
                }

                // ── Page 1 Normal mode ─────────────────────────────────────────
                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _)
                    | (KeyCode::Char('q'), KeyModifiers::NONE) => return Ok(None),

                    // Open Page 2 (GWIP Note Detail & Action UX)
                    (KeyCode::Enter, _) | (KeyCode::Char('e'), KeyModifiers::NONE) => {
                        self.open_detail_page();
                    }

                    // Copy ONLY note content to clipboard
                    (KeyCode::Char('c'), KeyModifiers::NONE)
                    | (KeyCode::Char('y'), KeyModifiers::NONE) => {
                        self.copy_selected_note_content();
                    }

                    // Open in VS Code directly
                    (KeyCode::Char('v'), KeyModifiers::CONTROL) => {
                        self.open_in_vscode();
                    }

                    // Open containing folder
                    (KeyCode::Char('o'), KeyModifiers::NONE) => {
                        self.open_folder();
                    }

                    (KeyCode::Up, _) | (KeyCode::Char('k'), KeyModifiers::NONE) => self.move_select(-1),
                    (KeyCode::Down, _) | (KeyCode::Char('j'), KeyModifiers::NONE) => self.move_select(1),
                    (KeyCode::PageUp, _) => self.move_select(-10),
                    (KeyCode::PageDown, _) => self.move_select(10),
                    (KeyCode::Home, _) => { self.list_state.select(Some(0)); self.scroll_offset = 0; }
                    (KeyCode::End, _) => {
                        let last = self.filtered_indices.len().saturating_sub(1);
                        self.list_state.select(Some(last));
                    }

                    // Scroll preview
                    (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                        self.scroll_offset = self.scroll_offset.saturating_sub(5);
                    }
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        self.scroll_offset = self.scroll_offset.saturating_add(5);
                    }

                    // Delete
                    (KeyCode::Delete, _) | (KeyCode::Char('d'), KeyModifiers::NONE) => {
                        if let Some(sel) = self.list_state.selected() {
                            self.modal = Modal::ConfirmDelete(sel);
                        }
                    }

                    // New note
                    (KeyCode::Char('n'), KeyModifiers::NONE) => {
                        self.modal = Modal::NewNoteField(0, String::new(), String::new(), String::new());
                    }

                    // Search typing
                    (KeyCode::Backspace, _) => {
                        self.query.pop();
                        self.filter_items();
                    }
                    (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                        self.query.push(c);
                        self.filter_items();
                    }

                    _ => {}
                }
            }
        }
    }

    // ── Render ────────────────────────────────────────────────────────────────
    fn render_ui(&mut self, f: &mut Frame) {
        let area = f.area();

        if self.page == AppPage::Detail {
            self.render_detail_page(f, area);
        } else {
            self.render_list_page(f, area);
        }

        // Modals (rendered on top)
        self.render_modal(f, area);
    }

    fn render_list_page(&mut self, f: &mut Frame, area: Rect) {
        // Full background
        f.render_widget(
            Block::default().style(Style::default().bg(C_BG)),
            area,
        );

        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Banner
                Constraint::Length(3), // Search bar
                Constraint::Min(5),    // Content area
                Constraint::Length(3), // Status bar / shortcuts
            ])
            .split(area);

        // ── Banner ────────────────────────────────────────────────────────────
        let banner_spans = if let Some((ref msg, is_error)) = self.status_msg {
            let color = if is_error { Color::Rgb(255, 80, 80) } else { C_GREEN };
            vec![Span::styled(msg.clone(), Style::default().fg(color).add_modifier(Modifier::BOLD))]
        } else {
            let note_count = self.items.len();
            vec![
                Span::styled("📝  ", Style::default().fg(C_ACCENT)),
                Span::styled("NOTES", Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD)),
                Span::styled(" — Fancybash Note Manager", Style::default().fg(C_TEXT)),
                Span::styled(
                    format!("  ({note_count} notes)"),
                    Style::default().fg(C_DIM),
                ),
            ]
        };

        let banner = Paragraph::new(Line::from(banner_spans))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(C_BORDER))
                    .style(Style::default().bg(C_BG)),
            );
        f.render_widget(banner, outer[0]);

        // ── Search Bar ────────────────────────────────────────────────────────
        let match_count = self.filtered_indices.len();
        let note_count = self.items.len();
        let search_text = Line::from(vec![
            Span::styled(" 🔍 ", Style::default().fg(C_ACCENT)),
            Span::styled(&self.query, Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
            Span::styled("█", Style::default().fg(C_BORDER)),
            Span::styled(
                format!("  ({match_count}/{note_count})"),
                Style::default().fg(C_DIM),
            ),
        ]);
        let search_bar = Paragraph::new(search_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_ACCENT))
                .title(Span::styled(" Search Notes ", Style::default().fg(C_ACCENT)))
                .style(Style::default().bg(C_BG)),
        );
        f.render_widget(search_bar, outer[1]);

        // ── Main content: left list + right preview ────────────────────────────
        let content_panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(outer[2]);

        self.render_list(f, content_panes[0]);
        self.render_preview(f, content_panes[1]);

        // ── Status Bar / Shortcuts ────────────────────────────────────────────
        let status_spans = vec![
            Span::styled(" ↑↓ ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
            Span::styled("Navigate", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_BORDER)),
            Span::styled("↵ ", Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("Open", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_BORDER)),
            Span::styled("n ", Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled("New", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_BORDER)),
            Span::styled("d ", Style::default().fg(Color::Rgb(255, 100, 100)).add_modifier(Modifier::BOLD)),
            Span::styled("Delete", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_BORDER)),
            Span::styled("Ctrl+V ", Style::default().fg(C_VIOLET).add_modifier(Modifier::BOLD)),
            Span::styled("Code", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_BORDER)),
            Span::styled("⎋ ", Style::default().fg(C_YELLOW).add_modifier(Modifier::BOLD)),
            Span::styled("Quit ", Style::default().fg(C_DIM)),
        ];

        let status_bar = Paragraph::new(Line::from(status_spans))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(C_BORDER))
                    .style(Style::default().bg(C_BG)),
            );
        f.render_widget(status_bar, outer[3]);
    }

    fn render_detail_page(&mut self, f: &mut Frame, area: Rect) {
        // Deep space background fill
        f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header banner
                Constraint::Min(0),    // GWIP 2-pane body
                Constraint::Length(3), // Footer status bar
            ])
            .split(area);

        let sel_note = self.selected_note();
        let (cat, title, created) = sel_note
            .map(|n| (n.category.as_str(), n.title.as_str(), n.created_time.as_str()))
            .unwrap_or(("General", "Untitled", "—"));

        // ── Header Banner ──────────────────────────────────────────────────────
        let header_spans = if let Some((ref msg, is_error)) = self.status_msg {
            let color = if is_error { Color::Rgb(255, 80, 80) } else { C_GREEN };
            vec![Span::styled(msg.clone(), Style::default().fg(color).add_modifier(Modifier::BOLD))]
        } else {
            let mode_pill = if self.is_editing_content {
                Span::styled(" STEP 2/2: EDIT NOTE ", Style::default().fg(C_BG).bg(C_CYAN).add_modifier(Modifier::BOLD))
            } else {
                Span::styled(" STEP 2/2: CHOOSE NOTE ACTION ", Style::default().fg(C_BG).bg(C_VIOLET).add_modifier(Modifier::BOLD))
            };

            vec![
                Span::styled(" 📝 FANCYBASH NOTES  ", Style::default().fg(C_BG).bg(C_VIOLET).add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                mode_pill,
                Span::raw("  "),
                Span::styled(format!("📁 {} / {}", cat, title), Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
                Span::raw("  •  "),
                Span::styled(format!("🕐 {created}"), Style::default().fg(C_DIM)),
            ]
        };

        let header = Paragraph::new(Line::from(header_spans))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(C_VIOLET))
                    .style(Style::default().bg(C_CARD)),
            )
            .alignment(Alignment::Left);
        f.render_widget(header, outer[0]);

        // ── Body Layout: Left Pane (32%) | Right Pane (68%) ───────────────────
        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(32), Constraint::Percentage(68)])
            .split(outer[1]);

        self.draw_action_menu(f, body[0]);
        self.draw_detail_right_panel(f, body[1]);

        // ── Footer status bar ──────────────────────────────────────────────────
        let footer_spans = vec![
            Span::styled(" ↑↓ ", Style::default().fg(C_VIOLET).add_modifier(Modifier::BOLD)),
            Span::styled("Navigate", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_BORDER)),
            Span::styled("↵ ", Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)),
            Span::styled("Select", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_BORDER)),
            Span::styled("d ", Style::default().fg(Color::Rgb(255, 100, 100)).add_modifier(Modifier::BOLD)),
            Span::styled("Delete", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_BORDER)),
            Span::styled("Ctrl+V ", Style::default().fg(C_GREEN).add_modifier(Modifier::BOLD)),
            Span::styled("Code", Style::default().fg(C_DIM)),
            Span::styled("  ·  ", Style::default().fg(C_BORDER)),
            Span::styled("⎋ ", Style::default().fg(C_YELLOW).add_modifier(Modifier::BOLD)),
            Span::styled("Back ", Style::default().fg(C_DIM)),
        ];

        let footer = Paragraph::new(Line::from(footer_spans))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(C_VIOLET))
                    .style(Style::default().bg(C_CARD)),
            )
            .alignment(Alignment::Center);
        f.render_widget(footer, outer[2]);
    }

    fn draw_action_menu(&self, f: &mut Frame, area: Rect) {
        let active = !self.is_editing_content;
        let border_color = if active { C_VIOLET } else { C_BORDER };

        let items: Vec<ListItem> = ACTION_ITEMS.iter().enumerate().map(|(idx, item)| {
            let sel = idx == self.action_cursor;
            if sel {
                let label_style = if active {
                    Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(C_TEXT).add_modifier(Modifier::BOLD)
                };
                ListItem::new(Line::from(vec![
                    Span::styled("❯ ", Style::default().fg(C_VIOLET).add_modifier(Modifier::BOLD)),
                    Span::styled(item.emoji, Style::default().fg(C_WHITE)),
                    Span::raw(" "),
                    Span::styled(format!("{:<15}", item.label), label_style),
                    Span::styled(format!(" [{}]", item.shortcut), Style::default().fg(C_YELLOW)),
                ])).style(Style::default().bg(C_CARD))
            } else {
                ListItem::new(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(item.emoji, Style::default().fg(C_DIM)),
                    Span::raw(" "),
                    Span::styled(format!("{:<15}", item.label), Style::default().fg(C_DIM)),
                    Span::styled(format!(" [{}]", item.shortcut), Style::default().fg(Color::Rgb(70, 78, 100))),
                ]))
            }
        }).collect();

        let title_span = if active {
            Span::styled(" 📌 NOTE ACTIONS (ACTIVE) ", Style::default().fg(C_VIOLET).add_modifier(Modifier::BOLD))
        } else {
            Span::styled(" 📌 NOTE ACTIONS ", Style::default().fg(C_DIM))
        };

        let mut state = ListState::default();
        state.select(Some(self.action_cursor));

        f.render_stateful_widget(
            List::new(items).block(
                Block::default()
                    .title(title_span)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(border_color))
                    .style(Style::default().bg(C_CARD)),
            ),
            area,
            &mut state,
        );
    }

    fn draw_detail_right_panel(&self, f: &mut Frame, area: Rect) {
        let editor_active = self.is_editing_content;

        let inner_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5), // Note Metadata & Live Stats Card
                Constraint::Min(0),    // Interactive Editor / Content Preview
            ])
            .split(area);

        let sel_note = self.selected_note();
        let (cat, title, path_str) = sel_note
            .map(|n| (n.category.clone(), n.title.clone(), n.file_path.to_string_lossy().to_string()))
            .unwrap_or(("General".to_string(), "Untitled".to_string(), "—".to_string()));

        let words = self.edit_buffer.split_whitespace().count();
        let chars = self.edit_buffer.chars().count();
        let lines_cnt = self.edit_buffer.lines().count();
        let file_bytes = self.edit_buffer.len();

        // ── 1. Note Info & Metrics Card ───────────────────────────────────────
        f.render_widget(
            Paragraph::new(vec![
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("📁 ", Style::default().fg(C_YELLOW)),
                    Span::styled(cat, Style::default().fg(C_CAT).add_modifier(Modifier::BOLD)),
                    Span::styled(" / ", Style::default().fg(C_DIM)),
                    Span::styled(title, Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
                    Span::styled("  •  ", Style::default().fg(C_DIM)),
                    Span::styled(format!("📊 {words} words │ {chars} chars │ {lines_cnt} lines │ {file_bytes} bytes"), Style::default().fg(C_CYAN)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled("Path: ", Style::default().fg(C_DIM)),
                    Span::styled(path_str, Style::default().fg(C_TEXT)),
                ]),
            ])
            .block(
                Block::default()
                    .title(Span::styled(" ℹ  NOTE METADATA & METRICS ", Style::default().fg(C_VIOLET).add_modifier(Modifier::BOLD)))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(C_BORDER))
                    .style(Style::default().bg(C_CARD)),
            ),
            inner_layout[0],
        );

        // ── 2. Interactive Editor / Content Preview Box ───────────────────────
        let border_style = if editor_active {
            Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(C_BORDER)
        };

        let title_span = if editor_active {
            Span::styled(" ✏️ EDIT NOTE CONTENT (ACTIVE — Ctrl+S to Save) ", Style::default().fg(C_CYAN).add_modifier(Modifier::BOLD))
        } else {
            Span::styled(" 📄 NOTE CONTENT PREVIEW (Press Enter or e to edit) ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD))
        };

        let mut display_lines: Vec<Line> = Vec::new();
        let lines_vec: Vec<&str> = self.edit_buffer.split('\n').collect();
        let total = lines_vec.len();

        if self.edit_buffer.trim().is_empty() && !editor_active {
            display_lines.push(Line::from(Span::styled(
                "  (empty note — press Enter/e to edit content)",
                Style::default().fg(C_DIM).add_modifier(Modifier::ITALIC),
            )));
        } else {
            for (idx, line) in lines_vec.iter().enumerate() {
                if editor_active && idx == total - 1 {
                    display_lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(*line, Style::default().fg(C_WHITE)),
                        Span::styled("█", Style::default().fg(C_CYAN)),
                    ]));
                } else {
                    display_lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(*line, Style::default().fg(C_CONTENT)),
                    ]));
                }
            }
        }

        let editor_widget = Paragraph::new(display_lines)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(border_style)
                    .title(title_span)
                    .style(Style::default().bg(C_BG)),
            );

        f.render_widget(editor_widget, inner_layout[1]);
    }

    fn render_list(&mut self, f: &mut Frame, area: Rect) {
        // Group by category for the header rows
        let mut items: Vec<ListItem> = Vec::new();
        let mut last_cat: Option<String> = None;

        for (display_idx, &orig_idx) in self.filtered_indices.iter().enumerate() {
            let note = &self.items[orig_idx];
            let is_sel = self.list_state.selected() == Some(display_idx);

            // Category header separator
            let show_header = last_cat.as_deref() != Some(&note.category);
            if show_header {
                last_cat = Some(note.category.clone());
                let header_line = Line::from(vec![
                    Span::styled("  📁 ", Style::default().fg(C_YELLOW)),
                    Span::styled(
                        note.category.clone(),
                        Style::default().fg(C_CAT).add_modifier(Modifier::BOLD),
                    ),
                ]);
                items.push(ListItem::new(header_line));
            }

            let line = if is_sel {
                Line::from(vec![
                    Span::styled(" ▶ ", Style::default().fg(C_SELECTED_FG).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        format!("{:<width$}", note.title, width = area.width.saturating_sub(6) as usize),
                        Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD).bg(C_SELECTED_BG),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::styled("   ", Style::default()),
                    Span::styled(&note.title, Style::default().fg(C_TEXT)),
                ])
            };
            items.push(ListItem::new(line));
        }

        if items.is_empty() {
            let placeholder = ListItem::new(Line::from(Span::styled(
                "  No notes found…",
                Style::default().fg(C_DIM).add_modifier(Modifier::ITALIC),
            )));
            items.push(placeholder);
        }

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .title(Span::styled(
                    " Notes ",
                    Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().bg(C_BG)),
        );
        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_preview(&self, f: &mut Frame, area: Rect) {
        let note = self.selected_note();

        let title_span = note
            .map(|n| format!(" 📄 {} ", n.title))
            .unwrap_or_else(|| " Preview ".to_string());

        let preview_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER))
            .title(Span::styled(title_span, Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)))
            .style(Style::default().bg(C_BG));

        let inner = preview_block.inner(area);
        f.render_widget(preview_block, area);

        if inner.height < 3 || inner.width < 5 { return; }

        let mut lines: Vec<Line> = Vec::new();

        if let Some(note) = note {
            // Metadata header
            lines.push(Line::from(vec![
                Span::styled("  📁 ", Style::default().fg(C_YELLOW)),
                Span::styled(&note.category, Style::default().fg(C_CAT).add_modifier(Modifier::BOLD)),
                Span::styled(" / ", Style::default().fg(C_DIM)),
                Span::styled(&note.title, Style::default().fg(C_SELECTED_FG).add_modifier(Modifier::BOLD)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("  🕐 ", Style::default().fg(C_DIM)),
                Span::styled(&note.created_time, Style::default().fg(C_DIM)),
            ]));
            // Divider
            lines.push(Line::from(Span::styled(
                "─".repeat(inner.width as usize),
                Style::default().fg(C_BORDER),
            )));

            if note.content.trim().is_empty() {
                lines.push(Line::from(Span::styled(
                    "  (empty note — press Enter to open & edit)",
                    Style::default().fg(C_DIM).add_modifier(Modifier::ITALIC),
                )));
            } else {
                for content_line in note.content.lines() {
                    lines.push(Line::from(Span::styled(
                        format!("  {content_line}"),
                        Style::default().fg(C_CONTENT),
                    )));
                }
            }
        } else {
            lines.push(Line::from(Span::styled(
                "  Select a note to preview",
                Style::default().fg(C_DIM).add_modifier(Modifier::ITALIC),
            )));
        }

        // Apply scroll
        let skip = self.scroll_offset as usize;
        let visible: Vec<Line> = lines.into_iter().skip(skip).collect();

        f.render_widget(Paragraph::new(visible).wrap(Wrap { trim: false }), inner);
    }

    fn render_modal(&self, f: &mut Frame, area: Rect) {
        match &self.modal {
            Modal::None => {}

            Modal::ConfirmDelete(sel_idx) => {
                let note_title = self
                    .filtered_indices
                    .get(*sel_idx)
                    .and_then(|&orig| self.items.get(orig))
                    .map(|n| format!("'{}' ({})", n.title, n.category))
                    .unwrap_or_default();

                let dialog_area = centered_rect(54, 9, area);
                f.render_widget(Clear, dialog_area);
                let dialog = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("  Delete note ", Style::default().fg(C_TEXT)),
                        Span::styled(&note_title, Style::default().fg(C_PINK).add_modifier(Modifier::BOLD)),
                        Span::styled("?", Style::default().fg(C_TEXT)),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("  Press ", Style::default().fg(C_DIM)),
                        Span::styled("[Y]", Style::default().fg(C_PINK).add_modifier(Modifier::BOLD)),
                        Span::styled(" to confirm, any key to cancel", Style::default().fg(C_DIM)),
                    ]),
                    Line::from(""),
                ])
                .wrap(Wrap { trim: true })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double)
                        .border_style(Style::default().fg(C_PINK))
                        .title(Span::styled(
                            " ⚠  Confirm Delete ",
                            Style::default().fg(C_PINK).add_modifier(Modifier::BOLD),
                        ))
                        .style(Style::default().bg(Color::Rgb(25, 5, 15))),
                );
                f.render_widget(dialog, dialog_area);
            }

            Modal::NewNoteField(active_field, cat, title, content) => {
                let dialog_area = centered_rect(75, 18, area);
                f.render_widget(Clear, dialog_area);

                let outer_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(Style::default().fg(C_ACCENT))
                    .title(Span::styled(
                        " ✏  Create New Note ",
                        Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
                    ))
                    .style(Style::default().bg(Color::Rgb(8, 22, 18)));

                let inner = outer_block.inner(dialog_area);
                f.render_widget(outer_block, dialog_area);

                let rows = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1), // Instructions
                        Constraint::Length(3), // Category
                        Constraint::Length(3), // Title
                        Constraint::Min(5),    // Content
                        Constraint::Length(1), // Hint
                    ])
                    .split(inner);

                // Instructions
                f.render_widget(
                    Paragraph::new(Line::from(vec![
                        Span::styled(" Tab/Enter", Style::default().fg(C_YELLOW)),
                        Span::styled(" switch fields  │  ", Style::default().fg(C_DIM)),
                        Span::styled("Ctrl+S / Ctrl+Enter", Style::default().fg(C_GREEN)),
                        Span::styled(" Create Note  │  ", Style::default().fg(C_DIM)),
                        Span::styled("Esc", Style::default().fg(C_PINK)),
                        Span::styled(" Cancel", Style::default().fg(C_DIM)),
                    ])),
                    rows[0],
                );

                // Category field
                let cat_border_color = if *active_field == 0 { C_ACCENT } else { C_DIM };
                let cat_field = Paragraph::new(Line::from(vec![
                    Span::styled(cat.as_str(), Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
                    if *active_field == 0 { Span::styled("█", Style::default().fg(C_BORDER)) } else { Span::raw("") },
                ]))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(cat_border_color))
                        .title(Span::styled(" 1. Category ", Style::default().fg(cat_border_color))),
                );
                f.render_widget(cat_field, rows[1]);

                // Title field
                let title_border_color = if *active_field == 1 { C_ACCENT } else { C_DIM };
                let title_field = Paragraph::new(Line::from(vec![
                    Span::styled(title.as_str(), Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
                    if *active_field == 1 { Span::styled("█", Style::default().fg(C_BORDER)) } else { Span::raw("") },
                ]))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(title_border_color))
                        .title(Span::styled(" 2. Title ", Style::default().fg(title_border_color))),
                );
                f.render_widget(title_field, rows[2]);

                // Content field
                let content_border_color = if *active_field == 2 { C_CYAN } else { C_DIM };
                let mut content_lines: Vec<Line> = Vec::new();
                let lines_vec: Vec<&str> = content.split('\n').collect();
                let total = lines_vec.len();

                for (idx, line) in lines_vec.iter().enumerate() {
                    if *active_field == 2 && idx == total - 1 {
                        content_lines.push(Line::from(vec![
                            Span::styled(*line, Style::default().fg(C_WHITE)),
                            Span::styled("█", Style::default().fg(C_CYAN)),
                        ]));
                    } else {
                        content_lines.push(Line::from(Span::styled(*line, Style::default().fg(C_WHITE))));
                    }
                }

                let content_field = Paragraph::new(content_lines)
                    .wrap(Wrap { trim: false })
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .border_style(Style::default().fg(content_border_color))
                            .title(Span::styled(" 3. Note Content (Write text here) ", Style::default().fg(content_border_color))),
                    );
                f.render_widget(content_field, rows[3]);

                // Hint
                f.render_widget(
                    Paragraph::new(Line::from(Span::styled(
                        "  Leave Category blank to use 'General'  │  Press Ctrl+S or Ctrl+Enter to save",
                        Style::default().fg(C_DIM).add_modifier(Modifier::ITALIC),
                    ))),
                    rows[4],
                );
            }
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let x = r.x + (r.width.saturating_sub(popup_width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    Rect::new(x, y, popup_width.min(r.width), height.min(r.height))
}

fn is_dir_empty_or_no_notes(root_dir: &PathBuf) -> bool {
    if let Ok(entries) = fs::read_dir(root_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Ok(sub) = fs::read_dir(entry.path()) {
                    if sub.flatten().any(|e| e.path().is_file()) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn seed_sample_notes_if_empty(root_dir: &PathBuf) {
    if !is_dir_empty_or_no_notes(root_dir) {
        return;
    }
    let general = root_dir.join("General");
    let work = root_dir.join("Work");
    let _ = fs::create_dir_all(&general);
    let _ = fs::create_dir_all(&work);

    let files: &[(&PathBuf, &str, &str)] = &[
        (&general, "meet.txt",  "Team sync meeting notes:\n- Review Q3 roadmap\n- Finalize CLI TUI themes\n- Assign PR reviews"),
        (&general, "logo.txt",  "Brand identity assets:\n- SVG color spec: #00D2B4 (teal), #FF50A0 (pink)\n- Font: JetBrains Mono"),
        (&general, "Rihad.txt", "Personal developer profile:\n- Shell: zsh + fancybash\n- Editor: Neovim\n- Focus: Rust, TUI tools"),
        (&work,    "main.txt",  "Work branch deployment checklist:\n1. Run cargo test --all\n2. Bump CHANGELOG.md\n3. Tag release vX.Y.Z\n4. Push to origin"),
    ];
    for (dir, name, content) in files {
        let p = dir.join(name);
        if !p.exists() { let _ = fs::write(&p, content); }
    }
}

fn format_system_time(st: std::time::SystemTime) -> String {
    if let Ok(dur) = st.duration_since(std::time::UNIX_EPOCH) {
        let secs = dur.as_secs();
        let days = secs / 86400;
        let day_secs = secs % 86400;
        let h = (day_secs / 3600 + 6) % 24;
        let m = (day_secs % 3600) / 60;
        let s = day_secs % 60;
        let year = 1970 + days / 365;
        let doy = days % 365;
        let md = [31u64, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut month = 0usize;
        let mut rem = doy;
        for (i, &days_in_m) in md.iter().enumerate() {
            if rem < days_in_m { month = i; break; }
            rem -= days_in_m;
        }
        format!("{year:04}-{:02}-{:02} {h:02}:{m:02}:{s:02}", month + 1, rem + 1)
    } else {
        "—".to_string()
    }
}

fn print_notes_help(root_dir: &PathBuf) {
    let display_dir = root_dir.to_string_lossy();
    println!("\x1b[1;36m📝 Notes Manager — {}\x1b[0m\n", display_dir);
    println!("  \x1b[1;32mnotes\x1b[0m                 Browse all notes with interactive TUI & live preview");
    println!("  \x1b[1;32mnotes add [title]\x1b[0m     Add a note — pick category, enter title, write content");
    println!("  \x1b[1;32mnotes search <query>\x1b[0m  Full-text search inside all notes (e.g. \x1b[36mnotes search \"docker\"\x1b[0m)");
    println!("  \x1b[1;32mnotes find <query>\x1b[0m    Alias for \x1b[32mnotes search\x1b[0m");
    println!("  \x1b[1;32mnotes delete <name>\x1b[0m   Delete a note by title");
    println!("  \x1b[1;32mnotes --help\x1b[0m          Show full usage reference");
}

fn run_tui(app: &mut NotesApp) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = app.run_loop(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Ok(Some(note)) = res {
        println!("\x1b[1;36m📝 {}/{}\x1b[0m", note.category, note.title);
        println!("\x1b[2m🕐 {}\x1b[0m", note.created_time);
        println!("────────────────────────────────────────");
        println!("{}", note.content);
        println!("────────────────────────────────────────");
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            let _ = clipboard.set_text(&note.content);
            println!("\x1b[1;32m📋 Copied note content to clipboard.\x1b[0m");
        }
    }

    Ok(())
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Runs the interactive notes manager (`fancybash notes`).
pub fn run(action_opt: Option<&str>, args: &[String]) -> Result<(), Box<dyn Error>> {
    let root_dir = notes_dir_path();
    fs::create_dir_all(&root_dir)?;

    let action = action_opt.unwrap_or("tui");

    match action {
        "--help" | "-h" | "help" => {
            print_notes_help(&root_dir);
            return Ok(());
        }

        "add" => {
            let initial_title = args.join(" ");
            let mut app = NotesApp::new(root_dir);
            app.modal = Modal::NewNoteField(0, String::new(), initial_title, String::new());
            run_tui(&mut app)?;
        }

        "search" | "find" => {
            let query_str = args.join(" ");
            let mut app = NotesApp::new(root_dir);
            if !query_str.is_empty() {
                app.query = query_str;
                app.filter_items();
            }
            run_tui(&mut app)?;
        }

        "delete" | "rm" => {
            let target = args.join(" ");
            let mut app = NotesApp::new(root_dir);
            if !target.is_empty() {
                app.query = target;
                app.filter_items();
                if !app.filtered_indices.is_empty() {
                    app.modal = Modal::ConfirmDelete(0);
                }
            }
            run_tui(&mut app)?;
        }

        "tui" | "list" | "ls" | _ => {
            let mut app = NotesApp::new(root_dir);
            run_tui(&mut app)?;
        }
    }

    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp() -> PathBuf {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let c = COUNTER.fetch_add(1, Ordering::Relaxed);
        let p = std::env::temp_dir().join(format!("notes_test_{}_{c}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn test_create_and_load_note() {
        let dir = tmp();
        let mut app = NotesApp::new(dir.clone());
        let before = app.items.len();
        app.create_note("Work", "my-task", "initial note content");
        // create_note calls load_notes internally; check item count grew
        assert!(
            app.items.len() > before || app.items.iter().any(|n| n.title == "my-task"),
            "expected 'my-task' to exist after create_note"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_filter_items() {
        let dir = tmp();
        let mut app = NotesApp::new(dir.clone());
        app.query = "meet".to_string();
        app.filter_items();
        assert!(app.filtered_indices.iter().all(|&i| {
            let n = &app.items[i];
            n.title.contains("meet") || n.category.contains("meet") || n.content.contains("meet")
        }));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_delete_note() {
        let dir = tmp();
        let cat = dir.join("Test");
        fs::create_dir_all(&cat).unwrap();
        fs::write(cat.join("hello.txt"), "world").unwrap();
        let mut app = NotesApp::new(dir.clone());
        assert!(!app.items.is_empty());
        app.list_state.select(Some(0));
        app.delete_selected();
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_format_system_time() {
        let t = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000);
        let s = format_system_time(t);
        assert!(s.contains('-'));
    }

    #[test]
    fn test_edit_and_save_note() {
        let dir = tmp();
        let cat = dir.join("Work");
        fs::create_dir_all(&cat).unwrap();
        let file_p = cat.join("todo.txt");
        fs::write(&file_p, "old content").unwrap();

        let mut app = NotesApp::new(dir.clone());
        assert!(!app.items.is_empty());
        app.list_state.select(Some(0));

        // Save new content
        app.save_edited_note(0, "updated content line 1\nupdated line 2".to_string());

        let read_back = fs::read_to_string(&file_p).unwrap();
        assert_eq!(read_back, "updated content line 1\nupdated line 2");
        assert_eq!(app.items[0].content, "updated content line 1\nupdated line 2");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_copy_note_content() {
        let dir = tmp();
        let cat = dir.join("General");
        fs::create_dir_all(&cat).unwrap();
        fs::write(cat.join("sample.txt"), "pure note content without headers").unwrap();

        let mut app = NotesApp::new(dir.clone());
        app.list_state.select(Some(0));
        app.copy_selected_note_content();

        assert!(app.status_msg.is_some());
        let (msg, is_err) = app.status_msg.unwrap();
        assert!(!is_err);
        assert!(msg.contains("Copied note content"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_gwip_detail_page_navigation() {
        let dir = tmp();
        let cat = dir.join("Work");
        fs::create_dir_all(&cat).unwrap();
        fs::write(cat.join("project.txt"), "Initial note content").unwrap();

        let mut app = NotesApp::new(dir.clone());
        app.list_state.select(Some(0));

        // Open detail page (Page 2)
        app.open_detail_page();
        assert_eq!(app.page, AppPage::Detail);
        assert_eq!(app.edit_buffer, "Initial note content");
        assert_eq!(app.action_cursor, 0);
        assert!(!app.is_editing_content);

        // Modify edit buffer and save
        app.edit_buffer = "Modified in Page 2 editor".to_string();
        app.save_current_detail_edit();

        let file_p = cat.join("project.txt");
        let content_on_disk = fs::read_to_string(&file_p).unwrap();
        assert_eq!(content_on_disk, "Modified in Page 2 editor");
        assert_eq!(app.items[0].content, "Modified in Page 2 editor");

        let _ = fs::remove_dir_all(&dir);
    }
}
