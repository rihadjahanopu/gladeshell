#!/usr/bin/env zsh

set -euo pipefail

ZSHRC="$HOME/.zshrc"
URL="https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main/config.zsh"
FALLBACK_URL="https://fancybash.netlify.app/public/config.zsh"
START="# >>> fancy-zshrc >>>"
END="# <<< fancy-zshrc <<<"
SCRIPT_DIR="$(cd "$(dirname "${(%):-%x}")" 2>/dev/null && pwd || echo "$PWD")"
LOCAL_CONFIG="$SCRIPT_DIR/config.zsh"

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
    local total=6
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
    echo -e "   ✨ ${BOLD}${CYAN}F A N C Y B A S H${NC}  •  ${BOLD}Zsh Config Installer${NC}"
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
            *) current_shell=$(basename "${SHELL:-zsh}") ;;
        esac
    fi

    echo -e "\n${BLUE}──────────────────────────────────────────────────${NC}"
    echo -e " 🖥️  ${BOLD}SYSTEM INFORMATION${NC}"
    echo -e "${BLUE}──────────────────────────────────────────────────${NC}\n"
    echo -e "  💻  ${BOLD}OS:${NC}      ${CYAN}$os_name${NC}"
    echo -e "  👤  ${BOLD}User:${NC}    ${CYAN}$user${NC}"
    echo -e "  🐚  ${BOLD}Shell:${NC}   ${CYAN}$current_shell${NC}"
    echo -e "  ⚙️   ${BOLD}Arch:${NC}    ${CYAN}$arch${NC}\n"
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
            $sudo_cmd apt install -y curl git fzf gum glow bat zoxide chafa xclip wl-clipboard nano fonts-noto-color-emoji fonts-firacode fonts-cascadia-code fontconfig $vulkan_pkgs >/dev/null 2>&1 || true
            ;;
        pacman)
            $sudo_cmd pacman -Sy --noconfirm curl git fzf gum glow bat zoxide chafa xclip wl-clipboard nano ttf-noto-emoji ttf-fira-code ttf-cascadia-code fontconfig $vulkan_pkgs >/dev/null 2>&1 || true
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
            $sudo_cmd dnf install -y curl git fzf gum glow bat zoxide chafa xclip wl-clipboard nano google-noto-emoji-fonts fira-code-fonts cascadia-code-fonts fontconfig $vulkan_pkgs >/dev/null 2>&1 || true
            ;;
        apk)
            $sudo_cmd apk add --no-cache curl git fzf gum glow bat zoxide chafa xclip wl-clipboard nano font-noto-emoji font-fira-code fontconfig $vulkan_pkgs >/dev/null 2>&1 || true
            ;;
        brew)
            brew install curl git fzf gum glow bat zoxide chafa nano font-fira-code font-cascadia-code font-noto-emoji >/dev/null 2>&1 || true
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

# ─── Install Zsh Plugins ───────────────────
install_zsh_plugins() {
    printf "  ${CYAN}➜${NC} Setting up Zsh plugins...\n"

    local zsh_dir="$HOME/.zsh"
    mkdir -p "$zsh_dir"

    # zsh-syntax-highlighting
    if [ -d "$zsh_dir/zsh-syntax-highlighting" ]; then
        printf "  ${GREEN}✔${NC} zsh-syntax-highlighting already exists, skipping.\n"
    else
        (
            git clone --quiet https://github.com/zsh-users/zsh-syntax-highlighting.git \
                "$zsh_dir/zsh-syntax-highlighting" 2>/dev/null
        ) &
        spinner $! "Cloning zsh-syntax-highlighting..."
    fi

    # zsh-autosuggestions
    if [ -d "$zsh_dir/zsh-autosuggestions" ]; then
        printf "  ${GREEN}✔${NC} zsh-autosuggestions already exists, skipping.\n"
    else
        (
            git clone --quiet https://github.com/zsh-users/zsh-autosuggestions.git \
                "$zsh_dir/zsh-autosuggestions" 2>/dev/null
        ) &
        spinner $! "Cloning zsh-autosuggestions..."
    fi

    # zsh-completions
    if [ -d "$zsh_dir/zsh-completions" ]; then
        printf "  ${GREEN}✔${NC} zsh-completions already exists, skipping.\n"
    else
        (
            git clone --quiet https://github.com/zsh-users/zsh-completions.git \
                "$zsh_dir/zsh-completions" 2>/dev/null
        ) &
        spinner $! "Cloning zsh-completions..."
    fi

    printf "  ${GREEN}✔${NC} Zsh plugins ready in ${PURPLE}~/.zsh/${NC}\n"
}

# ─── Remove Old Config Block ───────────────
remove_old_config() {
    # 1. Clean up standalone NVM/Bun auto-added lines outside the fancybash block
    if grep -qF "$START" "$ZSHRC" 2>/dev/null; then
        if [ "$(uname)" = "Darwin" ]; then
            sed -i '' '/# >>> fancy-zshrc >>>/,/# <<< fancy-zshrc <<</!{ /nvm\.sh/d; /bash_completion/d; /_bun/d; /bun completions/d; }' "$ZSHRC" 2>/dev/null || true
        else
            sed -i '/# >>> fancy-zshrc >>>/,/# <<< fancy-zshrc <<</!{ /nvm\.sh/d; /bash_completion/d; /_bun/d; /bun completions/d; }' "$ZSHRC" 2>/dev/null || true
        fi
    fi

    # 2. Remove old fancy-zshrc block
    if grep -qF "$START" "$ZSHRC" 2>/dev/null; then
        printf "  ${YELLOW}⚠${NC} Found existing fancy-zshrc block — removing old config first...\n"
        local start_pattern end_pattern
        start_pattern="$(printf '%s' "$START" | sed 's/[]\/\$*.^[]/\\&/g')"
        end_pattern="$(printf '%s' "$END" | sed 's/[]\/\$*.^[]/\\&/g')"
        while grep -qF "$START" "$ZSHRC" 2>/dev/null; do
            if [ "$(uname)" = "Darwin" ]; then
                sed -i '' "/${start_pattern}/,/${end_pattern}/d" "$ZSHRC"
            else
                sed -i "/${start_pattern}/,/${end_pattern}/d" "$ZSHRC"
            fi
        done
        printf "  ${GREEN}✔${NC} Old config removed.\n"
    fi
}

# ─── Check Existing Installation ───────────
check_existing_install() {
    printf "  ${CYAN}➜${NC} Checking existing configuration...\n"
    if [ ! -f "$ZSHRC" ]; then
        printf "  ${YELLOW}⚠ Creating $ZSHRC...${NC}\n"
        touch "$ZSHRC"
    fi
    printf "  ${GREEN}✔${NC} Ready for installation.\n"
}

# ─── Backup ────────────────────────────────
backup_zshrc() {
    printf "  ${CYAN}➜${NC} Creating backup...\n"
    backup_file="$ZSHRC.backup.$(date +%Y%m%d_%H%M%S)"
    cp "$ZSHRC" "$backup_file"
    printf "  ${GREEN}✔${NC} Backup created: ${PURPLE}$(basename "$backup_file")${NC}\n"
}

# ─── Fetch & Append Config ─────────────────
install_config() {
    printf "  ${CYAN}➜${NC} Fetching and applying config...\n"
    tmpfile=$(mktemp)

    if [ -f "$LOCAL_CONFIG" ]; then
        printf "  ${GREEN}✔${NC} Using local config.zsh\n"
        cp "$LOCAL_CONFIG" "$tmpfile"
    else
        (
            if ! curl -fsSL --connect-timeout 5 "$URL" > "$tmpfile" 2>/dev/null || [ ! -s "$tmpfile" ] || head -1 "$tmpfile" | grep -qi "<!doctype\|<html"; then
                curl -fsSL --connect-timeout 10 "$FALLBACK_URL" > "$tmpfile" 2>/dev/null || rm -f "$tmpfile"
            fi
        ) &
        spinner $! "Downloading config..."
    fi

    if [ ! -s "$tmpfile" ]; then
        echo -e "${RED}  ✘ Download failed!${NC} Check your internet connection." >&2
        exit 1
    fi

    if head -1 "$tmpfile" | grep -qi "<!doctype\|<html"; then
        echo -e "${RED}  ✘ Invalid response (HTML instead of script)!${NC}" >&2
        rm -f "$tmpfile"
        exit 1
    fi

    {
        echo ""
        echo "$START"
        echo "# Installed: $(date '+%Y-%m-%d %H:%M:%S')"
        cat "$tmpfile"
        echo "$END"
    } >> "$ZSHRC"

    rm -f "$tmpfile"
    printf "  ${GREEN}✔${NC} Config successfully added to ~/.zshrc!\n"

    # Pre-compile Zsh bytecode for sub-50ms initial launch
    if command -v zcompile >/dev/null 2>&1; then
        zcompile "$ZSHRC" 2>/dev/null || true
    fi
}

# ─── Reload & Summary ──────────────────────
show_summary() {
    echo ""
    if source "$ZSHRC" 2>/dev/null; then
        printf "  ${GREEN}✨ Installation & auto-reload successful!${NC}\n\n"
    else
        printf "  ${YELLOW}⚠ Auto-reload skipped.${NC} Please run: ${BOLD}source ~/.zshrc${NC}\n\n"
    fi

    echo -e "\n${CYAN}──────────────────────────────────────────────────${NC}"
    echo -e " 🚀  ${BOLD}INSTALLATION SUMMARY${NC}"
    echo -e "${CYAN}──────────────────────────────────────────────────${NC}\n"
    echo -e "  📦  ${BOLD}Backup:${NC}   ${GREEN}$(basename "${backup_file:-none}")${NC}"
    echo -e "  ⚙️   ${BOLD}Config:${NC}   ${GREEN}~/.zshrc${NC}"
    echo -e "  🔄  ${BOLD}Reload:${NC}   ${PURPLE}source ~/.zshrc${NC}\n"
    echo -e "${CYAN}──────────────────────────────────────────────────${NC}\n"
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
    install_zsh_plugins

    draw_progress_bar 4 6
    check_existing_install
    remove_old_config

    draw_progress_bar 5 6
    backup_zshrc

    draw_progress_bar 6 6
    install_config

    show_summary
}

main "$@"
