// =============================================================================
//  src/tools/project_setup.rs — Interactive Web Project Generator (Phase 5)
//
//  Now includes:
//    patch_tsconfig()      — 1:1 port of _ui_patch_tsconfig
//    patch_viteconfig()    — 1:1 port of _ui_patch_viteconfig
//    run_vite()            — adds --force / --legacy-peer-deps retry
//    run_ui()              — calls patch_tsconfig + patch_viteconfig for Vite
// =============================================================================

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

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

// ── Helpers ───────────────────────────────────────────────────────────────────

fn resolve_cmd(program: &str) -> String {
    if cfg!(target_os = "windows") {
        match program {
            "npm" => "npm.cmd".to_string(),
            "npx" => "npx.cmd".to_string(),
            "pnpm" => "pnpm.cmd".to_string(),
            "yarn" => "yarn.cmd".to_string(),
            "bunx" => "bunx.cmd".to_string(),
            other => {
                if other.ends_with(".cmd") || other.ends_with(".exe") {
                    other.to_string()
                } else {
                    format!("{other}.exe")
                }
            }
        }
    } else {
        program.to_string()
    }
}

fn cmd_ok(program: &str, args: &[&str]) -> bool {
    let resolved = resolve_cmd(program);
    Command::new(&resolved)
        .args(args)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

// ── tsconfig / jsconfig patcher ───────────────────────────────────────────────
//  Port of: _ui_patch_tsconfig

/// Patch the nearest tsconfig/jsconfig with `baseUrl` and `@/*` path alias.
///
/// Priority order (mirrors shell version):
///   1. tsconfig.app.json  (Vite TS)
///   2. tsconfig.json      (Next.js TS / generic)
///   3. jsconfig.json      (JS projects)
///   4. Create jsconfig.json if pure JS and none exists
pub fn patch_tsconfig() {
    let candidates = ["tsconfig.app.json", "tsconfig.json", "jsconfig.json"];

    let tsconfig_path = candidates.iter().find(|&&p| Path::new(p).exists()).copied();

    let target = match tsconfig_path {
        Some(p) => {
            match p {
                "tsconfig.app.json" => println!("  info: Vite (TS) detected -> patching {p}"),
                "tsconfig.json"     => println!("  info: TypeScript project -> patching {p}"),
                _                   => println!("  info: JavaScript project -> patching {p}"),
            }
            p.to_string()
        }
        None => {
            // No config found — create jsconfig.json for JS projects
            let is_js_project = Path::new("package.json").exists()
                && fs::read_to_string("package.json")
                    .map(|c| !c.contains("\"typescript\""))
                    .unwrap_or(false);

            if is_js_project {
                println!("  note: JS project — creating jsconfig.json with @/* alias...");
                let content = "{\n  \"compilerOptions\": {\n    \"baseUrl\": \".\",\n    \"paths\": { \"@/*\": [\"./src/*\"] }\n  }\n}\n";
                if let Err(e) = fs::write("jsconfig.json", content) {
                    eprintln!("  Failed to create jsconfig.json: {e}");
                } else {
                    println!("  -> jsconfig.json created with @/* alias");
                }
            } else {
                println!("  warning: No tsconfig/jsconfig found, skipping...");
            }
            return;
        }
    };

    // Read the file
    let raw = match fs::read_to_string(&target) {
        Ok(r) => r,
        Err(e) => { eprintln!("  Cannot read {target}: {e}"); return; }
    };

    // Check if already configured
    if raw.contains("\"@/*\"") {
        println!("  ok: {target} paths already configured.");
        return;
    }

    println!("  patching: {target} with baseUrl & @/* paths...");

    let patched = inject_ts_paths(&raw);
    match fs::write(&target, &patched) {
        Ok(_)  => println!("  -> baseUrl & paths written to {target}"),
        Err(e) => eprintln!("  Failed to write {target}: {e}"),
    }
}

/// Inject `baseUrl` and `paths` into a JSON string containing `compilerOptions`.
fn inject_ts_paths(json: &str) -> String {
    let patch = "    \"baseUrl\": \".\",\n    \"paths\": { \"@/*\": [\"./src/*\"] }";

    if let Some(pos) = json.find("\"compilerOptions\"") {
        if let Some(brace) = json[pos..].find('{') {
            let insert_at = pos + brace + 1;
            return format!("{}\n{},\n{}", &json[..insert_at], patch, &json[insert_at..]);
        }
    }

    // Fallback
    format!("{{\n  \"compilerOptions\": {{\n{patch}\n  }}\n}}\n")
}

// ── vite.config patcher ───────────────────────────────────────────────────────
//  Port of: _ui_patch_viteconfig

/// Patch vite.config.ts/js with Tailwind import, tailwindcss() plugin, and @/* alias.
pub fn patch_viteconfig() {
    let viteconfig = ["vite.config.ts", "vite.config.js"]
        .iter()
        .find(|&&p| Path::new(p).exists())
        .copied();

    let path = match viteconfig {
        Some(p) => p,
        None => {
            println!("  warning: vite.config.ts/js not found, skipping...");
            return;
        }
    };

    println!("  patching: {path} with path alias & tailwind...");

    let mut content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => { eprintln!("  Cannot read {path}: {e}"); return; }
    };

    // 1. import path from "path"
    if !content.contains("import path from") {
        content = format!("import path from \"path\"\n{content}");
        println!("  -> Added: import path from \"path\"");
    } else {
        println!("  ok: path import already exists.");
    }

    // 2. import tailwindcss from "@tailwindcss/vite"
    if !content.contains("@tailwindcss/vite") {
        content = format!("import tailwindcss from \"@tailwindcss/vite\"\n{content}");
        println!("  -> Added: import tailwindcss from \"@tailwindcss/vite\"");
    } else {
        println!("  ok: tailwindcss import already exists.");
    }

    // 3. tailwindcss() in plugins array
    if !content.contains("tailwindcss()") {
        content = content.replace("plugins: [", "plugins: [tailwindcss(), ");
        println!("  -> Added: tailwindcss() to plugins");
    } else {
        println!("  ok: tailwindcss() plugin already exists.");
    }

    // 4. resolve.alias
    if !content.contains("\"@\"") && !content.contains("'@'") {
        let alias_block = "  resolve: {\n    alias: {\n      \"@\": path.resolve(import.meta.dirname, \"./src\"),\n    },\n  }";
        if let Some(pos) = content.rfind("})") {
            content.insert_str(pos, &format!("{alias_block}\n"));
            println!("  -> Added: resolve.alias @/* -> ./src");
        }
    } else {
        println!("  ok: resolve.alias already exists.");
    }

    match fs::write(path, &content) {
        Ok(_)  => println!("  [OK] {path} patched."),
        Err(e) => eprintln!("  Failed to write {path}: {e}"),
    }
}

// ── Public subcommand runners ─────────────────────────────────────────────────

/// `fancybash ii` — Interactive Project Initializer (Bun/NPM/PNPM/Yarn + .gitignore).
pub fn run_ii() -> Result<(), Box<dyn std::error::Error>> {
    let pm = prompt_select(
        "🚀 Select Package Manager:",
        &[
            "1) 🥐 Bun (Fast)",
            "2) 📦 NPM (Standard)",
            "3) 🟡 PNPM (Strict)",
            "4) 🧶 Yarn (Classic)",
        ],
    )?;

    if pm.contains("Bun") {
        Command::new(resolve_cmd("bun")).arg("init").arg("-y").status()?;
    } else if pm.contains("NPM") {
        Command::new(resolve_cmd("npm")).arg("init").arg("-y").status()?;
    } else if pm.contains("PNPM") {
        Command::new(resolve_cmd("pnpm")).arg("init").status()?;
    } else {
        Command::new(resolve_cmd("yarn")).arg("init").arg("-y").status()?;
    }

    if !Path::new(".gitignore").exists() {
        let gitignore = "node_modules/\n.env\n.env*.local\ndist/\nbuild/\n.next/\n.cache/\n*.log\n.DS_Store\n";
        fs::write(".gitignore", gitignore)?;
        println!("✅ .gitignore created.");
    }

    println!("✅ Project initialized successfully!");
    Ok(())
}

/// `fancybash next` — Interactive Next.js Project Generator.
pub fn run_next() -> Result<(), Box<dyn std::error::Error>> {
    let pm = prompt_select("⚡ Setup Next.js with:", &["1) Bun", "2) NPM"])?;

    if pm.contains("Bun") {
        Command::new(resolve_cmd("bunx")).arg("create-next-app@latest").arg(".").status()?;
    } else {
        Command::new(resolve_cmd("npx")).arg("create-next-app@latest").arg(".").status()?;
    }
    Ok(())
}

/// `fancybash vite` — Interactive Vite Project Generator with Tailwind v4.
///
/// On install failure, offers `--force` (Bun) or `--legacy-peer-deps` (NPM) retry.
pub fn run_vite() -> Result<(), Box<dyn std::error::Error>> {
    let pm     = prompt_select("⚡ Setup Vite with:", &["1) Bun", "2) NPM"])?;
    let add_tw = prompt_confirm("Add Tailwind CSS v4?", true)?;

    if pm.contains("Bun") {
        Command::new(resolve_cmd("bunx")).arg("create-vite@latest").arg(".").status()?;
        if add_tw {
            let ok = cmd_ok("bun", &["add", "tailwindcss", "@tailwindcss/vite"]);
            if !ok {
                let retry = prompt_confirm("Install failed. Retry with --force?", false).unwrap_or(false);
                if retry {
                    cmd_ok("bun", &["add", "tailwindcss", "@tailwindcss/vite", "--force"]);
                }
            }
        }
    } else {
        Command::new(resolve_cmd("npx")).arg("create-vite@latest").arg(".").status()?;
        if add_tw {
            let ok = cmd_ok("npm", &["install", "tailwindcss", "@tailwindcss/vite"]);
            if !ok {
                let retry = prompt_confirm("Install failed (peer deps?). Retry with --legacy-peer-deps?", false).unwrap_or(false);
                if retry {
                    cmd_ok("npm", &["install", "tailwindcss", "@tailwindcss/vite", "--legacy-peer-deps"]);
                }
            }
        }
    }

    if add_tw {
        run_css()?;
    }

    Ok(())
}

/// `fancybash ui` — Interactive Shadcn UI Setup.
///
/// Pre-flight: `patch_tsconfig()`  
/// Post-flight (Vite only): `patch_viteconfig()`
pub fn run_ui() -> Result<(), Box<dyn std::error::Error>> {
    // Auto-detect project type
    let project_type = if Path::new("tsconfig.app.json").exists() {
        println!("  Detected: Vite project");
        "vite"
    } else if Path::new("next.config.js").exists()
        || Path::new("next.config.ts").exists()
        || Path::new("next.config.mjs").exists()
        || fs::read_to_string("package.json")
            .map(|c| c.contains("\"next\""))
            .unwrap_or(false)
    {
        println!("  Detected: Next.js project");
        "nextjs"
    } else {
        println!("  Could not auto-detect project type.");
        let choice = prompt_select("Choose manually:", &["1) Vite (React)", "2) Next.js"])?;
        if choice.contains("Vite") { "vite" } else { "nextjs" }
    };

    let pm         = prompt_select("Package manager:", &["1) Bun", "2) NPM"])?;
    let components = prompt_text("Add components (e.g. button card input, or empty for default):")?;

    // STEP 1: patch tsconfig BEFORE shadcn init
    println!("\nPre-configuring path aliases before shadcn init...");
    patch_tsconfig();

    let use_bun = pm.contains("Bun");

    // Build shadcn command args
    let runner = resolve_cmd(if use_bun { "bunx" } else { "npx" });
    let init_prefix: &[&str] = if use_bun { &["--bun", "shadcn@latest", "init"] } else { &["shadcn@latest", "init"] };
    let add_prefix:  &[&str] = if use_bun { &["--bun", "shadcn@latest", "add"] } else { &["shadcn@latest", "add"] };
    let vite_flag:   &[&str] = if project_type == "vite" { &["-t", "vite"] } else { &[] };

    // Init
    let _ = Command::new(&runner).args(init_prefix).args(vite_flag).status();

    // Add components
    let mut add_cmd = Command::new(&runner);
    add_cmd.args(add_prefix);
    if components.trim().is_empty() {
        add_cmd.arg("button");
    } else {
        for comp in components.split_whitespace() {
            add_cmd.arg(comp);
        }
    }
    add_cmd.status()?;

    // STEP 2: patch vite.config for Vite projects
    if project_type == "vite" {
        println!("\nPatching vite.config with alias & tailwind...");
        patch_viteconfig();
    } else {
        println!("  Next.js detected — vite.config patch skipped.");
    }

    println!("\n---------------------------------------------------");
    println!("✅ Shadcn UI setup complete! Happy coding!");
    println!("---------------------------------------------------");
    Ok(())
}

fn prompt_text(msg: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}: ", msg);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_confirm(msg: &str, default_yes: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let suffix = if default_yes { "[Y/n]" } else { "[y/N]" };
    print!("{} {}: ", msg, suffix);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let val = input.trim();
    if val.is_empty() {
        Ok(default_yes)
    } else {
        Ok(val.eq_ignore_ascii_case("y"))
    }
}

fn prompt_select(msg: &str, options: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    println!("\n{}", msg);
    for (i, opt) in options.iter().enumerate() {
        println!("  {}) {}", i + 1, opt);
    }
    print!("Select option [1-{}]: ", options.len());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().unwrap_or(1);
    let idx = choice.saturating_sub(1).min(options.len().saturating_sub(1));
    Ok(options[idx].to_string())
}

/// `fancybash css` — Tailwind CSS v4 Auto-Installer & Full Config Patcher.
pub fn run_css() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new("package.json").exists() {
        return Err("package.json not found!".into());
    }

    let is_bun = Path::new("bun.lockb").exists();
    let pm = if is_bun { "bun" } else { "npm" };

    println!("📦 Installing Tailwind CSS v4 & dependencies via {pm}...");
    if is_bun {
        Command::new(resolve_cmd("bun"))
            .args(["add", "-D", "tailwindcss", "@tailwindcss/vite", "clsx", "tailwind-merge", "@types/node"])
            .status()?;
    } else {
        Command::new(resolve_cmd("npm"))
            .args(["install", "-D", "tailwindcss", "@tailwindcss/vite", "clsx", "tailwind-merge", "@types/node"])
            .status()?;
    }

    let css_candidates = [
        "src/index.css",
        "src/style.css",
        "src/app/globals.css",
        "app/globals.css",
    ];
    let target_css = css_candidates
        .iter()
        .find(|&&p| Path::new(p).exists())
        .copied()
        .unwrap_or("src/index.css");

    let css_file = Path::new(target_css);
    if let Some(parent) = css_file.parent() {
        fs::create_dir_all(parent)?;
    }

    let current = if css_file.exists() { fs::read_to_string(css_file)? } else { String::new() };
    if !current.contains("@import \"tailwindcss\";") {
        fs::write(css_file, format!("@import \"tailwindcss\";\n{current}"))?;
        println!("✅ Added @import \"tailwindcss\"; to {target_css}");
    }

    // Auto-patch tsconfig & vite.config for 100% full zero-config setup
    println!("\n🔧 Auto-configuring tsconfig path aliases & vite.config plugins...");
    patch_tsconfig();
    patch_viteconfig();

    println!("\n---------------------------------------------------");
    println!("🎉 Full Tailwind CSS v4 setup complete!");
    println!("---------------------------------------------------");
    Ok(())
}

/// `fancybash html` — Serve or run `index.html` with Bun, or open in default browser.
pub fn run_html() -> Result<(), Box<dyn std::error::Error>> {
    let html_file = if Path::new("index.html").exists() {
        "index.html"
    } else if Path::new("public/index.html").exists() {
        "public/index.html"
    } else {
        "index.html"
    };

    if cmd_ok("bun", &["--version"]) {
        println!("🚀 Serving {html_file} with Bun...");
        Command::new(resolve_cmd("bun")).arg("run").arg(html_file).status()?;
    } else {
        println!("🌐 Opening {html_file} in default browser...");
        let opener = if cfg!(target_os = "macos") {
            "open"
        } else if cfg!(target_os = "windows") {
            "explorer"
        } else {
            "xdg-open"
        };
        let _ = Command::new(opener).arg(html_file).status();
    }
    Ok(())
}

// ── Interactive Project Hub TUI ─────────────────────────────────────────────

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
pub fn run_project() -> Result<(), Box<dyn std::error::Error>> {
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

fn run_project_tui() -> Result<Option<String>, Box<dyn std::error::Error>> {
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

// =============================================================================
//  Unit tests
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_css_target_paths() {
        assert!(Path::new("Cargo.toml").exists());
    }

    #[test]
    fn test_inject_ts_paths_with_compiler_options() {
        let input = r#"{ "compilerOptions": { "strict": true } }"#;
        let output = inject_ts_paths(input);
        assert!(output.contains("\"@/*\""));
        assert!(output.contains("baseUrl"));
    }

    #[test]
    fn test_inject_ts_paths_fallback() {
        let input = r#"{ "include": ["src"] }"#;
        let output = inject_ts_paths(input);
        assert!(output.contains("compilerOptions"));
        assert!(output.contains("\"@/*\""));
    }
}
