// =============================================================================
//  src/tools/theme_picker.rs — Interactive TUI Theme Picker with Live Preview
//
//  Layout (fkill-style):
//    ┌─ BANNER ─────────────────────────────────────────────────────────────────┐
//    ├─ SEARCH ─────────────────────────────────────────────────────────────────┤
//    ├─ THEME LIST (40%) ──────┬─ LIVE PREVIEW (60%) ──────────────────────────┤
//    │ ▶ catppuccin  🐱        │  📄 Theme: catppuccin                         │
//    │   tokyonight  🌌        │  ─────────────────────────────────────────────│
//    │   dracula     🧛        │  🐱 user@host ~/path  [🌿 main]               │
//    │   ...                   │  ❯❯❯                                          │
//    ├─ STATUS / KEYBINDS ──────────────────────────────────────────────────────┤
// =============================================================================

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
use std::io;

use crate::core::prompt::{
    active_theme_id, render, set_active_theme, PromptContext, THEMES,
};

// ── Colour Palette ────────────────────────────────────────────────────────────
const C_BG:          Color = Color::Rgb(8, 10, 20);
const C_BORDER:      Color = Color::Rgb(130, 100, 255);   // violet accent (fkill-inspired)
const C_ACCENT:      Color = Color::Rgb(160, 130, 255);   // bright violet
const C_SELECTED_BG: Color = Color::Rgb(30, 20, 60);      // deep violet row bg
const C_SELECTED_FG: Color = Color::Rgb(200, 170, 255);   // selected text
const C_DIM:         Color = Color::Rgb(80, 80, 110);
const C_TEXT:        Color = Color::Rgb(210, 215, 235);
const C_GREEN:       Color = Color::Rgb(80, 220, 140);
const C_YELLOW:      Color = Color::Rgb(255, 210, 80);
const C_PINK:        Color = Color::Rgb(255, 80, 160);
const C_ACTIVE:      Color = Color::Rgb(0, 240, 180);     // ✅ active theme marker
const C_WHITE:       Color = Color::Rgb(255, 255, 255);

// ── App State ─────────────────────────────────────────────────────────────────

struct ThemePickerApp {
    /// Indices into THEMES that pass the current filter
    filtered: Vec<usize>,
    list_state: ListState,
    query: String,
    /// Currently-active theme index (persisted on disk)
    active_idx: usize,
    /// Confirm-apply dialog visible
    confirm: bool,
    /// Status message after apply
    status: Option<(String, bool)>,
}

impl ThemePickerApp {
    fn new() -> Self {
        let active_idx = active_theme_id();
        let filtered: Vec<usize> = (0..THEMES.len()).collect();

        // Pre-select the active theme
        let sel = filtered.iter().position(|&i| i == active_idx).unwrap_or(0);
        let mut list_state = ListState::default();
        list_state.select(Some(sel));

        Self {
            filtered,
            list_state,
            query: String::new(),
            active_idx,
            confirm: false,
            status: None,
        }
    }

    fn refilter(&mut self) {
        let q = self.query.to_lowercase();
        let prev_sel = self.selected_theme_idx();
        self.filtered = THEMES
            .iter()
            .enumerate()
            .filter(|(_, t)| {
                q.is_empty()
                    || t.name.to_lowercase().contains(&q)
                    || t.emoji.contains(&*q)
            })
            .map(|(i, _)| i)
            .collect();

        // Try to keep same theme selected; else clamp to 0
        let new_sel = if let Some(prev) = prev_sel {
            self.filtered.iter().position(|&i| i == prev).unwrap_or(0)
        } else {
            0
        };
        if self.filtered.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(new_sel));
        }
    }

    fn selected_theme_idx(&self) -> Option<usize> {
        let sel = self.list_state.selected()?;
        self.filtered.get(sel).copied()
    }

    fn move_select(&mut self, delta: i32) {
        if self.filtered.is_empty() { return; }
        let cur = self.list_state.selected().unwrap_or(0) as i32;
        let len = self.filtered.len() as i32;
        self.list_state.select(Some((cur + delta).rem_euclid(len) as usize));
    }

    fn apply_selected(&mut self) {
        if let Some(idx) = self.selected_theme_idx() {
            let name = THEMES[idx].name;
            match set_active_theme(name) {
                Ok(_) => {
                    self.active_idx = idx;
                    self.status = Some((
                        format!("✅ Theme '{}' applied! Run: source ~/.zshrc", name),
                        false,
                    ));
                }
                Err(e) => {
                    self.status = Some((format!("❌ {e}"), true));
                }
            }
        }
        self.confirm = false;
    }

    // ── Event loop ────────────────────────────────────────────────────────────
    fn run_loop<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.render_ui(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }

                // Confirm dialog
                if self.confirm {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                            self.apply_selected();
                        }
                        _ => { self.confirm = false; }
                    }
                    continue;
                }

                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _)
                    | (KeyCode::Char('q'), KeyModifiers::NONE)
                    | (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,

                    (KeyCode::Up, _)   | (KeyCode::Char('k'), KeyModifiers::NONE) => self.move_select(-1),
                    (KeyCode::Down, _) | (KeyCode::Char('j'), KeyModifiers::NONE) => self.move_select(1),
                    (KeyCode::PageUp, _)   => self.move_select(-10),
                    (KeyCode::PageDown, _) => self.move_select(10),
                    (KeyCode::Home, _) => { self.list_state.select(Some(0)); }
                    (KeyCode::End, _)  => {
                        let last = self.filtered.len().saturating_sub(1);
                        self.list_state.select(Some(last));
                    }

                    // Apply with Enter (show confirm dialog)
                    (KeyCode::Enter, _) => {
                        self.status = None;
                        self.confirm = true;
                    }

                    // Apply immediately with Space (no confirm)
                    (KeyCode::Char(' '), KeyModifiers::NONE) => {
                        self.status = None;
                        self.apply_selected();
                    }

                    // Search
                    (KeyCode::Backspace, _) => {
                        self.query.pop();
                        self.refilter();
                    }
                    (KeyCode::Char(c), KeyModifiers::NONE)
                    | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                        self.query.push(c);
                        self.refilter();
                    }

                    _ => {}
                }
            }
        }
        Ok(())
    }

    // ── Render ────────────────────────────────────────────────────────────────
    fn render_ui(&mut self, f: &mut Frame) {
        let area = f.area();

        // Full background
        f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Banner
                Constraint::Length(3), // Search
                Constraint::Min(5),    // Content
                Constraint::Length(3), // Footer
            ])
            .split(area);

        // ── Banner ────────────────────────────────────────────────────────────
        let n = THEMES.len();
        let active_name = THEMES[self.active_idx].name;
        let banner = Paragraph::new(Line::from(vec![
            Span::styled("🎨  ", Style::default().fg(C_ACCENT)),
            Span::styled("THEME PICKER", Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD)),
            Span::styled(" — Fancybash", Style::default().fg(C_TEXT)),
            Span::styled(format!("  ({n} themes)"), Style::default().fg(C_DIM)),
            Span::styled("  │  Active: ", Style::default().fg(C_DIM)),
            Span::styled(active_name, Style::default().fg(C_ACTIVE).add_modifier(Modifier::BOLD)),
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
        let match_count = self.filtered.len();
        let search_bar = Paragraph::new(Line::from(vec![
            Span::styled(" 🔍 ", Style::default().fg(C_ACCENT)),
            Span::styled(&self.query, Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
            Span::styled("█", Style::default().fg(C_BORDER)),
            Span::styled(format!("  ({match_count}/{n})"), Style::default().fg(C_DIM)),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_ACCENT))
                .title(Span::styled(" Search Theme ", Style::default().fg(C_ACCENT)))
                .style(Style::default().bg(C_BG)),
        );
        f.render_widget(search_bar, outer[1]);

        // ── Content panes ─────────────────────────────────────────────────────
        let panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
            .split(outer[2]);

        self.render_list(f, panes[0]);
        self.render_preview(f, panes[1]);

        // ── Footer ────────────────────────────────────────────────────────────
        let footer_text = if let Some((ref msg, is_err)) = self.status {
            let col = if is_err { Color::Rgb(255, 80, 80) } else { C_GREEN };
            Line::from(Span::styled(msg.clone(), Style::default().fg(col).add_modifier(Modifier::BOLD)))
        } else {
            Line::from(vec![
                Span::styled(" ↑↓/jk ", Style::default().fg(C_DIM)),
                Span::styled("Navigate", Style::default().fg(C_DIM)),
                Span::styled("  │  ", Style::default().fg(C_BORDER)),
                Span::styled("Type ", Style::default().fg(C_DIM)),
                Span::styled("Search", Style::default().fg(C_DIM)),
                Span::styled("  │  ", Style::default().fg(C_BORDER)),
                Span::styled("Enter ", Style::default().fg(C_YELLOW)),
                Span::styled("Apply (confirm)", Style::default().fg(C_DIM)),
                Span::styled("  │  ", Style::default().fg(C_BORDER)),
                Span::styled("Space ", Style::default().fg(C_ACCENT)),
                Span::styled("Quick Apply", Style::default().fg(C_DIM)),
                Span::styled("  │  ", Style::default().fg(C_BORDER)),
                Span::styled("q ", Style::default().fg(C_DIM)),
                Span::styled("Quit", Style::default().fg(C_DIM)),
            ])
        };
        f.render_widget(
            Paragraph::new(footer_text)
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(C_DIM))
                        .style(Style::default().bg(C_BG)),
                ),
            outer[3],
        );

        // ── Confirm dialog (on top) ────────────────────────────────────────────
        if self.confirm {
            self.render_confirm(f, area);
        }
    }

    fn render_list(&mut self, f: &mut Frame, area: Rect) {
        let mut items: Vec<ListItem> = Vec::new();

        for (display_idx, &theme_idx) in self.filtered.iter().enumerate() {
            let theme = &THEMES[theme_idx];
            let is_sel    = self.list_state.selected() == Some(display_idx);
            let is_active = theme_idx == self.active_idx;

            let active_marker = if is_active {
                Span::styled("✓ ", Style::default().fg(C_ACTIVE).add_modifier(Modifier::BOLD))
            } else {
                Span::raw("  ")
            };

            let cursor = if is_sel {
                Span::styled("▶ ", Style::default().fg(C_SELECTED_FG).add_modifier(Modifier::BOLD))
            } else {
                Span::raw("  ")
            };

            let emoji_span = Span::styled(
                format!("{:<3}", theme.emoji),
                Style::default().fg(C_YELLOW),
            );

            let name_style = if is_sel {
                Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD).bg(C_SELECTED_BG)
            } else if is_active {
                Style::default().fg(C_ACTIVE)
            } else {
                Style::default().fg(C_TEXT)
            };

            let num_span = Span::styled(
                format!("{:02} ", theme_idx),
                Style::default().fg(C_DIM),
            );
            let name_span = Span::styled(theme.name, name_style);

            let line = Line::from(vec![active_marker, cursor, num_span, emoji_span, name_span]);
            items.push(ListItem::new(line));
        }

        if items.is_empty() {
            items.push(ListItem::new(Line::from(Span::styled(
                "  No themes match…",
                Style::default().fg(C_DIM).add_modifier(Modifier::ITALIC),
            ))));
        }

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .title(Span::styled(
                    " Themes ",
                    Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().bg(C_BG)),
        );
        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_preview(&self, f: &mut Frame, area: Rect) {
        let theme_idx = self.selected_theme_idx().unwrap_or(0);
        let theme = &THEMES[theme_idx];

        let preview_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER))
            .title(Span::styled(
                format!(" 🎨 {} ", theme.name),
                Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
            ))
            .style(Style::default().bg(C_BG));

        let inner = preview_block.inner(area);
        f.render_widget(preview_block, area);

        if inner.height < 4 || inner.width < 20 { return; }

        let mut lines: Vec<Line> = Vec::new();

        // ── Theme metadata ────────────────────────────────────────────────────
        lines.push(Line::from(vec![
            Span::styled("  Theme  ", Style::default().fg(C_DIM)),
            Span::styled(theme.name, Style::default().fg(C_SELECTED_FG).add_modifier(Modifier::BOLD)),
            if theme_idx == self.active_idx {
                Span::styled("  ✓ ACTIVE", Style::default().fg(C_ACTIVE).add_modifier(Modifier::BOLD))
            } else {
                Span::raw("")
            },
        ]));
        lines.push(Line::from(vec![
            Span::styled("  Index  ", Style::default().fg(C_DIM)),
            Span::styled(format!("#{theme_idx:02}"), Style::default().fg(C_YELLOW)),
            Span::styled("   Emoji  ", Style::default().fg(C_DIM)),
            Span::styled(theme.emoji, Style::default().fg(C_YELLOW)),
            Span::styled("   Prompt  ", Style::default().fg(C_DIM)),
            Span::styled(theme.prompt_char, Style::default().fg(C_GREEN)),
        ]));
        if !theme.line1_prefix.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  Prefix ", Style::default().fg(C_DIM)),
                Span::styled(theme.line1_prefix, Style::default().fg(C_TEXT)),
                Span::styled(" / ", Style::default().fg(C_DIM)),
                Span::styled(theme.line2_prefix, Style::default().fg(C_TEXT)),
            ]));
        }

        // Divider
        lines.push(Line::from(Span::styled(
            "─".repeat(inner.width as usize),
            Style::default().fg(C_BORDER),
        )));

        // ── Live prompt preview using the real renderer ───────────────────────
        lines.push(Line::from(Span::styled(
            "  Live Prompt Preview:",
            Style::default().fg(C_DIM).add_modifier(Modifier::ITALIC),
        )));
        lines.push(Line::from(""));

        // Build a realistic PromptContext
        let mut ctx = PromptContext {
            theme_id: theme_idx,
            last_exit: 0,
            ..Default::default()
        };

        // Fill cwd = ~/projects/fancybash
        let cwd_str = b"~/projects/fancybash";
        ctx.cwd[..cwd_str.len()].copy_from_slice(cwd_str);
        ctx.cwd_len = cwd_str.len();

        // Fill user = rihad
        let user_str = b"rihad";
        ctx.user[..user_str.len()].copy_from_slice(user_str);
        ctx.user_len = user_str.len();

        // Fill host = arch
        let host_str = b"arch";
        ctx.host[..host_str.len()].copy_from_slice(host_str);
        ctx.host_len = host_str.len();

        // Fill git branch = main
        let branch_str = b"main";
        ctx.git_branch[..branch_str.len()].copy_from_slice(branch_str);
        ctx.git_branch_len = branch_str.len();
        ctx.git_dirty = false;

        // Render to buffer — use shell=2 (raw ANSI, no Zsh/Bash wrapping)
        ctx.shell = 2;
        let mut buf = [0u8; 1024];
        if let Ok(n) = render(&ctx, &mut buf) {
            let rendered = String::from_utf8_lossy(&buf[..n]);
            // Split into display lines, strip ANSI for ratatui display
            for raw_line in rendered.lines() {
                let clean = strip_ansi(raw_line);
                // Re-colorize based on theme colors (approximate)
                lines.push(Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(clean, Style::default().fg(C_TEXT).add_modifier(Modifier::BOLD)),
                ]));
            }
        }

        // ── Color swatches ────────────────────────────────────────────────────
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "─".repeat(inner.width as usize),
            Style::default().fg(C_BORDER),
        )));
        lines.push(Line::from(vec![
            Span::styled("  Colors  ", Style::default().fg(C_DIM)),
            Span::styled("██", Style::default().fg(theme_color_to_ratatui(theme.user_color))),
            Span::styled(" user  ", Style::default().fg(C_DIM)),
            Span::styled("██", Style::default().fg(theme_color_to_ratatui(theme.path_color))),
            Span::styled(" path  ", Style::default().fg(C_DIM)),
            Span::styled("██", Style::default().fg(theme_color_to_ratatui(theme.git_color))),
            Span::styled(" git", Style::default().fg(C_DIM)),
        ]));

        f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
    }

    fn render_confirm(&self, f: &mut Frame, area: Rect) {
        let theme_name = self
            .selected_theme_idx()
            .map(|i| THEMES[i].name)
            .unwrap_or("?");

        let dialog_area = centered_rect(56, 9, area);
        f.render_widget(Clear, dialog_area);
        let dialog = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  Apply theme ", Style::default().fg(C_TEXT)),
                Span::styled(theme_name, Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
                Span::styled(" as active?", Style::default().fg(C_TEXT)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Press ", Style::default().fg(C_DIM)),
                Span::styled("[Y/Enter]", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
                Span::styled(" to confirm, any key to cancel", Style::default().fg(C_DIM)),
            ]),
            Line::from(""),
        ])
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(C_ACCENT))
                .title(Span::styled(
                    " 🎨 Apply Theme ",
                    Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
                ))
                .style(Style::default().bg(Color::Rgb(12, 8, 28))),
        );
        f.render_widget(dialog, dialog_area);
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Convert prompt.rs u32 color encoding to ratatui Color.
fn theme_color_to_ratatui(c: u32) -> Color {
    if c & 0x8000_0000 != 0 {
        // True color
        let r = ((c >> 16) & 0xFF) as u8;
        let g = ((c >> 8)  & 0xFF) as u8;
        let b = (c & 0xFF) as u8;
        Color::Rgb(r, g, b)
    } else {
        // ANSI 256
        Color::Indexed((c & 0xFF) as u8)
    }
}

/// Strip ANSI escape sequences from a string slice.
fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\x1b' && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            // Skip until 'm' or other terminator
            i += 2;
            while i < bytes.len() && !bytes[i].is_ascii_alphabetic() { i += 1; }
            if i < bytes.len() { i += 1; } // skip the terminator letter
        } else {
            if let Some(c) = s[i..].chars().next() {
                out.push(c);
                i += c.len_utf8();
            } else {
                i += 1;
            }
        }
    }
    out
}

fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let x = r.x + (r.width.saturating_sub(popup_width)) / 2;
    let y = r.y + (r.height.saturating_sub(height)) / 2;
    Rect::new(x, y, popup_width.min(r.width), height.min(r.height))
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Launch the interactive TUI theme picker.
pub fn run_theme_picker() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = ThemePickerApp::new();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = app.run_loop(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result?;
    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_plain() {
        assert_eq!(strip_ansi("hello world"), "hello world");
    }

    #[test]
    fn test_strip_ansi_colored() {
        let input = "\x1b[1;32mrihad\x1b[0m@\x1b[38;5;51march\x1b[0m";
        let result = strip_ansi(input);
        assert_eq!(result, "rihad@arch");
    }

    #[test]
    fn test_theme_color_true_color() {
        let c = 0x8000_0000 | 0xca9ee6u32;
        let color = theme_color_to_ratatui(c);
        assert_eq!(color, Color::Rgb(0xca, 0x9e, 0xe6));
    }

    #[test]
    fn test_theme_color_ansi256() {
        let c = 51u32;
        let color = theme_color_to_ratatui(c);
        assert_eq!(color, Color::Indexed(51));
    }

    #[test]
    fn test_app_init_selects_active() {
        let app = ThemePickerApp::new();
        // active_idx should be valid
        assert!(app.active_idx < THEMES.len());
        // filtered should contain all themes initially
        assert_eq!(app.filtered.len(), THEMES.len());
    }

    #[test]
    fn test_refilter_narrows_list() {
        let mut app = ThemePickerApp::new();
        app.query = "cat".to_string();
        app.refilter();
        assert!(app.filtered.iter().all(|&i| THEMES[i].name.contains("cat")));
    }

    #[test]
    fn test_move_select_wraps() {
        let mut app = ThemePickerApp::new();
        app.list_state.select(Some(0));
        app.move_select(-1);
        assert_eq!(app.list_state.selected(), Some(app.filtered.len() - 1));
    }
}
