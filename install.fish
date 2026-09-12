#!/usr/bin/env fish

# ─── fancybash Bulletproof Fish Installer (install.fish) ──────────────────────
# Native Fish installer — auto-detects OS, installs dependencies & fonts,
# creates timestamped backups, and configures ~/.config/fish/config.fish cleanly.
# ──────────────────────────────────────────────────────────────────────────────

set -g FISHCONFIG "$HOME/.config/fish/config.fish"
set -g URL "https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main/config.fish"
set -g FALLBACK_URL "https://fancybash.netlify.app/public/config.fish"
set -g START "# >>> fancy-fishrc >>>"
set -g END "# <<< fancy-fishrc <<<"
set -g SCRIPT_DIR (cd (dirname (status filename)) 2>/dev/null; and pwd; or echo "$PWD")
set -g LOCAL_CONFIG "$SCRIPT_DIR/config.fish"

# ─── Colors & Formatting ───────────────────
set -g RED "\033[38;2;243;139;168m"
set -g GREEN "\033[38;2;166;227;161m"
set -g YELLOW "\033[38;2;249;226;175m"
set -g BLUE "\033[38;2;137;180;250m"
set -g PURPLE "\033[38;2;203;166;247m"
set -g CYAN "\033[38;2;148;226;213m"
set -g GRAY "\033[38;2;147;153;178m"
set -g BOLD "\033[1m"
set -g NC "\033[0m"

set -g tmpfile ""
set -g backup_file ""

# ─── Unattended / Auto-Yes Detection ──────────────
set -g AUTO_YES false
for arg in $argv
    if test "$arg" = "-y"; or test "$arg" = "--yes"; or test "$arg" = "--unattended"
        set -g AUTO_YES true
        break
    end
end
if test "$FANCYBASH_AUTO_YES" = "1"; or test "$NONINTERACTIVE" = "1"; or test "$CI" = "true"
    set -g AUTO_YES true
end

# ─── Safe Signal Trap & Cleanup ────────────
function _cleanup --on-event fish_exit --on-signal SIGINT --on-signal SIGTERM
    if command -v tput >/dev/null 2>&1
        tput cnorm 2>/dev/null; or true
    end
    if test -n "$tmpfile"; and test -f "$tmpfile"
        rm -f "$tmpfile" 2>/dev/null; or true
    end
end

# ─── Progress Bar ──────────────────────────
function draw_progress_bar
    set -l current $argv[1]
    set -l total 5
    set -l width 30
    set -l percentage (math "($current * 100) / $total")
    set -l completed (math "($width * $current) / $total")
    set -l remaining (math "$width - $completed")

    set -l bar ""
    set -l empty ""
    if test $completed -gt 0
        for i in (seq 1 $completed)
            set bar "{$bar}█"
        end
    end
    if test $remaining -gt 0
        for i in (seq 1 $remaining)
            set empty "{$empty}░"
        end
    end

    echo ""
    printf "{$BLUE}Progress:{$NC} [{$GREEN}%s{$GRAY}%s{$NC}] {$CYAN}%d%%{$NC} (Step %d/%d)\n" "$bar" "$empty" "$percentage" "$current" "$total"
end

# ─── Header ────────────────────────────────
function show_header
    echo ""
    echo -e "{$PURPLE}          ███████╗ █████╗ ███╗   ██╗ ██████╗██╗   ██╗██████╗  █████╗ ███████╗██╗  ██╗{$NC}"
    echo -e "{$PURPLE}          ██╔════╝██╔══██╗████╗  ██║██╔════╝╚██╗ ██╔╝██╔══██╗██╔══██╗██╔════╝██║  ██║{$NC}"
    echo -e "{$CYAN}          █████╗  ███████║██╔██╗ ██║██║      ╚████╔╝ ██████╔╝███████║███████╗███████║{$NC}"
    echo -e "{$CYAN}          ██╔══╝  ██╔══██║██║╚██╗██║██║       ╚██╔╝  ██╔══██║╚════██║██╔══██║╚════██║{$NC}"
    echo -e "{$BLUE}          ██║     ██║  ██║██║ ╚████║╚██████╗   ██║   ██████╔╝██║  ██║███████║██║  ██║{$NC}"
    echo -e "{$BLUE}          ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝   ╚═╝   ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝{$NC}"
    echo ""
    echo -e "   ✨ {$BOLD}{$CYAN}F A N C Y B A S H{$NC}  •  {$BOLD}Fish Config Installer{$NC}"
    echo ""
end

# ─── System Information ────────────────────
function show_sysinfo
    set -l os_name (uname -s 2>/dev/null; or echo "Linux")
    if test -f /etc/os-release
        set os_name (grep '^PRETTY_NAME=' /etc/os-release 2>/dev/null | cut -d '=' -f 2 | tr -d '"')
    else if test (uname -s 2>/dev/null) = "Darwin"
        set os_name "macOS "(sw_vers -productVersion 2>/dev/null; or echo "")
    end

    set -l arch (uname -m 2>/dev/null; or echo "x86_64")
    set -l user $USER
    test -z "$user"; and set user (whoami 2>/dev/null; or echo "user")
    set -l current_shell "fish "(fish --version 2>/dev/null | cut -d' ' -f3; or echo "3.x")

    echo -e "\n{$BLUE}──────────────────────────────────────────────────{$NC}"
    echo -e " 🖥️   {$BOLD}SYSTEM INFORMATION{$NC}"
    echo -e "{$BLUE}──────────────────────────────────────────────────{$NC}"
    echo -e "  💻  {$BOLD}OS:{$NC}      {$CYAN}$os_name{$NC}"
    echo -e "  👤  {$BOLD}User:{$NC}    {$CYAN}$user{$NC}"
    echo -e "  🐚  {$BOLD}Shell:{$NC}   {$CYAN}$current_shell{$NC}"
    echo -e "  ⚙️   {$BOLD}Arch:{$NC}    {$CYAN}$arch{$NC}"
    echo -e "{$BLUE}──────────────────────────────────────────────────{$NC}\n"
    echo ""
end

# ─── OS & Package Manager Detection ────────
function detect_pm
    if test (uname -s 2>/dev/null) = "Darwin"
        echo "brew"
    else if test -f /etc/os-release
        set -l id_like (grep -E '^ID_LIKE=|^ID=' /etc/os-release 2>/dev/null | cut -d= -f2 | tr -d '"')
        switch "$id_like"
            case "*debian*" "*ubuntu*" "*mint*" "*pop*"
                echo "apt"
            case "*arch*" "*manjaro*"
                echo "pacman"
            case "*fedora*" "*rhel*" "*centos*"
                echo "dnf"
            case "*alpine*"
                echo "apk"
            case "*"
                if command -v apt-get >/dev/null 2>&1; echo "apt"
                else if command -v pacman >/dev/null 2>&1; echo "pacman"
                else if command -v dnf >/dev/null 2>&1; echo "dnf"
                else if command -v apk >/dev/null 2>&1; echo "apk"
                else; echo "unknown"
                end
        end
    else
        echo "unknown"
    end
end

function is_intel_or_amd
    if test -f /proc/cpuinfo; and grep -qiE 'intel|amd' /proc/cpuinfo >/dev/null 2>&1
        return 0
    end
    if command -v lspci >/dev/null 2>&1; and lspci 2>/dev/null | grep -qiE 'intel|amd|radeon' >/dev/null 2>&1
        return 0
    end
    return 1
end

# ─── Check & Install Fonts ──────────────────
function check_and_install_fonts
    printf "  {$CYAN}➜{$NC} Checking system dependencies...\n"

    set -l missing_deps
    for cmd in curl grep git fzf gum glow bat
        if test "$cmd" = "bat"; and command -v batcat >/dev/null 2>&1
            continue
        end
        if not command -v $cmd >/dev/null 2>&1
            set -a missing_deps $cmd
        end
    end

    if test (uname -s 2>/dev/null) != "Darwin"
        if not command -v xclip >/dev/null 2>&1; and not command -v wl-copy >/dev/null 2>&1; and not command -v xsel >/dev/null 2>&1
            set -a missing_deps "clipboard-tool"
        end
        if is_intel_or_amd; and not command -v vulkaninfo >/dev/null 2>&1
            set -a missing_deps "vulkan-tools"
        end
    end

    set -l fonts_needed 0
    if command -v fc-list >/dev/null 2>&1
        if not fc-list : family 2>/dev/null | grep -qi "Fira Code\|FiraCode"; or not fc-list : family 2>/dev/null | grep -qi "Noto Color Emoji\|NotoColorEmoji"
            set fonts_needed 1
        end
    else
        set fonts_needed 1
    end

    if test (count $missing_deps) -eq 0; and test $fonts_needed -eq 0
        printf "  {$GREEN}✔{$NC} All dependencies & fonts are already installed!\n"
        return 0
    end

    set -l response ""
    if test "$AUTO_YES" = "true"
        set response "y"
    else
        # Interactive Prompt (Safely read from /dev/tty if piped)
        echo ""
        printf "{$YELLOW}  ❯ Missing components detected ($missing_deps).{$NC}\n"
        printf "    Would you like to auto-install missing dependencies and proceed? [{$GREEN}Y{$NC}/n]: "

        if isatty stdin
            read -l input_resp
            set response "$input_resp"
        else if test -c /dev/tty
            read -l input_resp < /dev/tty 2>/dev/null
            set response "$input_resp"
        end

        set response (string lower (string trim -- "$response"))
        test -z "$response"; and set response "y"
    end

    switch "$response"
        case "y" "yes"
            # proceed with installation
        case "*"
            printf "  {$YELLOW}⚠ Installation cancelled by user. No changes were made.{$NC}\n\n"
            return 1
    end

    set -l pm (detect_pm)
    set -l sudo_cmd ""
    if test (id -u 2>/dev/null; or echo 1000) -ne 0; and command -v sudo >/dev/null 2>&1
        set sudo_cmd "sudo"
    end

    printf "  {$CYAN}➜{$NC} Installing via $pm...\n"

    set -l vulkan_pkgs
    if test (uname -s 2>/dev/null) != "Darwin"; and is_intel_or_amd
        switch "$pm"
            case "apt"
                set vulkan_pkgs mesa-vulkan-drivers vulkan-tools
            case "pacman"
                set vulkan_pkgs vulkan-intel vulkan-radeon vulkan-tools
            case "dnf"
                set vulkan_pkgs mesa-vulkan-drivers vulkan-tools
            case "apk"
                set vulkan_pkgs vulkan-loader vulkan-tools
        end
    end

    switch "$pm"
        case "apt"
            if not command -v gum >/dev/null 2>&1; or not command -v glow >/dev/null 2>&1
                $sudo_cmd mkdir -p /etc/apt/keyrings >/dev/null 2>&1; or true
                curl -fsSL https://repo.charm.sh/apt/gpg.key 2>/dev/null | $sudo_cmd gpg --dearmor --yes -o /etc/apt/keyrings/charm.gpg >/dev/null 2>&1; or true
                echo "deb [signed-by=/etc/apt/keyrings/charm.gpg] https://repo.charm.sh/apt/ * *" | $sudo_cmd tee /etc/apt/sources.list.d/charm.list >/dev/null 2>&1; or true
            end
            $sudo_cmd apt update -qq >/dev/null 2>&1; or true
            $sudo_cmd apt install -y curl git fzf gum glow bat zoxide chafa xclip wl-clipboard nano fonts-noto-color-emoji fonts-firacode fonts-cascadia-code fontconfig $vulkan_pkgs >/dev/null 2>&1; or true
        case "pacman"
            $sudo_cmd pacman -Sy --noconfirm curl git fzf gum glow bat zoxide chafa xclip wl-clipboard nano ttf-noto-emoji ttf-fira-code ttf-cascadia-code fontconfig $vulkan_pkgs >/dev/null 2>&1; or true
        case "dnf"
            $sudo_cmd dnf install -y curl git fzf gum glow bat zoxide chafa xclip wl-clipboard nano google-noto-emoji-fonts fira-code-fonts cascadia-code-fonts fontconfig $vulkan_pkgs >/dev/null 2>&1; or true
        case "apk"
            $sudo_cmd apk add --no-cache curl git fzf gum glow bat zoxide chafa xclip wl-clipboard nano font-noto-emoji font-fira-code fontconfig $vulkan_pkgs >/dev/null 2>&1; or true
        case "brew"
            brew install curl git fzf gum glow bat zoxide chafa nano font-fira-code font-cascadia-code font-noto-emoji >/dev/null 2>&1; or true
        case "*"
            printf "  {$GRAY}ℹ Package manager not recognized. Skipping.{$NC}\n"
    end
    printf "  {$GREEN}✔{$NC} Dependencies process completed.\n"
end

# ─── Configure Fontconfig ──────────────────
function setup_fontconfig
    printf "  {$CYAN}➜{$NC} Checking font configuration...\n"
    set -l font_dir "$HOME/.config/fontconfig"
    set -l font_conf "$font_dir/fonts.conf"

    if test -f "$font_conf"
        printf "  {$GREEN}✔{$NC} Fontconfig already exists ({$GRAY}~/.config/fontconfig/fonts.conf{$NC})\n"
        return 0
    end

    mkdir -p "$font_dir" 2>/dev/null; or true
    echo '<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <alias>
    <family>monospace</family>
    <prefer>
      <family>Fira Code</family>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
  <alias>
    <family>sans-serif</family>
    <prefer>
      <family>Noto Color Emoji</family>
    </prefer>
  </alias>
</fontconfig>' > "$font_conf" 2>/dev/null; or true

    printf "  {$GREEN}✔{$NC} Created {$PURPLE}~/.config/fontconfig/fonts.conf{$NC}\n"

    if command -v fc-cache >/dev/null 2>&1
        fc-cache -f >/dev/null 2>&1; or true
        printf "  {$GREEN}✔{$NC} Font cache refreshed!\n"
    end
end

# ─── Install / Verify Rust Binary ──────────
function setup_rust_binary
    printf "  {$CYAN}➜{$NC} Installing fancybash Rust engine binary...\n"
    set -l bin_dir "$HOME/.local/bin"
    mkdir -p "$bin_dir"
    fish_add_path "$bin_dir" 2>/dev/null; or set -gx PATH "$bin_dir" $PATH

    # 1. Local pre-built binary in target/release
    if test -f "$SCRIPT_DIR/target/release/fancybash"
        cp "$SCRIPT_DIR/target/release/fancybash" "$bin_dir/fancybash"
        chmod +x "$bin_dir/fancybash"
        printf "  {$GREEN}✔{$NC} Installed local release binary to {$PURPLE}%s/fancybash{$NC}\n" "$bin_dir"
        return 0
    end

    # 2. Local Rust source build via cargo
    if test -f "$SCRIPT_DIR/Cargo.toml"; and command -v cargo >/dev/null 2>&1
        printf "  {$YELLOW}⚡ Building fancybash Rust engine (release mode)...{$NC}\n"
        pushd "$SCRIPT_DIR"
        cargo build --release
        popd
        if test -f "$SCRIPT_DIR/target/release/fancybash"
            cp "$SCRIPT_DIR/target/release/fancybash" "$bin_dir/fancybash"
            chmod +x "$bin_dir/fancybash"
            printf "  {$GREEN}✔{$NC} Built & installed to {$PURPLE}%s/fancybash{$NC}\n" "$bin_dir"
            return 0
        end
    end

    # 3. Existing system binary
    if command -v fancybash >/dev/null 2>&1
        printf "  {$GREEN}✔{$NC} fancybash binary active: "(command -v fancybash)"\n"
        return 0
    end

    # 4. Fallback cargo install from Git
    if command -v cargo >/dev/null 2>&1
        printf "  {$YELLOW}⚡ Installing via cargo from GitHub...{$NC}\n"
        cargo install --git https://github.com/rihadjahanopu/fancybash-rs --quiet 2>/dev/null; or true
        if command -v fancybash >/dev/null 2>&1
            printf "  {$GREEN}✔{$NC} Installed fancybash via cargo install!\n"
            return 0
        end
    end

    printf "  {$YELLOW}⚠ Could not auto-build Rust binary. Please install Rust (cargo) and build manually.{$NC}\n"
end

# ─── Check Existing Installation ───────────
function check_existing_install
    printf "  {$CYAN}➜{$NC} Checking existing configuration...\n"
    set -l target_dir (dirname "$FISHCONFIG")
    mkdir -p "$target_dir" 2>/dev/null; or true
    if not test -f "$FISHCONFIG"
        printf "  {$YELLOW}⚠ Creating $FISHCONFIG...{$NC}\n"
        touch "$FISHCONFIG" 2>/dev/null; or true
    end
    printf "  {$GREEN}✔{$NC} Ready for installation.\n"
end

# ─── Remove Old Config Block ───────────────
function remove_old_config
    if test -f "$FISHCONFIG"; and grep -qF "$START" "$FISHCONFIG" 2>/dev/null
        printf "  {$YELLOW}⚠{$NC} Found existing fancy-fishrc block — removing old config first...\n"
        while grep -qF "$START" "$FISHCONFIG" 2>/dev/null
            if test (uname -s 2>/dev/null) = "Darwin"
                sed -i '' '/# >>> fancy-fishrc >>>/,/# <<< fancy-fishrc <<</d' "$FISHCONFIG" 2>/dev/null; or true
            else
                sed -i '/# >>> fancy-fishrc >>>/,/# <<< fancy-fishrc <<</d' "$FISHCONFIG" 2>/dev/null; or true
            end
        end
        printf "  {$GREEN}✔{$NC} Old config removed.\n"
    end
end

# ─── Backup ────────────────────────────────
function backup_fishconfig
    printf "  {$CYAN}➜{$NC} Creating backup...\n"
    if test -f "$FISHCONFIG" -a -s "$FISHCONFIG"
        set -g backup_file "$FISHCONFIG.backup."(date +%Y%m%d_%H%M%S)
        cp "$FISHCONFIG" "$backup_file" 2>/dev/null; or true
        printf "  {$GREEN}✔{$NC} Backup created: {$PURPLE}"(basename "$backup_file")"{$NC}\n"
    else
        printf "  {$GRAY}ℹ No existing non-empty config to backup. Skipping.{$NC}\n"
    end
end

# ─── Fetch & Append Config ─────────────────
function install_config
    printf "  {$CYAN}➜{$NC} Applying fancybash Fish configuration...\n"

    echo "" >> "$FISHCONFIG"
    echo "$START" >> "$FISHCONFIG"
    echo "# Installed: "(date '+%Y-%m-%d %H:%M:%S' 2>/dev/null; or echo "today") >> "$FISHCONFIG"
    echo "# fancybash Rust Native Engine Initialization" >> "$FISHCONFIG"
    echo 'fish_add_path "$HOME/.local/bin"' >> "$FISHCONFIG"
    echo "" >> "$FISHCONFIG"

    if command -v fancybash >/dev/null 2>&1
        fancybash init fish 2>/dev/null >> "$FISHCONFIG"; or echo 'if type -q fancybash; fancybash init fish | source; end' >> "$FISHCONFIG"
    else
        echo 'if type -q fancybash' >> "$FISHCONFIG"
        echo '    fancybash init fish | source' >> "$FISHCONFIG"
        echo 'end' >> "$FISHCONFIG"
    end
    echo "$END" >> "$FISHCONFIG"

    printf "  {$GREEN}✔{$NC} Config successfully added to ~/.config/fish/config.fish!\n"
end

# ─── Reload & Summary ──────────────────────
function show_summary
    echo ""
    if source "$FISHCONFIG" 2>/dev/null
        printf "  {$GREEN}✨ Installation & auto-reload successful!{$NC}\n\n"
    else
        printf "  {$YELLOW}⚠ Auto-reload skipped.{$NC} Please run: {$BOLD}source ~/.config/fish/config.fish{$NC}\n\n"
    end

    echo -e "\n{$CYAN}──────────────────────────────────────────────────────────{$NC}"
    echo -e " 🚀  {$BOLD}INSTALLATION SUMMARY{$NC}"
    echo -e "{$CYAN}──────────────────────────────────────────────────────────{$NC}\n"
    echo -e "  📦  {$BOLD}Backup:{$NC}    {$GREEN}"(basename "$backup_file" 2>/dev/null; or echo "none")"{$NC}"
    echo -e "  ⚙️   {$BOLD}Config:{$NC}    {$GREEN}~/.config/fish/config.fish{$NC}"
    echo -e "  🔄  {$BOLD}Reload:{$NC}    {$PURPLE}source ~/.config/fish/config.fish{$NC}"
    echo -e "{$CYAN}──────────────────────────────────────────────────────────{$NC}\n"
    echo -e "  🎉  {$BOLD}Installation complete!{$NC}\n"
    echo ""
end

# ─── Main Execution ────────────────────────
show_header
show_sysinfo

draw_progress_bar 1 6
if not check_and_install_fonts
    exit 0
end

draw_progress_bar 2 6
setup_fontconfig

draw_progress_bar 3 6
setup_rust_binary

draw_progress_bar 4 6
check_existing_install
remove_old_config

draw_progress_bar 5 6
backup_fishconfig

draw_progress_bar 6 6
if install_config
    show_summary
else
    echo -e "{$RED}❌ Installation failed. Your existing configuration remains untouched.{$NC}"
end

