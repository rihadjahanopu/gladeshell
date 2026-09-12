#!/usr/bin/env bash
# =============================================================================
# setup.sh — fancybash contributor setup script (Bash)
# Run this once after cloning the repo:  bash setup.sh
# =============================================================================

set -euo pipefail

# ── colours ──────────────────────────────────────────────────────────────────
if [[ -t 1 ]]; then
  GREEN='\033[0;32m'
  YELLOW='\033[1;33m'
  CYAN='\033[0;36m'
  RED='\033[0;31m'
  RESET='\033[0m'
else
  GREEN='' YELLOW='' CYAN='' RED='' RESET=''
fi

info()    { printf "${CYAN}ℹ  %s${RESET}\n"  "$*"; }
success() { printf "${GREEN}✔  %s${RESET}\n" "$*"; }
warn()    { printf "${YELLOW}⚠  %s${RESET}\n" "$*"; }
error()   { printf "${RED}✖  %s${RESET}\n"  "$*" >&2; }

# ── sanity check ─────────────────────────────────────────────────────────────
if ! git rev-parse --git-dir &>/dev/null; then
  error "Not inside a Git repository. Please clone the project first."
  exit 1
fi

printf "\n${CYAN}══════════════════════════════════════════${RESET}\n"
printf "${CYAN}   fancybash — contributor setup          ${RESET}\n"
printf "${CYAN}══════════════════════════════════════════${RESET}\n\n"

# ── 1. Git hooks ─────────────────────────────────────────────────────────────
info "Configuring Git hooks..."

git config core.hooksPath .githooks
chmod +x .githooks/*

success "Git hooks configured  (core.hooksPath → .githooks)"

# ── done ─────────────────────────────────────────────────────────────────────
printf "\n${GREEN}✔  Setup complete! Happy contributing 🎉${RESET}\n\n"
