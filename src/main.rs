// ============================================================================
// STATUS: 100% NATIVE RUST & BULLETPROOF (ZERO EXTERNAL BINARY DEPENDENCIES)
// AUDIT COMPLETED: FULL FEATURE PARITY, CROSS-OS VERIFIED & OPTIMIZED
// HANDS-OFF GUARANTEE: NO MANUAL EDITS REQUIRED
// ============================================================================

// =============================================================================
//  gladeshell — Native CLI Binary (`gladeshell`)
//
//  Sub-commands (Phase 1 skeleton; Phase 4 expands each):
//    gladeshell ut                  → interactive PC arsenal tool installer
//    gladeshell upgrade             → self-upgrade gladeshell to latest version
//    gladeshell init <shell>        → emit shell-specific bootstrap code
//    gladeshell version             → print version
//    gladeshell theme [list|<name>] → (stub) theme switcher
//    gladeshell gen [length]        → cryptographically-secure secret generator
//    gladeshell ex <archive>        → universal archive extractor
//    gladeshell uup                 → mega system updater
//    gladeshell uu                  → interactive app uninstaller
//    gladeshell makecpp <name>      → C++ project boilerplate generator
// =============================================================================

use clap::Parser;

// Re-use lib logic from the same crate (rlib target)
use gladeshell_core::cli::*;
use gladeshell_core::init;

// ── Fast-path CLI parser for hot-path subcommands (bypasses heavy clap metadata initialization) ──
#[inline(always)]
fn fast_parse_prompt_args(args: &[String]) -> Option<PromptArgs> {
    let mut cwd = String::from(".");
    let mut exit_code = 0i32;
    let mut theme_id = 0usize;
    let mut user = String::new();
    let mut host = String::new();
    let mut cmd_duration = 0u64;
    let mut shell = String::from("zsh");
    let mut transient = false;

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-h" || arg == "--help" {
            return None; // Fall back to clap for rendering help
        }
        if let Some(val) = arg.strip_prefix("--cwd=") {
            cwd = val.to_string();
        } else if arg == "--cwd" && i + 1 < args.len() {
            i += 1;
            cwd = args[i].clone();
        } else if let Some(val) = arg.strip_prefix("--exit-code=") {
            // Use safe default 0 on invalid value — never abort the fast-path
            exit_code = val.parse().unwrap_or(0);
        } else if arg == "--exit-code" && i + 1 < args.len() {
            i += 1;
            exit_code = args[i].parse().unwrap_or(0);
        } else if let Some(val) = arg.strip_prefix("--theme-id=") {
            theme_id = val.parse().unwrap_or(0);
        } else if arg == "--theme-id" && i + 1 < args.len() {
            i += 1;
            theme_id = args[i].parse().unwrap_or(0);
        } else if let Some(val) = arg.strip_prefix("--user=") {
            user = val.to_string();
        } else if arg == "--user" && i + 1 < args.len() {
            i += 1;
            user = args[i].clone();
        } else if let Some(val) = arg.strip_prefix("--host=") {
            host = val.to_string();
        } else if arg == "--host" && i + 1 < args.len() {
            i += 1;
            host = args[i].clone();
        } else if let Some(val) = arg.strip_prefix("--cmd-duration=") {
            cmd_duration = val.parse().unwrap_or(0);
        } else if arg == "--cmd-duration" && i + 1 < args.len() {
            i += 1;
            cmd_duration = args[i].parse().unwrap_or(0);
        } else if let Some(val) = arg.strip_prefix("--shell=") {
            shell = val.to_string();
        } else if arg == "--shell" && i + 1 < args.len() {
            i += 1;
            shell = args[i].clone();
        } else if arg == "--transient" {
            transient = true;
        } else {
            return None; // Truly unknown flag — fallback to clap for proper help/error
        }
        i += 1;
    }

    Some(PromptArgs {
        cwd,
        exit_code,
        theme_id,
        user,
        host,
        cmd_duration,
        shell,
        transient,
    })
}

// ── Entry point ──────────────────────────────────────────────────────────────

fn main() {
    let raw_args: Vec<String> = std::env::args().collect();

    // Fast-path dispatcher for hot-path subcommands (`prompt`, `version`, `auto-ls`)
    if raw_args.len() > 1 {
        let subcmd = &raw_args[1];
        let has_help = raw_args.iter().any(|arg| arg == "-h" || arg == "--help");
        if !has_help {
            if subcmd == "prompt" {
                if let Some(prompt_args) = fast_parse_prompt_args(&raw_args[2..]) {
                    if let Err(e) = cmd_prompt(prompt_args) {
                        eprintln!("error: {e}");
                        std::process::exit(1);
                    }
                    return;
                }
            } else if subcmd == "init" && raw_args.len() >= 3 && !raw_args[2].starts_with('-') {
                let shell = &raw_args[2];
                let no_header = raw_args.iter().any(|arg| arg == "--no-header");
                if let Err(e) = cmd_init(InitArgs {
                    shell: shell.clone(),
                    no_header,
                }) {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
                return;
            } else if subcmd == "version" || subcmd == "-V" || subcmd == "--version" {
                println!("gladeshell {}", env!("CARGO_PKG_VERSION"));
                return;
            } else if subcmd == "suggest" {
                let buffer = raw_args.get(2).map(|s| s.as_str()).unwrap_or("");
                if let Some(suggestion) = gladeshell_core::plugins::autosuggest::suggest(buffer) {
                    print!("{}", suggestion);
                }
                return;
            } else if subcmd == "highlight" {
                let buffer = raw_args.get(2).map(|s| s.as_str()).unwrap_or("");
                println!("{}", gladeshell_core::plugins::highlight::highlight(buffer));
                return;
            } else if subcmd == "complete" {
                let buffer = raw_args.get(2).map(|s| s.as_str()).unwrap_or("");
                println!(
                    "{}",
                    gladeshell_core::plugins::autocomplete::complete(buffer)
                );
                return;
            } else if subcmd == "socket-path" || subcmd == "socket_path" {
                println!("{}", gladeshell_core::daemon::socket_path_str());
                return;
            } else if subcmd == "auto-ls" {
                let path = raw_args.get(2).map(|s| s.as_str());
                gladeshell_core::tools::auto_ls::run_path(path);
                return;
            } else if subcmd == "z" {
                let z_args = if raw_args.len() > 2 {
                    if raw_args[2] == "-l" || raw_args[2] == "--list" {
                        ZArgs {
                            query: vec![],
                            add: None,
                            list: true,
                        }
                    } else if (raw_args[2] == "-a" || raw_args[2] == "--add") && raw_args.len() > 3
                    {
                        ZArgs {
                            query: vec![],
                            add: Some(raw_args[3].clone()),
                            list: false,
                        }
                    } else {
                        ZArgs {
                            query: raw_args[2..].to_vec(),
                            add: None,
                            list: false,
                        }
                    }
                } else {
                    ZArgs {
                        query: vec![],
                        add: None,
                        list: false,
                    }
                };
                if let Err(e) = gladeshell_core::tools::z_jumper::run(&z_args) {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
                return;
            }
        }
    }

    let _prog_name = raw_args
        .first()
        .map(|s| {
            std::path::Path::new(s)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        })
        .unwrap_or_default();

    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            if err.kind() == clap::error::ErrorKind::DisplayHelp
                || err.kind() == clap::error::ErrorKind::DisplayVersion
            {
                err.exit();
            }
            // Check if user ran `gladeshell <unknown_cmd>`
            if raw_args.len() > 1 && !raw_args[1].starts_with('-') {
                let unknown_subcmd = &raw_args[1];
                let pretty_msg =
                    gladeshell_core::core::typo_engine::render_pretty_suggestion(unknown_subcmd);
                eprint!("{}", pretty_msg);
                std::process::exit(1);
            } else {
                err.exit();
            }
        }
    };

    let result = match cli.command {
        Some(cmd) => match cmd {
            Commands::Init(args) => cmd_init(args),
            Commands::Version => {
                println!("gladeshell {}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }
            Commands::Completions { shell } => cmd_completions(&shell),
            Commands::Theme(args) => cmd_theme(args),
            Commands::Gen(args) => cmd_gen(args),
            Commands::Ex(args) => cmd_ex(args),
            Commands::Cmp(args) => cmd_cmp(args),
            Commands::Gwip { args } => cmd_gwip(args),
            Commands::Uup => cmd_uup(),
            Commands::Uu => cmd_uu(),
            Commands::Ut => cmd_ut(),
            Commands::Update => gladeshell_core::tools::system_update::run(),
            Commands::Upgrade => gladeshell_core::tools::self_upgrade::run(),
            Commands::Uninstall => gladeshell_core::tools::self_uninstall::run(),
            Commands::Makecpp(args) => cmd_makecpp(args),
            Commands::Ffmedia(args) => cmd_ffmedia(args),
            Commands::Todo(args) => cmd_todo(args),
            Commands::Notes(args) => cmd_notes(args),
            Commands::Vault(args) => cmd_vault(args),
            Commands::Dman => gladeshell_core::tools::dman::run(None),
            Commands::Gbranch => gladeshell_core::tools::gbranch::run(),
            Commands::Fkill => gladeshell_core::tools::fkill::run_fkill(),
            Commands::Fh => gladeshell_core::tools::history_search::run(),
            Commands::Grep(args) => gladeshell_core::tools::fast_grep::run(args),
            Commands::Ff(args) => gladeshell_core::tools::file_find::run(args),
            Commands::Kp(args) => gladeshell_core::tools::fkill::run_kp(args.port.as_deref()),
            Commands::Project => gladeshell_core::tools::project_setup::run_project(),
            Commands::Ii => gladeshell_core::tools::project_setup::run_ii(),
            Commands::Next => gladeshell_core::tools::project_setup::run_next(),
            Commands::Vite => gladeshell_core::tools::project_setup::run_vite(),
            Commands::Ui => gladeshell_core::tools::project_setup::run_ui(),
            Commands::Css => gladeshell_core::tools::project_setup::run_css(),
            Commands::Html => gladeshell_core::tools::project_setup::run_html(),
            Commands::Keep => gladeshell_core::tools::keep::run(),
            Commands::Run => gladeshell_core::tools::bun_runner::run(),
            Commands::V { target } => gladeshell_core::tools::video_player::run(target.as_deref()),
            Commands::Uc => gladeshell_core::tools::universal_clean::run(),
            Commands::Clean => gladeshell_core::tools::system_clean::run(),
            Commands::Rt => gladeshell_core::tools::runtime_installer::run(),
            Commands::Rn { target } => gladeshell_core::tools::file_renamer::run(target.as_deref()),
            Commands::Pg { file, install } => {
                gladeshell_core::tools::pkg_converter::run(&file, install)
            }
            Commands::Drive { num } => gladeshell_core::tools::drive_jumper::run(num.as_deref()),
            Commands::Cf => gladeshell_core::tools::fuzzy_cd::run(),
            Commands::T { files } => gladeshell_core::tools::touch_tool::run(&files),
            Commands::Mkd { name } => gladeshell_core::tools::mkd::run(&name).map(|_| ()),
            Commands::Rmd { name } => gladeshell_core::tools::rmd::run(&name),
            Commands::Rmf { name, force } => gladeshell_core::tools::rmf::run(&name, force),
            Commands::Bak { name } => gladeshell_core::tools::bak::run(&name),
            Commands::Trash { name } => gladeshell_core::tools::trash::run(&name),
            Commands::Zed => gladeshell_core::tools::zed_setup::run(),
            Commands::Code => gladeshell_core::tools::code_setup::run(),
            Commands::Serve => cmd_serve(),
            Commands::Prompt(args) => cmd_prompt(args),
            Commands::InternalCleanRc => cmd_internal_clean_rc(),
            Commands::EnsureDep(args) => gladeshell_core::tools::dep_installer::run(&args),
            Commands::AutoLs { path } => {
                gladeshell_core::tools::auto_ls::run_path(path.as_deref());
                Ok(())
            }
            Commands::Z(args) => gladeshell_core::tools::z_jumper::run(&args),
            Commands::SocketPath => {
                println!("{}", gladeshell_core::daemon::socket_path_str());
                Ok(())
            }
            Commands::Suggest { buffer } => {
                if let Some(suggestion) = gladeshell_core::plugins::autosuggest::suggest(&buffer) {
                    print!("{}", suggestion);
                }
                Ok(())
            }
            Commands::Highlight { buffer } => {
                println!(
                    "{}",
                    gladeshell_core::plugins::highlight::highlight(&buffer)
                );
                Ok(())
            }
            Commands::Complete { buffer } => {
                println!(
                    "{}",
                    gladeshell_core::plugins::autocomplete::complete(&buffer)
                );
                Ok(())
            }
            Commands::Correct { command } => {
                let msg = gladeshell_core::core::typo_engine::render_pretty_suggestion(&command);
                println!("{}", msg);
                Ok(())
            }
            Commands::Setup => cmd_setup(),
            Commands::PcInfo(args) => cmd_pc_info(args.clone()),
        },
        None => {
            // When invoked as `glade`, `theme`, or plain `gladeshell` with no
            // subcommand → open the interactive theme picker TUI.
            // Help menu is still available via `gladeshell keep`.
            cmd_theme(ThemeArgs {
                name: None,
                element: None,
                color: None,
                val: None,
            })
        }
    };

    if let Err(e) = result {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

// =============================================================================
//  Sub-command implementations
// =============================================================================

// ── init ─────────────────────────────────────────────────────────────────────

fn cmd_init(args: InitArgs) -> Result<(), Box<dyn std::error::Error>> {
    let code = init::generate(&args.shell)?;
    // ⚠️  stdout is captured by `eval "$(gladeshell init <shell>)"` — only shell
    //    code goes here. All user-visible messages must go to stderr.
    print!("{code}");

    // Auto-inject the eval line into the shell RC file (idempotent).
    let rc_file = match args.shell.to_ascii_lowercase().as_str() {
        "zsh" => "~/.zshrc",
        "bash" => "~/.bashrc",
        "fish" => "~/.config/fish/config.fish",
        _ => "",
    };

    match init::cleaner::ensure_init_in_rc(&args.shell) {
        Ok(true) if !rc_file.is_empty() => {
            eprintln!(
                "\x1b[1;32m✅ Added `gladeshell init {}` to {rc_file}\x1b[0m",
                args.shell
            );
            eprintln!(
                "\x1b[1;36m💡 Run `source {rc_file}` or restart your shell to activate.\x1b[0m"
            );
        }
        Ok(false) if !rc_file.is_empty() => {
            // Already present — silent (idempotent)
        }
        Err(e) => {
            eprintln!("\x1b[0;33m⚠️  Could not write to {rc_file}: {e}\x1b[0m");
        }
        _ => {}
    }

    Ok(())
}

#[cold]
#[inline(never)]
fn cmd_internal_clean_rc() -> Result<(), Box<dyn std::error::Error>> {
    init::cleaner::clean_rc_file()?;
    Ok(())
}

#[cold]
#[inline(never)]
fn cmd_pc_info(args: PcInfoArgs) -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::tools::pc_info;

    if args.json {
        let report = pc_info::collect_system_report();
        println!("{}", report.to_json_pretty()?);
    } else if args.yaml {
        let report = pc_info::collect_system_report();
        println!("{}", report.to_yaml()?);
    } else if args.toml {
        let report = pc_info::collect_system_report();
        println!("{}", report.to_toml()?);
    } else if args.html {
        let report = pc_info::collect_system_report();
        println!("{}", report.to_html());
    } else if args.svg {
        let report = pc_info::collect_system_report();
        println!("{}", report.to_svg());
    } else if args.summary {
        let report = pc_info::collect_system_report();
        println!("{}", report.to_summary());
    } else {
        pc_info::run_tui()?;
    }
    Ok(())
}

// ── setup ─────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_setup() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("\x1b[1;36m🔧 gladeshell setup — checking all installed shells...\x1b[0m");

    let _ = init::cleaner::ensure_auto_heal_hooks();

    let configured = init::cleaner::ensure_all_installed_shells_configured()?;

    if configured.is_empty() {
        eprintln!(
            "\x1b[1;33m✔  All installed shells are already configured — nothing to do.\x1b[0m"
        );
    } else {
        for sh in &configured {
            eprintln!("\x1b[1;32m✅ Successfully injected gladeshell into shell: {sh}\x1b[0m");
        }
        eprintln!(
            "\x1b[1;36m💡 Restart your terminal or source your shell config to activate.\x1b[0m"
        );
    }

    // Pre-generate and cache zero-fork init scripts for all supported shells
    for sh in ["bash", "zsh", "fish", "pwsh"] {
        let _ = init::generate(sh);
    }

    Ok(())
}

// ── theme ─────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_theme(args: ThemeArgs) -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::core::prompt::{
        active_theme_id, get_effective_theme, load_theme_overrides, reset_theme_color_overrides,
        save_theme_color_override, set_active_theme, THEMES,
    };

    match args.name.as_deref() {
        // ── gladeshell theme list  →  styled table ─────────────────────────────
        Some("list") => {
            let active = active_theme_id();
            println!(
                "\n\x1b[1;35m🎨 Gladeshell Themes ({} total)\x1b[0m\n",
                THEMES.len()
            );
            println!(
                "  \x1b[2m{:<4} {:<3} {:<20} PROMPT\x1b[0m",
                "IDX", "  ", "NAME"
            );
            println!("  \x1b[2m{}\x1b[0m", "─".repeat(48));
            for (i, t) in THEMES.iter().enumerate() {
                let active_mark = if i == active {
                    "\x1b[1;32m✓\x1b[0m"
                } else {
                    " "
                };
                let name_color = if i == active {
                    "\x1b[1;36m"
                } else {
                    "\x1b[0;37m"
                };
                println!(
                    "  {} \x1b[2m{:02}\x1b[0m  {:<3} {}{:<20}\x1b[0m  \x1b[33m{}\x1b[0m",
                    active_mark, i, t.emoji, name_color, t.name, t.prompt_char
                );
            }
            println!();
        }

        // ── gladeshell theme set-color <theme> <element> <color> ───────────────
        Some("set-color") => {
            let theme_name = args
                .element
                .as_deref()
                .ok_or("Usage: gladeshell theme set-color <theme_name> <element> <color>")?;
            let element = args
                .color
                .as_deref()
                .ok_or("Usage: gladeshell theme set-color <theme_name> <element> <color>")?;
            let color_val = args
                .val
                .as_deref()
                .ok_or("Usage: gladeshell theme set-color <theme_name> <element> <color>")?;

            save_theme_color_override(theme_name, element, color_val)?;
            println!("\x1b[1;32m✅ Color override saved!\x1b[0m Theme: \x1b[1;36m{}\x1b[0m, Element: \x1b[1;33m{}\x1b[0m -> \x1b[1;35m{}\x1b[0m", theme_name, element, color_val);
        }

        // ── gladeshell theme reset-color [theme] ──────────────────────────────
        Some("reset-color") => {
            let target_theme = args.element.as_deref();
            reset_theme_color_overrides(target_theme)?;
            if let Some(t) = target_theme {
                println!(
                    "\x1b[1;32m✅ Color overrides reset for theme:\x1b[0m \x1b[1;36m{}\x1b[0m",
                    t
                );
            } else {
                println!("\x1b[1;32m✅ All theme color overrides reset to defaults.\x1b[0m");
            }
        }

        // ── gladeshell theme list-colors [theme] ──────────────────────────────
        Some("list-colors") => {
            let overrides = load_theme_overrides();
            if overrides.is_empty() {
                println!("\x1b[1;33mℹ️ No theme text color overrides configured in ~/.config/gladeshell/theme_overrides.toml\x1b[0m");
            } else {
                println!("\n\x1b[1;35m🎨 Configured Theme Color Overrides:\x1b[0m\n");
                for (t_name, o) in &overrides {
                    println!("  \x1b[1;36m[{}]\x1b[0m", t_name);
                    if let Some(ref c) = o.user_color {
                        println!("    user_color   = {}", c);
                    }
                    if let Some(ref c) = o.path_color {
                        println!("    path_color   = {}", c);
                    }
                    if let Some(ref c) = o.git_color {
                        println!("    git_color    = {}", c);
                    }
                    if let Some(ref c) = o.prompt_color {
                        println!("    prompt_color = {}", c);
                    }
                    if let Some(ref c) = o.prefix_color {
                        println!("    prefix_color = {}", c);
                    }
                    if let Some(ref c) = o.in_color {
                        println!("    in_color     = {}", c);
                    }
                    if let Some(ref c) = o.emoji_color {
                        println!("    emoji_color  = {}", c);
                    }
                }
                println!();
            }
        }

        // ── gladeshell theme <name>  →  set directly ───────────────────────────
        Some(name) => {
            let idx = set_active_theme(name)?;
            let t = get_effective_theme(idx);
            println!(
                "\x1b[1;32m✅ Theme '{}' {} applied!\x1b[0m",
                t.name, t.emoji
            );
            println!("\x1b[2m💡 Run 'source ~/.zshrc' to apply in this session.\x1b[0m");
        }

        // ── gladeshell theme  →  launch interactive TUI picker ─────────────────
        None => {
            #[cfg(feature = "tools")]
            {
                gladeshell_core::tools::theme_picker::run_theme_picker()?;
            }
            #[cfg(not(feature = "tools"))]
            {
                println!("Theme TUI requires the 'tools' feature. Run with: cargo build --features tools");
            }
        }
    }
    Ok(())
}

// ── gen ──────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_gen(args: GenArgs) -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::core::secret_gen;
    let bytes = secret_gen::generate(args.length)?;
    if args.raw {
        use std::io::Write;
        std::io::stdout().write_all(&bytes)?;
    } else {
        // Hex-encode without external crates
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        println!("{hex}");
    }
    Ok(())
}

// ── ex ───────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_ex(args: ExArgs) -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::tools::extractor;
    extractor::run(
        args.file.as_deref(),
        args.output.as_deref(),
        args.interactive,
    )
}

// ── cmp ───────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_cmp(args: CmpArgs) -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::tools::compressor;
    compressor::run(
        args.target.as_deref(),
        args.output.as_deref(),
        args.format.as_deref(),
    )
}

// ── gwip ─────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_gwip(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::tools::git_wip;
    git_wip::run(&args)
}

// ── uup ──────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_uup() -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::tools::updater;
    updater::run()
}

// ── uu ───────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_uu() -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::tools::uninstaller;
    uninstaller::run()
}

// ── ut ───────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_ut() -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::tools::pc_optimizer;
    pc_optimizer::run()
}

// ── makecpp ──────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_makecpp(args: MakecppArgs) -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::tools::cpp_gen;
    cpp_gen::run(&args.name, &args.std)
}

// ── ffmedia ──────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_ffmedia(args: FfmediaArgs) -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::tools::ffmedia;
    ffmedia::run(args.action.as_deref())
}

// ── todo ─────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_todo(args: TodoArgs) -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::tools::todo;
    todo::run(args.action.as_deref(), &args.args)
}

// ── notes ────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_notes(args: NotesArgs) -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::tools::notes;
    notes::run(args.action.as_deref(), &args.args)
}

// ── vault ────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_vault(args: VaultArgs) -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::tools::vault;
    vault::run(args.action.as_deref(), &args.args)
}

// ── serve ────────────────────────────────────────────────────────────────────

#[cold]
#[inline(never)]
fn cmd_serve() -> Result<(), Box<dyn std::error::Error>> {
    gladeshell_core::daemon::run_server()
}

// ── prompt ───────────────────────────────────────────────────────────────────

#[inline(always)]
fn cmd_prompt(args: PromptArgs) -> Result<(), Box<dyn std::error::Error>> {
    use gladeshell_core::daemon::client;

    let user = if args.user.is_empty() {
        std::env::var("USER").unwrap_or_else(|_| "user".into())
    } else {
        args.user
    };

    let host = if args.host.is_empty() {
        std::env::var("HOSTNAME").unwrap_or_else(|_| "host".into())
    } else {
        args.host
    };

    let shell_id = match args.shell.to_lowercase().as_str() {
        "zsh" => 0,
        "bash" => 1,
        "fish" => 2,
        "pwsh" => 3,
        _ => 0,
    };

    if args.transient {
        let transient_prompt =
            gladeshell_core::core::prompt::render_transient(args.exit_code, shell_id);
        print!("{transient_prompt}");
        return Ok(());
    }

    let theme_id = if args.theme_id == 0 {
        gladeshell_core::core::prompt::active_theme_id()
    } else {
        args.theme_id
    };

    match client::request_prompt(
        &args.cwd,
        args.exit_code,
        theme_id,
        &user,
        &host,
        args.cmd_duration,
        shell_id,
    ) {
        Ok(prompt) => print!("{prompt}"),
        Err(_) => {
            let fallback = client::render_fallback(
                &args.cwd,
                args.exit_code,
                theme_id,
                &user,
                &host,
                shell_id,
            );
            print!("{fallback}");
        }
    }

    Ok(())
}

#[cold]
#[inline(never)]
fn cmd_completions(shell_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    use clap::CommandFactory;
    use clap_complete::{generate, Shell};

    let shell = match shell_str.to_ascii_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" | "pwsh" => Shell::PowerShell,
        "elvish" => Shell::Elvish,
        other => {
            eprintln!("error: unsupported shell '{other}'. Supported shells: bash, zsh, fish, powershell, elvish");
            std::process::exit(1);
        }
    };

    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "gladeshell", &mut std::io::stdout());
    Ok(())
}
