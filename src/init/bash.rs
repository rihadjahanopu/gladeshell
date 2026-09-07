// =============================================================================
//  src/init/bash.rs — Bash bootstrap code generator
//
//  Generates valid Bash 4.x+ syntax.  The output is meant to be eval'd:
//    eval "$(fancybash init bash)"
//
//  Hook strategy:
//    • PROMPT_COMMAND — fires before each prompt render (replaces precmd).
//    • trap DEBUG      — fires before each command (cmd-duration tracking).
//    • PS1             — set from the lib's render output via $(__fb_prompt).
// =============================================================================

use crate::core::{aliases::Shell, env};
use super::header_comment;

pub fn generate() -> String {
    let mut out = String::with_capacity(8192);

    out.push_str(&header_comment("#"));

    // ── Guard: skip re-sourcing ───────────────────────────────────────────────
    out.push_str(r#"
# Guard against double-sourcing
if [[ -n "${_FANCYBASH_BASH_LOADED:-}" ]]; then
    return 0 2>/dev/null || true
fi
export _FANCYBASH_BASH_LOADED=1
"#);

    // ── Environment variables ─────────────────────────────────────────────────
    out.push_str("\n# ── Environment variables ──\n");
    for var in env::canonical_env_vars() {
        out.push_str(&env::render_env_bash(&var));
        out.push('\n');
    }

    // ── PATH entries ─────────────────────────────────────────────────────────
    out.push_str("\n# ── PATH ──\n");
    for entry in env::canonical_path_entries() {
        out.push_str(&env::render_path_bash(&entry));
        out.push('\n');
    }

    // ── NVM lazy-load ─────────────────────────────────────────────────────────
    out.push_str(r#"
# ── NVM lazy-load (zero startup cost) ──
export NVM_DIR="${NVM_DIR:-$HOME/.config/nvm}"
[[ ! -d "$NVM_DIR" && -d "$HOME/.nvm" ]] && export NVM_DIR="$HOME/.nvm"

_fb_lazy_load_nvm() {
    unset -f nvm node npm npx
    [[ -s "$NVM_DIR/nvm.sh" ]] && \. "$NVM_DIR/nvm.sh"
    [[ -s "$NVM_DIR/bash_completion" ]] && \. "$NVM_DIR/bash_completion"
}
nvm()  { _fb_lazy_load_nvm; nvm  "$@"; }
node() { _fb_lazy_load_nvm; node "$@"; }
npm()  { _fb_lazy_load_nvm; npm  "$@"; }
npx()  { _fb_lazy_load_nvm; npx  "$@"; }
"#);

    // ── Bun ───────────────────────────────────────────────────────────────────
    out.push_str(r#"
# ── Bun completions (lazy) ──
[[ -s "$BUN_INSTALL/_bun" ]] && \. "$BUN_INSTALL/_bun"
"#);

    // ── Aliases ───────────────────────────────────────────────────────────────
    out.push_str("\n# ── Aliases ──\n");
    out.push_str(&crate::core::aliases::AliasFile::from_toml(
        include_str!("../../aliases.toml"),
    )
    .map(|af| af.render(Shell::Bash))
    .unwrap_or_else(|e| format!("# aliases.toml parse error: {e}\n")));

    // ── Prompt helper functions ───────────────────────────────────────────────
    out.push_str(r#"
# ── fancybash prompt helpers ──

# Git branch (reads from cached file — no fork at prompt time)
_fb_git_cache="/tmp/.fb_git_$$"
_fb_update_git() {
    (
        local b
        b=$(git branch --show-current 2>/dev/null) || b=$(git rev-parse --short HEAD 2>/dev/null)
        if [[ -n "$b" ]]; then
            local dirty=""
            [[ -n "$(git status --porcelain --untracked-files=no 2>/dev/null)" ]] && dirty=" ❗"
            printf ' [🌿 %s%s]' "$b" "$dirty" > "$_fb_git_cache"
        else
            rm -f "$_fb_git_cache"
        fi
    ) &>/dev/null &
    disown
}

_fb_git_segment() {
    [[ -f "$_fb_git_cache" ]] && cat "$_fb_git_cache" 2>/dev/null
}

# Command duration tracking
_fb_timer_start=0
_fb_cmd_duration=""
trap '_fb_timer_start=$SECONDS' DEBUG

# Prompt renderer
__fb_prompt() {
    local exit_code=$?
    PS1=$(fancybash prompt --cwd "$PWD" --exit-code "$exit_code" --user "$USER" --host "$HOSTNAME" 2>/dev/null)
}

PROMPT_COMMAND="__fb_prompt"
"#);

    // ── Shell options ─────────────────────────────────────────────────────────
    out.push_str(r#"
# ── Shell options ──
shopt -s histappend checkwinsize globstar autocd
HISTSIZE=50000
HISTFILESIZE=100000
HISTCONTROL=ignoreboth:erasedups

# ── Autocompletion ──
if [[ -f /usr/share/bash-completion/bash_completion ]]; then
    \. /usr/share/bash-completion/bash_completion
elif [[ -f /etc/bash_completion ]]; then
    \. /etc/bash_completion
fi
"#);

    out.push_str("\n# fancybash bash init complete\n");
    out
}
