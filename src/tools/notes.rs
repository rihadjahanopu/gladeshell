// =============================================================================
//  src/tools/notes.rs — Plain-text notes manager (`notes`) (Phase 5)
//
//  Pure Rust Ratatui + Crossterm dual-pane notes manager UI matching design:
//  Left Pane: Search Notes prompt, match count, category folder list (📁 Category ➔ Title).
//  Right Pane: Live note content preview with created date and green divider.
// =============================================================================

use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

fn notes_dir_path() -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join(".my_notes")
}

#[derive(Debug, Clone)]
pub struct NoteItem {
    pub category: String,
    pub title: String,
    pub file_path: PathBuf,
    pub created_time: String,
    pub content: String,
}

pub struct NotesApp {
    pub root_dir: PathBuf,
    pub items: Vec<NoteItem>,
    pub filtered_indices: Vec<usize>,
    pub list_state: ListState,
    pub query: String,
}

impl NotesApp {
    pub fn new(root_dir: PathBuf) -> Self {
        let mut app = Self {
            root_dir,
            items: Vec::new(),
            filtered_indices: Vec::new(),
            list_state: ListState::default(),
            query: String::new(),
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
                                        .unwrap_or_else(|_| "2026-09-07 16:14:05".to_string());

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

        self.items
            .sort_by(|a, b| a.category.cmp(&b.category).then_with(|| a.title.cmp(&b.title)));
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
            self.list_state.select(Some(0));
        }
    }

    pub fn run_loop<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<Option<NoteItem>> {
        loop {
            terminal.draw(|f| self.render_ui(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != event::KeyEventKind::Press {
                    continue;
                }

                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _)
                    | (KeyCode::Char('q'), KeyModifiers::NONE)
                    | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        return Ok(None);
                    }
                    (KeyCode::Enter, _) => {
                        if let Some(sel) = self.list_state.selected() {
                            if sel < self.filtered_indices.len() {
                                let orig_idx = self.filtered_indices[sel];
                                return Ok(Some(self.items[orig_idx].clone()));
                            }
                        }
                        return Ok(None);
                    }
                    (KeyCode::Up, _) | (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                        self.move_select(-1);
                    }
                    (KeyCode::Down, _) | (KeyCode::Char('j'), KeyModifiers::CONTROL) => {
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

    fn render_ui(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();

        let outer_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        frame.render_widget(outer_block, area);

        let inner_margin = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(42), // Left Pane: Notes Search & List
                Constraint::Percentage(58), // Right Pane: Live Note Content Box
            ])
            .split(inner_margin);

        // ── Render Left Pane ─────────────────────────────────────────────────
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // 🔍 Search Notes ➔ ...
                Constraint::Length(1), // Counter (4/4)
                Constraint::Length(1), // Green/DarkGray Divider line
                Constraint::Min(4),    // List of notes
            ])
            .split(main_chunks[0]);

        // Search Input Line
        let search_line = Line::from(vec![
            Span::styled("🔍 Search Notes ➔ ", Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)),
            Span::styled(&self.query, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled("|", Style::default().fg(Color::Green)),
        ]);
        frame.render_widget(Paragraph::new(search_line), left_chunks[0]);

        // Counter Line
        let total_count = self.items.len();
        let match_count = self.filtered_indices.len();
        let counter_str = format!("{}/{}", match_count, total_count);
        let counter_line = Line::from(vec![
            Span::styled(counter_str, Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
        ]);
        frame.render_widget(Paragraph::new(counter_line), left_chunks[1]);

        // Divider Line
        let divider_len = left_chunks[2].width as usize;
        let divider_str = "─".repeat(divider_len);
        frame.render_widget(Paragraph::new(Line::from(Span::styled(&divider_str, Style::default().fg(Color::DarkGray)))), left_chunks[2]);

        // List Items
        let list_items: Vec<ListItem> = self
            .filtered_indices
            .iter()
            .enumerate()
            .map(|(i, &orig_idx)| {
                let is_cursor = self.list_state.selected() == Some(i);
                let item = &self.items[orig_idx];

                let bar_span = if is_cursor {
                    Span::styled("█ ", Style::default().fg(Color::Rgb(255, 0, 128)))
                } else {
                    Span::raw("  ")
                };

                let folder_span = Span::styled("📁 ", Style::default().fg(Color::Yellow));
                let cat_span = Span::styled(&item.category, Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD));
                let arrow_span = Span::styled(" ➔ ", Style::default().fg(Color::LightCyan));
                let title_span = Span::styled(
                    &item.title,
                    Style::default().fg(Color::LightGreen).add_modifier(if is_cursor { Modifier::BOLD } else { Modifier::empty() }),
                );

                ListItem::new(Line::from(vec![
                    bar_span,
                    folder_span,
                    cat_span,
                    arrow_span,
                    title_span,
                ]))
            })
            .collect();

        let list_widget = List::new(list_items)
            .block(Block::default().borders(Borders::NONE));

        frame.render_stateful_widget(list_widget, left_chunks[3], &mut self.list_state);

        // ── Render Right Pane ────────────────────────────────────────────────
        let selected_note = self
            .list_state
            .selected()
            .and_then(|idx| self.filtered_indices.get(idx))
            .map(|&orig_idx| &self.items[orig_idx]);

        render_note_preview(frame, main_chunks[1], selected_note);
    }
}

fn render_note_preview(frame: &mut ratatui::Frame, area: Rect, note: Option<&NoteItem>) {
    let preview_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner = preview_block.inner(area);
    frame.render_widget(preview_block, area);

    if inner.height < 3 || inner.width < 5 {
        return;
    }

    let mut lines = Vec::new();

    if let Some(item) = note {
        lines.push(Line::from(vec![
            Span::styled(&item.title, Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
        ]));

        lines.push(Line::from(vec![
            Span::styled("Created: ", Style::default().fg(Color::LightGreen)),
            Span::styled(&item.created_time, Style::default().fg(Color::LightGreen)),
        ]));

        lines.push(Line::from(Span::styled(
            "─".repeat(inner.width as usize),
            Style::default().fg(Color::Green),
        )));

        if item.content.trim().is_empty() {
            lines.push(Line::from(Span::styled("  (empty note)", Style::default().fg(Color::DarkGray))));
        } else {
            for content_line in item.content.lines() {
                lines.push(Line::from(Span::styled(content_line, Style::default().fg(Color::LightGreen))));
            }
        }
    } else {
        lines.push(Line::from(Span::styled("No note selected", Style::default().fg(Color::DarkGray))));
    }

    frame.render_widget(Paragraph::new(lines), inner);
}

fn seed_sample_notes_if_empty(root_dir: &PathBuf) {
    let general = root_dir.join("General");
    let work = root_dir.join("Work");
    let _ = fs::create_dir_all(&general);
    let _ = fs::create_dir_all(&work);

    let meet_file = general.join("meet.txt");
    if !meet_file.exists() {
        let _ = fs::write(&meet_file, "Team sync meeting notes:\n- Review Q3 roadmap\n- Finalize CLI TUI themes");
    }

    let logo_file = general.join("logo.txt");
    if !logo_file.exists() {
        let _ = fs::write(&logo_file, "Brand identity logo assets and SVG color specs.");
    }

    let rihad_file = general.join("Rihad.txt");
    if !rihad_file.exists() {
        let _ = fs::write(&rihad_file, "Personal developer profile & shell config tweaks.");
    }

    let main_file = work.join("main.txt");
    if !main_file.exists() {
        let _ = fs::write(&main_file, "Work main branch deployment checklist.");
    }
}

fn format_system_time(st: std::time::SystemTime) -> String {
    if let Ok(dur) = st.duration_since(std::time::UNIX_EPOCH) {
        let secs = dur.as_secs();
        let days_since_epoch = secs / 86400;
        let day_secs = secs % 86400;
        let hours = (day_secs / 3600 + 6) % 24; // GMT+6 approximation
        let mins = (day_secs % 3600) / 60;
        let seconds = day_secs % 60;

        let year = 1970 + days_since_epoch / 365;
        let doy = days_since_epoch % 365;
        let months_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut month = 0usize;
        let mut rem = doy;
        for (i, &md) in months_days.iter().enumerate() {
            if rem < md {
                month = i;
                break;
            }
            rem -= md;
        }
        let day = rem + 1;
        format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month + 1, day, hours, mins, seconds)
    } else {
        "2026-09-07 16:14:05".to_string()
    }
}

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

            if let Ok(Some(selected_note)) = res {
                println!("\x1b[1;36m📝 Note: {}/{}\x1b[0m", selected_note.category, selected_note.title);
                println!("\x1b[1;32mCreated: {}\x1b[0m", selected_note.created_time);
                println!("────────────────────────────────────────");
                println!("{}", selected_note.content);
                println!("────────────────────────────────────────");

                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(&selected_note.content);
                    println!("\x1b[1;32m📋 Note content copied to clipboard.\x1b[0m");
                }
            }
        }

        "add" => {
            let title = if !args.is_empty() {
                args.join(" ")
            } else {
                "Untitled Note".to_string()
            };
            let cat_dir = root_dir.join("General");
            fs::create_dir_all(&cat_dir)?;
            let file_path = cat_dir.join(format!("{}.txt", title));
            fs::write(&file_path, "New note content")?;
            println!("✅ Note created: General/{}", title);
        }

        "delete" | "rm" => {
            if let Some(target) = args.first() {
                let path = root_dir.join("General").join(format!("{}.txt", target));
                if path.exists() {
                    let _ = fs::remove_file(path);
                    println!("🗑️ Note deleted.");
                }
            }
        }

        _ => {
            println!("Usage: fancybash notes [add | list | search | delete]");
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn list_categories(root_dir: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let mut cats = Vec::new();
    if let Ok(entries) = fs::read_dir(root_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                if let Some(name) = entry.file_name().to_str() {
                    cats.push(name.to_string());
                }
            }
        }
    }
    cats.sort();
    Ok(cats)
}

#[allow(dead_code)]
fn collect_all_notes(root_dir: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }
    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_categories_empty() {
        let temp = std::env::temp_dir().join(format!("test_notes_{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();
        let cats = list_categories(&temp).unwrap();
        assert!(cats.is_empty());
        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_collect_all_notes_empty_dir() {
        let temp = std::env::temp_dir().join(format!("test_notes_collect_{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();
        let notes = collect_all_notes(&temp).unwrap();
        assert!(notes.is_empty());
        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_note_write_and_read() {
        let temp = std::env::temp_dir().join(format!("test_note_rw_{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();
        let file = temp.join("test.txt");
        fs::write(&file, "hello fancybash notes").unwrap();
        let content = fs::read_to_string(&file).unwrap();
        assert_eq!(content, "hello fancybash notes");
        let _ = fs::remove_dir_all(&temp);
    }
}

