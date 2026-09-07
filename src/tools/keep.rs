// =============================================================================
//  src/tools/keep.rs — Master Command Center / Help Menu UI
// =============================================================================

use std::error::Error;

const CYAN: &str   = "\x1b[38;5;51m";
const PINK: &str   = "\x1b[38;5;213m";
const PURPLE: &str = "\x1b[38;5;141m";
const GREEN: &str  = "\x1b[38;5;82m";
const YELLOW: &str = "\x1b[38;5;220m";
const ORANGE: &str = "\x1b[38;5;208m";
const BLUE: &str   = "\x1b[38;5;75m";
const RED: &str    = "\x1b[38;5;203m";
const WHITE: &str  = "\x1b[38;5;255m";
const GRAY: &str   = "\x1b[38;5;245m";
const BOLD: &str   = "\x1b[1m";
const DIM: &str    = "\x1b[2m";
const RESET: &str  = "\x1b[0m";

fn print_category(icon: &str, title: &str, color: &str) {
    println!("\n  {color}┌─────────────────────────────────────────────────────────────────────┐{RESET}");
    println!("  {color}│{RESET} {BOLD}{icon}  {title}{RESET}{color}{RESET}");
    println!("  {color}└─────────────────────────────────────────────────────────────────────┘{RESET}");
}

fn print_cmd(cmd: &str, desc: &str, example: &str, cmd_color: &str) {
    if example.is_empty() {
        println!("     {BOLD}{cmd_color}{cmd:<12}{RESET} {GRAY}│{RESET} {desc}");
    } else {
        println!("     {BOLD}{cmd_color}{cmd:<12}{RESET} {GRAY}│{RESET} {desc:<35} {DIM}{example}{RESET}");
    }
}

fn print_alias(alias: &str, equals: &str, full: &str, color: &str) {
    println!("     {BOLD}{color}{alias:<6}{RESET} {GRAY}{equals}{RESET} {DIM}{full}{RESET}");
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // Header
    println!("{CYAN} ╔══════════════════════════════════════════════════════════════════════════╗{RESET}");
    println!("{CYAN} ║{RESET}  {BOLD}{PINK}🚀  MASTER COMMAND CENTER {RESET}{CYAN}│{RESET} {GRAY}Developer Rihad's Ultimate Bash Environment{RESET}");
    println!("{CYAN} ╚══════════════════════════════════════════════════════════════════════════╝{RESET}");
    println!("{GRAY}  v2.0 • Modern Terminal UX • fancybash-rs{RESET}");

    // NAVIGATION
    print_category("📂", "NAVIGATION & MOVEMENT", CYAN);
    print_cmd("..", "Parent directory", "", YELLOW);
    print_cmd("...", "Two levels up", "", YELLOW);
    print_cmd("....", "Three levels up", "", YELLOW);
    print_cmd("dev", "Go to ~/Developer", "", GREEN);
    print_cmd("fr / ba / fu", "Frontend / Backend / Fullstack", "", GREEN);
    print_cmd("fig / ar / de", "Figma / Archive / Dev folders", "", GREEN);
    print_cmd("des / doc / dow", "Desktop / Documents / Downloads", "", GREEN);
    print_cmd("bv / ch / gp", "Brave / Chrome / Photos Downloads", "", GREEN);

    // FILE MANAGEMENT
    print_category("📄", "FILE & FOLDER MANAGEMENT", PINK);
    print_cmd("mkd <name>", "Create & enter directory", "mkd new-project", YELLOW);
    print_cmd("t <file>", "Create file with feedback", "t index.html", YELLOW);
    print_cmd("rmd <name>", "Force remove directory", "rmd old-folder", RED);
    print_cmd("rmf <file>", "Remove file (safe)", "rmf file.txt", RED);
    print_cmd("bak <file>", "Create backup copy", "bak .env", BLUE);
    print_cmd("trash <file>", "Move to system trash", "trash junk.txt", ORANGE);
    print_cmd("to", "Open current folder in VS Code", "", CYAN);

    // NPM COMMANDS
    print_category("📦", "NPM COMMANDS", GREEN);
    print_alias("ni", "→", "npm install", GREEN);
    print_alias("nid", "→", "npm install -D", GREEN);
    print_alias("nr", "→", "npm run", GREEN);
    print_alias("nrd", "→", "npm run dev", YELLOW);
    print_alias("nrb", "→", "npm run build", YELLOW);
    print_alias("nrs", "→", "npm run start", YELLOW);

    // BUN COMMANDS
    print_category("🥐", "BUN COMMANDS (Ultra Fast)", YELLOW);
    print_alias("bi", "→", "bun install", YELLOW);
    print_alias("br", "→", "bun run", YELLOW);
    print_alias("brd", "→", "bun run dev", GREEN);
    print_alias("bhot", "→", "bun --hot", CYAN);
    print_alias("w", "→", "bun --watch", CYAN);
    print_alias("brb", "→", "bun run build", GREEN);
    print_alias("brs", "→", "bun run start", GREEN);

    // GIT
    print_category("🌿", "GIT VERSION CONTROL", PURPLE);
    print_cmd("gi", "Initialize new repository", "", GREEN);
    print_cmd("gs", "Check status (short format)", "", BLUE);
    print_cmd("ga", "Stage all files", "", YELLOW);
    print_cmd("gcm <msg>", "Commit with message", "gcm 'feat: add login'", GREEN);
    print_cmd("gps / gpl", "Push / Pull from remote", "", PINK);
    print_cmd("gl", "View beautiful git log", "", CYAN);
    print_cmd("gco <branch>", "Checkout branch", "gco main", YELLOW);
    print_cmd("gcb <name>", "Create & checkout branch", "gcb feature-x", GREEN);
    print_cmd("gwip", "Quick WIP commit + auto push", "", PINK);

    // PROJECT INITIALIZATION
    print_category("⚡", "PROJECT INITIALIZATION", ORANGE);
    print_cmd("ii", "Initialize project (Bun/NPM choice)", "", GREEN);
    print_cmd("next", "Setup Next.js project", "", CYAN);
    print_cmd("ui", "Setup Shadcn UI with components", "ui", BLUE);
    print_cmd("vite", "Setup Vite project with Tailwind", "", PURPLE);
    print_cmd("css", "Auto-install Tailwind CSS", "", BLUE);
    print_cmd("run", "Bun Run JS & TS File (Interactive)", "", YELLOW);

    // C/C++ DEVELOPMENT
    print_category("⚙️", "C/C++ DEVELOPMENT", CYAN);
    print_cmd("makecpp", "C/C++ boilerplate (cd, git, vscode)", "makecpp proj", BLUE);
    print_cmd("make run", "Compile and run the C/C++ project", "", GREEN);
    print_cmd("make clean", "Remove compiled binary file", "", RED);

    // SYSTEM & MAINTENANCE
    print_category("⚙️", "SYSTEM & MAINTENANCE", BLUE);
    print_cmd("uup", "MEGA UPDATE: Apt+Snap+Flatpak+Bun+Node", "", PINK);
    print_cmd("uu", "UNINSTALLER: Remove apps interactively", "", RED);
    print_cmd("uc", "Universal Clean (OS, Logs, Cache)", "", YELLOW);
    print_cmd("ut", "Setup CLI tools for PC optimization", "", CYAN);
    print_cmd("rt", "Install Node(nvm), Bun, Deno", "", YELLOW);
    print_cmd("kp <port>", "Kill process on port", "kp 3000", RED);

    // UTILITY TOOLS
    print_category("💻", "UTILITY TOOLS", CYAN);
    print_cmd("ex <file>", "Extract any archive", "ex file.zip", GREEN);
    print_cmd("gen <len>", "Generate random secret key", "gen 32", PURPLE);
    print_cmd("v", "Interactive video player for directory", "", PINK);

    // ADVANCED TOOLS
    print_category("⚡", "ADVANCED INTERACTIVE TOOLS", PURPLE);
    print_cmd("todo", "Interactive Todo Task Manager", "todo", GREEN);
    print_cmd("notes", "Fuzzy Notes Manager with live preview", "notes", PINK);
    print_cmd("ffmedia", "24-in-1 FFmpeg Multimedia Suite", "ffmedia", CYAN);
    print_cmd("vault", "Hardened AES-256 Multi-Vault Manager", "vault", RED);
    print_cmd("gbranch", "Modern interactive Git branch manager", "gbranch", PURPLE);
    print_cmd("fkill", "Advanced interactive process killer", "fkill", RED);
    print_cmd("dman", "Docker Desktop interactive TUI manager", "dman", CYAN);

    // FOOTER PRO TIPS
    println!("\n  {PURPLE}┌─────────────────────────────────────────────────────────────────────┐{RESET}");
    println!("  {PURPLE}│{RESET}   ✨ {BOLD}PRO TIPS:{RESET}");
    println!("  {PURPLE}│{RESET}    • {YELLOW}cd <folder>{RESET} automatically lists files with colors");
    println!("  {PURPLE}│{RESET}    • Type {CYAN}folder name only{RESET} to auto-cd");
    println!("  {PURPLE}│{RESET}    • {WHITE}fancybash keep{RESET} displays this command reference center");
    println!("  {PURPLE}└─────────────────────────────────────────────────────────────────────┘{RESET}\n");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keep_run_does_not_panic() {
        assert!(run().is_ok());
    }
}
