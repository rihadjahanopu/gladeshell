#!/usr/bin/env bash
# ==============================================================================
# 🚀 fancybash-rs — Universal Web Installer (Prebuilt Binary + Cargo Fallback)
# Usage: curl -fsSL https://fancybash.netlify.app/i.sh | bash
# ==============================================================================

set -e

# --- 🎨 Color Codes ---
BOLD="\033[1m"
CYAN="\033[38;5;51m"
GREEN="\033[38;5;82m"
YELLOW="\033[38;5;220m"
PINK="\033[38;5;213m"
PURPLE="\033[38;5;141m"
RED="\033[38;5;203m"
GRAY="\033[38;5;245m"
RESET="\033[0m"

echo -e "${CYAN}╔══════════════════════════════════════════════════════════════════════════╗${RESET}"
echo -e "${CYAN}║${RESET}  ${BOLD}${PINK}🚀  FANCYBASH-RS UNIVERSAL INSTALLER ${RESET}${CYAN}│${RESET} ${GRAY}Fast Rust Terminal Engine${RESET}  ${CYAN}║${RESET}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════════════════╝${RESET}\n"

# --- Destination directory ---
BIN_DIR="$HOME/.cargo/bin"
[ ! -d "$BIN_DIR" ] && BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"
export PATH="$BIN_DIR:$PATH"

REPO_OWNER="rihadjahanopu"
REPO_NAME="fancybash"
INSTALLED=0

# --- 1. Detect OS & Architecture ---
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$ARCH" in
    x86_64|amd64) TARGET_ARCH="x86_64" ;;
    aarch64|arm64) TARGET_ARCH="aarch64" ;;
    *) TARGET_ARCH="$ARCH" ;;
esac

case "$OS" in
    linux) TARGET_OS="unknown-linux-gnu" ;;
    darwin) TARGET_OS="apple-darwin" ;;
    *) TARGET_OS="$OS" ;;
esac

TARGET_TRIPLE="${TARGET_ARCH}-${TARGET_OS}"
RELEASE_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/latest/download/fancybash-${TARGET_TRIPLE}.tar.gz"

# --- 2. Try Instant Prebuilt Binary Download ---
echo -e "${CYAN}🔍 Checking prebuilt release binary for [${TARGET_TRIPLE}]...${RESET}"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

if curl -fsSL "$RELEASE_URL" -o "$TMP_DIR/fancybash.tar.gz" 2>/dev/null; then
    echo -e "${GREEN}📦 Downloading prebuilt binary (Instant install)...${RESET}"
    tar -xzf "$TMP_DIR/fancybash.tar.gz" -C "$TMP_DIR"
    if [ -f "$TMP_DIR/fancybash" ]; then
        mv "$TMP_DIR/fancybash" "$BIN_DIR/fancybash"
        chmod +x "$BIN_DIR/fancybash"
        INSTALLED=1
        echo -e "${GREEN}⚡ Prebuilt binary installed instantly!${RESET}"
    fi
fi

# --- 3. Fallback: Local / Remote Cargo Build if Prebuilt is unavailable ---
if [ $INSTALLED -eq 0 ]; then
    echo -e "${YELLOW}ℹ️ Prebuilt binary not found for target. Falling back to Cargo build...${RESET}"

    if ! command -v cargo &>/dev/null; then
        echo -e "${YELLOW}⚡ Cargo/Rust toolchain not found. Installing Rust...${RESET}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env" 2>/dev/null || true
        export PATH="$HOME/.cargo/bin:$PATH"
    fi

    if command -v cargo &>/dev/null; then
        echo -e "${CYAN}🛠️ Compiling fancybash via Cargo...${RESET}"
        if [ -f "./Cargo.toml" ]; then
            cargo install --path . --force
        else
            cargo install --git "https://github.com/${REPO_OWNER}/${REPO_NAME}" --force
        fi
        INSTALLED=1
    else
        echo -e "${RED}❌ Error: Rust installation failed. Please install Rust: https://rustup.rs${RESET}"
        exit 1
    fi
fi

if ! command -v fancybash &>/dev/null && [ ! -f "$BIN_DIR/fancybash" ]; then
    echo -e "${RED}❌ Installation failed!${RESET}"
    exit 1
fi

# --- 4. Shell Integration & Auto-Injection ---
DETECTED_SHELL=$(basename "${SHELL:-bash}")
CONFIG_FILE=""
INIT_LINE=""

case "$DETECTED_SHELL" in
    zsh)
        CONFIG_FILE="$HOME/.zshrc"
        INIT_LINE='eval "$(fancybash init zsh)"'
        ;;
    bash)
        CONFIG_FILE="$HOME/.bashrc"
        INIT_LINE='eval "$(fancybash init bash)"'
        ;;
    fish)
        CONFIG_FILE="$HOME/.config/fish/config.fish"
        INIT_LINE='fancybash init fish | source'
        mkdir -p "$HOME/.config/fish"
        ;;
    *)
        CONFIG_FILE="$HOME/.bashrc"
        INIT_LINE='eval "$(fancybash init bash)"'
        ;;
esac

if [ -f "$CONFIG_FILE" ]; then
    if ! grep -q "fancybash init" "$CONFIG_FILE" 2>/dev/null; then
        echo -e "\n# fancybash shell initialization" >> "$CONFIG_FILE"
        echo "$INIT_LINE" >> "$CONFIG_FILE"
        echo -e "${GREEN}✨ Added initialization to: ${CONFIG_FILE}${RESET}"
    else
        echo -e "${GRAY}ℹ️ Initialization already present in: ${CONFIG_FILE}${RESET}"
    fi
else
    touch "$CONFIG_FILE" 2>/dev/null || true
    echo "$INIT_LINE" >> "$CONFIG_FILE"
    echo -e "${GREEN}✨ Created and configured: ${CONFIG_FILE}${RESET}"
fi

# --- 5. Success Banner ---
echo -e "\n${GREEN}══════════════════════════════════════════════════════════════════════════${RESET}"
echo -e "${BOLD}${GREEN}🎉 fancybash-rs installed successfully!${RESET}"
echo -e "${CYAN}💡 To activate immediately in your current terminal, run:${RESET}"

if [ "$DETECTED_SHELL" = "zsh" ]; then
    echo -e "   ${YELLOW}source ~/.zshrc${RESET}"
elif [ "$DETECTED_SHELL" = "fish" ]; then
    echo -e "   ${YELLOW}source ~/.config/fish/config.fish${RESET}"
else
    echo -e "   ${YELLOW}source ~/.bashrc${RESET}"
fi

echo -e "${GREEN}══════════════════════════════════════════════════════════════════════════${RESET}\n"
