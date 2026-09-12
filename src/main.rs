// =============================================================================
//  fancybash — Native CLI Binary (`fancybash`)
//
//  Sub-commands (Phase 1 skeleton; Phase 4 expands each):
//    fancybash ut                  → interactive PC arsenal tool installer
//    fancybash upgrade             → self-upgrade fancybash to latest version
//    fancybash init <shell>        → emit shell-specific bootstrap code
//    fancybash version             → print version
//    fancybash theme [list|<name>] → (stub) theme switcher
//    fancybash gen [length]        → cryptographically-secure secret generator
//    fancybash ex <archive>        → universal archive extractor
//    fancybash uup                 → mega system updater
//    fancybash uu                  → interactive app uninstaller
//    fancybash makecpp <name>      → C++ project boilerplate generator
// =============================================================================

use clap::{Parser, Subcommand};

// Re-use lib logic from the same crate (rlib target)
use fancybash_core::init;

// ── Top-level CLI parser ─────────────────────────────────────────────────────

/// ⚡ fancybash — ultra-high-performance modular shell environment
#[derive(Parser, Debug)]
#[command(
    name = "fancybash",
    version,
    author,
    about = "Zero-fork prompt engine & cross-shell dev environment kit",
    long_about = None,
    propagate_version = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Emit shell-specific bootstrap code to stdout
    ///
    /// Usage (add to shell rc file):
    ///   eval "$(fancybash init zsh)"
    ///   eval "$(fancybash init bash)"
    ///   fancybash init fish | source
    ///   fancybash init pwsh | Invoke-Expression
    Init(InitArgs),

    /// Print the fancybash version
    Version,

    /// Theme management (list available themes or set active theme)
    Theme(ThemeArgs),

    /// Generate a cryptographically-secure secret key
    ///
    /// Output is hex-encoded. Default length: 32 bytes (256-bit).
    Gen(GenArgs),

    /// Universal archive extractor (zip, tar.gz, rar, 7z, bz2, xz, …)
    Ex(ExArgs),

    /// Mega system updater with interactive multi-select menu (uup)
    Uup,

    /// Interactive Git stage, commit & push with auto-rebase on conflict (gwip/gcommit)
    #[command(trailing_var_arg = true)]
    Gwip {
        /// Optional: [type] [message] — e.g. `feat "new login"` or `"my message"`
        args: Vec<String>,
    },

    /// Interactive universal app uninstaller (apt, snap, flatpak, AppImage)
    Uu,

    /// PC Arsenal — interactive multi-distro CLI tool installer & optimizer
    Ut,

    /// Non-interactive system package & maintenance updater (APT, Pacman, DNF, Brew, Flatpak, Snap)
    Update,

    /// Self-upgrade fancybash to the latest version (curl install script)
    #[command(alias = "self-upgrade", alias = "self-update")]
    Upgrade,

    /// Self-uninstall fancybash and restore original shell configuration
    Uninstall,

    /// Generate a C++ project boilerplate
    Makecpp(MakecppArgs),

    /// Interactive 24-in-1 FFmpeg multimedia suite (compress, trim, concat, convert, ...)
    Ffmedia(FfmediaArgs),

    /// Interactive 3-tier task manager (todo add, todo done, todo list, todo clear)
    Todo(TodoArgs),

    /// Plain-text markdown notes manager (notes add, notes list, notes search, notes delete)
    Notes(NotesArgs),

    /// Hardened AES-256 Multi-Vault Manager (vault lock, vault unlock, vault create, vault list)
    Vault(VaultArgs),

    /// Interactive Docker TUI Manager (containers, images, volumes, networks, compose)
    Dman,

    /// Interactive Git Branch Switcher & Manager
    Gbranch,

    /// Interactive Process Killer (sysinfo)
    Fkill,

    /// Interactive Ratatui Fuzzy History Search (fh)
    #[command(alias = "history")]
    Fh,

    /// High-performance native ripgrep search engine (grep / rg)
    #[command(alias = "rg")]
    Grep(fancybash_core::tools::fast_grep::GrepArgs),

    /// Kill process running on a specific port (kp <port>)
    Kp(KpArgs),

    /// Interactive Project Initializer (Bun, NPM, PNPM, Yarn + .gitignore)
    Ii,

    /// Setup Next.js project
    Next,

    /// Setup Vite (React/Vue) project + Tailwind CSS
    Vite,

    /// Setup Shadcn UI components
    Ui,

    /// Install & configure Tailwind CSS v4
    Css,

    /// Master Command Center Help Menu UI (keep / help)
    Keep,

    /// Interactive Bun JS/TS File Runner (run)
    Run,

    /// Interactive Video Search & Background Player (v)
    V {
        /// Optional target file or directory
        target: Option<String>,
    },

    /// Universal System Cleaner & Optimizer (uc)
    Uc,

    /// Non-interactive System Maintenance Cache Cleaner (clean)
    Clean,

    /// Interactive JS Runtime & NVM Installer (rt)
    Rt,

    /// Smart Batch File Renamer (rn)
    Rn {
        /// Target directory (defaults to .)
        target: Option<String>,
    },

    /// Universal Package Converter (pg <file> [-i])
    Pg {
        /// Package file to convert
        file: String,

        /// Install after conversion
        #[arg(short, long)]
        install: bool,
    },

    /// Smart External Media Drive Jumper (drive [1|2])
    Drive {
        /// Drive number (1 or 2)
        num: Option<String>,
    },

    /// Interactive Fuzzy Directory Navigator (cf)
    Cf,

    /// Create files with confirmation feedback (t <file1> <file2> ...)
    T {
        /// File names to create
        #[arg(value_name = "FILES", required = true)]
        files: Vec<String>,
    },

    /// Run the persistent background Unix socket server daemon
    Serve,

    /// Request a prompt string from the running daemon (or render directly as fallback)
    Prompt(PromptArgs),

    /// Hidden command: Clean and reorder the ~/.zshrc file using Native Rust
    #[command(hide = true)]
    InternalCleanRc,

    /// Hidden command: Ensure a system dependency is installed (used from shell init)
    #[command(name = "ensure-dep", hide = true)]
    EnsureDep(fancybash_core::tools::dep_installer::DepArgs),

    /// Native Rust Auto-LS directory change summary
    #[command(name = "auto-ls")]
    AutoLs,
}

#[derive(clap::Args, Debug)]
struct PromptArgs {
    /// Working directory (defaults to $PWD)
    #[arg(long, default_value = ".")]
    cwd: String,

    /// Exit code of previous command
    #[arg(long, default_value_t = 0)]
    exit_code: i32,

    /// Theme ID (0-55)
    #[arg(long, default_value_t = 0)]
    theme_id: usize,

    /// Username
    #[arg(long, default_value = "")]
    user: String,

    /// Hostname
    #[arg(long, default_value = "")]
    host: String,

    /// Command duration in milliseconds
    #[arg(long, default_value_t = 0)]
    cmd_duration: u64,

    /// Target shell (zsh | bash | fish | pwsh)
    #[arg(long, default_value = "zsh")]
    shell: String,
}

// ── Sub-command argument structs ─────────────────────────────────────────────

#[derive(clap::Args, Debug)]
struct InitArgs {
    /// Target shell: bash | zsh | fish | pwsh
    #[arg(value_name = "SHELL")]
    shell: String,

    /// Suppress the header comment block in generated output
    #[arg(long, default_value_t = false)]
    no_header: bool,
}

#[derive(clap::Args, Debug)]
struct ThemeArgs {
    /// Theme name to activate, or "list" to show all available themes
    #[arg(value_name = "THEME")]
    name: Option<String>,
}

#[derive(clap::Args, Debug)]
struct GenArgs {
    /// Number of random bytes to generate (output is 2× hex digits)
    #[arg(value_name = "BYTES", default_value_t = 32)]
    length: usize,

    /// Output raw bytes to stdout (not hex-encoded)
    #[arg(long)]
    raw: bool,
}

#[derive(clap::Args, Debug)]
struct ExArgs {
    /// Archive file to extract
    #[arg(value_name = "FILE")]
    file: std::path::PathBuf,

    /// Destination directory (defaults to current directory)
    #[arg(short, long, value_name = "DIR")]
    output: Option<std::path::PathBuf>,
}

#[derive(clap::Args, Debug)]
struct MakecppArgs {
    /// Project name (used for directory and CMakeLists.txt target)
    #[arg(value_name = "NAME")]
    name: String,

    /// C++ standard to use (11, 14, 17, 20, 23)
    #[arg(long, default_value = "17")]
    std: String,
}

#[derive(clap::Args, Debug)]
struct FfmediaArgs {
    /// Action preset to execute (optional)
    #[arg(value_name = "ACTION")]
    action: Option<String>,
}

#[derive(clap::Args, Debug)]
struct TodoArgs {
    /// Action: add | list | done | clear
    #[arg(value_name = "ACTION")]
    action: Option<String>,

    /// Additional arguments (task text or number)
    #[arg(value_name = "ARGS")]
    args: Vec<String>,
}

#[derive(clap::Args, Debug)]
struct NotesArgs {
    /// Action: add | list | search | delete
    #[arg(value_name = "ACTION")]
    action: Option<String>,

    /// Additional arguments (title, query, etc.)
    #[arg(value_name = "ARGS")]
    args: Vec<String>,
}

#[derive(clap::Args, Debug)]
struct VaultArgs {
    /// Action: lock | unlock | create | list
    #[arg(value_name = "ACTION")]
    action: Option<String>,

    /// Additional arguments (vault name)
    #[arg(value_name = "ARGS")]
    args: Vec<String>,
}

#[derive(clap::Args, Debug)]
struct KpArgs {
    /// Port number (e.g. 3000)
    #[arg(value_name = "PORT")]
    port: Option<String>,
}

// ── Entry point ──────────────────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Some(cmd) => match cmd {
            Commands::Init(args) => cmd_init(args),
            Commands::Version => {
                println!("fancybash {}", env!("CARGO_PKG_VERSION"));
                Ok(())
            }
            Commands::Theme(args) => cmd_theme(args),
            Commands::Gen(args) => cmd_gen(args),
            Commands::Ex(args) => cmd_ex(args),
            Commands::Gwip { args } => cmd_gwip(args),
            Commands::Uup => cmd_uup(),
            Commands::Uu  => cmd_uu(),
            Commands::Ut  => cmd_ut(),
            Commands::Update => fancybash_core::tools::system_update::run(),
            Commands::Upgrade => fancybash_core::tools::self_upgrade::run(),
            Commands::Uninstall => fancybash_core::tools::self_uninstall::run(),
            Commands::Makecpp(args) => cmd_makecpp(args),
            Commands::Ffmedia(args) => cmd_ffmedia(args),
            Commands::Todo(args) => cmd_todo(args),
            Commands::Notes(args) => cmd_notes(args),
            Commands::Vault(args) => cmd_vault(args),
            Commands::Dman => fancybash_core::tools::dman::run(None),
            Commands::Gbranch => fancybash_core::tools::gbranch::run(),
            Commands::Fkill => fancybash_core::tools::fkill::run_fkill(),
            Commands::Fh => fancybash_core::tools::history_search::run(),
            Commands::Grep(args) => fancybash_core::tools::fast_grep::run(args),
            Commands::Kp(args) => fancybash_core::tools::fkill::run_kp(args.port.as_deref()),
            Commands::Ii => fancybash_core::tools::project_setup::run_ii(),
            Commands::Next => fancybash_core::tools::project_setup::run_next(),
            Commands::Vite => fancybash_core::tools::project_setup::run_vite(),
            Commands::Ui => fancybash_core::tools::project_setup::run_ui(),
            Commands::Css => fancybash_core::tools::project_setup::run_css(),
            Commands::Keep => fancybash_core::tools::keep::run(),
            Commands::Run => fancybash_core::tools::bun_runner::run(),
            Commands::V { target } => fancybash_core::tools::video_player::run(target.as_deref()),
            Commands::Uc => fancybash_core::tools::universal_clean::run(),
            Commands::Clean => fancybash_core::tools::system_clean::run(),
            Commands::Rt => fancybash_core::tools::runtime_installer::run(),
            Commands::Rn { target } => fancybash_core::tools::file_renamer::run(target.as_deref()),
            Commands::Pg { file, install } => fancybash_core::tools::pkg_converter::run(&file, install),
            Commands::Drive { num } => fancybash_core::tools::drive_jumper::run(num.as_deref()),
            Commands::Cf => fancybash_core::tools::fuzzy_cd::run(),
            Commands::T { files } => fancybash_core::tools::touch_tool::run(&files),
            Commands::Serve => cmd_serve(),
            Commands::Prompt(args) => cmd_prompt(args),
            Commands::InternalCleanRc => cmd_internal_clean_rc(),
            Commands::EnsureDep(args) => fancybash_core::tools::dep_installer::run(&args),
            Commands::AutoLs => {
                fancybash_core::tools::auto_ls::run();
                Ok(())
            }
        },
        None => fancybash_core::tools::keep::run(),
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
    print!("{code}");
    Ok(())
}

fn cmd_internal_clean_rc() -> Result<(), Box<dyn std::error::Error>> {
    init::cleaner::clean_rc_file()?;
    Ok(())
}

// ── theme ─────────────────────────────────────────────────────────────────────

fn cmd_theme(args: ThemeArgs) -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::core::prompt::{set_active_theme, THEMES};
    match args.name.as_deref() {
        Some("list") => {
            println!("Available themes ({} total):\n", THEMES.len());
            for (i, theme) in THEMES.iter().enumerate() {
                println!("  {:02}  {}", i, theme.name);
            }
        }
        Some(name) => {
            let _idx = set_active_theme(name)?;
            println!("✨ Theme set to: {}", name);
            println!("💡 Run 'source ~/.zshrc' to apply new theme.");
        }
        None => {
            let theme_names: Vec<String> = THEMES.iter().map(|t| t.name.to_string()).collect();
            println!("\n🎨 Select Fancybash Theme:");
            for (i, t) in theme_names.iter().enumerate() {
                println!("  {}) {}", i + 1, t);
            }
            print!("Select theme [1-{}]: ", theme_names.len());
            std::io::Write::flush(&mut std::io::stdout())?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let choice: usize = input.trim().parse().unwrap_or(1);
            let idx = choice.saturating_sub(1).min(theme_names.len().saturating_sub(1));
            let selected = &theme_names[idx];
            let _idx = set_active_theme(selected)?;
            println!("✨ Active theme updated to: {}", selected);
            println!("💡 Run 'source ~/.zshrc' or open a new terminal session.");
        }
    }
    Ok(())
}

// ── gen ──────────────────────────────────────────────────────────────────────

fn cmd_gen(args: GenArgs) -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::core::secret_gen;
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

fn cmd_ex(args: ExArgs) -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::tools::extractor;
    extractor::run(&args.file, args.output.as_deref())
}

// ── gwip ─────────────────────────────────────────────────────────────────────

fn cmd_gwip(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::tools::git_wip;
    git_wip::run(&args)
}

// ── uup ──────────────────────────────────────────────────────────────────────

fn cmd_uup() -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::tools::updater;
    updater::run()
}

// ── uu ───────────────────────────────────────────────────────────────────────

fn cmd_uu() -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::tools::uninstaller;
    uninstaller::run()
}

// ── ut ───────────────────────────────────────────────────────────────────────

fn cmd_ut() -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::tools::pc_optimizer;
    pc_optimizer::run()
}





// ── makecpp ──────────────────────────────────────────────────────────────────

fn cmd_makecpp(args: MakecppArgs) -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::tools::cpp_gen;
    cpp_gen::run(&args.name, &args.std)
}

// ── ffmedia ──────────────────────────────────────────────────────────────────

fn cmd_ffmedia(args: FfmediaArgs) -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::tools::ffmedia;
    ffmedia::run(args.action.as_deref())
}

// ── todo ─────────────────────────────────────────────────────────────────────

fn cmd_todo(args: TodoArgs) -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::tools::todo;
    todo::run(args.action.as_deref(), &args.args)
}

// ── notes ────────────────────────────────────────────────────────────────────

fn cmd_notes(args: NotesArgs) -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::tools::notes;
    notes::run(args.action.as_deref(), &args.args)
}

// ── vault ────────────────────────────────────────────────────────────────────

fn cmd_vault(args: VaultArgs) -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::tools::vault;
    vault::run(args.action.as_deref(), &args.args)
}

// ── serve ────────────────────────────────────────────────────────────────────

fn cmd_serve() -> Result<(), Box<dyn std::error::Error>> {
    fancybash_core::daemon::run_server()
}

// ── prompt ───────────────────────────────────────────────────────────────────

fn cmd_prompt(args: PromptArgs) -> Result<(), Box<dyn std::error::Error>> {
    use fancybash_core::daemon::client;

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

    let theme_id = if args.theme_id == 0 {
        fancybash_core::core::prompt::active_theme_id()
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
