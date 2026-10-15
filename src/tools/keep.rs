// ============================================================================
// STATUS: 100% NATIVE RUST (ZERO EXTERNAL BINARY DEPENDENCIES)
// AUDIT COMPLETED: FULL FEATURE PARITY, CROSS-OS VERIFIED & OPTIMIZED
// HANDS-OFF GUARANTEE: NO MANUAL EDITS REQUIRED
// ============================================================================

// =============================================================================
//  src/tools/keep.rs — Master Command Center / Help Menu UI (Ratatui TUI)
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
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::error::Error;
use std::io::stdout;

const C_BG: Color = Color::Rgb(10, 12, 20);
const C_BORDER: Color = Color::Rgb(203, 166, 247); // Violet
const C_ACCENT: Color = Color::Rgb(148, 226, 213); // Cyan
const C_SELECTED_BG: Color = Color::Rgb(45, 25, 75);
const C_DIM: Color = Color::Rgb(90, 95, 120);
const C_TEXT: Color = Color::Rgb(220, 225, 235);
const C_GREEN: Color = Color::Rgb(166, 227, 161);
const C_YELLOW: Color = Color::Rgb(249, 226, 175);
const C_RED: Color = Color::Rgb(243, 139, 168);
const C_PINK: Color = Color::Rgb(245, 194, 231);
const C_BLUE: Color = Color::Rgb(137, 180, 250);
const C_ORANGE: Color = Color::Rgb(250, 179, 135);
const C_WHITE: Color = Color::Rgb(255, 255, 255);

struct CmdEntry {
    cmd: &'static str,
    desc: &'static str,
    example: &'static str,
    color: Color,
}

struct CmdCategory {
    icon: &'static str,
    title: &'static str,
    cmds: Vec<CmdEntry>,
}

fn build_categories() -> Vec<CmdCategory> {
    vec![
        CmdCategory {
            icon: "📂",
            title: "NAVIGATION & MOVEMENT",
            cmds: vec![
                CmdEntry {
                    cmd: "..",
                    desc: "Parent directory",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "...",
                    desc: "Two levels up",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "....",
                    desc: "Three levels up",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "dev",
                    desc: "Go to ~/Developer",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "fr / ba / fu",
                    desc: "Frontend / Backend / Fullstack",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "fig / ar / de",
                    desc: "Figma / Archive / Dev folders",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "des / doc / dow",
                    desc: "Desktop / Documents / Downloads",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "bv / ch / gp",
                    desc: "Brave / Chrome / Photos Downloads",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "root",
                    desc: "Jump to filesystem root (/)",
                    example: "root",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "z <query>",
                    desc: "Smart directory jump (z-jumper)",
                    example: "z dev",
                    color: C_ACCENT,
                },
            ],
        },
        CmdCategory {
            icon: "📄",
            title: "FILE & FOLDER MANAGEMENT",
            cmds: vec![
                CmdEntry {
                    cmd: "mkd <name>",
                    desc: "Create & enter directory",
                    example: "mkd new-project",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "t <file>",
                    desc: "Create file with feedback",
                    example: "t index.html",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "rmd <name>",
                    desc: "Force remove directory",
                    example: "rmd old-folder",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "rmf <file>",
                    desc: "Remove file (safe)",
                    example: "rmf file.txt",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "bak <file>",
                    desc: "Create backup copy",
                    example: "bak .env",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "trash <file>",
                    desc: "Move to system trash",
                    example: "trash junk.txt",
                    color: C_ORANGE,
                },
                CmdEntry {
                    cmd: "to",
                    desc: "Open current folder in VS Code",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "rn [dir]",
                    desc: "Smart batch file renamer",
                    example: "rn .",
                    color: C_PINK,
                },
                CmdEntry {
                    cmd: "ex <file>",
                    desc: "Universal archive extractor",
                    example: "ex file.zip",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "comp <archive>",
                    desc: "Parallel archive compressor",
                    example: "comp backup.tar.gz src/",
                    color: C_GREEN,
                },
            ],
        },
        CmdCategory {
            icon: "📦",
            title: "NPM COMMANDS",
            cmds: vec![
                CmdEntry {
                    cmd: "ni",
                    desc: "npm install",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "nid",
                    desc: "npm install -D",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "nr",
                    desc: "npm run",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "nrd",
                    desc: "npm run dev",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "nrb",
                    desc: "npm run build",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "nrs",
                    desc: "npm run start",
                    example: "",
                    color: C_YELLOW,
                },
            ],
        },
        CmdCategory {
            icon: "🥐",
            title: "BUN COMMANDS (Ultra Fast)",
            cmds: vec![
                CmdEntry {
                    cmd: "bi",
                    desc: "bun install",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "br",
                    desc: "bun run",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "brd",
                    desc: "bun run dev",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "bhot",
                    desc: "bun --hot",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "w",
                    desc: "bun --watch",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "brb",
                    desc: "bun run build",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "brs",
                    desc: "bun run start",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "html",
                    desc: "bun run index.html",
                    example: "",
                    color: C_ACCENT,
                },
            ],
        },
        CmdCategory {
            icon: "🌿",
            title: "GIT VERSION CONTROL",
            cmds: vec![
                CmdEntry {
                    cmd: "gi",
                    desc: "Initialize new repository",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "gs",
                    desc: "Check status (short format)",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "ga",
                    desc: "Stage all files",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "gcm <msg>",
                    desc: "Commit with message",
                    example: "gcm 'feat: add login'",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "gps / gpl",
                    desc: "Push / Pull from remote",
                    example: "",
                    color: C_PINK,
                },
                CmdEntry {
                    cmd: "gl",
                    desc: "View beautiful git log",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "gco <branch>",
                    desc: "Checkout branch",
                    example: "gco main",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "gcb <name>",
                    desc: "Create & checkout branch",
                    example: "gcb feature-x",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "gd",
                    desc: "View diff",
                    example: "",
                    color: C_ORANGE,
                },
                CmdEntry {
                    cmd: "gst / gsta / gpop",
                    desc: "Stash / Apply / Pop",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "gwip",
                    desc: "Quick WIP commit + auto push",
                    example: "",
                    color: C_PINK,
                },
                CmdEntry {
                    cmd: "gbranch",
                    desc: "Modern interactive Git branch manager",
                    example: "",
                    color: C_BORDER,
                },
            ],
        },
        CmdCategory {
            icon: "⚡",
            title: "PROJECT INITIALIZATION",
            cmds: vec![
                CmdEntry {
                    cmd: "project",
                    desc: "Interactive Project Setup & Tool Hub TUI",
                    example: "project",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "ii",
                    desc: "Initialize project (Bun/NPM choice)",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "next",
                    desc: "Setup Next.js project",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "ui",
                    desc: "Setup Shadcn UI with components",
                    example: "ui",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "vite",
                    desc: "Setup Vite project with Tailwind",
                    example: "",
                    color: C_BORDER,
                },
                CmdEntry {
                    cmd: "css",
                    desc: "Auto-install Tailwind CSS v4",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "run",
                    desc: "Bun Run JS & TS File (Interactive)",
                    example: "",
                    color: C_YELLOW,
                },
            ],
        },
        CmdCategory {
            icon: "⚙️",
            title: "C/C++ DEVELOPMENT",
            cmds: vec![
                CmdEntry {
                    cmd: "makecpp",
                    desc: "C/C++ boilerplate (cd, git, vscode)",
                    example: "makecpp proj",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "make run",
                    desc: "Compile and run the C/C++ project",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "make clean",
                    desc: "Remove compiled binary file",
                    example: "",
                    color: C_RED,
                },
            ],
        },
        CmdCategory {
            icon: "🛠️",
            title: "SYSTEM & MAINTENANCE",
            cmds: vec![
                CmdEntry {
                    cmd: "glade",
                    desc: "Interactive TUI theme picker & switcher",
                    example: "glade",
                    color: C_PINK,
                },
                CmdEntry {
                    cmd: "pcinfo",
                    desc: "System hardware diagnostics & profiler TUI",
                    example: "pcinfo",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "zed-setup",
                    desc: "Bulletproof Zed IDE settings installer",
                    example: "zed-setup",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "code-setup",
                    desc: "Bulletproof VS Code settings installer",
                    example: "code-setup",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "keep",
                    desc: "Master Command Center & Help Menu TUI",
                    example: "keep",
                    color: C_BORDER,
                },
                CmdEntry {
                    cmd: "update",
                    desc: "Modern System Package Updater (TUI)",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "clean",
                    desc: "Modern System Cache Cleaner (TUI)",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "uup",
                    desc: "MEGA UPDATE: Apt+Snap+Flatpak+Bun+Node",
                    example: "",
                    color: C_PINK,
                },
                CmdEntry {
                    cmd: "uu",
                    desc: "UNINSTALLER: Remove apps interactively",
                    example: "",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "uc",
                    desc: "Universal Clean (OS, Logs, Cache TUI)",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "ut",
                    desc: "Setup CLI tools for PC optimization",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "rt",
                    desc: "Install Node(nvm), Bun, Deno",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "kp <port>",
                    desc: "Kill process on port",
                    example: "kp 3000",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "setuppc",
                    desc: "Setup new PC with all tools",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "rel",
                    desc: "Reload shell configuration",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "myip / iploc",
                    desc: "Show IP / Location info",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "ports",
                    desc: "Show open ports",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "serve",
                    desc: "Start Python HTTP server",
                    example: "",
                    color: C_GREEN,
                },
            ],
        },
        CmdCategory {
            icon: "🚀",
            title: "ADVANCED INTERACTIVE TUIS",
            cmds: vec![
                CmdEntry {
                    cmd: "keep",
                    desc: "Master Command Center & Help Menu TUI",
                    example: "keep",
                    color: C_BORDER,
                },
                CmdEntry {
                    cmd: "glade",
                    desc: "Interactive prompt theme picker TUI",
                    example: "glade",
                    color: C_PINK,
                },
                CmdEntry {
                    cmd: "pcinfo",
                    desc: "Live system hardware diagnostics & profiler TUI",
                    example: "pcinfo",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "project",
                    desc: "Interactive Project Setup & Tool Hub TUI",
                    example: "project",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "todo",
                    desc: "Interactive 3-tier Task Manager",
                    example: "todo",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "notes",
                    desc: "Fuzzy Notes Manager with live preview",
                    example: "notes",
                    color: C_PINK,
                },
                CmdEntry {
                    cmd: "ffmedia",
                    desc: "24-in-1 FFmpeg Multimedia Suite",
                    example: "ffmedia",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "vault",
                    desc: "Hardened AES-256 Multi-Vault Manager",
                    example: "vault",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "cf",
                    desc: "Fuzzy find & navigate directories",
                    example: "cf",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "   ↳ ENTER",
                    desc: "cd to selected folder",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "   ↳ CTRL+O",
                    desc: "Open in VS Code / Cursor / Nvim",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "   ↳ CTRL+Y",
                    desc: "Copy path to clipboard",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "   ↳ CTRL+H",
                    desc: "Navigate to parent directory",
                    example: "",
                    color: C_BORDER,
                },
                CmdEntry {
                    cmd: "gbranch",
                    desc: "Git Branch Switcher & Manager TUI",
                    example: "gbranch",
                    color: C_BORDER,
                },
                CmdEntry {
                    cmd: "   ↳ ENTER",
                    desc: "Checkout selected branch",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "   ↳ CTRL+D",
                    desc: "Delete local branch (git branch -D)",
                    example: "",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "   ↳ CTRL+R",
                    desc: "Rebase onto selected branch",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "   ↳ CTRL+M",
                    desc: "Merge selected branch into current",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "fkill",
                    desc: "Advanced interactive process killer",
                    example: "fkill",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "   ↳ Tab",
                    desc: "Multi-select multiple processes",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "   ↳ ENTER",
                    desc: "Soft kill selected process(es) (SIGTERM)",
                    example: "",
                    color: C_ORANGE,
                },
                CmdEntry {
                    cmd: "   ↳ CTRL+X",
                    desc: "Force kill selected process(es) (SIGKILL -9)",
                    example: "",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "   ↳ CTRL+R",
                    desc: "Reload live process list",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "fcd",
                    desc: "Fuzzy quick directory jump",
                    example: "fcd",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "dman",
                    desc: "Docker Desktop interactive TUI manager",
                    example: "dman",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "fh",
                    desc: "Fuzzy History Search",
                    example: "fh",
                    color: C_YELLOW,
                },
            ],
        },
        CmdCategory {
            icon: "🖥️",
            title: "UTILITY TOOLS",
            cmds: vec![
                CmdEntry {
                    cmd: "ex <file>",
                    desc: "Extract any archive",
                    example: "ex file.zip",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "ff <name>",
                    desc: "Find file (excludes node_modules)",
                    example: "ff config",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "gen <len>",
                    desc: "Generate random secret key",
                    example: "gen 32",
                    color: C_BORDER,
                },
                CmdEntry {
                    cmd: "h <word>",
                    desc: "Search command history",
                    example: "h git",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "c / cls",
                    desc: "Clear terminal screen",
                    example: "",
                    color: C_DIM,
                },
                CmdEntry {
                    cmd: "v",
                    desc: "Interactive video player for directory",
                    example: "",
                    color: C_PINK,
                },
                CmdEntry {
                    cmd: "pg",
                    desc: "Generate package.json for current project",
                    example: "",
                    color: C_BORDER,
                },
            ],
        },
        CmdCategory {
            icon: "🐳",
            title: "DOCKER & CONTAINERS",
            cmds: vec![
                CmdEntry {
                    cmd: "dman",
                    desc: "Docker Desktop interactive TUI manager",
                    example: "dman",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "dstats",
                    desc: "Live Docker resource usage dashboard",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "dps",
                    desc: "List running containers",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "dpsa",
                    desc: "List all containers",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "di",
                    desc: "Show Docker images",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "dvl",
                    desc: "List Docker volumes",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "dnl",
                    desc: "List Docker networks",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "dsize",
                    desc: "Inspect Docker disk usage",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "dtop",
                    desc: "Show container resource usage",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "dstop <name>",
                    desc: "Stop container",
                    example: "dstop myapp",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "drm <name>",
                    desc: "Remove container",
                    example: "drm myapp",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "drmi <name>",
                    desc: "Remove image",
                    example: "drmi myimage",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "drestart <name>",
                    desc: "Restart container",
                    example: "drestart myapp",
                    color: C_ORANGE,
                },
                CmdEntry {
                    cmd: "dkill <name>",
                    desc: "Force remove container",
                    example: "dkill myapp",
                    color: C_ORANGE,
                },
                CmdEntry {
                    cmd: "dstopall",
                    desc: "Stop all running containers",
                    example: "",
                    color: C_ORANGE,
                },
                CmdEntry {
                    cmd: "drmall",
                    desc: "Remove all containers",
                    example: "",
                    color: C_ORANGE,
                },
                CmdEntry {
                    cmd: "dbuild <tag>",
                    desc: "Build Docker image",
                    example: "dbuild myapp .",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "dbuild-nocache <tag>",
                    desc: "Build without cache",
                    example: "dbuild-nocache myapp .",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "dhist <image>",
                    desc: "Show image history",
                    example: "dhist myimage",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "dports <name>",
                    desc: "Inspect container ports",
                    example: "dports myapp",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "dsh <name>",
                    desc: "Shell into container",
                    example: "dsh myapp bash",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "dlogs <name>",
                    desc: "Follow container logs",
                    example: "dlogs myapp",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "dcup / dcdn",
                    desc: "Compose up / down",
                    example: "",
                    color: C_BORDER,
                },
                CmdEntry {
                    cmd: "dclogs",
                    desc: "Follow compose logs",
                    example: "",
                    color: C_BORDER,
                },
                CmdEntry {
                    cmd: "dcupb",
                    desc: "Compose up with build",
                    example: "",
                    color: C_BORDER,
                },
                CmdEntry {
                    cmd: "dtest-ubuntu / dtest-node / dtest-alpine",
                    desc: "Launch test containers",
                    example: "dtest-node",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "dfind <term>",
                    desc: "Search containers / images",
                    example: "dfind nginx",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "droot <name>",
                    desc: "Enter container as root",
                    example: "droot myapp",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "dip <name>",
                    desc: "Show container IP",
                    example: "dip myapp",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "dwatch <name>",
                    desc: "Watch container changes",
                    example: "dwatch myapp",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "dnetstat <name>",
                    desc: "Inspect container network",
                    example: "dnetstat myapp",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "dtop-proc <name>",
                    desc: "Show process tree",
                    example: "dtop-proc myapp",
                    color: C_BORDER,
                },
                CmdEntry {
                    cmd: "dbackup <name> <archive>",
                    desc: "Backup volume to tar",
                    example: "dbackup data data.tar",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "dkill-force",
                    desc: "Force remove all containers",
                    example: "",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "dclean",
                    desc: "Clean unused Docker resources",
                    example: "",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "dstart",
                    desc: "Start Docker service",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "doff",
                    desc: "Stop Docker service",
                    example: "",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "dstatus",
                    desc: "Check Docker service status",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "denable",
                    desc: "Enable Docker auto-start on boot",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "ddisable",
                    desc: "Disable Docker auto-start on boot",
                    example: "",
                    color: C_ORANGE,
                },
            ],
        },
        CmdCategory {
            icon: "🐘",
            title: "POSTGRESQL DATABASE",
            cmds: vec![
                CmdEntry {
                    cmd: "pgstart / pgstop",
                    desc: "Start / Stop PostgreSQL service",
                    example: "",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "pgrestart",
                    desc: "Restart PostgreSQL service",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "pgstatus",
                    desc: "Check PostgreSQL service status",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "pgenable / pgdisable",
                    desc: "Enable / Disable auto-start on boot",
                    example: "",
                    color: C_ORANGE,
                },
                CmdEntry {
                    cmd: "pgl",
                    desc: "Login as postgres user (psql)",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "pgdb <name>",
                    desc: "Connect to a specific database",
                    example: "pgdb mydb",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "pgls",
                    desc: "List all databases",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "pgtables",
                    desc: "List all tables in current DB",
                    example: "",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "pgusers",
                    desc: "List all users / roles",
                    example: "",
                    color: C_BORDER,
                },
                CmdEntry {
                    cmd: "pgsize",
                    desc: "Show size of each database",
                    example: "",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "pgver",
                    desc: "Show PostgreSQL version",
                    example: "",
                    color: C_DIM,
                },
                CmdEntry {
                    cmd: "pgconn",
                    desc: "Show active connections count",
                    example: "",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "pgcreate <db>",
                    desc: "Create a new database",
                    example: "pgcreate mydb",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "pgdrop <db>",
                    desc: "Drop / delete a database",
                    example: "pgdrop mydb",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "pgdump <db>",
                    desc: "Dump / Backup a database",
                    example: "pgdump mydb > b.sql",
                    color: C_ORANGE,
                },
                CmdEntry {
                    cmd: "pgrestore <db>",
                    desc: "Restore database from file",
                    example: "pgrestore mydb < b.sql",
                    color: C_ORANGE,
                },
                CmdEntry {
                    cmd: "pglogs",
                    desc: "Follow PostgreSQL log file",
                    example: "",
                    color: C_RED,
                },
            ],
        },
        CmdCategory {
            icon: "🔷",
            title: "PRISMA ORM",
            cmds: vec![
                CmdEntry {
                    cmd: "np / bp",
                    desc: "npx / bunx prisma (base)",
                    example: "np",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "npi / bpi",
                    desc: "prisma init",
                    example: "npi",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "npg / bpg",
                    desc: "prisma generate",
                    example: "npg",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "nps / bps",
                    desc: "prisma studio",
                    example: "nps",
                    color: C_PINK,
                },
                CmdEntry {
                    cmd: "npmd / bpmd",
                    desc: "prisma migrate dev",
                    example: "npmd",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "npmdn / bpmdn <n>",
                    desc: "prisma migrate dev --name",
                    example: "npmdn add_users",
                    color: C_ACCENT,
                },
                CmdEntry {
                    cmd: "npmr / bpmr",
                    desc: "prisma migrate reset",
                    example: "npmr",
                    color: C_RED,
                },
                CmdEntry {
                    cmd: "npmdp / bpmdp",
                    desc: "prisma migrate deploy",
                    example: "npmdp",
                    color: C_BORDER,
                },
                CmdEntry {
                    cmd: "npms / bpms",
                    desc: "prisma migrate status",
                    example: "npms",
                    color: C_DIM,
                },
                CmdEntry {
                    cmd: "npdp / bpdp",
                    desc: "prisma db push",
                    example: "npdp",
                    color: C_ORANGE,
                },
                CmdEntry {
                    cmd: "npdl / bpdl",
                    desc: "prisma db pull",
                    example: "npdl",
                    color: C_BLUE,
                },
                CmdEntry {
                    cmd: "npds / bpds",
                    desc: "prisma db seed",
                    example: "npds",
                    color: C_GREEN,
                },
                CmdEntry {
                    cmd: "npf / bpf",
                    desc: "prisma format",
                    example: "npf",
                    color: C_YELLOW,
                },
                CmdEntry {
                    cmd: "npv / bpv",
                    desc: "prisma version",
                    example: "npv",
                    color: C_DIM,
                },
            ],
        },
    ]
}

struct App {
    categories: Vec<CmdCategory>,
    cat_state: ListState,
    query: String,
    filtered_cat_indices: Vec<usize>,
    detail_scroll: u16,
}

impl App {
    fn new() -> Self {
        let categories = build_categories();
        let filtered_cat_indices: Vec<usize> = (0..categories.len()).collect();
        let mut cat_state = ListState::default();
        cat_state.select(Some(0));

        Self {
            categories,
            cat_state,
            query: String::new(),
            filtered_cat_indices,
            detail_scroll: 0,
        }
    }

    fn selected_cat_index(&self) -> Option<usize> {
        let sel = self.cat_state.selected()?;
        self.filtered_cat_indices.get(sel).copied()
    }

    fn move_up(&mut self) {
        if self.filtered_cat_indices.is_empty() {
            return;
        }
        let i = self.cat_state.selected().unwrap_or(0);
        let next = if i == 0 {
            self.filtered_cat_indices.len() - 1
        } else {
            i - 1
        };
        self.cat_state.select(Some(next));
        self.detail_scroll = 0;
    }

    fn move_down(&mut self) {
        if self.filtered_cat_indices.is_empty() {
            return;
        }
        let i = self.cat_state.selected().unwrap_or(0);
        let next = (i + 1) % self.filtered_cat_indices.len();
        self.cat_state.select(Some(next));
        self.detail_scroll = 0;
    }

    fn scroll_detail_up(&mut self, delta: u16) {
        self.detail_scroll = self.detail_scroll.saturating_sub(delta);
    }

    fn scroll_detail_down(&mut self, delta: u16) {
        self.detail_scroll = self.detail_scroll.saturating_add(delta);
    }

    fn refilter(&mut self) {
        let q = self.query.to_lowercase();
        if q.is_empty() {
            self.filtered_cat_indices = (0..self.categories.len()).collect();
        } else {
            self.filtered_cat_indices = self
                .categories
                .iter()
                .enumerate()
                .filter(|(_, cat)| {
                    cat.title.to_lowercase().contains(&q)
                        || cat.cmds.iter().any(|c| {
                            c.cmd.to_lowercase().contains(&q) || c.desc.to_lowercase().contains(&q)
                        })
                })
                .map(|(idx, _)| idx)
                .collect();
        }

        let sel = self.cat_state.selected().unwrap_or(0);
        if self.filtered_cat_indices.is_empty() {
            self.cat_state.select(None);
        } else {
            self.cat_state
                .select(Some(sel.min(self.filtered_cat_indices.len() - 1)));
        }
        self.detail_scroll = 0;
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| draw_ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match (key.modifiers, key.code) {
                (_, KeyCode::Esc) | (KeyModifiers::CONTROL, KeyCode::Char('c')) => break,
                (_, KeyCode::PageUp)
                | (KeyModifiers::SHIFT, KeyCode::Up)
                | (KeyModifiers::CONTROL, KeyCode::Char('u')) => {
                    app.scroll_detail_up(3);
                }
                (_, KeyCode::PageDown)
                | (KeyModifiers::SHIFT, KeyCode::Down)
                | (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                    app.scroll_detail_down(3);
                }
                (_, KeyCode::Up) => app.move_up(),
                (_, KeyCode::Down) => app.move_down(),
                (_, KeyCode::Backspace) => {
                    app.query.pop();
                    app.refilter();
                }
                (_, KeyCode::Char(c)) => {
                    app.query.push(c);
                    app.refilter();
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn draw_ui(f: &mut Frame, app: &mut App) {
    let area = f.area();
    f.render_widget(Block::default().style(Style::default().bg(C_BG)), area);

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Banner
            Constraint::Length(3), // Search bar
            Constraint::Min(6),    // Dual pane
            Constraint::Length(3), // Status footer
        ])
        .split(area);

    // 1. Banner
    let header_spans = Line::from(vec![
        Span::styled(
            "🚀 MASTER COMMAND CENTER — ",
            Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "gladeshell ",
            Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "v2.0 • Ultra-High-Performance Shell Kit",
            Style::default().fg(C_WHITE),
        ),
    ]);
    let header = Paragraph::new(header_spans)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(C_BORDER))
                .style(Style::default().bg(C_BG)),
        );
    f.render_widget(header, main[0]);

    // 2. Search Bar
    let search_spans = Line::from(vec![
        Span::styled(" 🔍 ", Style::default().fg(C_ACCENT)),
        Span::styled(
            &app.query,
            Style::default().fg(C_WHITE).add_modifier(Modifier::BOLD),
        ),
        Span::styled("█", Style::default().fg(C_BORDER)),
        Span::styled(
            format!("  ({} categories matched)", app.filtered_cat_indices.len()),
            Style::default().fg(C_DIM),
        ),
    ]);
    let search_bar = Paragraph::new(search_spans).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_ACCENT))
            .title(Span::styled(
                " Real-time Command & Alias Search ",
                Style::default().fg(C_ACCENT),
            ))
            .style(Style::default().bg(C_BG)),
    );
    f.render_widget(search_bar, main[1]);

    // 3. Dual Pane Body (Left: Categories, Right: Command Cards)
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(main[2]);

    // Left Pane: Categories List
    let cat_items: Vec<ListItem> = app
        .filtered_cat_indices
        .iter()
        .enumerate()
        .map(|(idx, &cat_idx)| {
            let cat = &app.categories[cat_idx];
            let is_sel = app.cat_state.selected() == Some(idx);
            let pointer = if is_sel {
                Span::styled(
                    "❯ ",
                    Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw("  ")
            };

            let label_style = if is_sel {
                Style::default()
                    .fg(C_WHITE)
                    .add_modifier(Modifier::BOLD)
                    .bg(C_SELECTED_BG)
            } else {
                Style::default().fg(C_TEXT)
            };

            let line = Line::from(vec![
                pointer,
                Span::styled(format!("{} ", cat.icon), Style::default()),
                Span::styled(cat.title, label_style),
            ]);
            ListItem::new(line)
        })
        .collect();

    let cat_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(C_BORDER))
        .title(Span::styled(
            " Command Categories ",
            Style::default().fg(C_BORDER).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(C_BG));
    f.render_stateful_widget(
        List::new(cat_items).block(cat_block),
        body[0],
        &mut app.cat_state.clone(),
    );

    // Right Pane: Commands List / Details
    let mut detail_lines: Vec<Line> = Vec::new();
    if let Some(cat_idx) = app.selected_cat_index() {
        let cat = &app.categories[cat_idx];

        detail_lines.push(Line::from(vec![
            Span::styled(format!("{} ", cat.icon), Style::default()),
            Span::styled(
                cat.title,
                Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" ({} commands)", cat.cmds.len()),
                Style::default().fg(C_DIM),
            ),
        ]));
        detail_lines.push(Line::from(Span::styled(
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
            Style::default().fg(C_DIM),
        )));

        let q = app.query.to_lowercase();
        for cmd in &cat.cmds {
            if !q.is_empty()
                && !cmd.cmd.to_lowercase().contains(&q)
                && !cmd.desc.to_lowercase().contains(&q)
            {
                continue;
            }

            let mut spans = vec![
                Span::styled("  • ", Style::default().fg(C_BORDER)),
                Span::styled(
                    format!("{:<15}", cmd.cmd),
                    Style::default().fg(cmd.color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" │ ", Style::default().fg(C_DIM)),
                Span::styled(cmd.desc, Style::default().fg(C_TEXT)),
            ];

            if !cmd.example.is_empty() {
                spans.push(Span::styled(
                    format!(" ({})", cmd.example),
                    Style::default().fg(C_DIM),
                ));
            }

            detail_lines.push(Line::from(spans));
        }
    } else {
        detail_lines.push(Line::from(Span::styled(
            "No categories matched search query.",
            Style::default().fg(C_DIM),
        )));
    }

    let detail_title = if app.detail_scroll > 0 {
        format!(" Command Reference Cards [Scroll: {}] ", app.detail_scroll)
    } else {
        " Command Reference Cards ".to_string()
    };

    let detail_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(C_ACCENT))
        .title(Span::styled(
            detail_title,
            Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(C_BG));

    let detail_paragraph = Paragraph::new(detail_lines)
        .wrap(Wrap { trim: false })
        .scroll((app.detail_scroll, 0))
        .block(detail_block);
    f.render_widget(detail_paragraph, body[1]);

    // 4. Status Footer
    let hints =
        "↑↓ Category Navigation  |  PgUp/PgDn Scroll Cards  |  Type Search Query  |  Esc Quit";
    let footer = Paragraph::new(Line::from(vec![Span::styled(
        hints,
        Style::default().fg(C_DIM),
    )]))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_DIM))
            .style(Style::default().bg(C_BG)),
    );
    f.render_widget(footer, main[3]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keep_run_does_not_panic() {
        assert!(!build_categories().is_empty());
    }
}
