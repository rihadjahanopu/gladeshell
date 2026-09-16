// =============================================================================
//  src/tools/project_setup/hub.rs — Interactive Project Tools TUI & Hub (`project`)
// =============================================================================

use std::error::Error;
use std::io;

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
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Terminal,
};

use super::html::run_html;
use super::ii::run_ii;
use super::next::run_next;
use super::shadcn_ui::run_ui;
use super::tailwind::run_css;
use super::utils::prompt_text;
use super::vite::run_vite;

#[derive(Debug, Clone)]
pub struct ProjectToolItem {
    pub key: &'static str,
    pub title: &'static str,
    pub category: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
    pub cmd_hint: &'static str,
    pub features: &'static [&'static str],
}

pub const PROJECT_TOOLS: &[ProjectToolItem] = &[
    ProjectToolItem {
        key: "vite",
        title: "Vite Project Generator",
        category: "Frontend / Web",
        icon: "⚡",
        description: "Generate Vite (React/Vue/TS/JS) project with optional Tailwind v4 setup",
        cmd_hint: "fancybash vite / vite",
        features: &["Bun / NPM runner choice", "Tailwind CSS v4 auto-install", "TypeScript / JavaScript"],
    },
    ProjectToolItem {
        key: "next",
        title: "Next.js App Router Setup",
        category: "Fullstack / Framework",
        icon: "🚀",
        description: "Initialize official Next.js App Router project with Bun or NPM",
        cmd_hint: "fancybash next / next",
        features: &["create-next-app@latest", "Bun / NPM runner", "App Router ready"],
    },
    ProjectToolItem {
        key: "ui",
        title: "Shadcn UI Component Setup",
        category: "UI Components",
        icon: "🎨",
        description: "Setup Shadcn UI and auto-patch tsconfig.json and vite.config.ts with @/* path aliases",
        cmd_hint: "fancybash ui / ui",
        features: &["Path alias auto-patching (@/*)", "Custom component installer", "Vite & Next.js auto-detect"],
    },
    ProjectToolItem {
        key: "css",
        title: "Tailwind CSS v4 Auto-Installer",
        category: "Styling & Utility",
        icon: "📦",
        description: "Install Tailwind CSS v4, @tailwindcss/vite, clsx, and inject @import into main CSS",
        cmd_hint: "fancybash css / css",
        features: &["Tailwind CSS v4 engine", "@tailwindcss/vite plugin", "clsx + tailwind-merge"],
    },
    ProjectToolItem {
        key: "html",
        title: "Serve / Run index.html",
        category: "Execution / Web",
        icon: "🌐",
        description: "Serve index.html with Bun dev server or open directly in system browser",
        cmd_hint: "fancybash html / html",
        features: &["Bun HTML dev runner", "Browser auto-open fallback", "Zero configuration"],
    },
    ProjectToolItem {
        key: "ii",
        title: "Initialize Project (Bun / NPM / PNPM)",
        category: "Project Scaffolding",
        icon: "🥐",
        description: "Quickly initialize package.json and create standard .gitignore file",
        cmd_hint: "fancybash ii / ii",
        features: &["Bun / NPM / PNPM / Yarn", "Auto .gitignore creation", "Zero configuration"],
    },
    ProjectToolItem {
        key: "makecpp",
        title: "C/C++ Project Boilerplate",
        category: "C / C++ Native",
        icon: "⚙️",
        description: "Generate C++ project structure with src, include, CMakeLists.txt, build.sh, and .gitignore",
        cmd_hint: "fancybash makecpp / makecpp",
        features: &["CMake & Makefile setup", "C++17 / C++20 standard", "Modular directory layout"],
    },
    ProjectToolItem {
        key: "run",
        title: "Interactive Bun JS/TS Runner",
        category: "Execution Tool",
        icon: "🏃",
        description: "Interactively scan directory and execute JS/TS files instantly with Bun",
        cmd_hint: "fancybash run / run",
        features: &["Interactive file selector", "Instant Bun runner", "TS & JS support"],
    },
    ProjectToolItem {
        key: "pg",
        title: "Universal Package Converter",
        category: "Package Utility",
        icon: "🔄",
        description: "Convert lockfiles and dependency commands between npm, bun, pnpm, and yarn",
        cmd_hint: "fancybash pg / pg",
        features: &["Multi-package manager", "Lockfile converter", "Auto dependency detection"],
    },
];

/// Interactive Project Tools TUI & Hub (`fancybash project` / `project`)
pub fn run_project() -> Result<(), Box<dyn Error>> {
    let choice = run_project_tui()?;
    match choice.as_deref() {
        Some("vite") => run_vite(),
        Some("next") => run_next(),
        Some("ui") => run_ui(),
        Some("css") => run_css(),
        Some("html") => run_html(),
        Some("ii") => run_ii(),
        Some("makecpp") => {
            let name = prompt_text("Enter C++ project name")?;
            if name.is_empty() {
                println!("Operation cancelled.");
                Ok(())
            } else {
                crate::tools::cpp_gen::run(&name, "20")
            }
        }
        Some("run") => crate::tools::bun_runner::run(),
        Some("pg") => crate::tools::pkg_converter::run("package.json", false),
        _ => {
            println!("Operation cancelled.");
            Ok(())
        }
    }
}

fn run_project_tui() -> Result<Option<String>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut query = String::new();
    let mut selected_idx: usize = 0;

    let res = loop {
        let q = query.to_lowercase();
        let filtered: Vec<&ProjectToolItem> = PROJECT_TOOLS
            .iter()
            .filter(|item| {
                item.key.to_lowercase().contains(&q)
                    || item.title.to_lowercase().contains(&q)
                    || item.category.to_lowercase().contains(&q)
                    || item.description.to_lowercase().contains(&q)
            })
            .collect();

        if filtered.is_empty() {
            selected_idx = 0;
        } else if selected_idx >= filtered.len() {
            selected_idx = filtered.len() - 1;
        }

        terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(10),
                    Constraint::Length(3),
                ])
                .split(size);

            let header_text = vec![
                Line::from(vec![
                    Span::styled(" 🚀 FANCYBASH PROJECT HUB ", Style::default().fg(Color::Black).bg(Color::Rgb(0, 220, 240)).add_modifier(Modifier::BOLD)),
                    Span::raw("  "),
                    Span::styled("Interactive Web & Native Boilerplate Center", Style::default().fg(Color::Rgb(255, 200, 80)).add_modifier(Modifier::BOLD)),
                ]),
            ];
            let header = Paragraph::new(header_text)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Rgb(0, 220, 240))));
            f.render_widget(header, chunks[0]);

            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
                .split(chunks[1]);

            let list_items: Vec<ListItem> = filtered
                .iter()
                .enumerate()
                .map(|(idx, item)| {
                    let is_sel = idx == selected_idx;
                    let prefix = if is_sel { "▶ " } else { "  " };
                    let style = if is_sel {
                        Style::default().fg(Color::Rgb(255, 200, 80)).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Rgb(220, 220, 230))
                    };
                    let line = Line::from(vec![
                        Span::styled(prefix, if is_sel { Style::default().fg(Color::Rgb(255, 200, 80)) } else { Style::default().fg(Color::Rgb(100, 110, 130)) }),
                        Span::styled(format!("{} ", item.icon), Style::default()),
                        Span::styled(item.title, style),
                        Span::styled(format!(" ({})", item.key), Style::default().fg(Color::Rgb(100, 110, 130))),
                    ]);
                    ListItem::new(line)
                })
                .collect();

            let title_str = format!(" 📦 Available Tools ({}) ", filtered.len());
            let list_widget = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title(title_str).border_style(Style::default().fg(Color::Rgb(100, 110, 130))));
            f.render_widget(list_widget, main_chunks[0]);

            if let Some(selected_item) = filtered.get(selected_idx) {
                let mut detail_lines = vec![
                    Line::from(vec![
                        Span::styled(format!("{} ", selected_item.icon), Style::default()),
                        Span::styled(selected_item.title, Style::default().fg(Color::Rgb(0, 220, 240)).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(vec![
                        Span::styled("Category: ", Style::default().fg(Color::Rgb(100, 110, 130))),
                        Span::styled(selected_item.category, Style::default().fg(Color::Rgb(220, 100, 240)).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(vec![
                        Span::styled("Command:  ", Style::default().fg(Color::Rgb(100, 110, 130))),
                        Span::styled(selected_item.cmd_hint, Style::default().fg(Color::Rgb(80, 220, 120)).add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(""),
                    Line::from(Span::styled("Description:", Style::default().fg(Color::Rgb(255, 200, 80)).add_modifier(Modifier::BOLD))),
                    Line::from(Span::styled(selected_item.description, Style::default().fg(Color::Rgb(220, 220, 230)))),
                    Line::from(""),
                    Line::from(Span::styled("Key Features:", Style::default().fg(Color::Rgb(255, 200, 80)).add_modifier(Modifier::BOLD))),
                ];

                for feat in selected_item.features {
                    detail_lines.push(Line::from(vec![
                        Span::styled("  ✔ ", Style::default().fg(Color::Rgb(80, 220, 120))),
                        Span::styled(*feat, Style::default().fg(Color::Rgb(200, 200, 210))),
                    ]));
                }

                let detail_widget = Paragraph::new(detail_lines)
                    .wrap(Wrap { trim: true })
                    .block(Block::default().borders(Borders::ALL).title(" ℹ️ Tool Info ").border_style(Style::default().fg(Color::Rgb(0, 220, 240))));
                f.render_widget(detail_widget, main_chunks[1]);
            } else {
                let empty_widget = Paragraph::new("No matching tools found.")
                    .block(Block::default().borders(Borders::ALL).title(" ℹ️ Tool Info "));
                f.render_widget(empty_widget, main_chunks[1]);
            }

            let query_disp = if query.is_empty() { "(none)" } else { &query };
            let footer_text = vec![
                Line::from(vec![
                    Span::styled(" [↑/↓] ", Style::default().fg(Color::Rgb(255, 200, 80)).add_modifier(Modifier::BOLD)),
                    Span::raw("Navigate  │ "),
                    Span::styled("[Enter] ", Style::default().fg(Color::Rgb(80, 220, 120)).add_modifier(Modifier::BOLD)),
                    Span::raw("Launch  │ "),
                    Span::styled("[Esc/q] ", Style::default().fg(Color::Rgb(255, 100, 100)).add_modifier(Modifier::BOLD)),
                    Span::raw("Quit  │ "),
                    Span::styled("Search: ", Style::default().fg(Color::Rgb(100, 110, 130))),
                    Span::styled(query_disp, Style::default().fg(Color::Rgb(0, 220, 240)).add_modifier(Modifier::BOLD)),
                ]),
            ];
            let footer = Paragraph::new(footer_text)
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Rgb(100, 110, 130))));
            f.render_widget(footer, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') if query.is_empty() => {
                    break Ok(None);
                }
                KeyCode::Esc => {
                    if !query.is_empty() {
                        query.clear();
                    } else {
                        break Ok(None);
                    }
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    break Ok(None);
                }
                KeyCode::Enter => {
                    if let Some(selected_item) = filtered.get(selected_idx) {
                        break Ok(Some(selected_item.key.to_string()));
                    }
                }
                KeyCode::Up => {
                    if selected_idx > 0 {
                        selected_idx -= 1;
                    } else if !filtered.is_empty() {
                        selected_idx = filtered.len() - 1;
                    }
                }
                KeyCode::Down => {
                    if !filtered.is_empty() {
                        if selected_idx + 1 < filtered.len() {
                            selected_idx += 1;
                        } else {
                            selected_idx = 0;
                        }
                    }
                }
                KeyCode::Backspace => {
                    query.pop();
                }
                KeyCode::Char(c) => {
                    query.push(c);
                }
                _ => {}
            }
        }
    };

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    res
}
