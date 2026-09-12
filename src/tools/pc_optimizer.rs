// =============================================================================
//  src/tools/pc_optimizer.rs — `ut` PC Arsenal Tool Installer (Group 16)
//
//  1:1 port of the shell `ut()` function from config.sh / config.zsh.
//  Detects distro, shows an fzf multi-select menu of 70+ curated CLI tools
//  grouped by category (PERF/DISK/SECURE/NET/DEV/MODERN/SYS), installs the
//  selected ones, applies per-package auto-configuration, and prints a summary.
// =============================================================================

use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

// ── Distro / package-manager detection ───────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum PkgManager {
    Apt,
    Dnf,
    Yum,
    Pacman,
    Zypper,
    Apk,
    Xbps,
    Unknown,
}

impl PkgManager {
    /// Column index into a `|`-separated mapping string (0-based).
    fn col_index(&self) -> usize {
        match self {
            PkgManager::Apt => 0,
            PkgManager::Dnf | PkgManager::Yum => 1,
            PkgManager::Pacman => 2,
            PkgManager::Zypper => 3,
            PkgManager::Apk => 4,
            PkgManager::Xbps => 5,
            PkgManager::Unknown => 0,
        }
    }

    fn install_cmd(&self) -> Vec<String> {
        match self {
            PkgManager::Apt | PkgManager::Unknown => {
                vec!["sudo".into(), "apt".into(), "install".into(), "-y".into()]
            }
            PkgManager::Dnf => vec!["sudo".into(), "dnf".into(), "install".into(), "-y".into()],
            PkgManager::Yum => vec!["sudo".into(), "yum".into(), "install".into(), "-y".into()],
            PkgManager::Pacman => vec![
                "sudo".into(),
                "pacman".into(),
                "-S".into(),
                "--noconfirm".into(),
                "--needed".into(),
            ],
            PkgManager::Zypper => {
                vec!["sudo".into(), "zypper".into(), "install".into(), "-y".into()]
            }
            PkgManager::Apk => vec!["sudo".into(), "apk".into(), "add".into()],
            PkgManager::Xbps => vec!["sudo".into(), "xbps-install".into(), "-y".into()],
        }
    }

    fn update_cmd(&self) -> Option<Vec<String>> {
        match self {
            PkgManager::Apt | PkgManager::Unknown => {
                Some(vec!["sudo".into(), "apt".into(), "update".into(), "-y".into()])
            }
            PkgManager::Dnf => Some(vec!["sudo".into(), "dnf".into(), "check-update".into()]),
            PkgManager::Yum => Some(vec!["sudo".into(), "yum".into(), "check-update".into()]),
            PkgManager::Pacman => Some(vec!["sudo".into(), "pacman".into(), "-Sy".into()]),
            PkgManager::Zypper => {
                Some(vec!["sudo".into(), "zypper".into(), "refresh".into()])
            }
            PkgManager::Apk => Some(vec!["sudo".into(), "apk".into(), "update".into()]),
            PkgManager::Xbps => Some(vec!["sudo".into(), "xbps-install".into(), "-Su".into()]),
        }
    }

    fn cleanup_cmd(&self) -> Option<Vec<String>> {
        match self {
            PkgManager::Apt | PkgManager::Unknown => {
                Some(vec!["sudo".into(), "apt".into(), "autoremove".into(), "-y".into()])
            }
            PkgManager::Dnf => {
                Some(vec!["sudo".into(), "dnf".into(), "autoremove".into(), "-y".into()])
            }
            PkgManager::Yum => {
                Some(vec!["sudo".into(), "yum".into(), "autoremove".into(), "-y".into()])
            }
            PkgManager::Pacman => {
                Some(vec!["sudo".into(), "pacman".into(), "-Sc".into(), "--noconfirm".into()])
            }
            _ => None,
        }
    }

    fn is_installed(&self, pkg: &str) -> bool {
        match self {
            PkgManager::Apt | PkgManager::Unknown => Command::new("dpkg-query")
                .args(["-W", "-f=${Status}", pkg])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).contains("ok installed"))
                .unwrap_or(false),
            PkgManager::Dnf | PkgManager::Yum | PkgManager::Zypper => Command::new("rpm")
                .args(["-q", pkg])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false),
            PkgManager::Pacman => Command::new("pacman")
                .args(["-Q", pkg])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false),
            PkgManager::Apk => Command::new("apk")
                .args(["info", "-e", pkg])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false),
            PkgManager::Xbps => Command::new("xbps-query")
                .arg(pkg)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false),
        }
    }

    pub fn label(&self) -> &str {
        match self {
            PkgManager::Apt | PkgManager::Unknown => "apt",
            PkgManager::Dnf => "dnf",
            PkgManager::Yum => "yum",
            PkgManager::Pacman => "pacman",
            PkgManager::Zypper => "zypper",
            PkgManager::Apk => "apk",
            PkgManager::Xbps => "xbps",
        }
    }
}

pub struct Distro {
    pub id: String,
    pub pkg_manager: PkgManager,
}

impl Distro {
    pub fn detect() -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("/etc/os-release")
            .map_err(|_| "Cannot detect distribution: /etc/os-release not found")?;

        let id = content
            .lines()
            .find(|l| l.starts_with("ID="))
            .map(|l| l.trim_start_matches("ID=").trim_matches('"').to_lowercase())
            .unwrap_or_else(|| "unknown".into());

        let pkg_manager = match id.as_str() {
            "ubuntu" | "deepin" | "debian" | "linuxmint" | "pop" | "elementary" | "zorin"
            | "kali" | "parrot" | "neon" | "raspbian" => PkgManager::Apt,
            "fedora" | "rhel" | "centos" | "rocky" | "almalinux" | "nobara" | "amzn" => {
                if cmd_exists("dnf") {
                    PkgManager::Dnf
                } else {
                    PkgManager::Yum
                }
            }
            "arch" | "manjaro" | "endeavouros" | "garuda" | "cachyos" | "artix" => {
                PkgManager::Pacman
            }
            s if s.starts_with("opensuse") || s.starts_with("suse") => PkgManager::Zypper,
            "alpine" => PkgManager::Apk,
            "void" => PkgManager::Xbps,
            _ => {
                eprintln!("⚠️  Unknown distro '{}'. Falling back to apt.", id);
                PkgManager::Unknown
            }
        };

        Ok(Distro { id, pkg_manager })
    }
}

// ── Tool database ─────────────────────────────────────────────────────────────

/// Category colours (ANSI).
fn cat_ansi(cat: &str) -> &'static str {
    match cat {
        "PERF" => "\x1b[1;35m",
        "DISK" => "\x1b[1;31m",
        "SECU" => "\x1b[1;32m",
        "NET " => "\x1b[1;36m",
        "DEV " => "\x1b[1;34m",
        "MOD " => "\x1b[1;33m",
        _      => "\x1b[2m",
    }
}

/// `(category_4chars, generic_key, description, pkg_map_apt|dnf|pacman|zypper|apk|xbps)`
const TOOLS: &[(&str, &str, &str, &str)] = &[
    // ── PERF ─────────────────────────────────────────────────────────────────
    ("PERF", "zram-tools",    "RAM optimization using zRAM",
     "zram-tools|zram-generator-defaults|zram-generator|zram-generator|zram-tools|zramctl"),
    ("PERF", "earlyoom",      "Prevent system freeze when RAM is low",
     "earlyoom|earlyoom|earlyoom|earlyoom|earlyoom|earlyoom"),
    ("PERF", "htop",          "Classic interactive process monitor",
     "htop|htop|htop|htop|htop|htop"),
    ("PERF", "btop",          "Modern & beautiful resource dashboard",
     "btop|btop|btop|btop|btop|btop"),
    ("PERF", "glances",       "Full system statistics at a glance",
     "glances|glances|glances|glances|glances|glances"),
    ("PERF", "atop",          "Advanced system & process monitor",
     "atop|atop|atop|atop|atop|atop"),
    ("PERF", "sysstat",       "System performance tools (sar, iostat)",
     "sysstat|sysstat|sysstat|sysstat|sysstat|sysstat"),
    ("PERF", "stress-ng",     "Stress test CPU / RAM / IO",
     "stress-ng|stress-ng|stress-ng|stress-ng|stress-ng|stress-ng"),
    ("PERF", "smem",          "Report memory usage with PSS/USS",
     "smem|smem|smem|smem|smem|smem"),
    ("PERF", "preload",       "Adaptive readahead daemon (speed up apps)",
     "preload|preloader|preload|preloader|preload|preload"),
    ("PERF", "cpufrequtils",  "CPU frequency scaling utilities",
     "cpufrequtils|cpufrequtils|cpupower|cpufrequtils|cpufrequtils|cpufrequtils"),
    // ── DISK ─────────────────────────────────────────────────────────────────
    ("DISK", "ncdu",          "Disk usage analyzer (NCurses)",
     "ncdu|ncdu|ncdu|ncdu|ncdu|ncdu"),
    ("DISK", "gdu",           "Fast disk usage analyzer (Go based)",
     "gdu|gdu-disk-usage-analyzer|gdu|gdu|gdu|gdu"),
    ("DISK", "duf",           "Visual Disk Usage/Free utility",
     "duf|duf|duf|duf|duf|duf"),
    ("DISK", "dust",          "A more intuitive 'du' in Rust",
     "dust|dust|dust|du-dust|dust|dust"),
    ("DISK", "bleachbit",     "Clean system junk & maintain privacy",
     "bleachbit|bleachbit|bleachbit|bleachbit|bleachbit|bleachbit"),
    ("DISK", "gparted",       "GNOME Partition Editor",
     "gparted|gparted|gparted|gparted|gparted|gparted"),
    ("DISK", "smartmontools", "Control & monitor SMART storage",
     "smartmontools|smartmontools|smartmontools|smartmontools|smartmontools|smartmontools"),
    ("DISK", "tree",          "List directories in a tree-like format",
     "tree|tree|tree|tree|tree|tree"),
    ("DISK", "ranger",        "VIM-inspired file manager for terminal",
     "ranger|ranger|ranger|ranger|ranger|ranger"),
    ("DISK", "mc",            "Midnight Commander twin-panel file manager",
     "mc|mc|mc|mc|mc|mc"),
    // ── SECU ─────────────────────────────────────────────────────────────────
    ("SECU", "ufw",           "Uncomplicated Firewall",
     "ufw|ufw|ufw|ufw|ufw|ufw"),
    ("SECU", "fail2ban",      "Protect against brute-force attacks",
     "fail2ban|fail2ban|fail2ban|fail2ban|fail2ban|fail2ban"),
    ("SECU", "rkhunter",      "Rootkit and exploit scanner",
     "rkhunter|rkhunter|rkhunter|rkhunter|rkhunter|rkhunter"),
    ("SECU", "lynis",         "Security auditing tool for Linux",
     "lynis|lynis|lynis|lynis|lynis|lynis"),
    ("SECU", "clamav",        "Open source antivirus engine",
     "clamav|clamav|clamav|clamav|clamav|clamav"),
    ("SECU", "firejail",      "Sandbox security for applications",
     "firejail|firejail|firejail|firejail|firejail|firejail"),
    ("SECU", "gnupg",         "GNU Privacy Guard for encryption",
     "gnupg2|gnupg2|gnupg|gpg2|gnupg|gnupg"),
    // ── NET ──────────────────────────────────────────────────────────────────
    ("NET ", "speedtest-cli", "Test internet bandwidth via CLI",
     "speedtest-cli|speedtest-cli|speedtest-cli|speedtest|speedtest-cli|speedtest-cli"),
    ("NET ", "vnstat",        "Console-based network traffic monitor",
     "vnstat|vnstat|vnstat|vnstat|vnstat|vnstat"),
    ("NET ", "nmap",          "Network exploration & security auditing",
     "nmap|nmap|nmap|nmap|nmap|nmap"),
    ("NET ", "iftop",         "Display bandwidth usage on an interface",
     "iftop|iftop|iftop|iftop|iftop|iftop"),
    ("NET ", "nload",         "Real-time network traffic visualization",
     "nload|nload|nload|nload|nload|nload"),
    ("NET ", "nethogs",       "Net usage per process (top for network)",
     "nethogs|nethogs|nethogs|nethogs|nethogs|nethogs"),
    ("NET ", "curl",          "CLI tool for transferring data",
     "curl|curl|curl|curl|curl|curl"),
    ("NET ", "wget",          "Retrieve files using HTTP/HTTPS/FTP",
     "wget|wget|wget|wget|wget|wget"),
    ("NET ", "aria2",         "High-speed multi-source download utility",
     "aria2|aria2|aria2|aria2|aria2|aria2"),
    ("NET ", "wireguard",     "Fast, modern, secure VPN tunnel",
     "wireguard|wireguard-tools|wireguard-tools|wireguard-tools|wireguard-tools|wireguard"),
    ("NET ", "mtr-tiny",      "Combined ping and traceroute tool",
     "mtr-tiny|mtr|mtr|mtr|mtr|mtr"),
    ("NET ", "tcpdump",       "Powerful command-line packet analyzer",
     "tcpdump|tcpdump|tcpdump|tcpdump|tcpdump|tcpdump"),
    // ── DEV ──────────────────────────────────────────────────────────────────
    ("DEV ", "git",           "Distributed version control system",
     "git|git|git|git|git|git"),
    ("DEV ", "docker.io",     "OS-level virtualization (Docker)",
     "docker.io|docker|docker|docker|docker|docker"),
    ("DEV ", "docker-compose","Define & run multi-container apps",
     "docker-compose|docker-compose|docker-compose|docker-compose|docker-compose|docker-compose"),
    ("DEV ", "build-essential","Essential packages for compiling code",
     "build-essential|gcc-c++|base-devel|patterns-devel-base-devel|build-base|base-devel"),
    ("DEV ", "micro",         "Modern intuitive terminal-based editor",
     "micro|micro|micro|micro|micro|micro"),
    ("DEV ", "neovim",        "Extensible text editor (Vim 2.0)",
     "neovim|neovim|neovim|neovim|neovim|neovim"),
    ("DEV ", "tmux",          "Terminal multiplexer for managing sessions",
     "tmux|tmux|tmux|tmux|tmux|tmux"),
    ("DEV ", "screen",        "Full-screen window manager / multiplexer",
     "screen|screen|screen|screen|screen|screen"),
    ("DEV ", "python3-pip",   "The Python package installer",
     "python3-pip|python3-pip|python-pip|python3-pip|py3-pip|python3-pip"),
    ("DEV ", "nodejs",        "JavaScript runtime environment",
     "nodejs|nodejs|nodejs|nodejs|nodejs|nodejs"),
    ("DEV ", "golang-go",     "The Go programming language",
     "golang-go|golang|go|go|go|go"),
    ("DEV ", "rsync",         "Fast, versatile remote/local file-copy",
     "rsync|rsync|rsync|rsync|rsync|rsync"),
    ("DEV ", "jq",            "Command-line JSON processor",
     "jq|jq|jq|jq|jq|jq"),
    ("DEV ", "yq",            "Command-line YAML/XML processor",
     "yq|yq|yq|yq|yq|yq"),
    ("DEV ", "gh",            "GitHub CLI",
     "gh|gh|github-cli|gh|github-cli|github-cli"),
    ("DEV ", "lazygit",       "Simple terminal UI for git commands",
     "lazygit|lazygit|lazygit|lazygit|lazygit|lazygit"),
    ("DEV ", "httpie",        "User-friendly HTTP client",
     "httpie|httpie|httpie|httpie|httpie|httpie"),
    // ── MOD (Modern CLI) ─────────────────────────────────────────────────────
    ("MOD ", "bat",           "Cat clone with syntax highlighting",
     "bat|bat|bat|bat|bat|bat"),
    ("MOD ", "eza",           "Modern replacement for 'ls' with icons",
     "eza|eza|eza|eza|eza|eza"),
    ("MOD ", "ripgrep",       "Extremely fast grep alternative",
     "ripgrep|ripgrep|ripgrep|ripgrep|ripgrep|ripgrep"),
    ("MOD ", "fd-find",       "Simple, fast alternative to 'find'",
     "fd-find|fd-find|fd|fd|fd|fd"),
    ("MOD ", "zoxide",        "Smarter cd command (learns your habits)",
     "zoxide|zoxide|zoxide|zoxide|zoxide|zoxide"),
    ("MOD ", "procs",         "Modern replacement for 'ps' in Rust",
     "procs|procs|procs|procs|procs|procs"),
    ("MOD ", "tldr",          "Simplified community-driven man pages",
     "tldr|tldr|tldr|tldr|tldr|tldr"),
    ("MOD ", "chafa",         "Terminal graphics for the 21st century",
     "chafa|chafa|chafa|chafa|chafa|chafa"),
    ("MOD ", "fzf",           "General-purpose fuzzy finder",
     "fzf|fzf|fzf|fzf|fzf|fzf"),
    ("MOD ", "git-delta",     "Syntax-highlighting pager for git diffs",
     "git-delta|git-delta|git-delta|git-delta|git-delta|git-delta"),
    // ── SYS ──────────────────────────────────────────────────────────────────
    ("SYS ", "fastfetch",     "High-performance neofetch alternative",
     "fastfetch|fastfetch|fastfetch|fastfetch|fastfetch|fastfetch"),
    ("SYS ", "inxi",          "Full-featured system information script",
     "inxi|inxi|inxi|inxi|inxi|inxi"),
    ("SYS ", "lm-sensors",    "Read temperature / voltage / fan sensors",
     "lm-sensors|lm_sensors|lm_sensors|sensors|lm-sensors|lm-sensors"),
    ("SYS ", "unzip",         "Decompress ZIP files",
     "unzip|unzip|unzip|unzip|unzip|unzip"),
    ("SYS ", "p7zip-full",    "7z file archiver with high compression",
     "p7zip-full|p7zip|p7zip|p7zip|p7zip|p7zip"),
    ("SYS ", "zsh",           "The Z Shell (advanced bash alternative)",
     "zsh|zsh|zsh|zsh|zsh|zsh"),
    ("SYS ", "xclip",         "Command line interface to X selections",
     "xclip|xclip|xclip|xclip|xclip|xclip"),
    ("SYS ", "wl-clipboard",  "Command line copy/paste for Wayland",
     "wl-clipboard|wl-clipboard|wl-clipboard|wl-clipboard|wl-clipboard|wl-clipboard"),
    ("SYS ", "acpi",          "Display battery and thermal information",
     "acpi|acpi|acpi|acpi|acpi|acpi"),
    ("SYS ", "socat",         "Multipurpose relay for bidirectional data",
     "socat|socat|socat|socat|socat|socat"),
    ("SYS ", "lsof",          "List open files and network connections",
     "lsof|lsof|lsof|lsof|lsof|lsof"),
    ("SYS ", "strace",        "Trace system calls and signals",
     "strace|strace|strace|strace|strace|strace"),
];

// ── Resolve package name for current distro ───────────────────────────────────

fn resolve_pkg<'a>(map: &'a str, pm: &PkgManager) -> &'a str {
    let idx = pm.col_index();
    map.split('|').nth(idx).unwrap_or(map.split('|').next().unwrap_or(""))
}

// ── fzf multi-select UI ───────────────────────────────────────────────────────

fn launch_fzf(items: &[String], pm_label: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let prompt_msg = format!("⚡ Select tools to install/configure ({pm_label}):");
    let chosen = match inquire::MultiSelect::new(&prompt_msg, items.to_vec()).prompt() {
        Ok(v) => v,
        Err(_) => return Ok(vec![]),
    };

    let mut indices = vec![];
    for sel in &chosen {
        if let Some(idx) = items.iter().position(|i| i == sel) {
            indices.push(idx);
        }
    }
    Ok(indices)
}

// ── Per-tool auto-configuration ───────────────────────────────────────────────

fn add_config_to_rc(rc: &PathBuf, marker: &str, content: &str) {
    if let Ok(existing) = fs::read_to_string(rc) {
        if existing.contains(marker) {
            return;
        }
    }
    if let Ok(mut f) = fs::OpenOptions::new().append(true).open(rc) {
        let _ = writeln!(f, "\n# {}\n{}", marker, content);
    }
}

fn auto_config(generic: &str, pm: &PkgManager, rc_file: &PathBuf, shell_name: &str) {
    const CYAN: &str = "\x1b[1;36m";
    const NC: &str = "\x1b[0m";

    match generic {
        "docker.io" => {
            let user = std::env::var("USER").unwrap_or_default();
            if !user.is_empty() {
                let _ = Command::new("sudo")
                    .args(["usermod", "-aG", "docker", &user])
                    .status();
            }
            let _ = Command::new("sudo")
                .args(["systemctl", "enable", "--now", "docker"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        "tmux" => {
            let path = dirs_home().join(".tmux.conf");
            if !path.exists() {
                let _ = fs::write(
                    &path,
                    "set -g mouse on\nset -g default-terminal \"screen-256color\"\n",
                );
            }
        }
        "git" => {
            for (k, v) in [
                ("color.ui", "auto"),
                ("core.editor", "nano"),
                ("init.defaultBranch", "main"),
            ] {
                let _ = Command::new("git")
                    .args(["config", "--global", k, v])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            }
        }
        "neovim" => {
            let nvim_dir = dirs_home().join(".config/nvim");
            let _ = fs::create_dir_all(&nvim_dir);
            let init_vim = nvim_dir.join("init.vim");
            if !init_vim.exists() {
                let _ = fs::write(
                    &init_vim,
                    "set number\nset relativenumber\nset mouse=a\nset termguicolors\n",
                );
            }
            add_config_to_rc(rc_file, "Neovim Alias", "alias v='nvim'\nalias vim='nvim'");
        }
        "zram-tools" => {
            if pm == &PkgManager::Apt {
                let _ = Command::new("sudo")
                    .args(["bash", "-c",
                        "echo -e 'PERCENT=60\\nALGO=zstd\\nPRIORITY=100' > /etc/default/zramswap"])
                    .status();
                let _ = Command::new("sudo")
                    .args(["systemctl", "restart", "zramswap"])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
            } else if pm == &PkgManager::Pacman {
                let _ = Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm", "zram-generator"])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status();
                let _ = Command::new("sudo")
                    .args(["bash", "-c",
                        "echo -e '[zram0]\\nzram-size = ram / 2\\ncompression-algorithm = zstd' \
                         > /etc/systemd/zram-generator.conf"])
                    .status();
                let _ = Command::new("sudo").args(["systemctl", "daemon-reload"]).status();
            }
        }
        "micro" => {
            let dir = dirs_home().join(".config/micro");
            let _ = fs::create_dir_all(&dir);
            let settings = dir.join("settings.json");
            if !settings.exists() {
                let _ = fs::write(&settings, r#"{"mouse": true, "clipboard": "terminal"}"#);
            }
        }
        "ufw" => {
            if cmd_exists("ufw") {
                let _ = Command::new("sudo").args(["ufw", "allow", "ssh"]).status();
                let _ = Command::new("sudo").args(["ufw", "--force", "enable"]).status();
            }
        }
        "htop" => {
            let dir = dirs_home().join(".config/htop");
            let _ = fs::create_dir_all(&dir);
            let rc = dir.join("htoprc");
            if !rc.exists() {
                let _ = fs::write(&rc, "highlight_megabytes=1\nshow_program_path=1\n");
            }
        }
        "acpi" => {
            add_config_to_rc(rc_file, "Battery Status", "alias battery='acpi -V'");
        }
        "bat" => {
            let bat_cmd = if pm == &PkgManager::Apt { "batcat" } else { "bat" };
            add_config_to_rc(
                rc_file,
                "Batcat Alias",
                &format!("alias cat='{bat_cmd} -p'\nalias bat='{bat_cmd}'"),
            );
        }
        "eza" => {
            add_config_to_rc(
                rc_file,
                "Eza Alias",
                "alias ls='eza --icons --group-directories-first'",
            );
        }
        "zoxide" => {
            add_config_to_rc(
                rc_file,
                "Zoxide Init",
                &format!(
                    "[ -x \"$(command -v zoxide)\" ] && eval \"$(zoxide init {shell_name})\""
                ),
            );
            add_config_to_rc(rc_file, "Zoxide Alias", "alias cd='z'");
        }
        "preload" => {
            println!("{CYAN}🔧 Enabling Preload service...{NC}");
            let _ = Command::new("sudo")
                .args(["systemctl", "enable", "--now", "preload"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        "earlyoom" => {
            println!("{CYAN}🔧 Configuring EarlyOOM (Memory Protection)...{NC}");
            if std::path::Path::new("/etc/default/earlyoom").exists() {
                let _ = Command::new("sudo")
                    .args([
                        "sed", "-i",
                        "s/EARLYOOM_ARGS=.*/EARLYOOM_ARGS=\"-m 10 -s 5\"/",
                        "/etc/default/earlyoom",
                    ])
                    .status();
            }
            let _ = Command::new("sudo")
                .args(["systemctl", "enable", "--now", "earlyoom"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        "lm-sensors" => {
            println!("{CYAN}🔍 Detecting Hardware Sensors...{NC}");
            let _ = Command::new("sudo")
                .args(["sensors-detect", "--auto"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            let _ = Command::new("sudo")
                .args(["systemctl", "enable", "--now", "lm_sensors"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            add_config_to_rc(rc_file, "Sensor Alias", "alias temp='sensors'");
        }
        "fzf" => {
            if shell_name == "bash" || shell_name == "zsh" {
                add_config_to_rc(
                    rc_file,
                    "FZF Integration",
                    &format!("eval \"$(fzf --{shell_name})\""),
                );
            }
        }
        _ => {}
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn cmd_exists(name: &str) -> bool {
    crate::core::utils::cmd_exists(name)
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/root"))
}

fn detect_shell_name() -> String {
    std::env::var("SHELL")
        .map(|s| s.rsplit('/').next().unwrap_or("bash").to_string())
        .unwrap_or_else(|_| "bash".into())
}

fn rc_file_for(shell_name: &str) -> PathBuf {
    let home = dirs_home();
    match shell_name {
        "zsh"  => home.join(".zshrc"),
        "fish" => home.join(".config/fish/config.fish"),
        _      => home.join(".bashrc"),
    }
}

// ── Main entry point ──────────────────────────────────────────────────────────

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    const GREEN:  &str = "\x1b[1;32m";
    const YELLOW: &str = "\x1b[1;33m";
    const CYAN:   &str = "\x1b[1;36m";
    const RED:    &str = "\x1b[1;31m";
    const BOLD:   &str = "\x1b[1m";
    const DIM:    &str = "\x1b[2m";
    const NC:     &str = "\x1b[0m";

    // 1. Distro detection
    let distro = Distro::detect()?;
    let pm = &distro.pkg_manager;
    println!(
        "{CYAN}🖥️  Detected: {BOLD}{}{NC} | Package Manager: {BOLD}{}{NC}",
        distro.id,
        pm.label()
    );



    // 3. Build display items
    let installed_count = TOOLS
        .iter()
        .filter(|(_, _, _, map)| pm.is_installed(resolve_pkg(map, pm)))
        .count();

    println!(
        "\n{BOLD}⚡ PC Arsenal — {installed_count}/{} tools installed{NC}\n",
        TOOLS.len()
    );

    let display_items: Vec<String> = TOOLS
        .iter()
        .enumerate()
        .map(|(idx, (cat, generic, desc, map))| {
            let pkg = resolve_pkg(map, pm);
            let status_icon = if pm.is_installed(pkg) {
                format!("{GREEN}●{NC}")
            } else {
                format!("{DIM}○{NC}")
            };
            let cat_color = cat_ansi(cat);
            format!(
                "{}  {DIM}[{:>3}]{NC}  {cat_color}{cat}{NC}  {BOLD}{:<18}{NC}  {DIM}{}{NC}",
                status_icon,
                idx + 1,
                generic,
                desc
            )
        })
        .collect();

    // 4. fzf multi-select
    let selected_indices = launch_fzf(&display_items, pm.label())?;

    if selected_indices.is_empty() {
        println!("\n{YELLOW}👋 Operation cancelled or nothing selected.{NC}");
        return Ok(());
    }

    // 5. Sync package lists
    println!("{CYAN}🔄 Syncing package lists...{NC}");
    if let Some(cmd) = pm.update_cmd() {
        let _ = Command::new(&cmd[0])
            .args(&cmd[1..])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }

    let shell_name = detect_shell_name();
    let rc = rc_file_for(&shell_name);

    let mut installed_list: Vec<String> = vec![];
    let mut failed_list:    Vec<String> = vec![];

    println!(
        "{CYAN}🔧 Processing {} tools on {}...{NC}\n",
        selected_indices.len(),
        distro.id
    );

    // 6. Install loop
    for idx in &selected_indices {
        let (_, generic, _, map) = TOOLS[*idx];
        let pkg = resolve_pkg(map, pm);

        print!("{BOLD}📦 {generic} ({pkg})... {NC}");
        let _ = std::io::stdout().flush();

        if pm.is_installed(pkg) {
            println!("{GREEN}✔ Already installed{NC}");
            installed_list.push(generic.to_string());
        } else {
            let cmd = pm.install_cmd();
            let ok = Command::new(&cmd[0])
                .args(&cmd[1..])
                .arg(pkg)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            if ok {
                println!("{GREEN}[INSTALLED]{NC}");
                installed_list.push(generic.to_string());
            } else {
                println!("{RED}[FAILED]{NC}");
                failed_list.push(format!("{generic} ({pkg})"));
                continue; // skip auto-config on failure
            }
        }

        auto_config(generic, pm, &rc, &shell_name);
    }

    // 7. History env
    add_config_to_rc(
        &rc,
        "HISTORY",
        "export HISTFILE=\"$HOME/.bash_history\"\nexport HISTSIZE=50000\nexport HISTFILESIZE=50000\nshopt -s histappend",
    );

    // 8. Cleanup
    if let Some(cmd) = pm.cleanup_cmd() {
        let _ = Command::new(&cmd[0])
            .args(&cmd[1..])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }

    // 9. Summary
    println!("\n{GREEN}✅ Deployment Complete on {}!{NC}", distro.id);
    println!("{GREEN}📦 Installed/confirmed: {} tools{NC}", installed_list.len());

    if !failed_list.is_empty() {
        println!("{RED}❌ Failed ({}):{NC}", failed_list.len());
        for f in &failed_list {
            println!("  - {f}");
        }
    }

    if installed_list.iter().any(|t| t == "docker.io") {
        println!("{YELLOW}⚠️  Log out and back in for Docker group changes.{NC}");
    }
    if installed_list.iter().any(|t| t == "zoxide") {
        println!(
            "{CYAN}💡 Run 'source {}' to enable zoxide.{NC}",
            rc.display()
        );
    }

    Ok(())
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tools_list_non_empty() {
        assert!(!TOOLS.is_empty(), "TOOLS table must not be empty");
    }

    #[test]
    fn test_all_tools_have_6_pkg_columns() {
        for (cat, generic, _desc, map) in TOOLS {
            let cols: Vec<&str> = map.split('|').collect();
            assert_eq!(
                cols.len(),
                6,
                "Tool {cat}/{generic} must have exactly 6 pkg columns, got {}",
                cols.len()
            );
        }
    }

    #[test]
    fn test_resolve_pkg_apt() {
        let map = "bat|bat|bat|bat|bat|bat";
        assert_eq!(resolve_pkg(map, &PkgManager::Apt), "bat");
    }

    #[test]
    fn test_resolve_pkg_pacman() {
        // fd-find -> "fd" on pacman (col index 2)
        let map = "fd-find|fd-find|fd|fd|fd|fd";
        assert_eq!(resolve_pkg(map, &PkgManager::Pacman), "fd");
    }

    #[test]
    fn test_cmd_exists_sh() {
        assert!(cmd_exists("sh"));
    }
}
