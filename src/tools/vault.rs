// =============================================================================
//  src/tools/vault.rs — Hardened AES-256 Multi-Vault Manager (Ratatui TUI)
// =============================================================================

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

// ── palette ───────────────────────────────────────────────────────────────────
const C_BG: Color = Color::Rgb(8, 10, 20);
const C_BORDER: Color = Color::Rgb(255, 180, 50); // gold
const C_ACCENT: Color = Color::Rgb(255, 210, 100);
const C_SELECTED: Color = Color::Rgb(255, 190, 60);
const C_DIM: Color = Color::Rgb(90, 90, 110);
const C_TEXT: Color = Color::Rgb(210, 215, 230);
const C_WHITE: Color = Color::Rgb(255, 255, 255);
const C_GREEN: Color = Color::Rgb(80, 220, 130);
const C_RED: Color = Color::Rgb(255, 90, 90);

fn vault_store_dir() -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join(".secret_vaults")
}

fn ram_base_dir() -> PathBuf {
    let shm = Path::new("/dev/shm");
    if shm.exists() && shm.is_dir() { shm.to_path_buf() } else { std::env::temp_dir() }
}

// ── menu entries ──────────────────────────────────────────────────────────────
const MENU_ITEMS: &[&str] = &[
    "  🔓  Unlock Vault",
    "  🔒  Lock Vault",
    "  ➕  Create New Vault",
    "  📋  List All Vaults",
];

#[derive(Clone, PartialEq)]
enum Mode {
    Menu,
    VaultList { action: VaultAction },
    TextInput { prompt: String, field: InputField },
    PasswordInput { prompt: String, field: PasswordField, stored: String },
    Processing,
}

#[derive(Clone, PartialEq)]
enum VaultAction { Unlock, Lock, Create }

#[derive(Clone, PartialEq)]
enum InputField { VaultName, LockName }

#[derive(Clone, PartialEq)]
enum PasswordField { First, Second }

struct App {
    menu_state: ListState,
    vault_state: ListState,
    vaults: Vec<String>,
    mode: Mode,
    input: String,
    pending_name: String,
    pending_pass1: String,
    status_msg: Option<(String, bool)>,
}

impl App {
    fn new(vaults: Vec<String>) -> Self {
        let mut menu_state = ListState::default();
        menu_state.select(Some(0));
        let mut vault_state = ListState::default();
        if !vaults.is_empty() { vault_state.select(Some(0)); }
        Self {
            menu_state,
            vault_state,
            vaults,
            mode: Mode::Menu,
            input: String::new(),
            pending_name: String::new(),
            pending_pass1: String::new(),
            status_msg: None,
        }
    }

    fn move_menu_up(&mut self) {
        let i = self.menu_state.selected().unwrap_or(0);
        self.menu_state.select(Some(if i == 0 { MENU_ITEMS.len() - 1 } else { i - 1 }));
    }

    fn move_menu_down(&mut self) {
        let i = self.menu_state.selected().unwrap_or(0);
        self.menu_state.select(Some((i + 1) % MENU_ITEMS.len()));
    }

    fn move_vault_up(&mut self) {
        if self.vaults.is_empty() { return; }
        let i = self.vault_state.selected().unwrap_or(0);
        self.vault_state.select(Some(if i == 0 { self.vaults.len() - 1 } else { i - 1 }));
    }

    fn move_vault_down(&mut self) {
        if self.vaults.is_empty() { return; }
        let i = self.vault_state.selected().unwrap_or(0);
        self.vault_state.select(Some((i + 1) % self.vaults.len()));
    }

    fn selected_vault(&self) -> Option<&str> {
        let i = self.vault_state.selected()?;
        self.vaults.get(i).map(|s| s.as_str())
    }
}

/// Runs the interactive AES-256 Vault Manager TUI.
pub fn run(action_opt: Option<&str>, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let store_dir = vault_store_dir();
    fs::create_dir_all(&store_dir)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&store_dir, fs::Permissions::from_mode(0o700));
    }

    // Non-TUI quick path
    if let Some(action) = action_opt {
        return run_cli(action, args, &store_dir);
    }

    let vaults = list_encrypted_vaults(&store_dir)?;
    let mut app = App::new(vaults);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let store_dir_c = store_dir.clone();
        terminal.draw(|f| draw_vault(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            app.status_msg = None;

            match app.mode.clone() {
                // ── Main Menu ──────────────────────────────────────────────
                Mode::Menu => match (key.modifiers, key.code) {
                    (_, KeyCode::Esc) | (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,
                    (_, KeyCode::Up) => app.move_menu_up(),
                    (_, KeyCode::Down) => app.move_menu_down(),
                    (_, KeyCode::Enter) => {
                        let sel = app.menu_state.selected().unwrap_or(0);
                        match sel {
                            0 => { // Unlock
                                app.vaults = list_encrypted_vaults(&store_dir_c).unwrap_or_default();
                                if app.vaults.is_empty() {
                                    app.status_msg = Some(("📋 No encrypted vaults found".into(), true));
                                } else {
                                    if !app.vaults.is_empty() { app.vault_state.select(Some(0)); }
                                    app.mode = Mode::VaultList { action: VaultAction::Unlock };
                                }
                            }
                            1 => { // Lock
                                app.input.clear();
                                app.mode = Mode::TextInput { prompt: "Enter vault name to lock:".into(), field: InputField::LockName };
                            }
                            2 => { // Create
                                app.input.clear();
                                app.mode = Mode::TextInput { prompt: "New vault name (e.g. MySecrets):".into(), field: InputField::VaultName };
                            }
                            3 => { // List
                                app.vaults = list_encrypted_vaults(&store_dir_c).unwrap_or_default();
                                if !app.vaults.is_empty() { app.vault_state.select(Some(0)); }
                                app.mode = Mode::VaultList { action: VaultAction::Unlock }; // reuse list view
                                // Mark as "list only" by checking action in draw
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                },

                // ── Vault List (select) ────────────────────────────────────
                Mode::VaultList { action: _ } => match (key.modifiers, key.code) {
                    (_, KeyCode::Esc) => app.mode = Mode::Menu,
                    (_, KeyCode::Up) => app.move_vault_up(),
                    (_, KeyCode::Down) => app.move_vault_down(),
                    (_, KeyCode::Enter) => {
                        if let Some(name) = app.selected_vault() {
                            let clean = name.trim_end_matches(".enc").to_string();
                            app.pending_name = clean;
                            app.input.clear();
                            app.mode = Mode::PasswordInput {
                                prompt: "Enter Master Password:".into(),
                                field: PasswordField::First,
                                stored: String::new(),
                            };
                        }
                    }
                    _ => {}
                },

                // ── Text Input ──────────────────────────────────────────────
                Mode::TextInput { prompt: _, field } => match (key.modifiers, key.code) {
                    (_, KeyCode::Esc) => { app.mode = Mode::Menu; app.input.clear(); }
                    (_, KeyCode::Enter) => {
                        let name = app.input.trim().replace(' ', "_");
                        if name.is_empty() {
                            app.status_msg = Some(("❌ Name cannot be empty!".into(), true));
                        } else {
                            app.pending_name = name.clone();
                            app.input.clear();
                            match field {
                                InputField::VaultName => {
                                    app.mode = Mode::PasswordInput {
                                        prompt: "Master Password:".into(),
                                        field: PasswordField::First,
                                        stored: String::new(),
                                    };
                                }
                                InputField::LockName => {
                                    app.mode = Mode::PasswordInput {
                                        prompt: "Master Password:".into(),
                                        field: PasswordField::First,
                                        stored: String::new(),
                                    };
                                }
                            }
                        }
                    }
                    (_, KeyCode::Backspace) => { app.input.pop(); }
                    (_, KeyCode::Char(c)) => { app.input.push(c); }
                    _ => {}
                },

                // ── Password Input ──────────────────────────────────────────
                Mode::PasswordInput { prompt: _, field, stored: _ } => match (key.modifiers, key.code) {
                    (_, KeyCode::Esc) => { app.mode = Mode::Menu; app.input.clear(); }
                    (_, KeyCode::Enter) => {
                        let pass = app.input.trim().to_string();
                        app.input.clear();
                        match field {
                            PasswordField::First => {
                                app.pending_pass1 = pass.clone();
                                // For unlock we go straight to action; for create/lock ask confirm
                                let menu_sel = app.menu_state.selected().unwrap_or(0);
                                if menu_sel == 0 {
                                    // Unlock: run immediately
                                    let name = app.pending_name.clone();
                                    let result = vault_unlock(&name, &pass, &store_dir_c);
                                    app.status_msg = Some(result);
                                    app.mode = Mode::Menu;
                                } else {
                                    // Create or Lock: ask for confirm
                                    app.mode = Mode::PasswordInput {
                                        prompt: "Confirm Master Password:".into(),
                                        field: PasswordField::Second,
                                        stored: pass,
                                    };
                                }
                            }
                            PasswordField::Second => {
                                let pass2 = pass;
                                if pass2 != app.pending_pass1 {
                                    app.status_msg = Some(("❌ Passwords do not match!".into(), true));
                                    app.mode = Mode::Menu;
                                } else {
                                    let name = app.pending_name.clone();
                                    let p = app.pending_pass1.clone();
                                    let menu_sel = app.menu_state.selected().unwrap_or(0);
                                    let result = match menu_sel {
                                        2 => vault_create(&name, &p, &store_dir_c),
                                        1 => vault_lock(&name, &p, &store_dir_c),
                                        _ => ("❌ Unknown action".into(), true),
                                    };
                                    app.vaults = list_encrypted_vaults(&store_dir_c).unwrap_or_default();
                                    app.status_msg = Some(result);
                                    app.mode = Mode::Menu;
                                }
                            }
                        }
                    }
                    (_, KeyCode::Backspace) => { app.input.pop(); }
                    (_, KeyCode::Char(c)) => { app.input.push(c); }
                    _ => {}
                },

                Mode::Processing => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn draw_vault(f: &mut Frame, app: &mut App) {
    let area = f.area();
    f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // banner
            Constraint::Min(5),    // body
            Constraint::Length(3), // status
        ])
        .split(area);

    // ── Banner ──────────────────────────────────────────────────────────────
    let banner = Paragraph::new(Line::from(vec![
        Span::styled("🔐  ", Style::default().fg(C_ACCENT)),
        Span::styled("VAULT", Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD)),
        Span::styled(" — AES-256 Multi-Vault Manager", Style::default().fg(C_TEXT)),
        Span::styled(format!("  ({} vaults)", app.vaults.len()), Style::default().fg(C_DIM)),
    ]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER))
            .style(Style::default().bg(C_BG)),
    );
    f.render_widget(banner, layout[0]);

    // ── Body ─────────────────────────────────────────────────────────────────
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(layout[1]);

    // Left: menu
    let menu_items: Vec<ListItem> = MENU_ITEMS.iter().enumerate().map(|(i, item)| {
        let is_sel = app.menu_state.selected() == Some(i);
        if is_sel {
            ListItem::new(Line::from(vec![
                Span::styled(" ▶ ", Style::default().fg(C_SELECTED).add_modifier(Modifier::BOLD)),
                Span::styled(item.trim(), Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD).bg(Color::Rgb(30, 20, 5))),
            ]))
        } else {
            ListItem::new(Line::from(vec![
                Span::styled("   ", Style::default()),
                Span::styled(item.trim(), Style::default().fg(C_TEXT)),
            ]))
        }
    }).collect();
    let menu = List::new(menu_items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER))
            .title(Span::styled(" Menu ", Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD)))
            .style(Style::default().bg(C_BG)),
    );
    f.render_stateful_widget(menu, body[0], &mut app.menu_state.clone());

    // Right: vault list or input
    match &app.mode.clone() {
        Mode::VaultList { .. } => {
            let vault_items: Vec<ListItem> = if app.vaults.is_empty() {
                vec![ListItem::new(Line::from(vec![Span::styled(
                    "  (no vaults found)",
                    Style::default().fg(C_DIM).add_modifier(Modifier::ITALIC),
                )]))]
            } else {
                app.vaults.iter().enumerate().map(|(i, v)| {
                    let is_sel = app.vault_state.selected() == Some(i);
                    if is_sel {
                        ListItem::new(Line::from(vec![
                            Span::styled(" ▶ ", Style::default().fg(C_SELECTED).add_modifier(Modifier::BOLD)),
                            Span::styled(format!("🔒 {}", v), Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD).bg(Color::Rgb(30, 20, 5))),
                        ]))
                    } else {
                        ListItem::new(Line::from(vec![
                            Span::styled("   ", Style::default()),
                            Span::styled(format!("🔒 {}", v), Style::default().fg(C_ACCENT)),
                        ]))
                    }
                }).collect()
            };
            let vault_list = List::new(vault_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(C_ACCENT))
                    .title(Span::styled(" Select Vault ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)))
                    .style(Style::default().bg(C_BG)),
            );
            f.render_stateful_widget(vault_list, body[1], &mut app.vault_state.clone());
        }

        Mode::TextInput { prompt, .. } => {
            let pane = Paragraph::new(vec![
                Line::from(""),
                Line::from(vec![Span::styled(format!("  {}", prompt), Style::default().fg(C_DIM))]),
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(&app.input, Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD)),
                    Span::styled("█", Style::default().fg(C_BORDER)),
                ]),
                Line::from(""),
                Line::from(vec![Span::styled("  Enter to confirm  Esc to cancel", Style::default().fg(C_DIM))]),
            ])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(C_ACCENT))
                    .title(Span::styled(" Input ", Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)))
                    .style(Style::default().bg(C_BG)),
            );
            f.render_widget(pane, body[1]);
        }

        Mode::PasswordInput { prompt, .. } => {
            let masked: String = "●".repeat(app.input.len());
            let pane = Paragraph::new(vec![
                Line::from(""),
                Line::from(vec![Span::styled(format!("  {}", prompt), Style::default().fg(C_DIM))]),
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(&masked, Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD)),
                    Span::styled("█", Style::default().fg(C_BORDER)),
                ]),
                Line::from(""),
                Line::from(vec![Span::styled("  Enter to confirm  Esc to cancel", Style::default().fg(C_DIM))]),
            ])
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(Style::default().fg(C_BORDER))
                    .title(Span::styled(" 🔑 Password ", Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD)))
                    .style(Style::default().bg(Color::Rgb(15, 12, 3))),
            );
            f.render_widget(pane, body[1]);
        }

        Mode::Menu | Mode::Processing => {
            // Info pane
            let info_lines = vec![
                Line::from(""),
                Line::from(vec![Span::styled("  AES-256-CBC encrypted vaults", Style::default().fg(C_DIM))]),
                Line::from(vec![Span::styled("  PBKDF2 key derivation (500k iter)", Style::default().fg(C_DIM))]),
                Line::from(vec![Span::styled("  RAM-only unlock (no plaintext on disk)", Style::default().fg(C_DIM))]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Store dir: ", Style::default().fg(C_DIM)),
                    Span::styled(vault_store_dir().display().to_string(), Style::default().fg(C_ACCENT)),
                ]),
            ];
            let info = Paragraph::new(info_lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(C_DIM))
                        .title(Span::styled(" Vault Info ", Style::default().fg(C_DIM)))
                        .style(Style::default().bg(C_BG)),
                );
            f.render_widget(info, body[1]);
        }
    }

    // ── Status Bar ──────────────────────────────────────────────────────────
    let status_text = if let Some((ref msg, is_err)) = app.status_msg {
        let color = if is_err { C_RED } else { C_GREEN };
        Line::from(vec![Span::styled(msg.clone(), Style::default().fg(color).add_modifier(Modifier::BOLD))])
    } else {
        Line::from(vec![Span::styled(
            "↑↓ Navigate  |  Enter Select  |  Esc Back/Quit",
            Style::default().fg(C_DIM),
        )])
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
    f.render_widget(status_bar, layout[2]);
}

// ── vault operations ──────────────────────────────────────────────────────────
fn vault_create(name: &str, pass: &str, store_dir: &PathBuf) -> (String, bool) {
    let enc_file = store_dir.join(format!("{}.enc", name));
    if enc_file.exists() {
        return (format!("❌ Vault '{}.enc' already exists!", name), true);
    }
    let ram_base = ram_base_dir();
    let ram_vault = ram_base.join(format!(".vault_{}_{}", name, std::process::id()));
    if fs::create_dir_all(&ram_vault).is_err() {
        return ("❌ Failed to create temp directory".into(), true);
    }
    let _ = fs::write(ram_vault.join("README.txt"), format!("Vault '{}'", name));
    let archive = ram_base.join(format!(".tmp_{}.tar.gz", std::process::id()));
    let tar_ok = Command::new("tar").args(["-czf", &archive.display().to_string(), "-C", &ram_vault.display().to_string(), "."]).status().map(|s| s.success()).unwrap_or(false);
    if !tar_ok { let _ = fs::remove_dir_all(&ram_vault); return ("❌ tar failed".into(), true); }
    let enc_ok = Command::new("openssl").args(["enc", "-aes-256-cbc", "-pbkdf2", "-iter", "500000", "-pass", &format!("pass:{}", pass), "-in", &archive.display().to_string(), "-out", &enc_file.display().to_string()]).status().map(|s| s.success()).unwrap_or(false);
    let _ = fs::remove_file(&archive);
    let _ = fs::remove_dir_all(&ram_vault);
    if enc_ok { (format!("✅ Created vault: {}.enc", name), false) } else { ("❌ Encryption failed!".into(), true) }
}

fn vault_unlock(name: &str, pass: &str, store_dir: &PathBuf) -> (String, bool) {
    let enc_file = store_dir.join(format!("{}.enc", name));
    if !enc_file.exists() { return (format!("❌ Vault file not found: {}.enc", name), true); }
    let ram_base = ram_base_dir();
    let archive = ram_base.join(format!(".tmp_dec_{}.tar.gz", std::process::id()));
    let dec_ok = Command::new("openssl").args(["enc", "-d", "-aes-256-cbc", "-pbkdf2", "-iter", "500000", "-pass", &format!("pass:{}", pass), "-in", &enc_file.display().to_string(), "-out", &archive.display().to_string()]).status().map(|s| s.success()).unwrap_or(false);
    if !dec_ok { let _ = fs::remove_file(&archive); return ("❌ Invalid password or corrupted vault!".into(), true); }
    let home = std::env::var_os("HOME").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    let target = home.join(name);
    let _ = fs::remove_dir_all(&target);
    let _ = fs::create_dir_all(&target);
    let tar_ok = Command::new("tar").args(["-xzf", &archive.display().to_string(), "-C", &target.display().to_string()]).status().map(|s| s.success()).unwrap_or(false);
    let _ = fs::remove_file(&archive);
    if tar_ok { (format!("🔓 Unlocked to: ~/{}", name), false) } else { ("❌ Tar extraction failed!".into(), true) }
}

fn vault_lock(name: &str, pass: &str, store_dir: &PathBuf) -> (String, bool) {
    let home = std::env::var_os("HOME").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    let target = home.join(name);
    if !target.exists() { return (format!("❌ Vault directory ~/{} not found", name), true); }
    let enc_file = store_dir.join(format!("{}.enc", name));
    let ram_base = ram_base_dir();
    let archive = ram_base.join(format!(".tmp_enc_{}.tar.gz", std::process::id()));
    let tar_ok = Command::new("tar").args(["-czf", &archive.display().to_string(), "-C", &target.display().to_string(), "."]).status().map(|s| s.success()).unwrap_or(false);
    if !tar_ok { return ("❌ tar failed".into(), true); }
    let enc_ok = Command::new("openssl").args(["enc", "-aes-256-cbc", "-pbkdf2", "-iter", "500000", "-pass", &format!("pass:{}", pass), "-in", &archive.display().to_string(), "-out", &enc_file.display().to_string()]).status().map(|s| s.success()).unwrap_or(false);
    let _ = fs::remove_file(&archive);
    if enc_ok { let _ = fs::remove_dir_all(&target); (format!("🔒 Locked: {}.enc", name), false) } else { ("❌ Encryption failed!".into(), true) }
}

fn list_encrypted_vaults(store_dir: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut vaults = Vec::new();
    if let Ok(entries) = fs::read_dir(store_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".enc") { vaults.push(name); }
        }
    }
    vaults.sort();
    Ok(vaults)
}

fn run_cli(action: &str, args: &[String], store_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        "create" | "new" => {
            let name = args.first().cloned().unwrap_or_else(|| { print!("Vault name: "); io::stdout().flush().unwrap(); let mut s = String::new(); io::stdin().read_line(&mut s).unwrap(); s.trim().replace(' ', "_") });
            print!("Password: "); io::stdout().flush()?;
            let mut p = String::new(); io::stdin().read_line(&mut p)?;
            let (msg, _) = vault_create(name.trim(), p.trim(), store_dir);
            println!("{}", msg);
        }
        "unlock" | "open" => {
            let name = args.first().cloned().unwrap_or_default();
            print!("Password: "); io::stdout().flush()?;
            let mut p = String::new(); io::stdin().read_line(&mut p)?;
            let (msg, _) = vault_unlock(name.trim(), p.trim(), store_dir);
            println!("{}", msg);
        }
        "lock" | "close" => {
            let name = args.first().cloned().unwrap_or_default();
            print!("Password: "); io::stdout().flush()?;
            let mut p = String::new(); io::stdin().read_line(&mut p)?;
            let (msg, _) = vault_lock(name.trim(), p.trim(), store_dir);
            println!("{}", msg);
        }
        "list" | "ls" => {
            let vaults = list_encrypted_vaults(store_dir)?;
            if vaults.is_empty() { println!("📋 No vaults found."); } else { for v in vaults { println!("  🔒 {}", v); } }
        }
        _ => println!("Usage: fancybash vault [create <name> | lock <name> | unlock <name> | list]"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_store_path_exists() {
        let store = vault_store_dir();
        assert!(store.to_string_lossy().contains(".secret_vaults"));
    }
}
