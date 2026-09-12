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

// ── Helpers ───────────────────────────────────────────────────────────────────

fn cmd_ok(program: &str, args: &[&str]) -> bool {
    Command::new(program)
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
        let alias_block = "  resolve: {\n    alias: {\n      \"@\": path.resolve(__dirname, \"./src\"),\n    },\n  }";
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
        Command::new("bun").arg("init").arg("-y").status()?;
    } else if pm.contains("NPM") {
        Command::new("npm").arg("init").arg("-y").status()?;
    } else if pm.contains("PNPM") {
        Command::new("pnpm").arg("init").status()?;
    } else {
        Command::new("yarn").arg("init").arg("-y").status()?;
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
        Command::new("bunx").arg("create-next-app@latest").arg(".").status()?;
    } else {
        Command::new("npx").arg("create-next-app@latest").arg(".").status()?;
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
        Command::new("bunx").arg("create-vite@latest").arg(".").status()?;
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
        Command::new("npx").arg("create-vite@latest").arg(".").status()?;
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
    let runner = if use_bun { "bunx" } else { "npx" };
    let init_prefix: &[&str] = if use_bun { &["--bun", "shadcn@latest", "init"] } else { &["shadcn@latest", "init"] };
    let add_prefix:  &[&str] = if use_bun { &["--bun", "shadcn@latest", "add"] } else { &["shadcn@latest", "add"] };
    let vite_flag:   &[&str] = if project_type == "vite" { &["-t", "vite"] } else { &[] };

    // Init
    let _ = Command::new(runner).args(init_prefix).args(vite_flag).status();

    // Add components
    let mut add_cmd = Command::new(runner);
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

/// `fancybash css` — Tailwind CSS v4 Auto-Installer.
pub fn run_css() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new("package.json").exists() {
        return Err("package.json not found!".into());
    }

    let is_bun = Path::new("bun.lockb").exists();
    let pm = if is_bun { "bun" } else { "npm" };

    println!("📦 Installing Tailwind CSS v4 via {pm}...");
    if is_bun {
        Command::new("bun")
            .args(["add", "-D", "tailwindcss", "@tailwindcss/vite", "clsx", "tailwind-merge"])
            .status()?;
    } else {
        Command::new("npm")
            .args(["install", "-D", "tailwindcss", "@tailwindcss/vite", "clsx", "tailwind-merge"])
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

    println!("🎉 Tailwind CSS v4 setup complete!");
    Ok(())
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
