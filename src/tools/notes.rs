// =============================================================================
//  src/tools/notes.rs — FANCYBASH Notes Manager (Modern fkill-style TUI)
// =============================================================================

use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

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

// ── Colour Palette (fkill-consistent, notes-tinted cyan/green) ───────────────
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

// ── Modal Mode ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Modal {
    None,
    ConfirmDelete(usize),                // index into filtered_indices
    NewNoteField(u8, String, String),    // active_field (0=cat,1=title), cat, title
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

    fn create_note(&mut self, category: &str, title: &str) {
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
        if let Err(e) = fs::write(&file_path, "") {
            self.status_msg = Some((format!("❌ {e}"), true));
        } else {
            self.status_msg = Some((format!("✅ Created '{cat}/{ttl}'"), false));
            self.load_notes();
        }
        self.modal = Modal::None;
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
                    Modal::NewNoteField(active, cat, title) => {
                        let (mut active, mut cat, mut title) = (*active, cat.clone(), title.clone());
                        match key.code {
                            KeyCode::Esc => { self.modal = Modal::None; }
                            KeyCode::Tab => { active = 1 - active; self.modal = Modal::NewNoteField(active, cat, title); }
                            KeyCode::Enter => {
                                let c = cat.clone();
                                let t = title.clone();
                                self.create_note(&c, &t);
                            }
                            KeyCode::Backspace => {
                                if active == 0 { cat.pop(); } else { title.pop(); }
                                self.modal = Modal::NewNoteField(active, cat, title);
                            }
                            KeyCode::Char(c) => {
                                if active == 0 { cat.push(c); } else { title.push(c); }
                                self.modal = Modal::NewNoteField(active, cat, title);
                            }
                            _ => {}
                        }
                        continue;
                    }
                    Modal::None => {}
                }

                // ── Normal mode ───────────────────────────────────────────────
                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _)
                    | (KeyCode::Char('q'), KeyModifiers::NONE)
                    | (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Ok(None),

                    (KeyCode::Enter, _) => {
                        if let Some(note) = self.selected_note().cloned() {
                            return Ok(Some(note));
                        }
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
                        self.modal = Modal::NewNoteField(0, String::new(), String::new());
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
                Constraint::Length(3), // Status / footer
            ])
            .split(area);

        // ── Banner ────────────────────────────────────────────────────────────
        let note_count = self.items.len();
        let banner = Paragraph::new(Line::from(vec![
            Span::styled("📝  ", Style::default().fg(C_ACCENT)),
            Span::styled("NOTES", Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD)),
            Span::styled(" — Fancybash Note Manager", Style::default().fg(C_TEXT)),
            Span::styled(
                format!("  ({note_count} notes)"),
                Style::default().fg(C_DIM),
            ),
        ]))
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

        // ── Status / Footer ───────────────────────────────────────────────────
        let status_text = if let Some((ref msg, is_error)) = self.status_msg {
            let color = if is_error { Color::Rgb(255, 80, 80) } else { C_GREEN };
            Line::from(vec![Span::styled(msg.clone(), Style::default().fg(color).add_modifier(Modifier::BOLD))])
        } else {
            Line::from(vec![
                Span::styled(" ↑↓/jk Navigate", Style::default().fg(C_DIM)),
                Span::styled("  │  ", Style::default().fg(C_BORDER)),
                Span::styled("Type to search", Style::default().fg(C_DIM)),
                Span::styled("  │  ", Style::default().fg(C_BORDER)),
                Span::styled("n New", Style::default().fg(C_ACCENT)),
                Span::styled("  │  ", Style::default().fg(C_BORDER)),
                Span::styled("d Delete", Style::default().fg(C_PINK)),
                Span::styled("  │  ", Style::default().fg(C_BORDER)),
                Span::styled("Enter Open", Style::default().fg(C_GREEN)),
                Span::styled("  │  ", Style::default().fg(C_BORDER)),
                Span::styled("q Quit", Style::default().fg(C_DIM)),
            ])
        };
        let status_bar = Paragraph::new(status_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(C_DIM))
                    .style(Style::default().bg(C_BG)),
            );
        f.render_widget(status_bar, outer[3]);

        // ── Modals (rendered last, on top) ────────────────────────────────────
        self.render_modal(f, area);
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

            Modal::NewNoteField(active_field, cat, title) => {
                let dialog_area = centered_rect(60, 12, area);
                f.render_widget(Clear, dialog_area);

                let outer_block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(Style::default().fg(C_ACCENT))
                    .title(Span::styled(
                        " ✏  New Note ",
                        Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
                    ))
                    .style(Style::default().bg(Color::Rgb(8, 22, 18)));

                let inner = outer_block.inner(dialog_area);
                f.render_widget(outer_block, dialog_area);

                let rows = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ])
                    .split(inner);

                // Instructions
                f.render_widget(
                    Paragraph::new(Line::from(vec![
                        Span::styled("  Tab", Style::default().fg(C_YELLOW)),
                        Span::styled(" to switch fields  │  ", Style::default().fg(C_DIM)),
                        Span::styled("Enter", Style::default().fg(C_GREEN)),
                        Span::styled(" to create  │  ", Style::default().fg(C_DIM)),
                        Span::styled("Esc", Style::default().fg(C_DIM)),
                        Span::styled(" cancel", Style::default().fg(C_DIM)),
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
                        .title(Span::styled(" Category ", Style::default().fg(cat_border_color))),
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
                        .title(Span::styled(" Title ", Style::default().fg(title_border_color))),
                );
                f.render_widget(title_field, rows[2]);

                // Hint
                f.render_widget(
                    Paragraph::new(Line::from(Span::styled(
                        "  Leave Category blank to use 'General'",
                        Style::default().fg(C_DIM).add_modifier(Modifier::ITALIC),
                    ))),
                    rows[3],
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

// ── Public entry point ────────────────────────────────────────────────────────

/// Runs the interactive notes manager (`fancybash notes`).
pub fn run(action_opt: Option<&str>, args: &[String]) -> Result<(), Box<dyn Error>> {
    let root_dir = notes_dir_path();
    fs::create_dir_all(&root_dir)?;

    let action = action_opt.unwrap_or("tui");

    match action {
        "tui" | "list" | "ls" | "search" => {
            let mut app = NotesApp::new(root_dir);

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
                    println!("\x1b[1;32m📋 Copied to clipboard.\x1b[0m");
                }
            }
        }

        "add" => {
            let title = if !args.is_empty() { args.join(" ") } else { "Untitled".to_string() };
            let cat_dir = root_dir.join("General");
            fs::create_dir_all(&cat_dir)?;
            fs::write(cat_dir.join(format!("{title}.txt")), "")?;
            println!("✅ Note created: General/{title}");
        }

        "delete" | "rm" => {
            if let Some(target) = args.first() {
                let path = root_dir.join("General").join(format!("{target}.txt"));
                if path.exists() {
                    fs::remove_file(path)?;
                    println!("🗑  Note deleted: {target}");
                } else {
                    println!("❌ Not found: {target}");
                }
            }
        }

        _ => println!("Usage: fancybash notes [add | list | search | delete]"),
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
        app.create_note("Work", "my-task");
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
}
