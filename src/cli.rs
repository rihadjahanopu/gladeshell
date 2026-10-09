// =============================================================================
//  src/cli.rs — Top-Level CLI Parser & Subcommand Declarations
// =============================================================================

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// ⚡ gladeshell — ultra-high-performance modular shell environment
#[derive(Parser, Debug)]
#[command(
    name = "gladeshell",
    version,
    author,
    about = "Zero-fork prompt engine & cross-shell dev environment kit",
    long_about = None,
    propagate_version = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Emit shell-specific bootstrap code to stdout
    ///
    /// Usage (add to shell rc file):
    ///   eval "$(gladeshell init zsh)"
    ///   eval "$(gladeshell init bash)"
    ///   gladeshell init fish | source
    ///   gladeshell init pwsh | Invoke-Expression
    Init(InitArgs),

    /// Print the gladeshell version
    Version,

    /// Generate shell auto-completions for Bash, Zsh, Fish, PowerShell, or Elvish
    #[command(alias = "completion")]
    Completions {
        /// Target shell (bash, zsh, fish, powershell, elvish)
        #[arg(value_name = "SHELL")]
        shell: String,
    },

    /// Theme management (list available themes or set active theme)
    #[command(alias = "glade", alias = "glade_theme")]
    Theme(ThemeArgs),

    /// Generate a cryptographically-secure secret key
    ///
    /// Output is hex-encoded. Default length: 32 bytes (256-bit).
    Gen(GenArgs),

    /// Universal archive extractor (zip, tar.gz, rar, 7z, bz2, xz, …)
    Ex(ExArgs),

    /// High-performance parallel multithreaded archive compressor (7z, zip, tar.gz, tar.xz, …)
    #[command(alias = "comp", alias = "pack", alias = "compress")]
    Cmp(CmpArgs),

    /// Mega system updater with interactive multi-select menu (uup)
    Uup,

    /// Interactive Git stage, commit & push with auto-rebase on conflict (gwip/gcommit)
    #[command(trailing_var_arg = true)]
    Gwip {
        /// Optional: `[type]` `[message]` — e.g. `feat "new login"` or `"my message"`
        args: Vec<String>,
    },

    /// Interactive universal app uninstaller (apt, snap, flatpak, AppImage)
    Uu,

    /// PC Arsenal — interactive multi-distro CLI tool installer & optimizer
    Ut,

    /// Non-interactive system package & maintenance updater (APT, Pacman, DNF, Brew, Flatpak, Snap)
    Update,

    /// Self-upgrade gladeshell to the latest version (curl install script)
    #[command(alias = "self-upgrade", alias = "self-update")]
    Upgrade,

    /// Self-uninstall gladeshell and restore original shell configuration
    Uninstall,

    /// Generate a C++ project boilerplate
    Makecpp(MakecppArgs),

    /// Interactive 24-in-1 FFmpeg multimedia suite (compress, trim, concat, convert, ...)
    #[command(alias = "ffstudio", alias = "fftool", alias = "glade_ffmpeg")]
    Ffmedia(FfmediaArgs),

    /// Interactive 3-tier task manager (todo add, todo done, todo list, todo clear)
    Todo(TodoArgs),

    /// Plain-text markdown notes manager (notes add, notes list, notes search, notes delete)
    Notes(NotesArgs),

    /// Hardened AES-256 Multi-Vault Manager (vault lock, vault unlock, vault create, vault list)
    #[command(alias = "secvault", alias = "fvault")]
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
    Grep(crate::tools::fast_grep::GrepArgs),

    /// High-performance native fast file finder (ff / file-find)
    #[command(alias = "file-find", alias = "find-file")]
    Ff(crate::tools::file_find::FfArgs),

    /// Kill process running on a specific port (`kp <port>`)
    Kp(KpArgs),

    /// Interactive Project Initializer (Bun, NPM, PNPM, Yarn + .gitignore)
    Ii,

    /// Interactive Project Setup & Tool Center TUI (project / projects)
    #[command(alias = "projects")]
    Project,

    /// Setup Next.js project
    Next,

    /// Setup Vite (React/Vue) project + Tailwind CSS
    Vite,

    /// Setup Shadcn UI components
    Ui,

    /// Install & configure Tailwind CSS v4
    Css,

    /// Serve or run index.html with Bun or system browser
    Html,

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

    /// Universal Package Converter (`pg <file> [-i]`)
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
    #[command(alias = "fcd")]
    Cf,

    /// Create files with confirmation feedback (`t <file1> <file2> ...`)
    T {
        /// File names to create
        #[arg(value_name = "FILES", required = true)]
        files: Vec<String>,
    },

    /// Create directory and enter it (`mkd <name>`)
    Mkd {
        /// Directory name to create
        name: String,
    },

    /// Force remove directory recursively (`rmd <name>`)
    Rmd {
        /// Directory or file name to remove
        name: String,
    },

    /// Remove file with confirmation (`rmf <file>`)
    Rmf {
        /// File name to remove
        name: String,

        /// Skip interactive confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Create backup copy (.bak) (`bak <file>`)
    Bak {
        /// File or directory name to back up
        name: String,
    },

    /// Move file to system trash safely (`trash <file>`)
    Trash {
        /// File or directory name to trash
        name: String,
    },

    /// Bulletproof Zed IDE settings installer (gladeshell edition)
    #[command(name = "zed-setup", alias = "zed", alias = "zed_setup")]
    Zed,

    /// Bulletproof VS Code settings + extensions installer (gladeshell edition)
    #[command(name = "code-setup", alias = "vscode", alias = "code_setup")]
    Code,

    /// Run the persistent background Unix socket server daemon
    Serve,

    /// Request a prompt string from the running daemon (or render directly as fallback)
    Prompt(PromptArgs),

    /// Hidden command: Clean and reorder the ~/.zshrc file using Native Rust
    #[command(hide = true)]
    InternalCleanRc,

    /// Hidden command: Ensure a system dependency is installed (used from shell init)
    #[command(name = "ensure-dep", hide = true)]
    EnsureDep(crate::tools::dep_installer::DepArgs),

    /// Native Rust Auto-LS directory change summary
    #[command(name = "auto-ls")]
    AutoLs {
        /// Optional target path
        path: Option<String>,
    },

    /// Smart Frecent Directory Jumper (`z <query>` or `z --add <path>`)
    Z(ZArgs),

    /// Check for command typos and suggest intended subcommand (correct)
    #[command(alias = "typo")]
    Correct {
        /// Mistyped command name
        #[arg(value_name = "COMMAND")]
        command: String,
    },

    /// Print the user-specific daemon Unix socket path
    #[command(name = "socket-path", alias = "socket_path")]
    SocketPath,

    /// Native Rust Autosuggestion engine lookup
    Suggest {
        /// Typed input buffer string
        #[arg(default_value = "")]
        buffer: String,
    },

    /// Native Rust Syntax Highlighting engine lookup
    Highlight {
        /// Typed input buffer string
        #[arg(default_value = "")]
        buffer: String,
    },

    /// Native Rust Autocompletion engine lookup
    Complete {
        /// Typed input buffer string
        #[arg(default_value = "")]
        buffer: String,
    },

    /// Auto-inject `eval "$(gladeshell init <shell>)"` into your shell RC file
    ///
    /// Detects your current shell and writes the eval line into ~/.zshrc,
    /// ~/.bashrc, or ~/.config/fish/config.fish automatically.
    /// Idempotent — safe to run multiple times.
    Setup,

    /// Advanced System Hardware Diagnostics & Live Sensors Profiler (pc-info / pcinfo)
    #[command(name = "pc-info", alias = "pcinfo")]
    PcInfo(PcInfoArgs),
}

#[derive(Args, Debug, Clone)]
pub struct PcInfoArgs {
    /// Output system report in JSON format
    #[arg(long)]
    pub json: bool,

    /// Output system report in YAML format
    #[arg(long)]
    pub yaml: bool,

    /// Output system report in TOML format
    #[arg(long)]
    pub toml: bool,

    /// Output system report in HTML format (interactive dashboard page)
    #[arg(long)]
    pub html: bool,

    /// Output system report as an SVG status badge (for GitHub Profile READMEs)
    #[arg(long, alias = "badge")]
    pub svg: bool,

    /// Output compact fastfetch/neofetch-style text summary
    #[arg(long, short = 's')]
    pub summary: bool,

    /// Launch interactive Ratatui TUI Dashboard
    #[arg(long, short)]
    pub tui: bool,
}

#[derive(Args, Debug, Clone)]
pub struct PromptArgs {
    /// Working directory (defaults to $PWD)
    #[arg(long, default_value = ".")]
    pub cwd: String,

    /// Exit code of previous command
    #[arg(long, default_value_t = 0)]
    pub exit_code: i32,

    /// Theme ID (0-55)
    #[arg(long, default_value_t = 0)]
    pub theme_id: usize,

    /// Username
    #[arg(long, default_value = "")]
    pub user: String,

    /// Hostname
    #[arg(long, default_value = "")]
    pub host: String,

    /// Command duration in milliseconds
    #[arg(long, default_value_t = 0)]
    pub cmd_duration: u64,

    /// Target shell (zsh | bash | fish | pwsh)
    #[arg(long, default_value = "zsh")]
    pub shell: String,

    /// Render compact transient prompt (❯ ) on accept-line
    #[arg(long)]
    pub transient: bool,
}

#[derive(Args, Debug, Clone)]
pub struct ZArgs {
    /// Directory query or keywords to jump to
    #[arg(value_name = "QUERY")]
    pub query: Vec<String>,

    /// Add directory path to frecency database
    #[arg(long, short = 'a')]
    pub add: Option<String>,

    /// List top frecent directories
    #[arg(long, short = 'l')]
    pub list: bool,
}

#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    /// Target shell: bash | zsh | fish | pwsh
    #[arg(value_name = "SHELL")]
    pub shell: String,

    /// Suppress the header comment block in generated output
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
}

#[derive(Args, Debug, Clone)]
pub struct ThemeArgs {
    /// Theme name to activate, "list", "set-color", "reset-color", or "list-colors"
    #[arg(value_name = "THEME_OR_ACTION")]
    pub name: Option<String>,

    /// Theme name (when action is set-color/reset-color) or color element (user, path, git, etc.)
    #[arg(value_name = "ARG1")]
    pub element: Option<String>,

    /// Color element (when action is set-color) or color code
    #[arg(value_name = "ARG2")]
    pub color: Option<String>,

    /// Color value (#ff0055, 214, cyan, default)
    #[arg(value_name = "ARG3")]
    pub val: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct GenArgs {
    /// Number of random bytes to generate (output is 2× hex digits)
    #[arg(value_name = "BYTES", default_value_t = 32)]
    pub length: usize,

    /// Output raw bytes to stdout (not hex-encoded)
    #[arg(long)]
    pub raw: bool,
}

#[derive(Args, Debug, Clone)]
pub struct ExArgs {
    /// Archive file to extract (optional: launch interactive TUI if omitted)
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Destination directory (defaults to current directory)
    #[arg(short, long, value_name = "DIR")]
    pub output: Option<PathBuf>,

    /// Launch interactive TUI archive manager mode
    #[arg(short = 'i', long = "interactive")]
    pub interactive: bool,
}

#[derive(Args, Debug, Clone)]
pub struct CmpArgs {
    /// File or directory to compress (defaults to current directory)
    #[arg(value_name = "TARGET")]
    pub target: Option<String>,

    /// Output archive name (e.g. my_backup or archive.7z)
    #[arg(short, long, value_name = "OUTPUT")]
    pub output: Option<String>,

    /// Archive format (7z, zip, tar.gz, tar.xz, tar.bz2, tar)
    #[arg(short, long, value_name = "FORMAT")]
    pub format: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct MakecppArgs {
    /// Project name (used for directory and CMakeLists.txt target)
    #[arg(value_name = "NAME")]
    pub name: String,

    /// C++ standard to use (11, 14, 17, 20, 23)
    #[arg(long, default_value = "17")]
    pub std: String,
}

#[derive(Args, Debug, Clone)]
pub struct FfmediaArgs {
    /// Action preset to execute (optional)
    #[arg(value_name = "ACTION")]
    pub action: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct TodoArgs {
    /// Action: add | list | done | clear
    #[arg(value_name = "ACTION")]
    pub action: Option<String>,

    /// Additional arguments (task text or number)
    #[arg(value_name = "ARGS")]
    pub args: Vec<String>,
}

#[derive(Args, Debug, Clone)]
pub struct NotesArgs {
    /// Action: add | list | search | delete
    #[arg(value_name = "ACTION")]
    pub action: Option<String>,

    /// Additional arguments (title, query, etc.)
    #[arg(value_name = "ARGS")]
    pub args: Vec<String>,
}

#[derive(Args, Debug, Clone)]
pub struct VaultArgs {
    /// Action: lock | unlock | create | list
    #[arg(value_name = "ACTION")]
    pub action: Option<String>,

    /// Additional arguments (vault name)
    #[arg(value_name = "ARGS")]
    pub args: Vec<String>,
}

#[derive(Args, Debug, Clone)]
pub struct KpArgs {
    /// Port number (e.g. 3000)
    #[arg(value_name = "PORT")]
    pub port: Option<String>,
}
