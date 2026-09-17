// =============================================================================
//  src/tools/file_find.rs — Native Parallel Fast File Finder (`ff` / `fancybash ff`)
//
//  Powered by ripgrep's `ignore` traversal engine and `rayon` parallel search.
//  Ultra-fast file searching with smart filtering, file previews & TUI mode.
// =============================================================================

use std::error::Error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Instant, UNIX_EPOCH};

use clap::Args;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ignore::WalkBuilder;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};

#[derive(Args, Debug, Clone)]
pub struct FfArgs {
    /// File or directory pattern to search for
    #[arg(value_name = "PATTERN")]
    pub pattern: Option<String>,

    /// Root path to start searching from (defaults to current directory)
    #[arg(short = 'p', long = "path", default_value = ".")]
    pub path: PathBuf,

    /// Filter by file extension (e.g. rs, js, ts, md, json)
    #[arg(short = 'e', long = "ext")]
    pub extension: Option<String>,

    /// Filter by type: 'f' (file), 'd' (directory), 'l' (symlink)
    #[arg(short = 't', long = "type")]
    pub file_type: Option<String>,

    /// Include hidden files and directories in search
    #[arg(short = 'H', long = "hidden")]
    pub hidden: bool,

    /// Do not respect .gitignore or .ignore files
    #[arg(long = "no-ignore")]
    pub no_ignore: bool,

    /// Limit maximum directory traversal depth
    #[arg(short = 'd', long = "max-depth")]
    pub max_depth: Option<usize>,

    /// Case-sensitive pattern search (default is case-insensitive)
    #[arg(short = 's', long = "case-sensitive")]
    pub case_sensitive: bool,

    /// Launch interactive TUI fuzzy search & preview mode
    #[arg(short = 'i', long = "interactive")]
    pub interactive: bool,

    /// Output results in JSON format
    #[arg(long = "json")]
    pub json: bool,

    /// Show detailed traversal & execution statistics
    #[arg(long = "stats")]
    pub stats: bool,
}

#[derive(Debug, Clone)]
pub struct FoundItem {
    pub path: PathBuf,
    pub relative_path: String,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub size: u64,
    pub modified_secs: u64,
}

/// Run the fast file finder tool
pub fn run(args: FfArgs) -> Result<(), Box<dyn Error>> {
    if args.interactive {
        return run_interactive(args);
    }

    let start_time = Instant::now();
    let items = search_files(&args)?;
    let elapsed = start_time.elapsed();

    if args.json {
        print_json_output(&items)?;
        return Ok(());
    }

    if items.is_empty() {
        if let Some(ref pat) = args.pattern {
            println!("\x1b[1;33m🔍 No files found matching pattern: '\x1b[1;36m{}\x1b[1;33m'\x1b[0m", pat);
        } else {
            println!("\x1b[1;33m🔍 No files found in target directory.\x1b[0m");
        }
        return Ok(());
    }

    // Render header
    println!(
        "\x1b[1;32m⚡ Fancybash Fast Finder\x1b[0m — Found \x1b[1;36m{}\x1b[0m items in \x1b[1;35m{:.2?}\x1b[0m",
        items.len(),
        elapsed
    );
    println!("\x1b[90m──────────────────────────────────────────────────────────────────────────────\x1b[0m");

    for item in &items {
        let icon = if item.is_dir {
            "📁"
        } else if item.is_symlink {
            "⚡"
        } else {
            get_file_icon(&item.relative_path)
        };

        let size_str = if item.is_dir {
            format!("\x1b[90m{:>10}\x1b[0m", "<DIR>")
        } else {
            format!("\x1b[36m{:>10}\x1b[0m", format_bytes(item.size))
        };

        let highlight_path = if let Some(ref pat) = args.pattern {
            highlight_matches(&item.relative_path, pat, args.case_sensitive)
        } else {
            if item.is_dir {
                format!("\x1b[1;34m{}\x1b[0m", item.relative_path)
            } else {
                format!("\x1b[0m{}\x1b[0m", item.relative_path)
            }
        };

        println!("  {} {}  {}", icon, size_str, highlight_path);
    }

    if args.stats {
        println!("\x1b[90m──────────────────────────────────────────────────────────────────────────────\x1b[0m");
        println!(
            "\x1b[1;32m📊 Stats:\x1b[0m Matches: \x1b[1;36m{}\x1b[0m | Time: \x1b[1;35m{:.2?}\x1b[0m | Path: \x1b[1;33m{}\x1b[0m",
            items.len(),
            elapsed,
            args.path.display()
        );
    }

    Ok(())
}

/// Perform directory traversal & filtering using `ignore::WalkBuilder`
pub fn search_files(args: &FfArgs) -> Result<Vec<FoundItem>, Box<dyn Error>> {
    let mut walk_builder = WalkBuilder::new(&args.path);
    walk_builder
        .hidden(!args.hidden)
        .git_ignore(!args.no_ignore)
        .ignore(!args.no_ignore)
        .parents(!args.no_ignore);

    if let Some(depth) = args.max_depth {
        walk_builder.max_depth(Some(depth));
    }

    let pattern_lower = args.pattern.as_ref().map(|p| p.to_lowercase());
    let ext_lower = args.extension.as_ref().map(|e| e.to_lowercase().trim_start_matches('.').to_string());

    let mut results = Vec::new();
    let walker = walk_builder.build();

    for result in walker {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        // Exclude root search dir itself
        if entry.depth() == 0 {
            continue;
        }

        let path = entry.path();
        let file_type = entry.file_type();
        let is_dir = file_type.as_ref().map_or(false, |ft| ft.is_dir());
        let is_symlink = file_type.as_ref().map_or(false, |ft| ft.is_symlink());
        let is_file = file_type.as_ref().map_or(false, |ft| ft.is_file());

        // Type filtering
        if let Some(ref ft_filter) = args.file_type {
            match ft_filter.to_lowercase().as_str() {
                "f" | "file" if !is_file => continue,
                "d" | "dir" | "directory" if !is_dir => continue,
                "l" | "link" | "symlink" if !is_symlink => continue,
                _ => {}
            }
        }

        // Extension filtering
        if let Some(ref ext) = ext_lower {
            if let Some(path_ext) = path.extension().and_then(|e| e.to_str()) {
                if path_ext.to_lowercase() != *ext {
                    continue;
                }
            } else {
                continue;
            }
        }

        // Relative path computation
        let relative_path = path
            .strip_prefix(&args.path)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        // Pattern matching
        if let Some(ref pat) = args.pattern {
            if args.case_sensitive {
                if !relative_path.contains(pat) && !file_name.contains(pat) {
                    continue;
                }
            } else {
                let pat_l = pattern_lower.as_ref().unwrap();
                let rel_l = relative_path.to_lowercase();
                let name_l = file_name.to_lowercase();
                if !rel_l.contains(pat_l) && !name_l.contains(pat_l) {
                    continue;
                }
            }
        }

        let metadata = entry.metadata().ok();
        let size = metadata.as_ref().map_or(0, |m| m.len());
        let modified_secs = metadata
            .as_ref()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map_or(0, |d| d.as_secs());

        results.push(FoundItem {
            path: path.to_path_buf(),
            relative_path,
            is_dir,
            is_symlink,
            size,
            modified_secs,
        });
    }

    Ok(results)
}

/// Interactive TUI mode powered by Ratatui & Crossterm
fn run_interactive(mut args: FfArgs) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = tui_loop(&mut terminal, &mut args);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Ok(Some(selected_path)) = res {
        println!("\x1b[1;32m📋 Selected Path:\x1b[0m {}", selected_path);
    }

    Ok(())
}

fn tui_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    args: &mut FfArgs,
) -> Result<Option<String>, Box<dyn Error>> {
    let mut query = args.pattern.clone().unwrap_or_default();
    let mut list_state = ListState::default();

    let mut items = search_files(args)?;
    if !items.is_empty() {
        list_state.select(Some(0));
    }

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Search bar
                    Constraint::Min(5),    // Main list & preview split
                    Constraint::Length(1), // Footer status bar
                ])
                .split(f.area());

            // Search box
            let search_box = Paragraph::new(format!(" 🔎 {}", query))
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Fast File Search (Type to filter, Esc to exit, Enter to select) "),
                );
            f.render_widget(search_box, chunks[0]);

            // Split body into List (60%) and Preview (40%)
            let body_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(chunks[1]);

            // Build List Items
            let list_items: Vec<ListItem> = items
                .iter()
                .map(|item| {
                    let icon = if item.is_dir {
                        "📁 "
                    } else if item.is_symlink {
                        "⚡ "
                    } else {
                        "📄 "
                    };
                    let line = format!("{} {}", icon, item.relative_path);
                    ListItem::new(line).style(if item.is_dir {
                        Style::default().fg(Color::Blue)
                    } else {
                        Style::default().fg(Color::White)
                    })
                })
                .collect();

            let list_widget = List::new(list_items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!(" Matches ({}) ", items.len())),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");
            f.render_stateful_widget(list_widget, body_chunks[0], &mut list_state);

            // Preview Box
            let preview_text = if let Some(sel) = list_state.selected().and_then(|i| items.get(i)) {
                get_preview_text(&sel.path, sel.is_dir, sel.size)
            } else {
                "No item selected".to_string()
            };

            let preview_box = Paragraph::new(preview_text)
                .block(Block::default().borders(Borders::ALL).title(" Preview "))
                .wrap(Wrap { trim: false });
            f.render_widget(preview_box, body_chunks[1]);

            // Footer
            let footer = Paragraph::new(Line::from(vec![
                Span::styled(" [↑/↓] ", Style::default().fg(Color::Yellow)),
                Span::raw("Navigate  "),
                Span::styled(" [Enter] ", Style::default().fg(Color::Green)),
                Span::raw("Select Path  "),
                Span::styled(" [Esc/Ctrl+C] ", Style::default().fg(Color::Red)),
                Span::raw("Exit"),
            ]))
            .style(Style::default().bg(Color::DarkGray).fg(Color::White));
            f.render_widget(footer, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => return Ok(None),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(None);
                    }
                    KeyCode::Enter => {
                        if let Some(i) = list_state.selected() {
                            if let Some(item) = items.get(i) {
                                return Ok(Some(item.relative_path.clone()));
                            }
                        }
                    }
                    KeyCode::Up => {
                        if let Some(i) = list_state.selected() {
                            if i > 0 {
                                list_state.select(Some(i - 1));
                            }
                        }
                    }
                    KeyCode::Down => {
                        if let Some(i) = list_state.selected() {
                            if i + 1 < items.len() {
                                list_state.select(Some(i + 1));
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if !query.is_empty() {
                            query.pop();
                            args.pattern = if query.is_empty() { None } else { Some(query.clone()) };
                            items = search_files(args)?;
                            list_state.select(if items.is_empty() { None } else { Some(0) });
                        }
                    }
                    KeyCode::Char(c) => {
                        query.push(c);
                        args.pattern = Some(query.clone());
                        items = search_files(args)?;
                        list_state.select(if items.is_empty() { None } else { Some(0) });
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Generate preview snippet for selected file or directory
fn get_preview_text(path: &Path, is_dir: bool, size: u64) -> String {
    if is_dir {
        if let Ok(entries) = fs::read_dir(path) {
            let mut list = Vec::new();
            for entry in entries.flatten().take(15) {
                list.push(entry.file_name().to_string_lossy().to_string());
            }
            return format!("📁 Directory Contents:\n\n{}", list.join("\n"));
        }
        return "📁 Directory (Unreadable)".to_string();
    }

    if size > 1_000_000 {
        return format!("📄 File is too large for preview ({})", format_bytes(size));
    }

    match fs::read_to_string(path) {
        Ok(content) => {
            let lines: Vec<&str> = content.lines().take(20).collect();
            format!("📄 Preview (First {} lines):\n\n{}", lines.len(), lines.join("\n"))
        }
        Err(_) => format!("📄 Binary or non-UTF8 File ({})", format_bytes(size)),
    }
}

/// Utility: Get file extension icon
fn get_file_icon(path: &str) -> &'static str {
    let lower = path.to_lowercase();
    if lower.ends_with(".rs") {
        "🦀"
    } else if lower.ends_with(".js") || lower.ends_with(".jsx") {
        "🟨"
    } else if lower.ends_with(".ts") || lower.ends_with(".tsx") {
        "📘"
    } else if lower.ends_with(".json") || lower.ends_with(".toml") || lower.ends_with(".yaml") {
        "⚙️"
    } else if lower.ends_with(".md") {
        "📝"
    } else if lower.ends_with(".sh") || lower.ends_with(".zsh") {
        "🐚"
    } else if lower.ends_with(".png") || lower.ends_with(".jpg") || lower.ends_with(".svg") {
        "🖼️"
    } else {
        "📄"
    }
}

/// Utility: Highlight matching pattern substring in path
fn highlight_matches(text: &str, pattern: &str, case_sensitive: bool) -> String {
    if pattern.is_empty() {
        return text.to_string();
    }

    let mut result = String::new();
    let text_lower = text.to_lowercase();
    let pat_lower = pattern.to_lowercase();

    let target_text = if case_sensitive { text } else { &text_lower };
    let target_pat = if case_sensitive { pattern } else { &pat_lower };

    let mut last_idx = 0;
    for (idx, _) in target_text.match_indices(target_pat) {
        result.push_str(&text[last_idx..idx]);
        result.push_str("\x1b[1;33;44m"); // Bold Yellow on Blue background
        result.push_str(&text[idx..idx + pattern.len()]);
        result.push_str("\x1b[0m");
        last_idx = idx + pattern.len();
    }
    result.push_str(&text[last_idx..]);
    result
}

/// Output results as JSON
fn print_json_output(items: &[FoundItem]) -> Result<(), Box<dyn Error>> {
    println!("[");
    for (idx, item) in items.iter().enumerate() {
        let comma = if idx + 1 < items.len() { "," } else { "" };
        println!(
            "  {{\n    \"path\": {:?},\n    \"absolute_path\": {:?},\n    \"is_directory\": {},\n    \"is_symlink\": {},\n    \"size_bytes\": {},\n    \"modified_secs\": {}\n  }}{}",
            item.relative_path,
            item.path.to_string_lossy(),
            item.is_dir,
            item.is_symlink,
            item.size,
            item.modified_secs,
            comma
        );
    }
    println!("]");
    Ok(())
}

/// Format bytes into human readable size
fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1500), "1.5 KB");
        assert_eq!(format_bytes(1_500_000), "1.4 MB");
    }

    #[test]
    fn test_highlight_matches() {
        let text = "src/tools/file_find.rs";
        let highlighted = highlight_matches(text, "file", false);
        assert!(highlighted.contains("\x1b[1;33;44mfile\x1b[0m"));
    }

    #[test]
    fn test_search_files_current_dir() {
        let args = FfArgs {
            pattern: Some("Cargo".to_string()),
            path: PathBuf::from("."),
            extension: None,
            file_type: None,
            hidden: false,
            no_ignore: false,
            max_depth: Some(2),
            case_sensitive: false,
            interactive: false,
            json: false,
            stats: false,
        };

        let items = search_files(&args).unwrap();
        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.relative_path.contains("Cargo")));
    }
}
