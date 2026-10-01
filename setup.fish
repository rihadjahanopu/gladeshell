#!/usr/bin/env fish
# =============================================================================
# setup.fish — gladeshell contributor setup script (Fish)
# Run this once after cloning the repo:  fish setup.fish
# =============================================================================

# ── colours ──────────────────────────────────────────────────────────────────
function _info;    set_color cyan;   echo "ℹ  $argv"; set_color normal; end
function _success; set_color green;  echo "✔  $argv"; set_color normal; end
function _warn;    set_color yellow; echo "⚠  $argv"; set_color normal; end
function _error;   set_color red;    echo "✖  $argv" >&2; set_color normal; end

# ── sanity check ─────────────────────────────────────────────────────────────
if not git rev-parse --git-dir &>/dev/null
  _error "Not inside a Git repository. Please clone the project first."
  exit 1
end

set_color cyan
echo ""
echo "══════════════════════════════════════════"
echo "   gladeshell — contributor setup          "
echo "══════════════════════════════════════════"
echo ""
set_color normal

# ── 1. Git hooks ─────────────────────────────────────────────────────────────
_info "Configuring Git hooks..."

git config core.hooksPath .githooks
chmod +x .githooks/*

_success "Git hooks configured  (core.hooksPath → .githooks)"

# ── done ─────────────────────────────────────────────────────────────────────
echo ""
set_color green
echo "✔  Setup complete! Happy contributing 🎉"
set_color normal
echo ""
