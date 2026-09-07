#!/usr/bin/env bash
# ==============================================================================
# 🗑️ fancybash-rs — Universal Web Uninstaller
# Usage: curl -fsSL https://fancybash.netlify.app/u.sh | bash
# ==============================================================================

set -e

BOLD="\033[1m"
CYAN="\033[38;5;51m"
GREEN="\033[38;5;82m"
YELLOW="\033[38;5;220m"
RED="\033[38;5;203m"
GRAY="\033[38;5;245m"
RESET="\033[0m"

echo -e "${CYAN}╔══════════════════════════════════════════════════════════════════════════╗${RESET}"
echo -e "${CYAN}║${RESET}  ${BOLD}${RED}🗑️  FANCYBASH-RS UNINSTALLER ${RESET}${CYAN}│${RESET} ${GRAY}Removing fancybash configurations${RESET}  ${CYAN}║${RESET}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════════════════╝${RESET}\n"

# Remove shell integration lines
FILES=(
  "$HOME/.bashrc"
  "$HOME/.zshrc"
  "$HOME/.config/fish/config.fish"
)

for file in "${FILES[@]}"; do
  if [ -f "$file" ]; then
    sed -i '/# >>> fancy-.* >>>/,/# <<< fancy-.* <<</d' "$file" 2>/dev/null || true
    sed -i '/# fancybash shell initialization/d' "$file" 2>/dev/null || true
    sed -i '/fancybash init/d' "$file" 2>/dev/null || true
    echo -e "${GREEN}✅ Cleaned fancybash block from: ${file}${RESET}"
  fi
done

# Remove binary if present in user bin directories
for bin in "$HOME/.cargo/bin/fancybash" "$HOME/.local/bin/fancybash"; do
  if [ -f "$bin" ]; then
    rm -f "$bin"
    echo -e "${GREEN}✅ Removed binary: ${bin}${RESET}"
  fi
done

echo -e "\n${BOLD}${GREEN}🎉 fancybash-rs uninstalled successfully!${RESET}"
echo -e "${GRAY}Your original shell configuration has been restored.${RESET}\n"
