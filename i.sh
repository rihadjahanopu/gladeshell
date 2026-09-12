#!/bin/bash

# ─── fancybash Universal Dynamic Installer (i.sh) ────────────────────────────
# Automatically detects OS, Linux Distro, Architecture, and active Shell
# (Zsh, Bash, Fish, PowerShell) and runs the appropriate installer.
# ──────────────────────────────────────────────────────────────────────────────

set -euo pipefail

REPO_BASE_URL="https://raw.githubusercontent.com/rihadjahanopu/fancybash-rs/refs/heads/main"
ALT_REPO_BASE_URL="https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main"
FALLBACK_BASE_URL="https://fancybash.netlify.app/public"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" 2>/dev/null && pwd || echo "$PWD")"

# ─── Colors ───────────────────────────────────────────────────────────────────
RED='\033[38;2;243;139;168m'
GREEN='\033[38;2;166;227;161m'
YELLOW='\033[38;2;249;226;175m'
BLUE='\033[38;2;137;180;250m'
PURPLE='\033[38;2;203;166;247m'
CYAN='\033[38;2;148;226;213m'
BOLD='\033[1m'
NC='\033[0m'

printf "\n${BOLD}${PURPLE}⚡ fancybash Universal Installer${NC}\n"
printf "${CYAN}──────────────────────────────────────────${NC}\n\n"

# ─── 1. Detect OS & Architecture ─────────────────────────────────────────────
OS_TYPE="$(uname -s 2>/dev/null || echo "Unknown")"
ARCH_TYPE="$(uname -m 2>/dev/null || echo "Unknown")"
DISTRO_NAME=""

case "$OS_TYPE" in
    Linux*)
        OS="Linux"
        if [ -f /etc/os-release ]; then
            DISTRO_NAME="$(grep -E '^PRETTY_NAME=' /etc/os-release | cut -d= -f2 | tr -d '"')"
        elif command -v lsb_release &>/dev/null; then
            DISTRO_NAME="$(lsb_release -d -s)"
        fi
        [ -z "$DISTRO_NAME" ] && DISTRO_NAME="Generic Linux"
        ;;
    Darwin*)
        OS="macOS"
        DISTRO_NAME="macOS $(sw_vers -productVersion 2>/dev/null || echo "")"
        ;;
    CYGWIN*|MINGW*|MSYS*|Windows_NT*)
        OS="Windows"
        DISTRO_NAME="Windows (CLI)"
        ;;
    *)
        OS="Unknown"
        DISTRO_NAME="Unknown OS"
        ;;
esac

printf "  ${CYAN}➜ System:${NC}       ${BOLD}%s${NC} (%s)\n" "$OS" "$DISTRO_NAME"
printf "  ${CYAN}➜ Architecture:${NC} ${BOLD}%s${NC}\n" "$ARCH_TYPE"

# ─── 2. Detect Target Shell ──────────────────────────────────────────────────
# Bulletproof priority:
#   1. Parent process (PPID) — detects the ACTUAL shell that launched this script
#   2. $SHELL env var        — user's configured login shell (fallback)
#   3. Version vars          — $ZSH_VERSION / $BASH_VERSION / $FISH_VERSION
#   4. Binary/OS fallback    — last resort
USER_SHELL=""

# ── 1st: Parent process check ─────────────────────────────────────────────────
# $SHELL can lie (e.g. login shell is zsh but user runs: bash i.sh).
# Checking PPID gives us the real shell that spawned this script.
PARENT_CMD=""
if [ -n "${PPID:-}" ]; then
    # Try `ps` first — most portable across Linux/macOS
    PARENT_CMD=$(ps -p "$PPID" -o comm= 2>/dev/null | sed 's/^-//' | xargs basename 2>/dev/null)
    # Fallback: /proc filesystem (Linux only)
    if [ -z "$PARENT_CMD" ] && [ -f "/proc/$PPID/comm" ]; then
        PARENT_CMD=$(sed 's/^-//' "/proc/$PPID/comm" 2>/dev/null)
    fi
fi

case "${PARENT_CMD:-}" in
    zsh)  USER_SHELL="zsh"  ;;
    bash) USER_SHELL="bash" ;;
    fish) USER_SHELL="fish" ;;
esac

# ── 2nd: $SHELL env var ───────────────────────────────────────────────────────
# Used when PPID check is inconclusive (e.g. script piped via curl | sh)
if [ -z "$USER_SHELL" ] && [ -n "${SHELL:-}" ]; then
    case "$(basename "$SHELL")" in
        zsh)  USER_SHELL="zsh"  ;;
        bash) USER_SHELL="bash" ;;
        fish) USER_SHELL="fish" ;;
    esac
fi

# ── 3rd: Version vars ─────────────────────────────────────────────────────────
# Useful when $SHELL is unset or points to a generic /bin/sh
if [ -z "$USER_SHELL" ]; then
    if [ -n "${ZSH_VERSION:-}" ]; then
        USER_SHELL="zsh"
    elif [ -n "${BASH_VERSION:-}" ]; then
        USER_SHELL="bash"
    elif [ -n "${FISH_VERSION:-}" ]; then
        USER_SHELL="fish"
    fi
fi

# ── 4th: Binary / OS fallback ────────────────────────────────────────────────
if [ -z "$USER_SHELL" ]; then
    if command -v zsh &>/dev/null && [ "$OS" = "macOS" ]; then
        USER_SHELL="zsh"   # macOS default shell since Catalina
    else
        USER_SHELL="bash"
    fi
fi

printf "  ${CYAN}➜ Shell:${NC}        ${BOLD}%s${NC}\n" "$USER_SHELL"
printf "${CYAN}──────────────────────────────────────────${NC}\n\n"

# ─── 3. Delegate Installation ────────────────────────────────────────────────
# Save original script args BEFORE the function (inside function $@ = func args)
SCRIPT_ARGS=(${@:+"$@"})

# Export detected shell so install.sh/install.zsh can display it correctly.
# (PPID-based re-detection inside install.sh fails because exec inherits
#  the terminal's PPID, not i.sh's PID.)
export FANCYBASH_SHELL="$USER_SHELL"

run_installer() {
    local target_script="$1"
    local local_file="$SCRIPT_DIR/$target_script"

    # If running locally and file exists
    if [ -f "$local_file" ]; then
        printf "  ${GREEN}✔${NC} Running local ${BOLD}%s${NC}...\n\n" "$target_script"
        if [ "$target_script" = "install.zsh" ] && command -v zsh &>/dev/null; then
            exec zsh "$local_file" "${SCRIPT_ARGS[@]}"
        elif [ "$target_script" = "install.fish" ] && command -v fish &>/dev/null; then
            exec fish "$local_file" "${SCRIPT_ARGS[@]}"
        else
            exec bash "$local_file" "${SCRIPT_ARGS[@]}"
        fi
    else
        # Running remotely via curl pipe (GitHub Primary ➔ Website Fallback)
        local primary_url="$REPO_BASE_URL/$target_script"
        local alt_url="$ALT_REPO_BASE_URL/$target_script"
        local fallback_url="$FALLBACK_BASE_URL/$target_script"
        printf "  ${GREEN}✔${NC} Fetching and running ${BOLD}%s${NC}...\n\n" "$target_script"
        
        local tmp_installer
        tmp_installer="$(mktemp "${TMPDIR:-/tmp}/fancybash_install.XXXXXX")"
        chmod +x "$tmp_installer"
        trap 'rm -f "$tmp_installer"' SIGINT SIGTERM

        local fetch_success=false
        if command -v curl &>/dev/null; then
            if curl -fsSL --connect-timeout 5 "$primary_url" -o "$tmp_installer" 2>/dev/null && [ -s "$tmp_installer" ] && ! head -1 "$tmp_installer" | grep -qi "<!doctype\|<html"; then
                fetch_success=true
            elif curl -fsSL --connect-timeout 5 "$alt_url" -o "$tmp_installer" 2>/dev/null && [ -s "$tmp_installer" ] && ! head -1 "$tmp_installer" | grep -qi "<!doctype\|<html"; then
                fetch_success=true
            else
                printf "  ${YELLOW}⚠ Primary source unavailable. Switching to Website fallback...${NC}\n"
                if curl -fsSL --connect-timeout 10 "$fallback_url" -o "$tmp_installer" 2>/dev/null && [ -s "$tmp_installer" ]; then
                    fetch_success=true
                fi
            fi
        elif command -v wget &>/dev/null; then
            if wget -qO "$tmp_installer" --timeout=5 "$primary_url" 2>/dev/null && [ -s "$tmp_installer" ] && ! head -1 "$tmp_installer" | grep -qi "<!doctype\|<html"; then
                fetch_success=true
            elif wget -qO "$tmp_installer" --timeout=5 "$alt_url" 2>/dev/null && [ -s "$tmp_installer" ] && ! head -1 "$tmp_installer" | grep -qi "<!doctype\|<html"; then
                fetch_success=true
            else
                printf "  ${YELLOW}⚠ Primary source unavailable. Switching to Website fallback...${NC}\n"
                if wget -qO "$tmp_installer" --timeout=10 "$fallback_url" 2>/dev/null && [ -s "$tmp_installer" ]; then
                    fetch_success=true
                fi
            fi
        else
            printf "${RED}❌ Error: Neither curl nor wget was found on your system.${NC}\n" >&2
            exit 1
        fi

        if [ "$fetch_success" = false ] || [ ! -s "$tmp_installer" ]; then
            printf "${RED}❌ Error: Could not download installer from GitHub or Fallback Website.${NC}\n" >&2
            exit 1
        fi

        if [ "$target_script" = "install.zsh" ] && command -v zsh &>/dev/null; then
            exec zsh "$tmp_installer" "${SCRIPT_ARGS[@]}"
        elif [ "$target_script" = "install.fish" ] && command -v fish &>/dev/null; then
            exec fish "$tmp_installer" "${SCRIPT_ARGS[@]}"
        else
            exec bash "$tmp_installer" "${SCRIPT_ARGS[@]}"
        fi
    fi
}

case "$USER_SHELL" in
    zsh)
        run_installer "install.zsh" "$@"
        ;;
    fish)
        run_installer "install.fish" "$@"
        ;;
    bash)
        run_installer "install.sh" "$@"
        ;;
    *)
        if [ "$OS" = "Windows" ]; then
            printf "  ${YELLOW}⚠ Windows PowerShell installation detected.${NC}\n"
            printf "  Please run the following command in PowerShell:\n\n"
            printf "  ${BOLD}irm %s/i.ps1 | iex${NC}\n\n" "$FALLBACK_BASE_URL"
            exit 0
        else
            run_installer "install.sh" "$@"
        fi
        ;;
esac
