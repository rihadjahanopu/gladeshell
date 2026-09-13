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
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use pbkdf2::pbkdf2_hmac;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::Sha256;
use std::fs::{self, OpenOptions};
use std::io::{self, Cursor, Write};
use std::path::{Path, PathBuf};
use tar::{Archive, Builder};
use zeroize::Zeroizing;

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

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
enum Mode {
    Menu,
    VaultList { action: VaultAction },
    TextInput { prompt: String, field: InputField },
    PasswordInput { prompt: String, field: PasswordField, stored: String },
    Processing,
}

#[allow(dead_code)]
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

// ── Pure Rust Cryptography & In-Memory Archiving ──────────────────────────────
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const PBKDF2_ROUNDS: u32 = 500_000;

fn derive_key(password: &str, salt: &[u8; SALT_LEN]) -> Zeroizing<[u8; 32]> {
    let mut key = Zeroizing::new([0u8; 32]);
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ROUNDS, key.as_mut());
    key
}

fn encrypt_vault_payload(plaintext: &[u8], password: &str) -> Result<Vec<u8>, String> {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut salt = [0u8; SALT_LEN];
    rng.fill_bytes(&mut salt);

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rng.fill_bytes(&mut nonce_bytes);

    let derived_key = derive_key(password, &salt);
    let key = Key::<Aes256Gcm>::from_slice(derived_key.as_ref());
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| format!("AES-256-GCM encryption error: {}", e))?;

    let mut payload = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    payload.extend_from_slice(&salt);
    payload.extend_from_slice(&nonce_bytes);
    payload.extend_from_slice(&ciphertext);

    Ok(payload)
}

fn decrypt_vault_payload(vault_data: &[u8], password: &str) -> Result<Zeroizing<Vec<u8>>, String> {
    if vault_data.len() < SALT_LEN + NONCE_LEN {
        return Err("Vault file is truncated or invalid header format".into());
    }

    let (salt_bytes, rest) = vault_data.split_at(SALT_LEN);
    let (nonce_bytes, ciphertext) = rest.split_at(NONCE_LEN);

    let mut salt = [0u8; SALT_LEN];
    salt.copy_from_slice(salt_bytes);

    let derived_key = derive_key(password, &salt);
    let key = Key::<Aes256Gcm>::from_slice(derived_key.as_ref());
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Decryption failed: Incorrect password or corrupted vault file".to_string())?;

    Ok(Zeroizing::new(decrypted))
}

fn create_in_memory_tarball(dir_path: &Path) -> Result<Zeroizing<Vec<u8>>, String> {
    if !dir_path.exists() || !dir_path.is_dir() {
        return Err(format!("Directory path '{}' does not exist", dir_path.display()));
    }

    let buffer = Vec::new();
    let encoder = GzEncoder::new(buffer, Compression::default());
    let mut builder = Builder::new(encoder);

    let folder_name = dir_path.file_name().ok_or("Invalid directory name")?;
    builder
        .append_dir_all(folder_name, dir_path)
        .map_err(|e| format!("Tar build error: {}", e))?;

    let encoder = builder.into_inner().map_err(|e| format!("Tar finalize error: {}", e))?;
    let compressed_bytes = encoder.finish().map_err(|e| format!("Gz finish error: {}", e))?;

    Ok(Zeroizing::new(compressed_bytes))
}

fn extract_in_memory_tarball(tarball_data: &[u8], target_dir: &Path) -> Result<(), String> {
    fs::create_dir_all(target_dir).map_err(|e| format!("Failed to create output dir: {}", e))?;

    let cursor = Cursor::new(tarball_data);
    let decoder = GzDecoder::new(cursor);
    let mut archive = Archive::new(decoder);

    for entry_res in archive.entries().map_err(|e| format!("Tar read error: {}", e))? {
        let mut entry = entry_res.map_err(|e| format!("Tar entry error: {}", e))?;

        // ZipSlip mitigation
        let entry_path = entry.path().map_err(|e| e.to_string())?;
        let unpacked_target = target_dir.join(&entry_path);
        if let Some(parent) = unpacked_target.parent() {
            let _ = fs::create_dir_all(parent);
        }

        entry.unpack_in(target_dir).map_err(|e| format!("Tar unpack error: {}", e))?;
    }

    Ok(())
}

fn shred_file(path: &Path) -> Result<(), String> {
    if !path.exists() || !path.is_file() {
        return Ok(());
    }

    let meta = fs::metadata(path).map_err(|e| e.to_string())?;
    let file_len = meta.len();
    let mut file = OpenOptions::new().write(true).open(path).map_err(|e| e.to_string())?;
    let mut rng = ChaCha20Rng::from_entropy();

    let buf_size = 64 * 1024;
    let zeros = vec![0u8; buf_size];
    let ones = vec![0xFFu8; buf_size];

    // Pass 1: Zeros
    overwrite_bytes(&mut file, file_len, &zeros)?;
    // Pass 2: Ones
    overwrite_bytes(&mut file, file_len, &ones)?;

    // Pass 3: CSPRNG Random bytes
    use std::io::Seek;
    file.seek(std::io::SeekFrom::Start(0)).map_err(|e| e.to_string())?;
    let mut remaining = file_len;
    let mut rand_buf = vec![0u8; buf_size];
    while remaining > 0 {
        let chunk = (remaining as usize).min(buf_size);
        rng.fill_bytes(&mut rand_buf[..chunk]);
        file.write_all(&rand_buf[..chunk]).map_err(|e| e.to_string())?;
        remaining -= chunk as u64;
    }
    file.sync_all().map_err(|e| e.to_string())?;

    file.set_len(0).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    drop(file);

    let temp_name = format!(".shred_{}", rng.next_u64());
    let temp_path = path.with_file_name(temp_name);
    fs::rename(path, &temp_path).map_err(|e| e.to_string())?;
    fs::remove_file(temp_path).map_err(|e| e.to_string())?;

    Ok(())
}

fn overwrite_bytes(file: &mut fs::File, mut remaining: u64, buf: &[u8]) -> Result<(), String> {
    use std::io::Seek;
    file.seek(std::io::SeekFrom::Start(0)).map_err(|e| e.to_string())?;
    while remaining > 0 {
        let chunk = (remaining as usize).min(buf.len());
        file.write_all(&buf[..chunk]).map_err(|e| e.to_string())?;
        remaining -= chunk as u64;
    }
    file.sync_all().map_err(|e| e.to_string())?;
    Ok(())
}

fn shred_directory(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }

    if path.is_file() {
        return shred_file(path);
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                let _ = shred_directory(&p);
            } else {
                let _ = shred_file(&p);
            }
        }
    }

    let temp_name = format!(".shred_dir_{}", rand::random::<u64>());
    let temp_path = path.with_file_name(temp_name);
    let _ = fs::rename(path, &temp_path);
    let _ = fs::remove_dir(temp_path);
    Ok(())
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
    
    let tarball = match create_in_memory_tarball(&ram_vault) {
        Ok(t) => t,
        Err(e) => {
            let _ = fs::remove_dir_all(&ram_vault);
            return (format!("❌ Tar build failed: {}", e), true);
        }
    };
    let _ = shred_directory(&ram_vault);

    let encrypted_payload = match encrypt_vault_payload(&tarball, pass) {
        Ok(p) => p,
        Err(e) => return (format!("❌ Encryption failed: {}", e), true),
    };

    if fs::write(&enc_file, &encrypted_payload).is_err() {
        return ("❌ Failed to write vault file".into(), true);
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&enc_file, fs::Permissions::from_mode(0o700));
    }

    (format!("✅ Created vault: {}.enc", name), false)
}

fn vault_unlock(name: &str, pass: &str, store_dir: &PathBuf) -> (String, bool) {
    let enc_file = store_dir.join(format!("{}.enc", name));
    if !enc_file.exists() {
        return (format!("❌ Vault file not found: {}.enc", name), true);
    }

    let encrypted_data = match fs::read(&enc_file) {
        Ok(data) => data,
        Err(e) => return (format!("❌ Failed to read vault file: {}", e), true),
    };

    let decrypted_tarball = match decrypt_vault_payload(&encrypted_data, pass) {
        Ok(t) => t,
        Err(e) => return (format!("❌ {}", e), true),
    };

    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let target = home.join(name);
    let _ = fs::remove_dir_all(&target);

    if let Err(e) = extract_in_memory_tarball(&decrypted_tarball, &target) {
        return (format!("❌ Tar extraction failed: {}", e), true);
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&target, fs::Permissions::from_mode(0o700));
    }

    (format!("🔓 Unlocked to: ~/{}", name), false)
}

fn vault_lock(name: &str, pass: &str, store_dir: &PathBuf) -> (String, bool) {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let target = home.join(name);
    if !target.exists() {
        return (format!("❌ Vault directory ~/{} not found", name), true);
    }

    let enc_file = store_dir.join(format!("{}.enc", name));
    let tarball = match create_in_memory_tarball(&target) {
        Ok(t) => t,
        Err(e) => return (format!("❌ Tar archive failed: {}", e), true),
    };

    let encrypted_payload = match encrypt_vault_payload(&tarball, pass) {
        Ok(p) => p,
        Err(e) => return (format!("❌ Encryption failed: {}", e), true),
    };

    if fs::write(&enc_file, &encrypted_payload).is_err() {
        return ("❌ Failed to write encrypted vault file".into(), true);
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&enc_file, fs::Permissions::from_mode(0o700));
    }

    let _ = shred_directory(&target);
    (format!("🔒 Locked: {}.enc", name), false)
}

fn vault_delete(name: &str, pass: &str, store_dir: &PathBuf) -> (String, bool) {
    let enc_file = store_dir.join(format!("{}.enc", name));
    if !enc_file.exists() {
        return (format!("❌ Vault file not found: {}.enc", name), true);
    }

    let encrypted_data = match fs::read(&enc_file) {
        Ok(data) => data,
        Err(e) => return (format!("❌ Failed to read vault file: {}", e), true),
    };

    // Verify password first before destruction
    if let Err(e) = decrypt_vault_payload(&encrypted_data, pass) {
        return (format!("❌ Password verification failed: {}", e), true);
    }

    if let Err(e) = shred_file(&enc_file) {
        return (format!("❌ Shredding failed: {}", e), true);
    }

    (format!("💥 Securely deleted vault: {}.enc", name), false)
}

fn list_encrypted_vaults(store_dir: &PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut vaults = Vec::new();
    if let Ok(entries) = fs::read_dir(store_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".enc") {
                vaults.push(name);
            }
        }
    }
    vaults.sort();
    Ok(vaults)
}

fn run_cli(action: &str, args: &[String], store_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        "create" | "new" => {
            let name = args.first().cloned().unwrap_or_else(|| {
                print!("Vault name: ");
                io::stdout().flush().unwrap();
                let mut s = String::new();
                io::stdin().read_line(&mut s).unwrap();
                s.trim().replace(' ', "_")
            });
            let p = rpassword::prompt_password("Master Password: ")?;
            let (msg, _) = vault_create(name.trim(), p.trim(), store_dir);
            println!("{}", msg);
        }
        "unlock" | "open" => {
            let name = args.first().cloned().unwrap_or_default();
            let p = rpassword::prompt_password("Master Password: ")?;
            let (msg, _) = vault_unlock(name.trim(), p.trim(), store_dir);
            println!("{}", msg);
        }
        "lock" | "close" => {
            let name = args.first().cloned().unwrap_or_default();
            let p = rpassword::prompt_password("Master Password: ")?;
            let (msg, _) = vault_lock(name.trim(), p.trim(), store_dir);
            println!("{}", msg);
        }
        "delete" | "rm" => {
            let name = args.first().cloned().unwrap_or_default();
            let p = rpassword::prompt_password("Master Password (to confirm deletion): ")?;
            let (msg, _) = vault_delete(name.trim(), p.trim(), store_dir);
            println!("{}", msg);
        }
        "list" | "ls" => {
            let vaults = list_encrypted_vaults(store_dir)?;
            if vaults.is_empty() {
                println!("📋 No vaults found.");
            } else {
                for v in vaults {
                    println!("  🔒 {}", v);
                }
            }
        }
        _ => println!("Usage: fancybash vault [create <name> | lock <name> | unlock <name> | delete <name> | list]"),
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

    #[test]
    fn test_crypto_roundtrip() {
        let secret_data = b"Test secret payload data 12345";
        let password = "SuperSecretPassword123!";

        let encrypted = encrypt_vault_payload(secret_data, password).expect("Encryption failed");
        let decrypted = decrypt_vault_payload(&encrypted, password).expect("Decryption failed");

        assert_eq!(&decrypted[..], secret_data);
    }
}
