#!/bin/bash

# ─── fancybash Universal Uninstaller (u.sh) ──────────────────────────────────
# 1. Runs native Rust self-uninstaller `fancybash uninstall` if available
# 2. Cleans fancybash blocks from shell configs (.zshrc, .bashrc, config.fish)
# 3. Removes binary from ~/.local/bin and ~/.cargo/bin
# ──────────────────────────────────────────────────────────────────────────────

set -euo pipefail

REPO_BASE_URL="https://raw.githubusercontent.com/rihadjahanopu/fancybash-rs/refs/heads/main"
ALT_REPO_BASE_URL="https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main"
FALLBACK_BASE_URL="https://fancybash.netlify.app/public"

# ─── Colors ───────────────────────────────────────────────────────────────────
RED='\033[38;2;243;139;168m'
GREEN='\033[38;2;166;227;161m'
YELLOW='\033[38;2;249;226;175m'
CYAN='\033[38;2;148;226;213m'
PURPLE='\033[38;2;203;166;247m'
BOLD='\033[1m'
NC='\033[0m'

printf "\n${BOLD}${PURPLE}🗑  fancybash Universal Uninstaller${NC}\n"
printf "${CYAN}──────────────────────────────────────────${NC}\n\n"

# ─── 1. Try Native Rust Binary Uninstallation First ──────────────────────────
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"

if command -v fancybash &>/dev/null; then
    printf "  ${GREEN}✔${NC} Found fancybash binary. Running native self-uninstaller...\n\n"
    fancybash uninstall
    exit 0
fi

# ─── 2. Fallback: Shell Configuration Cleanup ────────────────────────────────
printf "  ${CYAN}➜${NC} Cleaning fancybash configurations from shell rc files...\n"

clean_file() {
    local file="$1"
    if [ -f "$file" ]; then
        if grep -qF "# >>> fancy-" "$file" 2>/dev/null || grep -qF "fancybash" "$file" 2>/dev/null; then
            if [ "$(uname)" = "Darwin" ]; then
                sed -i '' '/# >>> fancy-/,/# <<< fancy-/d' "$file" 2>/dev/null || true
                sed -i '' '/fancybash init/d' "$file" 2>/dev/null || true
            else
                sed -i '/# >>> fancy-/,/# <<< fancy-/d' "$file" 2>/dev/null || true
                sed -i '/fancybash init/d' "$file" 2>/dev/null || true
            fi
            printf "  ${GREEN}✔${NC} Cleaned config in: ${BOLD}%s${NC}\n" "$file"
        fi
    fi
}

clean_file "$HOME/.zshrc"
clean_file "$HOME/.bashrc"
clean_file "$HOME/.config/fish/config.fish"

# Remove binaries
rm -f "$HOME/.local/bin/fancybash" "$HOME/.cargo/bin/fancybash" 2>/dev/null || true

printf "\n  ${GREEN}🎉 fancybash uninstalled successfully!${NC}\n"
printf "  ${CYAN}💡 Please restart your shell or terminal for changes to take effect.${NC}\n\n"
