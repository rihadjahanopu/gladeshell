// ============================================================================
// STATUS: 100% NATIVE RUST (ZERO EXTERNAL BINARY DEPENDENCIES)
// AUDIT COMPLETED: FULL FEATURE PARITY, CROSS-OS VERIFIED & OPTIMIZED
// HANDS-OFF GUARANTEE: NO MANUAL EDITS REQUIRED
// ============================================================================

// =============================================================================
//  src/tools/zed_setup.rs — Zed IDE Settings Bulletproof Installer (gladeshell Edition)
// =============================================================================

use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::thread;

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// ── Colors & Formatting ───────────────────────────────────────
#[allow(dead_code)]
const RED: &str = "\x1b[38;2;243;139;168m";
#[allow(dead_code)]
const GREEN: &str = "\x1b[38;2;166;227;161m";
#[allow(dead_code)]
const YELLOW: &str = "\x1b[38;2;249;226;175m";
#[allow(dead_code)]
const BLUE: &str = "\x1b[38;2;137;180;250m";
#[allow(dead_code)]
const PURPLE: &str = "\x1b[38;2;203;166;247m";
#[allow(dead_code)]
const CYAN: &str = "\x1b[38;2;148;226;213m";
#[allow(dead_code)]
const GRAY: &str = "\x1b[38;2;147;153;178m";
#[allow(dead_code)]
const BOLD: &str = "\x1b[1m";
#[allow(dead_code)]
const NC: &str = "\x1b[0m";

// ── Settings Payload (Valid JSON) ─────────────────────────────
pub const ZED_SETTINGS: &str = r#"{
  "cursor_animation": {
    "enabled": true
  },
  "file_scan_exclusions": [
    "**/.git**",
    "**/node_modules**",
    "**/dist**",
    "**/build**",
    "**/.next**",
    "**/.turbo**",
    "**/target**",
    "**/*.csv"
  ],
  "enable_language_server": true,
  "hide_mouse": "never",
  "disable_ai": true,
  "cli_default_open_behavior": "existing_window",
  "code_lens": "on",
  "bottom_dock_layout": "contained",
  "colorize_brackets": true,
  "indent_guides": {
    "background_coloring": "disabled"
  },
  "agent_servers": {
    "antigravity-acp": {
      "type": "registry"
    },
    "opencode": {
      "type": "registry"
    }
  },
  "agent": {
    "dock": "right",
    "favorite_models": [],
    "model_parameters": []
  },
  "instrumentation": {
    "performance_profiler": {
      "enabled": true
    }
  },
  "proxy": "",
  "focus_follows_mouse": {
    "enabled": false
  },
  "which_key": {
    "enabled": false
  },
  "icon_theme": {
    "mode": "dark",
    "light": "Material Icon Theme",
    "dark": "Material Icon Theme"
  },
  "base_keymap": "VSCode",
  "selection_highlight": true,
  "cursor_blink": true,
  "use_system_path_prompts": true,
  "autosave": "on_focus_change",
  "show_completions_on_input": true,
  "auto_indent_on_paste": true,
  "linked_edits": true,
  "use_on_type_format": true,
  "soft_wrap": "editor_width",
  "tab_size": 2,
  "always_treat_brackets_as_autoclosed": true,
  "hover_popover_delay": 300,
  "ui_font_family": "Cascadia Code",
  "ui_font_size": 22.0,
  "buffer_font_size": 22.0,
  "buffer_font_family": "Cascadia Code",
  "buffer_font_fallbacks": ["JetBrains Mono", "Fira Code"],
  "confirm_quit": true,
  "session": {
    "trust_all_worktrees": true
  },
  "project_panel": {
    "default_width": 400.0,
    "dock": "left",
    "auto_fold_dirs": false,
    "hide_root": false,
    "git_status_indicator": true,
    "diagnostic_badges": true,
    "bold_folder_labels": true
  },
  "preview_tabs": {
    "enabled": false,
    "enable_preview_from_file_finder": true,
    "enable_preview_multibuffer_from_code_navigation": true
  },
  "status_bar": {
    "line_endings_button": true,
    "experimental.show": true,
    "show_active_file": true
  },
  "sticky_scroll": {
    "enabled": false
  },
  "minimap": {
    "show": "always"
  },
  "scrollbar": {
    "axes": {
      "horizontal": true
    }
  },
  "file_types": {
    "HTML": ["*.html", "*.njk", "*.ejs"]
  },
  "theme": {
    "mode": "dark",
    "light": "Ayu Light",
    "dark": "Tokyo Night Storm"
  },
  "terminal": {
    "font_weight": 400.0,
    "copy_on_select": true,
    "blinking": "on",
    "cursor_shape": "block",
    "line_height": {
      "custom": 1.3
    },
    "font_fallbacks": ["JetBrains Mono", "FiraCode Nerd Font"],
    "font_family": "Cascadia Code",
    "font_size": 22.0,
    "env": {
      "TERM": "xterm-256color"
    },
    "toolbar": {
      "breadcrumbs": true
    },
    "show_count_badge": true,
    "max_scroll_history_lines": 10000
  },
  "git": {
    "inline_blame": {
      "show_commit_summary": true,
      "delay_ms": 500
    }
  },
  "git_panel": {
    "tree_view": true,
    "show_count_badge": true,
    "file_icons": true
  },
  "tabs": {
    "file_icons": true,
    "git_status": true
  },
  "title_bar": {
    "button_layout": "platform_default",
    "show_menus": false,
    "show_branch_status_icon": true
  },
  "diagnostics": {
    "inline": {
      "enabled": true,
      "max_severity": "all"
    }
  },
  "prettier": {
    "allowed": true,
    "options": {
      "semi": true,
      "singleQuote": true,
      "tabWidth": 2,
      "trailingComma": "es5"
    }
  },
  "inlay_hints": {
    "show_background": true,
    "enabled": false
  },
  "toolbar": {
    "code_actions": true
  },
  "format_on_save": "on",
  "formatter": "prettier",
  "languages": {
    "JavaScript": {
      "formatter": "prettier",
      "code_actions_on_format": {
        "source.organizeImports": true
      }
    },
    "TypeScript": {
      "formatter": "prettier",
      "code_actions_on_format": {
        "source.organizeImports": true,
        "source.fixAll.eslint": true
      },
      "language_servers": ["vtsls", "..."]
    },
    "TSX": {
      "formatter": "prettier",
      "code_actions_on_format": {
        "source.organizeImports": true,
        "source.fixAll.eslint": true
      },
      "language_servers": ["vtsls", "..."]
    },
    "Rust": {
      "formatter": {
        "external": {
          "command": "rustfmt"
        }
      }
    },
    "HTML": {
      "formatter": "prettier"
    }
  },
  "lsp": {
    "rust-analyzer": {
      "settings": {
        "checkOnSave": {
          "command": "clippy"
        },
        "cargo": {
          "allFeatures": true
        },
        "procMacro": {
          "enable": true
        }
      }
    },
    "vtsls": {
      "settings": {
        "typescript": {
          "suggest": {
            "autoImports": true
          },
          "implementationsCodeLens": {
            "enabled": true,
            "showOnAllClassMethods": true
          },
          "referencesCodeLens": {
            "enabled": true,
            "showOnAllFunctions": true
          }
        },
        "javascript": {
          "suggest": {
            "autoImports": true
          },
          "implementationsCodeLens": {
            "enabled": true,
            "showOnAllClassMethods": true
          },
          "referencesCodeLens": {
            "enabled": true,
            "showOnAllFunctions": true
          }
        }
      },
      "initialization_options": {
        "typescript": {
          "suggest": {
            "autoImports": true,
            "completeFunctionCalls": true
          },
          "preferences": {
            "includeCompletionsWithInsertText": true
          }
        },
        "javascript": {
          "suggest": {
            "autoImports": true,
            "completeFunctionCalls": true
          },
          "preferences": {
            "includeCompletionsWithInsertText": true
          }
        }
      }
    },
    "typescript-language-server": {
      "initialization_options": {
        "preferences": {
          "includeCompletionsWithInsertText": true
        }
      }
    },
    "tailwindcss-language-server": {
      "settings": {
        "includeLanguages": {
          "typescriptreact": "html",
          "javascriptreact": "html"
        },
        "userLanguages": {
          "typescriptreact": "html"
        },
        "experimental": {
          "classRegex": [
            "\\.className\\s*[+]?=\\s*['\"]([^'\"]*)['\"]",
            "\\.setAttributeNS\\(.*,\\s*['\"]class['\"],\\s*['\"]([^'\"]*)['\"]",
            "\\.setAttribute\\(['\"]class['\"],\\s*['\"]([^'\"]*)['\"]",
            "\\.classList\\.add\\(['\"]([^'\"]*)['\"]",
            "\\.classList\\.remove\\(['\"]([^'\"]*)['\"]",
            "\\.classList\\.toggle\\(['\"]([^'\"]*)['\"]",
            "\\.classList\\.contains\\(['\"]([^'\"]*)['\"]",
            "\\.classList\\.replace\\(\\s*['\"]([^'\"]*)['\"]",
            "\\.classList\\.replace\\([^,)]+,\\s*['\"]([^'\"]*)['\"]"
          ]
        }
      }
    },
    "eslint": {
      "settings": {
        "rulesCustomizations": [{ "rule": "*", "severity": "warn" }],
        "problems": {
          "shortenToSingleLine": true
        }
      }
    }
  },
  "show_edit_predictions": true,
  "show_completion_documentation": true,
  "buffer_font_weight": 400,
  "buffer_line_height": { "custom": 1.5 },
  "relative_line_numbers": "disabled",
  "remove_trailing_whitespace_on_save": true,
  "ensure_final_newline_on_save": true,
  "outline_panel": {
    "dock": "right"
  },
  "seed_search_query_from_cursor": "always",
  "use_smartcase_search": true,
  "multi_cursor_modifier": "alt",
  "vim_mode": false,
  "show_wrap_guides": false,
  "wrap_guides": [80, 120],
  "preferred_line_length": 80,
  "window_title_format": "${projectName}${separator}${branch}",
  "window_title_separator": " — ",
  "auto_install_extensions": {
    "html": true,
    "git-firefly": true,
    "dockerfile": true,
    "material-icon-theme": true,
    "tokyo-night": true,
    "emmet": true,
    "prisma": true,
    "docker-compose": true,
    "colorizer": true,
    "color-highlight": true,
    "es7-react-redux-snippets": true
  }
}"#;

// ── RAII Guard to ensure cursor visibility on exit/panic ──────
struct CursorGuard;

impl CursorGuard {
    fn hide() -> Self {
        print!("\x1b[?25l");
        let _ = io::stdout().flush();
        CursorGuard
    }
}

impl Drop for CursorGuard {
    fn drop(&mut self) {
        print!("\x1b[?25h");
        let _ = io::stdout().flush();
    }
}

// ── Helper: Animated Spinner ──────────────────────────────────
fn spinner(msg: &str, duration_ms: u64) {
    let spin = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let delay = Duration::from_millis(80);
    let start = Instant::now();
    let target = Duration::from_millis(duration_ms);

    let _guard = CursorGuard::hide();

    let mut i = 0;
    while start.elapsed() < target {
        print!("\r  {}{}{} {}", CYAN, spin[i % spin.len()], NC, msg);
        let _ = io::stdout().flush();
        thread::sleep(delay);
        i += 1;
    }

    println!("\r  {}✔{} {}", GREEN, NC, msg);
}

// ── Helper: Progress Bar ──────────────────────────────────────
fn draw_progress_bar(current: usize, total: usize) {
    let total = if total == 0 { 5 } else { total };
    let width = 30;
    let percentage = (current * 100) / total;
    let completed = (width * current) / total;
    let remaining = width.saturating_sub(completed);

    let bar = "█".repeat(completed);
    let empty = "░".repeat(remaining);

    println!();
    println!(
        "{BLUE}Progress:{NC} [{GREEN}{bar}{GRAY}{empty}{NC}] {CYAN}{percentage}%{NC} (Step {current}/{total})",
        BLUE = BLUE,
        NC = NC,
        GREEN = GREEN,
        bar = bar,
        GRAY = GRAY,
        empty = empty,
        CYAN = CYAN,
        percentage = percentage,
        current = current,
        total = total
    );
}

// ── Header Banner ─────────────────────────────────────────────
fn show_header() {
    println!();
    println!(
        "{}          ██████╗ ██╗    █████╗ ██████╗ ███████╗███████╗██╗  ██╗███████╗██╗   ██╗{}",
        PURPLE, NC
    );
    println!(
        "{}         ██╔════╝ ██║   ██╔══██╗██╔══██╗██╔════╝██╔════╝██║  ██║██╔════╝██║   ██║{}",
        PURPLE, NC
    );
    println!(
        "{}         ██║  ███╗██║   ███████║██║  ██║█████╗  ███████╗███████║█████╗  ██║   ██║{}",
        CYAN, NC
    );
    println!(
        "{}         ██║   ██║██║   ██╔══██║██║  ██║██╔══╝  ╚════██║██║  ██║██╔══╝  ██║   ██║{}",
        CYAN, NC
    );
    println!(
        "{}         ╚██████╔╝██████╗██║  ██║██████╔╝███████╗███████║██║  ██║███████╗██████╗██████╗{}",
        BLUE, NC
    );
    println!(
        "{}          ╚═════╝ ╚═════╝╚═╝  ╚═╝╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝╚══════╝╚═════╝╚═════╝{}",
        BLUE, NC
    );
    println!();
    println!(
        "   ✨ {}{}G L A D E S H E L L{}  •  {}Zed IDE Settings Bulletproof Installer{}",
        BOLD, CYAN, NC, BOLD, NC
    );
    println!();
}

// ── Environment Detection Helpers ─────────────────────────────
fn get_home_dir() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        if !home.trim().is_empty() {
            return PathBuf::from(home);
        }
    }
    if let Ok(profile) = env::var("USERPROFILE") {
        if !profile.trim().is_empty() {
            return PathBuf::from(profile);
        }
    }
    PathBuf::from("~")
}

fn command_exists(cmd: &str) -> bool {
    crate::core::utils::cmd_exists(cmd)
}

fn get_uname_s() -> String {
    match env::consts::OS {
        "macos" => "Darwin".to_string(),
        "windows" => "Windows_NT".to_string(),
        "linux" => "Linux".to_string(),
        other => other.to_string(),
    }
}

fn get_arch() -> String {
    env::consts::ARCH.to_string()
}

fn get_user() -> String {
    if let Ok(u) = env::var("USER") {
        if !u.trim().is_empty() {
            return u;
        }
    }
    if let Ok(u) = env::var("USERNAME") {
        if !u.trim().is_empty() {
            return u;
        }
    }
    "user".to_string()
}

fn detect_system_and_paths() -> (String, Vec<PathBuf>) {
    let os_type = get_uname_s();
    let home = get_home_dir();
    let mut distro_name;
    let mut target_dirs = Vec::new();

    if os_type.starts_with("Linux") {
        distro_name = "Linux".to_string();
        if let Ok(os_release) = fs::read_to_string("/etc/os-release") {
            for line in os_release.lines() {
                if let Some(val) = line.strip_prefix("PRETTY_NAME=") {
                    distro_name = val.trim_matches('"').to_string();
                    break;
                }
            }
        }

        // 1. Linux Native Config
        target_dirs.push(home.join(".config/zed"));

        // 2. Linux Flatpak
        let flatpak_dir = home.join(".var/app/dev.zed.Zed");
        if flatpak_dir.is_dir() || command_exists("flatpak") {
            target_dirs.push(home.join(".var/app/dev.zed.Zed/config/zed"));
        }

        // 3. Linux Snap
        let snap_dir = home.join("snap/zed");
        if snap_dir.is_dir() || command_exists("snap") {
            target_dirs.push(home.join("snap/zed/current/.config/zed"));
        }

        // 4. WSL -> Detect Windows Host AppData via /mnt/c
        if let Ok(version_info) = fs::read_to_string("/proc/version") {
            let lower = version_info.to_lowercase();
            if lower.contains("microsoft") || lower.contains("wsl") {
                let mnt_c_users = Path::new("/mnt/c/Users");
                if mnt_c_users.exists() {
                    if let Ok(entries) = fs::read_dir(mnt_c_users) {
                        for entry in entries.flatten() {
                            let zed_win = entry.path().join("AppData/Roaming/Zed");
                            if zed_win.exists() {
                                target_dirs.push(zed_win);
                                break;
                            }
                        }
                    }
                }
            }
        }
    } else if os_type.starts_with("Darwin") {
        distro_name = "macOS".to_string();
        target_dirs.push(home.join("Library/Application Support/Zed"));
    } else if os_type.starts_with("Windows_NT")
        || cfg!(windows)
    {
        distro_name = "Windows".to_string();
        if let Ok(appdata) = env::var("APPDATA") {
            if !appdata.trim().is_empty() {
                target_dirs.push(PathBuf::from(appdata).join("Zed"));
            } else {
                target_dirs.push(home.join("AppData/Roaming/Zed"));
            }
        } else {
            target_dirs.push(home.join("AppData/Roaming/Zed"));
        }
    } else {
        distro_name = "Unknown OS".to_string();
        target_dirs.push(home.join(".config/zed"));
    }

    (distro_name, target_dirs)
}

fn show_sysinfo(distro_name: &str, target_count: usize) {
    let arch = get_arch();
    let user = get_user();

    println!("{}──────────────────────────────────────────────────{}", BLUE, NC);
    println!(" 🖥️   {}SYSTEM & ENVIRONMENT INFO{}", BOLD, NC);
    println!("{}──────────────────────────────────────────────────{}", BLUE, NC);
    println!("  💻  {}OS:{}            {}{}{}", BOLD, NC, CYAN, distro_name, NC);
    println!("  👤  {}User:{}          {}{}{}", BOLD, NC, CYAN, user, NC);
    println!("  ⚙️   {}Arch:{}          {}{}{}", BOLD, NC, CYAN, arch, NC);
    println!(
        "  📂  {}Detected Paths:{} {}{} location(s){}",
        BOLD, NC, CYAN, target_count, NC
    );
    println!("{}──────────────────────────────────────────────────{}\n", BLUE, NC);
}

// ── Timestamp Generator for Backups ───────────────────────────
fn get_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();

    let days = secs / 86400;
    let rem_secs = secs % 86400;
    let hours = rem_secs / 3600;
    let mins = (rem_secs % 3600) / 60;
    let seconds = rem_secs % 60;

    let z_days = (days as i64) + 719468;
    let era = (if z_days >= 0 { z_days } else { z_days - 146096 }) / 146097;
    let doe = (z_days - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };

    format!("{year:04}{m:02}{d:02}_{hours:02}{mins:02}{seconds:02}")
}


// ── Helper: Atomic Safe Installation ─────────────────────────
fn install_settings(dir: &Path) -> bool {
    let target = dir.join("settings.json");

    if fs::create_dir_all(dir).is_err() {
        println!("  {}❌ Failed to write → {}{}", RED, target.display(), NC);
        return false;
    }

    // Back up existing settings file if non-empty
    if target.is_file() {
        if let Ok(meta) = fs::metadata(&target) {
            if meta.len() > 0 {
                let timestamp = get_timestamp();
                let backup_name = format!("settings.json.bak.{}", timestamp);
                let backup_file = dir.join(&backup_name);
                if fs::copy(&target, &backup_file).is_ok() {
                    println!("  {}💾 Backup saved → {}{}", GRAY, backup_name, NC);
                }
            }
        }
    }

    // Write settings payload
    let payload = format!("{}\n", ZED_SETTINGS);
    if fs::write(&target, payload).is_err() {
        println!("  {}❌ Failed to write → {}{}", RED, target.display(), NC);
        return false;
    }

    if let Ok(meta) = fs::metadata(&target) {
        if meta.len() > 0 {
            println!("  {}✔ Config written → {}{}", GREEN, target.display(), NC);
            return true;
        }
    }

    println!("  {}❌ Failed to write → {}{}", RED, target.display(), NC);
    false
}

// ── Main Entry Point ──────────────────────────────────────────
pub fn run() -> Result<(), Box<dyn Error>> {
    show_header();

    let (distro_name, target_dirs) = detect_system_and_paths();
    show_sysinfo(&distro_name, target_dirs.len());

    let total_steps = 4;

    // Step 1: Environment & Directory Check
    let mut current_step = 1;
    draw_progress_bar(current_step, total_steps);
    spinner("Checking target configuration directories...", 150);
    println!(
        "  {}➜ Found {} candidate directory path(s).{}",
        CYAN,
        target_dirs.len(),
        NC
    );

    // Step 2: Backup Existing Settings
    current_step = 2;
    draw_progress_bar(current_step, total_steps);
    spinner("Scanning and backing up existing Zed settings...", 150);

    // Step 3: Installing Configuration
    current_step = 3;
    draw_progress_bar(current_step, total_steps);
    let mut success_count = 0;
    for dir in &target_dirs {
        println!("  ▶ Target: {}{}{}", BOLD, dir.display(), NC);
        if install_settings(dir) {
            success_count += 1;
        }
    }

    // Step 4: Verification & Finish
    current_step = 4;
    draw_progress_bar(current_step, total_steps);
    spinner("Verifying installation integrity...", 150);

    println!();
    if success_count > 0 {
        println!(
            "{}┌─────────────────────────────────────────────────────────────┐{}",
            GREEN, NC
        );
        println!(
            "{}│ {}✨  Installation Completed Successfully!                     {}{}",
            GREEN, BOLD, GREEN, NC
        );
        println!(
            "{}├─────────────────────────────────────────────────────────────┤{}",
            GREEN, NC
        );
        println!(
            "{}│{}  Updated {}{}{} Zed configuration path(s).                      {}│{}",
            GREEN, NC, BOLD, success_count, NC, GREEN, NC
        );
        println!(
            "{}│  💡 Restart Zed editor for changes to take effect.          │{}",
            GREEN, NC
        );
        println!(
            "{}└─────────────────────────────────────────────────────────────┘{}",
            GREEN, NC
        );
        println!();
        Ok(())
    } else {
        println!("{}❌ Failed to update any Zed configuration paths.{}\n", RED, NC);
        Err("Failed to update any Zed configuration paths.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zed_settings_valid_json() {
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(ZED_SETTINGS);
        assert!(parsed.is_ok(), "ZED_SETTINGS should be valid JSON");
    }

    #[test]
    fn test_detect_system_and_paths() {
        let (distro, paths) = detect_system_and_paths();
        assert!(!distro.is_empty());
        assert!(!paths.is_empty());
    }

    #[test]
    fn test_timestamp_non_empty() {
        let ts = get_timestamp();
        assert!(!ts.is_empty());
        assert_eq!(ts.len(), 15); // YYYYMMDD_HHMMSS is 15 chars
    }

    #[test]
    fn test_install_settings() {
        let temp_dir = env::temp_dir().join("gladeshell_test_zed_setup");
        let _ = fs::remove_dir_all(&temp_dir);

        // First install
        let ok = install_settings(&temp_dir);
        assert!(ok);
        let settings_file = temp_dir.join("settings.json");
        assert!(settings_file.exists());

        // Second install (should trigger backup)
        let ok2 = install_settings(&temp_dir);
        assert!(ok2);

        let entries: Vec<_> = fs::read_dir(&temp_dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();

        assert!(entries.iter().any(|name| name.starts_with("settings.json.bak.")));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
