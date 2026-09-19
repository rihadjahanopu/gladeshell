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
    echo -e "   ✨ ${BOLD}${CYAN}F A N C Y B A S H${NC}  •  ${BOLD}Bash Config Installer${NC}"
    echo ""
}

# ─── System Information ────────────────────
detect_shell() {
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
    echo "$current_shell"
}

show_sysinfo() {
    local os_name=$(uname -s)
    if [ -f /etc/os-release ]; then
        os_name=$(grep '^PRETTY_NAME=' /etc/os-release | cut -d '=' -f 2 | tr -d '"')
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        os_name="macOS $(sw_vers -productVersion 2>/dev/null || echo '')"
    fi

    local arch=$(uname -m)
    local user=${USER:-$(whoami 2>/dev/null || echo "user")}
    local current_shell
    current_shell=$(detect_shell)

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

# ─── Check & Install Fonts & Essential Dependencies ───────
check_and_install_fonts() {
    printf "  ${CYAN}➜${NC} Checking system dependencies...\n"

    local missing_deps=()
    for cmd in curl git; do
        if ! command -v "$cmd" &>/dev/null; then
            missing_deps+=("$cmd")
        fi
    done

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

    case "$pm" in
        apt)
            $sudo_cmd apt update -qq >/dev/null 2>&1 || true
            $sudo_cmd apt install -y curl git fonts-noto-color-emoji fonts-firacode fonts-cascadia-code fontconfig >/dev/null 2>&1 || true
            ;;
        pacman)
            $sudo_cmd pacman -Sy --noconfirm curl git ttf-noto-emoji ttf-fira-code ttf-cascadia-code fontconfig >/dev/null 2>&1 || true
            ;;
        dnf)
            $sudo_cmd dnf install -y curl git google-noto-emoji-fonts fira-code-fonts cascadia-code-fonts fontconfig >/dev/null 2>&1 || true
            ;;
        apk)
            $sudo_cmd apk add --no-cache curl git font-noto-emoji font-fira-code fontconfig >/dev/null 2>&1 || true
            ;;
        brew)
            brew install curl git font-fira-code font-cascadia-code font-noto-emoji >/dev/null 2>&1 || true
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

# ─── Install Zsh Plugins (Conditional) ───────────
install_zsh_plugins() {
    local current_shell
    current_shell=$(detect_shell)
    if [ "$current_shell" != "zsh" ]; then
        return 0
    fi

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

# ─── Helper: Tildify Path ───────────────────
tildify() {
    if [[ "$1" == "$HOME"* ]]; then
        echo "~${1#"$HOME"}"
    else
        echo "$1"
    fi
}

# ─── Install / Verify Rust Binary ──────────
_copy_binary_to_dirs() {
    local src="$1"
    local local_bin="$HOME/.local/bin"
    local cargo_bin="$HOME/.cargo/bin"
    mkdir -p "$local_bin"
    cp "$src" "$local_bin/fancybash"
    chmod +x "$local_bin/fancybash"
    export PATH="$local_bin:$PATH"
    # Also copy to ~/.cargo/bin if cargo is installed (rustup puts it in PATH)
    if command -v cargo >/dev/null 2>&1 || [ -d "$cargo_bin" ]; then
        mkdir -p "$cargo_bin"
        cp "$src" "$cargo_bin/fancybash"
        chmod +x "$cargo_bin/fancybash"
        export PATH="$cargo_bin:$PATH"
        printf "  ${GREEN}✔${NC} Installed to ${PURPLE}%s/fancybash${NC} and ${PURPLE}%s/fancybash${NC}\n" "$(tildify "$local_bin")" "$(tildify "$cargo_bin")"
    else
        printf "  ${GREEN}✔${NC} Installed to ${PURPLE}%s/fancybash${NC}\n" "$(tildify "$local_bin")"
    fi
    # Show version
    local ver
    ver=$("$local_bin/fancybash" --version 2>/dev/null || echo "unknown")
    printf "  ${CYAN}ℹ${NC} Version: ${BOLD}%s${NC}\n" "$ver"
}

setup_rust_binary() {
    printf "  ${CYAN}➜${NC} Installing fancybash Rust engine binary...\n"

    # 1. Local pre-built binary in target/release
    if [ -f "$SCRIPT_DIR/target/release/fancybash" ]; then
        printf "  ${CYAN}⚡ Found local release binary — installing...${NC}\n"
        _copy_binary_to_dirs "$SCRIPT_DIR/target/release/fancybash"
        return 0
    fi

    # 2. Download pre-built release binary from GitHub Releases
    local os_type arch_type is_musl=false is_rosetta=false has_avx2=true
    os_type="$(uname -s | tr '[:upper:]' '[:lower:]')"
    arch_type="$(uname -m)"

    case "$os_type" in
        linux*) os_type="linux" ;;
        darwin*) os_type="darwin" ;;
        *) os_type="unknown" ;;
    esac

    case "$arch_type" in
        x86_64|amd64) arch_type="amd64" ;;
        aarch64|arm64) arch_type="arm64" ;;
        *) arch_type="unknown" ;;
    esac

    # Rosetta 2 detection on macOS (Apple Silicon running x86_64 shell)
    if [ "$os_type" = "darwin" ] && [ "$arch_type" = "amd64" ]; then
        if [ "$(sysctl -n sysctl.proc_translated 2>/dev/null)" = "1" ]; then
            arch_type="arm64"
            is_rosetta=true
            printf "  ${CYAN}ℹ${NC} Rosetta 2 detected — downloading native Apple Silicon (arm64) binary\n"
        fi
    fi

    # Alpine Linux musl libc detection
    if [ "$os_type" = "linux" ] && [ -f /etc/alpine-release ]; then
        is_musl=true
    fi

    # AVX2 instruction set probing
    if [ "$arch_type" = "amd64" ]; then
        if [ "$os_type" = "linux" ] && [ -f /proc/cpuinfo ] && ! grep -qi "avx2" /proc/cpuinfo 2>/dev/null; then
            has_avx2=false
        elif [ "$os_type" = "darwin" ] && ! sysctl -a 2>/dev/null | grep -q "AVX2"; then
            has_avx2=false
        fi
    fi

    if [ "$os_type" != "unknown" ] && [ "$arch_type" != "unknown" ]; then
        local dl_tmp
        dl_tmp="$(mktemp -d 2>/dev/null || mktemp -d -t 'fancybash')"
        local bin_tmp="$dl_tmp/fancybash"

        local repos=("rihadjahanopu/fancybash-rs" "rihadjahanopu/fancybash")
        local assets=(
            "fancybash-${os_type}-${arch_type}"
            "fancybash-${arch_type}-${os_type}"
            "fancybash-x86_64-unknown-linux-gnu"
            "fancybash-aarch64-unknown-linux-gnu"
            "fancybash-x86_64-apple-darwin"
            "fancybash-aarch64-apple-darwin"
        )

        if [ "$is_musl" = true ]; then
            assets=(
                "fancybash-x86_64-unknown-linux-musl"
                "fancybash-aarch64-unknown-linux-musl"
                "fancybash-${os_type}-${arch_type}-musl"
                "${assets[@]}"
            )
        fi

        if [ "$has_avx2" = false ]; then
            assets=(
                "fancybash-${os_type}-${arch_type}-baseline"
                "fancybash-x86_64-unknown-linux-gnu-baseline"
                "${assets[@]}"
            )
        fi

        assets+=("fancybash")

        printf "  ${CYAN}⚡ Attempting GitHub Release pre-built binary download...${NC}\n"
        for repo in "${repos[@]}"; do
            for asset in "${assets[@]}"; do
                local url="https://github.com/${repo}/releases/latest/download/${asset}"
                if curl -fsSL "$url" -o "$bin_tmp" 2>/dev/null; then
                    if [ -s "$bin_tmp" ]; then
                        chmod +x "$bin_tmp"
                        if "$bin_tmp" --version >/dev/null 2>&1; then
                            printf "  ${GREEN}✔${NC} Downloaded latest pre-built binary from GitHub Release (${repo})!\n"
                            _copy_binary_to_dirs "$bin_tmp"
                            rm -rf "$dl_tmp" 2>/dev/null || true
                            return 0
                        fi
                    fi
                fi
                # Also try tar.gz archive
                local tar_url="https://github.com/${repo}/releases/latest/download/${asset}.tar.gz"
                if curl -fsSL "$tar_url" -o "$dl_tmp/asset.tar.gz" 2>/dev/null; then
                    if tar -xzf "$dl_tmp/asset.tar.gz" -C "$dl_tmp" 2>/dev/null && [ -f "$bin_tmp" ]; then
                        chmod +x "$bin_tmp"
                        if "$bin_tmp" --version >/dev/null 2>&1; then
                            printf "  ${GREEN}✔${NC} Extracted latest pre-built binary from GitHub Release (${repo})!\n"
                            _copy_binary_to_dirs "$bin_tmp"
                            rm -rf "$dl_tmp" 2>/dev/null || true
                            return 0
                        fi
                    fi
                fi
            done
        done
        rm -rf "$dl_tmp" 2>/dev/null || true
    fi

    # 3. Build from local source if Cargo.toml exists
    if [ -f "$SCRIPT_DIR/Cargo.toml" ] && command -v cargo >/dev/null 2>&1; then
        printf "  ${YELLOW}⚡ Building fancybash from source (release mode)...${NC}\n"
        if (cd "$SCRIPT_DIR" && cargo build --release 2>&1); then
            if [ -f "$SCRIPT_DIR/target/release/fancybash" ]; then
                _copy_binary_to_dirs "$SCRIPT_DIR/target/release/fancybash"
                return 0
            fi
        fi
        printf "  ${RED}✗ cargo build failed.${NC}\n"
    fi

    # 4. Already installed on system PATH
    if command -v fancybash >/dev/null 2>&1; then
        local ver
        ver=$(fancybash --version 2>/dev/null || echo "unknown")
        printf "  ${GREEN}✔${NC} fancybash already installed: $(command -v fancybash) (${CYAN}%s${NC})\n" "$ver"
        return 0
    fi

    # 5. Fallback: cargo install from GitHub
    if command -v cargo >/dev/null 2>&1; then
        printf "  ${YELLOW}⚡ Installing via cargo from GitHub...${NC}\n"
        cargo install --git https://github.com/rihadjahanopu/fancybash-rs --quiet 2>/dev/null || true
        if command -v fancybash >/dev/null 2>&1; then
            printf "  ${GREEN}✔${NC} Installed fancybash via cargo install!\n"
            return 0
        fi
    fi

    printf "  ${YELLOW}⚠ Could not install fancybash binary. Please install Cargo or download binary manually.${NC}\n"
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
        local user_shell="$(basename "${SHELL:-bash}")"

        if [ "$user_shell" = "fish" ]; then
            local fish_cfg="$HOME/.config/fish/config.fish"
            mkdir -p "$(dirname "$fish_cfg")"
            if ! grep -qF "$MARKER" "$fish_cfg" 2>/dev/null; then
                {
                    echo ""
                    echo "# >>> fancy-fish >>>"
                    echo 'set -gx PATH $HOME/.cargo/bin $HOME/.local/bin $PATH'
                    echo 'if type -q fancybash'
                    echo '    fancybash init fish | source'
                    echo 'end'
                    echo "# <<< fancy-fish <<<"
                } >> "$fish_cfg"
                printf "  ${GREEN}✔${NC} Injected fancybash init into ~/.config/fish/config.fish\n"
            else
                printf "  ${GREEN}✔${NC} fancybash already configured in ~/.config/fish/config.fish\n"
            fi
        elif [ "$user_shell" = "zsh" ]; then
            local zshrc="$HOME/.zshrc"
            if ! grep -qF "$MARKER" "$zshrc" 2>/dev/null; then
                {
                    echo ""
                    echo "# >>> fancy-zshrc >>>"
                    echo 'export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"'
                    echo 'if (( ${+commands[fancybash]} )); then'
                    echo '    eval "$(fancybash init zsh)"'
                    echo 'fi'
                    echo "# <<< fancy-zshrc <<<"
                } >> "$zshrc"
                printf "  ${GREEN}✔${NC} Injected fancybash init into ~/.zshrc\n"
            else
                printf "  ${GREEN}✔${NC} fancybash already configured in ~/.zshrc\n"
            fi
        else
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
    install_zsh_plugins

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
