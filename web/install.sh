#!/usr/bin/env bash

set -euo pipefail

# ─── Windows NT Auto-Bridge ─────────────────────
if [[ "${OS:-}" = "Windows_NT" ]] && [[ "$(uname -s 2>/dev/null)" != MINGW64* ]] && [[ "$(uname -s 2>/dev/null)" != MSYS* ]]; then
    if command -v powershell.exe >/dev/null 2>&1; then
        powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/rihadjahanopu/fancybash/main/install.ps1 | iex"
        exit $?
    elif command -v powershell >/dev/null 2>&1; then
        powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/rihadjahanopu/fancybash/main/install.ps1 | iex"
        exit $?
    fi
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" 2>/dev/null && pwd || echo "$PWD")"

# ─── Terminal & Color Capability Probing ─────
detect_colors() {
    if [[ -t 1 ]] && command -v tput >/dev/null 2>&1 && [ "$(tput colors 2>/dev/null || echo 0)" -ge 8 ]; then
        RED='\033[38;2;243;139;168m'
        GREEN='\033[38;2;166;227;161m'
        YELLOW='\033[38;2;249;226;175m'
        BLUE='\033[38;2;137;180;250m'
        PURPLE='\033[38;2;203;166;247m'
        CYAN='\033[38;2;148;226;213m'
        GRAY='\033[38;2;147;153;178m'
        BOLD='\033[1m'
        NC='\033[0m'
    else
        RED='' GREEN='' YELLOW='' BLUE='' PURPLE='' CYAN='' GRAY='' BOLD='' NC=''
    fi
}
detect_colors

tmpfile=""
backup_files=()
AUTO_YES=false
TARGET_SHELL=""
ALL_SHELLS=false
MODE="install" # install | doctor | rollback | update

# ─── Smart CLI Argument Parser ─────────────────────
for arg in "$@"; do
    case "$arg" in
        -y|--yes|--unattended)
            AUTO_YES=true
            ;;
        --shell=*)
            TARGET_SHELL="${arg#*=}"
            ;;
        --all|--all-shells)
            ALL_SHELLS=true
            ;;
        --doctor|--check|doctor|check)
            MODE="doctor"
            ;;
        --rollback|--undo|rollback|undo)
            MODE="rollback"
            ;;
        --update|--upgrade|update|upgrade)
            MODE="update"
            ;;
    esac
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

# ─── Dynamic Progress Bar ──────────────────
draw_progress_bar() {
    local current=$1
    local total=${2:-5}
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
    echo -e "   ✨ ${BOLD}${CYAN}F A N C Y B A S H${NC}  •  ${BOLD}Smart Production Cross-Shell Engine (Bash, Zsh, Fish)${NC}"
    echo ""
}

# ─── Shell Resolution & System Probing ────
detect_shell() {
    local current_shell="${TARGET_SHELL:-${FANCYBASH_SHELL:-}}"
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

detect_all_installed_shells() {
    local installed=()
    for s in bash zsh fish; do
        if command -v "$s" >/dev/null 2>&1; then
            installed+=("$s")
        fi
    done
    echo "${installed[*]}"
}

get_target_rc() {
    local shell_name="${1:-$(detect_shell)}"
    case "$shell_name" in
        zsh)  echo "$HOME/.zshrc" ;;
        fish) echo "$HOME/.config/fish/config.fish" ;;
        *)
            if [ -f "$HOME/.bashrc" ]; then
                echo "$HOME/.bashrc"
            elif [ -f "$HOME/.bash_profile" ]; then
                echo "$HOME/.bash_profile"
            elif [ -n "${XDG_CONFIG_HOME:-}" ] && [ -f "$XDG_CONFIG_HOME/.bashrc" ]; then
                echo "$XDG_CONFIG_HOME/.bashrc"
            else
                echo "$HOME/.bashrc"
            fi
            ;;
    esac
}

tildify() {
    if [[ "$1" == "$HOME"* ]]; then
        echo "~${1#"$HOME"}"
    else
        echo "$1"
    fi
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
    local all_shells
    all_shells=$(detect_all_installed_shells)
    local target_rc
    target_rc=$(get_target_rc "$current_shell")

    echo -e "\n${BLUE}──────────────────────────────────────────────────${NC}"
    echo -e " 🖥️   ${BOLD}SYSTEM INFORMATION${NC}"
    echo -e "${BLUE}──────────────────────────────────────────────────${NC}"
    echo -e "  💻  ${BOLD}OS:${NC}            ${CYAN}$os_name${NC}"
    echo -e "  👤  ${BOLD}User:${NC}          ${CYAN}$user${NC}"
    echo -e "  🐚  ${BOLD}Active Shell:${NC}  ${CYAN}$current_shell${NC}"
    echo -e "  🌐  ${BOLD}System Shells:${NC} ${CYAN}$all_shells${NC}"
    echo -e "  📄  ${BOLD}Target Config:${NC} ${CYAN}$(tildify "$target_rc")${NC}"
    echo -e "  ⚙️   ${BOLD}Arch:${NC}          ${CYAN}$arch${NC}"
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
        echo ""
        printf "${YELLOW}  ❯ Missing components detected (${missing_deps[*]}).${NC}\n"
        printf "    Would you like to auto-install missing dependencies and proceed? [${GREEN}Y${NC}/n]: "

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

# ─── Check Existing Installation ───────────
check_existing_install() {
    local target_shells=()
    if [ "$ALL_SHELLS" = true ]; then
        read -r -a target_shells <<< "$(detect_all_installed_shells)"
    else
        target_shells=("$(detect_shell)")
    fi

    for sh in "${target_shells[@]}"; do
        local target_rc
        target_rc=$(get_target_rc "$sh")
        printf "  ${CYAN}➜${NC} Checking configuration file for ${BOLD}%s${NC} (${GRAY}%s${NC})...\n" "$sh" "$(tildify "$target_rc")"
        mkdir -p "$(dirname "$target_rc")"
        if [ ! -f "$target_rc" ]; then
            printf "  ${YELLOW}⚠ Creating %s...${NC}\n" "$(tildify "$target_rc")"
            touch "$target_rc"
        fi
    done
    printf "  ${GREEN}✔${NC} Ready for installation.\n"
}

# ─── Remove Old Config Block ───────────────
remove_old_config() {
    printf "  ${CYAN}➜${NC} Cleaning up legacy configuration blocks...\n"
    local rcs=(
        "${HOME}/.bashrc"
        "${HOME}/.bash_profile"
        "${HOME}/.zshrc"
        "${HOME}/.config/fish/config.fish"
    )
    if [ -n "${XDG_CONFIG_HOME:-}" ]; then
        rcs+=("$XDG_CONFIG_HOME/.bashrc" "$XDG_CONFIG_HOME/.bash_profile")
    fi

    local markers=(
        "# >>> fancy-bashrc >>>|# <<< fancy-bashrc <<<"
        "# >>> fancy-zshrc >>>|# <<< fancy-zshrc <<<"
        "# >>> fancy-fish >>>|# <<< fancy-fish <<<"
    )

    local cleaned=false
    for rc in "${rcs[@]}"; do
        [ -f "$rc" ] || continue
        for pair in "${markers[@]}"; do
            local start_m="${pair%%|*}"
            local end_m="${pair##*|}"
            if grep -qF "$start_m" "$rc" 2>/dev/null; then
                cleaned=true
                if [ "$(uname)" = "Darwin" ]; then
                    sed -i '' "/$start_m/,/$end_m/d" "$rc" 2>/dev/null || true
                else
                    sed -i "/$start_m/,/$end_m/d" "$rc" 2>/dev/null || true
                fi
            fi
        done
    done

    if [ "$cleaned" = true ]; then
        printf "  ${GREEN}✔${NC} Old config blocks removed across shell RC files.\n"
    else
        printf "  ${GREEN}✔${NC} Shell configuration files are clean.\n"
    fi
}

# ─── Backup Target RC Files ────────────────
backup_shell_rc() {
    local target_shells=()
    if [ "$ALL_SHELLS" = true ]; then
        read -r -a target_shells <<< "$(detect_all_installed_shells)"
    else
        target_shells=("$(detect_shell)")
    fi

    backup_files=()
    for sh in "${target_shells[@]}"; do
        local target_rc
        target_rc=$(get_target_rc "$sh")
        if [ -f "$target_rc" ]; then
            local bfile="$target_rc.backup.$(date +%Y%m%d_%H%M%S)"
            cp "$target_rc" "$bfile"
            backup_files+=("$bfile")
            printf "  ${GREEN}✔${NC} Backup created for ${BOLD}%s${NC}: ${PURPLE}%s${NC}\n" "$sh" "$(basename "$bfile")"
        fi
    done
}

# ─── Atomic RC Injection Helper ────────────
_atomic_inject_rc() {
    local file="$1"
    local content="$2"
    local dir
    dir="$(dirname "$file")"
    mkdir -p "$dir"
    local tmp
    tmp="$(mktemp "${dir}/.fancybash_tmp.XXXXXX" 2>/dev/null || mktemp -t 'fancybash_tmp')"
    cp "$file" "$tmp" 2>/dev/null || touch "$tmp"
    printf "%s\n" "$content" >> "$tmp"
    mv "$tmp" "$file"
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
    if command -v cargo >/dev/null 2>&1 || [ -d "$cargo_bin" ]; then
        mkdir -p "$cargo_bin"
        cp "$src" "$cargo_bin/fancybash"
        chmod +x "$cargo_bin/fancybash"
        export PATH="$cargo_bin:$PATH"
        printf "  ${GREEN}✔${NC} Installed to ${PURPLE}%s/fancybash${NC} and ${PURPLE}%s/fancybash${NC}\n" "$(tildify "$local_bin")" "$(tildify "$cargo_bin")"
    else
        printf "  ${GREEN}✔${NC} Installed to ${PURPLE}%s/fancybash${NC}\n" "$(tildify "$local_bin")"
    fi
    local ver
    ver=$("$local_bin/fancybash" --version 2>/dev/null || echo "unknown")
    printf "  ${CYAN}ℹ${NC} Version: ${BOLD}%s${NC}\n" "$ver"
}

setup_rust_binary() {
    printf "  ${CYAN}➜${NC} Installing fancybash Rust engine binary...\n"

    if [ -f "$SCRIPT_DIR/target/release/fancybash" ]; then
        printf "  ${CYAN}⚡ Found local release binary — installing...${NC}\n"
        _copy_binary_to_dirs "$SCRIPT_DIR/target/release/fancybash"
        return 0
    fi

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

    if [ "$os_type" = "darwin" ] && [ "$arch_type" = "amd64" ]; then
        if [ "$(sysctl -n sysctl.proc_translated 2>/dev/null)" = "1" ]; then
            arch_type="arm64"
            is_rosetta=true
            printf "  ${CYAN}ℹ${NC} Rosetta 2 detected — downloading native Apple Silicon (arm64) binary\n"
        fi
    fi

    if [ "$os_type" = "linux" ] && [ -f /etc/alpine-release ]; then
        is_musl=true
    fi

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

        local repos=("rihadjahanopu/fancybash" "rihadjahanopu/fancybash")
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

    if command -v fancybash >/dev/null 2>&1; then
        local ver
        ver=$(fancybash --version 2>/dev/null || echo "unknown")
        printf "  ${GREEN}✔${NC} fancybash already installed: $(command -v fancybash) (${CYAN}%s${NC})\n" "$ver"
        return 0
    fi

    if command -v cargo >/dev/null 2>&1; then
        printf "  ${YELLOW}⚡ Installing via cargo from GitHub...${NC}\n"
        cargo install --git https://github.com/rihadjahanopu/fancybash --quiet 2>/dev/null || true
        if command -v fancybash >/dev/null 2>&1; then
            printf "  ${GREEN}✔${NC} Installed fancybash via cargo install!\n"
            return 0
        fi
    fi

    printf "  ${YELLOW}⚠ Could not install fancybash binary. Please install Cargo or download binary manually.${NC}\n"
}

# ─── Fetch & Append Config ─────────────────
_inject_single_shell() {
    local current_shell="$1"
    local target_rc
    target_rc=$(get_target_rc "$current_shell")

    printf "  ${CYAN}➜${NC} Configuring fancybash for ${BOLD}%s${NC} (${GRAY}%s${NC})...\n" "$current_shell" "$(tildify "$target_rc")"

    if command -v fancybash >/dev/null 2>&1; then
        FANCYBASH_SHELL="$current_shell" fancybash setup
    else
        local MARKER='fancybash init'
        local block=""
        if [ "$current_shell" = "fish" ]; then
            if ! grep -qF "$MARKER" "$target_rc" 2>/dev/null; then
                block=$(cat << 'EOF'

# >>> fancy-fish >>>
set -gx PATH $HOME/.cargo/bin $HOME/.local/bin $PATH
if type -q fancybash
    fancybash init fish | source
end
# <<< fancy-fish <<<
EOF
                )
                _atomic_inject_rc "$target_rc" "$block"
                printf "  ${GREEN}✔${NC} Injected fancybash init into %s\n" "$(tildify "$target_rc")"
            else
                printf "  ${GREEN}✔${NC} fancybash already configured in %s\n" "$(tildify "$target_rc")"
            fi
        elif [ "$current_shell" = "zsh" ]; then
            if ! grep -qF "$MARKER" "$target_rc" 2>/dev/null; then
                block=$(cat << 'EOF'

# >>> fancy-zshrc >>>
export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"
if (( ${+commands[fancybash]} )); then
    eval "$(fancybash init zsh)"
fi
# <<< fancy-zshrc <<<
EOF
                )
                _atomic_inject_rc "$target_rc" "$block"
                printf "  ${GREEN}✔${NC} Injected fancybash init into %s\n" "$(tildify "$target_rc")"
            else
                printf "  ${GREEN}✔${NC} fancybash already configured in %s\n" "$(tildify "$target_rc")"
            fi
        else
            if ! grep -qF "$MARKER" "$target_rc" 2>/dev/null; then
                block=$(cat << EOF

# >>> fancy-bashrc >>>
# Installed: $(date '+%Y-%m-%d %H:%M:%S')
# fancybash Rust Native Engine - auto-loaded every shell session
export PATH="\$HOME/.cargo/bin:\$HOME/.local/bin:\$PATH"

if command -v fancybash >/dev/null 2>&1; then
    eval "\$(fancybash init bash)"
fi
# <<< fancy-bashrc <<<
EOF
                )
                _atomic_inject_rc "$target_rc" "$block"
                printf "  ${GREEN}✔${NC} Injected fancybash init into %s\n" "$(tildify "$target_rc")"
            else
                printf "  ${GREEN}✔${NC} fancybash already configured in %s\n" "$(tildify "$target_rc")"
            fi
        fi
    fi
}

install_config() {
    export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"
    local target_shells=()
    if [ "$ALL_SHELLS" = true ]; then
        read -r -a target_shells <<< "$(detect_all_installed_shells)"
    else
        target_shells=("$(detect_shell)")
    fi

    for sh in "${target_shells[@]}"; do
        _inject_single_shell "$sh"
    done
}

# ─── Doctor / Diagnostics Mode ─────────────
run_doctor() {
    show_header
    printf "  🩺 ${BOLD}${CYAN}FANCYBASH SYSTEM DOCTOR${NC}\n"
    printf "  ──────────────────────────────────────────────────\n\n"

    local current_shell
    current_shell=$(detect_shell)
    local all_shells
    all_shells=$(detect_all_installed_shells)

    printf "  🐚 Active Shell:      ${CYAN}%s${NC}\n" "$current_shell"
    printf "  🌐 Installed Shells:  ${CYAN}%s${NC}\n" "$all_shells"

    export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"
    if command -v fancybash >/dev/null 2>&1; then
        local bin_path ver
        bin_path=$(command -v fancybash)
        ver=$(fancybash --version 2>/dev/null || echo "unknown")
        printf "  ⚡ Binary:            ${GREEN}[OK]${NC} %s (${CYAN}%s${NC})\n" "$bin_path" "$ver"
    else
        printf "  ⚡ Binary:            ${RED}[MISSING]${NC} fancybash binary not found in PATH\n"
    fi

    if command -v fc-list &>/dev/null; then
        if fc-list : family | grep -qi "Fira Code\|FiraCode" && fc-list : family | grep -qi "Noto Color Emoji\|NotoColorEmoji"; then
            printf "  🔤 Powerline Fonts:   ${GREEN}[OK]${NC} Fira Code & Noto Color Emoji installed\n"
        else
            printf "  🔤 Powerline Fonts:   ${YELLOW}[WARN]${NC} Fira Code / Noto Color Emoji missing\n"
        fi
    else
        printf "  🔤 Powerline Fonts:   ${GRAY}[SKIP]${NC} fc-list not available\n"
    fi

    for sh in bash zsh fish; do
        local rc
        rc=$(get_target_rc "$sh")
        if [ -f "$rc" ]; then
            if grep -qF "fancybash init" "$rc" 2>/dev/null; then
                printf "  📄 Hook (%s):       ${GREEN}[OK]${NC} %s\n" "$sh" "$(tildify "$rc")"
            else
                printf "  📄 Hook (%s):       ${YELLOW}[MISSING]${NC} No fancybash hook in %s\n" "$sh" "$(tildify "$rc")"
            fi
        fi
    done

    printf "\n  ──────────────────────────────────────────────────\n"
    printf "  🎉 Doctor check completed.\n\n"
    exit 0
}

# ─── Rollback / Restoration Mode ───────────
run_rollback() {
    show_header
    printf "  🔄 ${BOLD}${YELLOW}FANCYBASH ROLLBACK & RESTORE${NC}\n"
    printf "  ──────────────────────────────────────────────────\n\n"

    remove_old_config

    local target_shells=()
    read -r -a target_shells <<< "$(detect_all_installed_shells)"

    local restored=false
    for sh in "${target_shells[@]}"; do
        local target_rc
        target_rc=$(get_target_rc "$sh")
        local latest_backup
        latest_backup=$(ls -t "${target_rc}.backup."* 2>/dev/null | head -n 1 || echo "")
        if [ -n "$latest_backup" ] && [ -f "$latest_backup" ]; then
            cp "$latest_backup" "$target_rc"
            restored=true
            printf "  ${GREEN}✔ Restored %s${NC} from ${PURPLE}%s${NC}\n" "$(tildify "$target_rc")" "$(basename "$latest_backup")"
        fi
    done

    if [ "$restored" = true ]; then
        printf "\n  🎉 Rollback completed successfully!\n\n"
    else
        printf "\n  ℹ Config blocks cleaned. No backup files found to restore.\n\n"
    fi
    exit 0
}

# ─── Reload & Summary ──────────────────────
show_summary() {
    local current_shell
    current_shell=$(detect_shell)
    local target_rc
    target_rc=$(get_target_rc "$current_shell")
    local reload_cmd

    case "$current_shell" in
        fish) reload_cmd="source ~/.config/fish/config.fish" ;;
        zsh)  reload_cmd="source ~/.zshrc" ;;
        *)    reload_cmd="source ~/.bashrc" ;;
    esac

    echo ""
    local reloaded=false
    case "$current_shell" in
        fish)
            if command -v fish >/dev/null 2>&1 && fish -c "source '$target_rc'" >/dev/null 2>&1; then
                reloaded=true
            fi
            ;;
        zsh)
            if command -v zsh >/dev/null 2>&1 && zsh -c "source '$target_rc'" >/dev/null 2>&1; then
                reloaded=true
            fi
            ;;
        *)
            if source "$target_rc" 2>/dev/null; then
                reloaded=true
            fi
            ;;
    esac

    if [ "$reloaded" = true ]; then
        printf "  ${GREEN}✨ Installation & auto-reload successful for %s!${NC}\n\n" "$current_shell"
    else
        printf "  ${YELLOW}⚠ Auto-reload skipped.${NC} Please run: ${BOLD}%s${NC}\n\n" "$reload_cmd"
    fi

    echo -e "\n${CYAN}──────────────────────────────────────────────────────────${NC}"
    echo -e " 🚀  ${BOLD}INSTALLATION SUMMARY${NC}"
    echo -e "${CYAN}──────────────────────────────────────────────────────────${NC}\n"
    printf "  🐚  ${BOLD}Active Shell:${NC}  ${CYAN}%s${NC}\n" "$current_shell"
    if [ ${#backup_files[@]} -gt 0 ]; then
        for b in "${backup_files[@]}"; do
            printf "  📦  ${BOLD}Backup:${NC}        ${GREEN}%s${NC}\n" "$(basename "$b")"
        done
    fi
    printf "  ⚙️   ${BOLD}Target Config:${NC} ${GREEN}%s${NC}\n" "$(tildify "$target_rc")"
    printf "  🔄  ${BOLD}Reload:${NC}        ${PURPLE}%s${NC}\n" "$reload_cmd"
    echo -e "${CYAN}──────────────────────────────────────────────────────────${NC}\n"
    echo -e "  🎉  ${BOLD}Installation complete!${NC}\n"
    echo ""
}

# ─── Main Execution ────────────────────────
main() {
    if [ "$MODE" = "doctor" ]; then
        run_doctor
    elif [ "$MODE" = "rollback" ]; then
        run_rollback
    fi

    show_header
    show_sysinfo

    draw_progress_bar 1 5
    if ! check_and_install_fonts; then
        exit 0
    fi

    draw_progress_bar 2 5
    setup_fontconfig

    draw_progress_bar 3 5
    setup_rust_binary

    draw_progress_bar 4 5
    check_existing_install
    remove_old_config
    backup_shell_rc

    draw_progress_bar 5 5
    install_config

    show_summary
}

main "$@"
