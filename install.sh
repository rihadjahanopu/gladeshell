#!/bin/bash

set -euo pipefail

BASHRC="$HOME/.bashrc"
URL="https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main/config.sh"
FALLBACK_URL="https://fancybash.netlify.app/public/config.sh"
START="# >>> fancy-bashrc >>>"
END="# <<< fancy-bashrc <<<"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" 2>/dev/null && pwd || echo "$PWD")"
LOCAL_CONFIG="$SCRIPT_DIR/config.sh"

# ─── Colors & Formatting ───────────────────
RED='\033[38;2;243;139;168m'
GREEN='\033[38;2;166;227;161m'
YELLOW='\033[38;2;249;226;175m'
BLUE='\033[38;2;137;180;250m'
PURPLE='\033[38;2;203;166;247m'
CYAN='\033[38;2;148;226;213m'
GRAY='\033[38;2;147;153;178m'
BOLD='\033[1m'
NC='\033[0m'

tmpfile=""
backup_file=""

# ─── Unattended / Auto-Yes Detection ──────────────
AUTO_YES=false
for arg in "$@"; do
    if [[ "$arg" == "-y" || "$arg" == "--yes" || "$arg" == "--unattended" ]]; then
        AUTO_YES=true
        break
    fi
done
if [[ "${FANCYBASH_AUTO_YES:-}" == "1" || "${NONINTERACTIVE:-}" == "1" || "${CI:-}" == "true" ]]; then
    AUTO_YES=true
fi

# ─── Safe Signal Trap & Cursor Restore ─────
cleanup() {
    tput cnorm 2>/dev/null || true
    if [ -n "$tmpfile" ] && [ -f "$tmpfile" ]; then
        rm -f "$tmpfile"
    fi
}
trap cleanup EXIT SIGINT SIGTERM

# ─── Spinner ───────────────────────────────
spinner() {
    local pid=$1 msg="$2" delay=0.08
    local spin=('⠋' '⠙' '⠹' '⠸' '⠼' '⠴' '⠦' '⠧' '⠇' '⠏')
    tput civis 2>/dev/null || true
    while kill -0 "$pid" 2>/dev/null; do
        for char in "${spin[@]}"; do
            printf "\r  ${CYAN}%s${NC} %s" "$char" "$msg"
            sleep $delay
        done
    done
    tput cnorm 2>/dev/null || true
    printf "\r  ${GREEN}✔${NC} %s\n" "$msg"
}

# ─── Progress Bar ──────────────────────────
draw_progress_bar() {
    local current=$1
    local total=5
    local width=30
    local percentage=$((current * 100 / total))
    local completed=$((width * current / total))
    local remaining=$((width - completed))

    local bar=$(printf "%${completed}s" | tr ' ' '█')
    local empty=$(printf "%${remaining}s" | tr ' ' '░')

    echo ""
    printf "${BLUE}Progress:${NC} [${GREEN}%s${GRAY}%s${NC}] ${CYAN}%d%%${NC} (Step %d/%d)\n" "$bar" "$empty" "$percentage" "$current" "$total"
}

# ─── Header ────────────────────────────────
show_header() {
    echo ""
    echo -e "${PURPLE}          ███████╗ █████╗ ███╗   ██╗ ██████╗██╗   ██╗██████╗  █████╗ ███████╗██╗  ██╗${NC}"
    echo -e "${PURPLE}          ██╔════╝██╔══██╗████╗  ██║██╔════╝╚██╗ ██╔╝██╔══██╗██╔══██╗██╔════╝██║  ██║${NC}"
    echo -e "${CYAN}          █████╗  ███████║██╔██╗ ██║██║      ╚████╔╝ ██████╔╝███████║███████╗███████║${NC}"
    echo -e "${CYAN}          ██╔══╝  ██╔══██║██║╚██╗██║██║       ╚██╔╝  ██╔══██╗██╔══██║╚════██║██╔══██║${NC}"
    echo -e "${BLUE}          ██║     ██║  ██║██║ ╚████║╚██████╗   ██║   ██████╔╝██║  ██║███████║██║  ██║${NC}"
    echo -e "${BLUE}          ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝   ╚═╝   ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝${NC}"
    echo ""
    echo -e "   ✨ ${BOLD}${CYAN}F A N C Y B A S H${NC}  •  ${BOLD}Bash Config Installer${NC}"
    echo ""
}

# ─── System Information ────────────────────
show_sysinfo() {
    local os_name=$(uname -s)
    if [ -f /etc/os-release ]; then
        os_name=$(grep '^PRETTY_NAME=' /etc/os-release | cut -d '=' -f 2 | tr -d '"')
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        os_name="macOS $(sw_vers -productVersion 2>/dev/null || echo '')"
    fi

    local arch=$(uname -m)
    local user=${USER:-$(whoami 2>/dev/null || echo "user")}
    # 1st: Use shell detected by i.sh (most reliable — passed via env var)
    # 2nd: PPID detection (for direct runs without i.sh)
    # 3rd: $SHELL fallback
    local current_shell="${FANCYBASH_SHELL:-}"
    if [ -z "$current_shell" ]; then
        local _ppid_cmd
        _ppid_cmd=$(ps -p "$PPID" -o comm= 2>/dev/null | sed 's/^-//' | xargs basename 2>/dev/null)
        [ -z "$_ppid_cmd" ] && [ -f "/proc/$PPID/comm" ] && \
            _ppid_cmd=$(sed 's/^-//' "/proc/$PPID/comm" 2>/dev/null)
        case "${_ppid_cmd:-}" in
            zsh|bash|fish|dash|sh) current_shell="$_ppid_cmd" ;;
            *) current_shell=$(basename "${SHELL:-bash}") ;;
        esac
    fi

    echo -e "\n${BLUE}──────────────────────────────────────────────────${NC}"
    echo -e " 🖥️   ${BOLD}SYSTEM INFORMATION${NC}"
    echo -e "${BLUE}──────────────────────────────────────────────────${NC}"
    echo -e "  💻  ${BOLD}OS:${NC}      ${CYAN}$os_name${NC}"
    echo -e "  👤  ${BOLD}User:${NC}    ${CYAN}$user${NC}"
    echo -e "  🐚  ${BOLD}Shell:${NC}   ${CYAN}$current_shell${NC}"
    echo -e "  ⚙️   ${BOLD}Arch:${NC}    ${CYAN}$arch${NC}"
    echo -e "${BLUE}──────────────────────────────────────────────────${NC}\n"
    echo ""
}

# ─── OS & Package Manager Detection ────────
detect_pm() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "brew"
    elif [ -f /etc/os-release ]; then
        # shellcheck disable=SC1091
        . /etc/os-release
        case "${ID_LIKE:-$ID}" in
            *debian*|*ubuntu*|*mint*|*pop*) echo "apt" ;;
            *arch*|*manjaro*)               echo "pacman" ;;
            *fedora*|*rhel*|*centos*)       echo "dnf" ;;
            *alpine*)                       echo "apk" ;;
            *)
                if command -v apt-get &>/dev/null; then echo "apt"
                elif command -v pacman &>/dev/null; then echo "pacman"
                elif command -v dnf &>/dev/null; then echo "dnf"
                elif command -v apk &>/dev/null; then echo "apk"
                else echo "unknown"; fi
                ;;
        esac
    else
        echo "unknown"
    fi
}

is_intel_or_amd() {
    if [ -f /proc/cpuinfo ] && grep -qiE 'intel|amd' /proc/cpuinfo 2>/dev/null; then
        return 0
    fi
    if command -v lspci &>/dev/null && lspci 2>/dev/null | grep -qiE 'intel|amd|radeon'; then
        return 0
    fi
    return 1
}

# ─── Check & Install Fonts ──────────────────
check_and_install_fonts() {
    printf "  ${CYAN}➜${NC} Checking system dependencies...\n"

    local missing_deps=()
    for cmd in curl grep git fzf gum glow bat zoxide chafa; do
        if [ "$cmd" = "bat" ] && command -v batcat &>/dev/null; then
            continue
        fi
        if ! command -v "$cmd" &>/dev/null; then
            missing_deps+=("$cmd")
        fi
    done

    if [[ "$OSTYPE" != "darwin"* ]]; then
        if ! command -v xclip &>/dev/null && ! command -v wl-copy &>/dev/null && ! command -v xsel &>/dev/null; then
            missing_deps+=("clipboard-tool")
        fi
        if is_intel_or_amd && ! command -v vulkaninfo &>/dev/null; then
            missing_deps+=("vulkan-tools")
        fi
    fi

    local fonts_needed=0
    if command -v fc-list &>/dev/null; then
        if ! fc-list : family | grep -qi "Fira Code\|FiraCode" || ! fc-list : family | grep -qi "Noto Color Emoji\|NotoColorEmoji"; then
            fonts_needed=1
        fi
    else
        fonts_needed=1
    fi

    if [ ${#missing_deps[@]} -eq 0 ] && [ $fonts_needed -eq 0 ]; then
        printf "  ${GREEN}✔${NC} All dependencies & fonts are already installed!\n"
        return 0
    fi

    local response=""
    if [[ "$AUTO_YES" == true ]]; then
        response="y"
    else
        # Interactive Prompt
        echo ""
        printf "${YELLOW}  ❯ Missing components detected (${missing_deps[*]}).${NC}\n"
        printf "    Would you like to auto-install missing dependencies and proceed? [${GREEN}Y${NC}/n]: "

        # Read from /dev/tty safely for curl piped scripts
        if [ -t 0 ]; then
            read -r response || response=""
        elif [ -c /dev/tty ]; then
            read -r response < /dev/tty || response=""
        fi

        response="$(echo "$response" | tr '[:upper:]' '[:lower:]' | xargs 2>/dev/null || echo "$response")"
        if [ -z "$response" ]; then
            response="y"
        fi
    fi

    if [[ "$response" != "y" && "$response" != "yes" ]]; then
        printf "  ${YELLOW}⚠ Installation cancelled by user. No changes were made.${NC}\n\n"
        return 1
    fi

    local pm=$(detect_pm)
    local sudo_cmd=""
    if [ "${EUID:-$(id -u)}" -ne 0 ] && command -v sudo &>/dev/null; then
        sudo_cmd="sudo"
    fi

    printf "  ${CYAN}➜${NC} Installing via ${pm}...\n"

    local vulkan_pkgs=""
    if [[ "$OSTYPE" != "darwin"* ]] && is_intel_or_amd; then
        case "$pm" in
            apt) vulkan_pkgs="mesa-vulkan-drivers vulkan-tools" ;;
            pacman) vulkan_pkgs="vulkan-intel vulkan-radeon vulkan-tools" ;;
            dnf) vulkan_pkgs="mesa-vulkan-drivers vulkan-tools" ;;
            apk) vulkan_pkgs="vulkan-loader vulkan-tools" ;;
        esac
    fi

    case "$pm" in
        apt)
            if ! command -v gum &>/dev/null || ! command -v glow &>/dev/null; then
                $sudo_cmd mkdir -p /etc/apt/keyrings 2>/dev/null || true
                curl -fsSL https://repo.charm.sh/apt/gpg.key | $sudo_cmd gpg --dearmor --yes -o /etc/apt/keyrings/charm.gpg 2>/dev/null || true
                echo "deb [signed-by=/etc/apt/keyrings/charm.gpg] https://repo.charm.sh/apt/ * *" | $sudo_cmd tee /etc/apt/sources.list.d/charm.list >/dev/null 2>&1 || true
            fi
            $sudo_cmd apt update -qq >/dev/null 2>&1 || true
            $sudo_cmd apt install -y curl git fzf gum glow bat zoxide chafa xclip wl-clipboard nano bash-completion fonts-noto-color-emoji fonts-firacode fonts-cascadia-code fontconfig $vulkan_pkgs >/dev/null 2>&1 || true
            ;;
        pacman)
            $sudo_cmd pacman -Sy --noconfirm curl git fzf gum glow bat zoxide chafa xclip wl-clipboard nano bash-completion ttf-noto-emoji ttf-fira-code ttf-cascadia-code fontconfig $vulkan_pkgs >/dev/null 2>&1 || true
            ;;
        dnf)
            if ! command -v gum &>/dev/null || ! command -v glow &>/dev/null; then
                echo '[charm]
name=Charm
baseurl=https://repo.charm.sh/yum/
enabled=1
gpgcheck=1
gpgkey=https://repo.charm.sh/yum/gpg.key' | $sudo_cmd tee /etc/yum.repos.d/charm.repo >/dev/null 2>&1 || true
            fi
            $sudo_cmd dnf install -y curl git fzf gum glow bat zoxide chafa xclip wl-clipboard nano bash-completion google-noto-emoji-fonts fira-code-fonts cascadia-code-fonts fontconfig $vulkan_pkgs >/dev/null 2>&1 || true
            ;;
        apk)
            $sudo_cmd apk add --no-cache curl git fzf gum glow bat zoxide chafa xclip wl-clipboard nano bash-completion font-noto-emoji font-fira-code fontconfig $vulkan_pkgs >/dev/null 2>&1 || true
            ;;
        brew)
            brew install curl git fzf gum glow bat zoxide chafa nano bash-completion font-fira-code font-cascadia-code font-noto-emoji >/dev/null 2>&1 || true
            ;;
        *)
            printf "  ${GRAY}ℹ Package manager not recognized. Skipping.${NC}\n"
            ;;
    esac
    printf "  ${GREEN}✔${NC} Dependencies process completed.\n"
}

# ─── Configure Fontconfig ──────────────────
setup_fontconfig() {
    printf "  ${CYAN}➜${NC} Checking font configuration...\n"
    local font_dir="$HOME/.config/fontconfig"
    local font_conf="$font_dir/fonts.conf"

    if [ -f "$font_conf" ]; then
        printf "  ${GREEN}✔${NC} Fontconfig already exists (${GRAY}~/.config/fontconfig/fonts.conf${NC})\n"
        return 0
    fi

    mkdir -p "$font_dir"
    cat << 'EOF' > "$font_conf"
<?xml version="1.0"?>
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
</fontconfig>
EOF
    printf "  ${GREEN}✔${NC} Created ${PURPLE}~/.config/fontconfig/fonts.conf${NC}\n"

    if command -v fc-cache &>/dev/null; then
        fc-cache -f &>/dev/null || true
        printf "  ${GREEN}✔${NC} Font cache refreshed!\n"
    fi
}

# ─── Remove Old Config Block ───────────────
remove_old_config() {
    if grep -qF "$START" "$BASHRC" 2>/dev/null; then
        printf "  ${YELLOW}⚠${NC} Found existing fancy-bashrc block — removing old config first...\n"
        while grep -qF "$START" "$BASHRC" 2>/dev/null; do
            if [ "$(uname)" = "Darwin" ]; then
                sed -i '' '/# >>> fancy-bashrc >>>/,/# <<< fancy-bashrc <<</d' "$BASHRC"
            else
                sed -i '/# >>> fancy-bashrc >>>/,/# <<< fancy-bashrc <<</d' "$BASHRC"
            fi
        done
        printf "  ${GREEN}✔${NC} Old config removed.\n"
    fi
}

# ─── Check Existing Installation ───────────
check_existing_install() {
    printf "  ${CYAN}➜${NC} Checking existing configuration...\n"
    if [ ! -f "$BASHRC" ]; then
        printf "  ${YELLOW}⚠ Creating $BASHRC...${NC}\n"
        touch "$BASHRC"
    fi
    printf "  ${GREEN}✔${NC} Ready for installation.\n"
}

# ─── Backup ────────────────────────────────
backup_bashrc() {
    printf "  ${CYAN}➜${NC} Creating backup...\n"
    backup_file="$BASHRC.backup.$(date +%Y%m%d_%H%M%S)"
    cp "$BASHRC" "$backup_file"
    printf "  ${GREEN}✔${NC} Backup created: ${PURPLE}$(basename "$backup_file")${NC}\n"
}

# ─── Install / Verify Rust Binary ──────────
setup_rust_binary() {
    printf "  ${CYAN}➜${NC} Installing fancybash Rust engine binary...\n"
    local bin_dir="$HOME/.local/bin"
    mkdir -p "$bin_dir"
    export PATH="$bin_dir:$PATH"

    # 1. Local pre-built binary in target/release
    if [ -f "$SCRIPT_DIR/target/release/fancybash" ]; then
        cp "$SCRIPT_DIR/target/release/fancybash" "$bin_dir/fancybash"
        chmod +x "$bin_dir/fancybash"
        printf "  ${GREEN}✔${NC} Installed local release binary to ${PURPLE}%s/fancybash${NC}\n" "$bin_dir"
        return 0
    fi

    # 2. Local Rust source build via cargo
    if [ -f "$SCRIPT_DIR/Cargo.toml" ] && command -v cargo >/dev/null 2>&1; then
        printf "  ${YELLOW}⚡ Building fancybash Rust engine (release mode)...${NC}\n"
        (cd "$SCRIPT_DIR" && cargo build --release)
        if [ -f "$SCRIPT_DIR/target/release/fancybash" ]; then
            cp "$SCRIPT_DIR/target/release/fancybash" "$bin_dir/fancybash"
            chmod +x "$bin_dir/fancybash"
            printf "  ${GREEN}✔${NC} Built & installed to ${PURPLE}%s/fancybash${NC}\n" "$bin_dir"
            return 0
        fi
    fi

    # 3. Existing system binary
    if command -v fancybash >/dev/null 2>&1; then
        printf "  ${GREEN}✔${NC} fancybash binary active: $(command -v fancybash)\n"
        return 0
    fi

    # 4. Fallback cargo install from Git
    if command -v cargo >/dev/null 2>&1; then
        printf "  ${YELLOW}⚡ Installing via cargo from GitHub...${NC}\n"
        cargo install --git https://github.com/rihadjahanopu/fancybash-rs --quiet 2>/dev/null || true
        if command -v fancybash >/dev/null 2>&1; then
            printf "  ${GREEN}✔${NC} Installed fancybash via cargo install!\n"
            return 0
        fi
    fi

    printf "  ${YELLOW}⚠ Could not auto-build Rust binary. Please install Rust (cargo) and build manually.${NC}\n"
}

# ─── Fetch & Append Config ─────────────────
install_config() {
    printf "  ${CYAN}➜${NC} Configuring fancybash for your shell...\n"
    export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"

    if command -v fancybash >/dev/null 2>&1; then
        # fancybash setup detects the shell and injects eval line idempotently
        fancybash setup
    else
        # Fallback: manually inject if binary not yet in PATH
        local MARKER='fancybash init'
        if ! grep -qF "$MARKER" "$BASHRC" 2>/dev/null; then
            {
                echo ""
                echo "$START"
                echo "# Installed: $(date '+%Y-%m-%d %H:%M:%S')"
                echo "# fancybash Rust Native Engine - auto-loaded every shell session"
                echo 'export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"'
                echo ""
                echo 'if command -v fancybash >/dev/null 2>&1; then'
                echo '    eval "$(fancybash init bash)"'
                echo 'fi'
                echo "$END"
            } >> "$BASHRC"
            printf "  ${GREEN}✔${NC} Injected fancybash init into ~/.bashrc\n"
        else
            printf "  ${GREEN}✔${NC} fancybash already configured in ~/.bashrc\n"
        fi
    fi
}

# ─── Reload & Summary ──────────────────────
show_summary() {
    echo ""
    if source "$BASHRC" 2>/dev/null; then
        printf "  ${GREEN}✨ Installation & auto-reload successful!${NC}\n\n"
    else
        printf "  ${YELLOW}⚠ Auto-reload skipped.${NC} Please run: ${BOLD}source ~/.bashrc${NC}\n\n"
    fi

    echo -e "\n${CYAN}──────────────────────────────────────────────────────────${NC}"
    echo -e " 🚀  ${BOLD}INSTALLATION SUMMARY${NC}"
    echo -e "${CYAN}──────────────────────────────────────────────────────────${NC}\n"
    echo -e "  📦  ${BOLD}Backup:${NC}    ${GREEN}$(basename "${backup_file:-none}")${NC}"
    echo -e "  ⚙️   ${BOLD}Config:${NC}    ${GREEN}~/.bashrc${NC}"
    echo -e "  🔄  ${BOLD}Reload:${NC}    ${PURPLE}source ~/.bashrc${NC}"
    echo -e "${CYAN}──────────────────────────────────────────────────────────${NC}\n"
    echo -e "  🎉  ${BOLD}Installation complete!${NC}\n"
    echo ""
}

# ─── Main Execution ────────────────────────
main() {
    show_header
    show_sysinfo

    draw_progress_bar 1 6
    if ! check_and_install_fonts; then
        exit 0
    fi

    draw_progress_bar 2 6
    setup_fontconfig

    draw_progress_bar 3 6
    setup_rust_binary

    draw_progress_bar 4 6
    check_existing_install
    remove_old_config

    draw_progress_bar 5 6
    backup_bashrc

    draw_progress_bar 6 6
    install_config

    show_summary
}

main "$@"

