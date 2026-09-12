#!/usr/bin/env zsh
# =============================================================================
# setup.zsh — fancybash contributor setup script (Zsh)
# Run this once after cloning the repo:  zsh setup.zsh
# =============================================================================

setopt ERR_EXIT PIPE_FAIL NO_UNSET

# ── colours ──────────────────────────────────────────────────────────────────
if [[ -t 1 ]]; then
  GREEN=$'\033[0;32m'
  YELLOW=$'\033[1;33m'
  CYAN=$'\033[0;36m'
  RED=$'\033[0;31m'
  RESET=$'\033[0m'
else
  GREEN='' YELLOW='' CYAN='' RED='' RESET=''
fi

info()    { print "${CYAN}ℹ  $*${RESET}"; }
success() { print "${GREEN}✔  $*${RESET}"; }
warn()    { print "${YELLOW}⚠  $*${RESET}"; }
error()   { print "${RED}✖  $*${RESET}" >&2; }

# ── sanity check ─────────────────────────────────────────────────────────────
if ! git rev-parse --git-dir &>/dev/null; then
  error "Not inside a Git repository. Please clone the project first."
  exit 1
fi

print "\n${CYAN}══════════════════════════════════════════${RESET}"
print "${CYAN}   fancybash — contributor setup          ${RESET}"
print "${CYAN}══════════════════════════════════════════${RESET}\n"

# ── 1. Git hooks ─────────────────────────────────────────────────────────────
info "Configuring Git hooks..."

git config core.hooksPath .githooks
chmod +x .githooks/*

success "Git hooks configured  (core.hooksPath → .githooks)"

# ── done ─────────────────────────────────────────────────────────────────────
print "\n${GREEN}✔  Setup complete! Happy contributing 🎉${RESET}\n"
