// =============================================================================
//  src/init/zsh.rs — Zsh bootstrap code generator
//
//  Generates valid Zsh 5.x+ syntax.  Eval'd via:
//    eval "$(fancybash init zsh)"
//  or added to .zshrc as:
//    source <(fancybash init zsh)
//
//  Hook strategy:
//    • precmd()     — called before each prompt render.
//    • preexec()    — called before each command (cmd-duration).
//    • PROMPT / PS1 — rendered in precmd using PROMPT_SUBST.
//    • zmodload     — Phase 2 will load libfancybash_core.so directly here.
// =============================================================================

use crate::core::{aliases::Shell, env};
use super::header_comment;

pub fn generate() -> String {
    let mut out = String::with_capacity(8192);

    out.push_str(&header_comment("#"));

    // ── Guard ────────────────────────────────────────────────────────────────
    out.push_str(r#"
# Guard against double-sourcing
[[ -n "${_FANCYBASH_ZSH_LOADED:-}" ]] && return 0
typeset -g _FANCYBASH_ZSH_LOADED=1
"#);

    // ── Core zsh options ─────────────────────────────────────────────────────
    out.push_str(r#"
# ── Core Zsh options ──
setopt PROMPT_SUBST
setopt AUTO_CD EXTENDED_GLOB HIST_IGNORE_DUPS HIST_IGNORE_ALL_DUPS
setopt SHARE_HISTORY INC_APPEND_HISTORY HIST_REDUCE_BLANKS
setopt AUTO_LIST AUTO_MENU COMPLETE_IN_WORD ALWAYS_TO_END
setopt NO_BEEP

HISTSIZE=50000
SAVEHIST=50000
HISTFILE="$HOME/.zsh_history"
"#);

    // ── Autocompletion engine ─────────────────────────────────────────────────
    out.push_str(r#"
# ── Zsh autocompletion engine ──
autoload -Uz compinit bashcompinit
typeset -U fpath
local -a _fb_fpaths=(
    "$HOME/.zsh/zsh-completions/src"
    "$HOME/.zsh/completion"
    "/usr/local/share/zsh/site-functions"
    "/usr/share/zsh/site-functions"
    "/usr/share/zsh/vendor-completions"
)
local _fb_fp
for _fb_fp in "${_fb_fpaths[@]}"; do
    [[ -d "$_fb_fp" ]] && fpath=("$_fb_fp" $fpath)
done
unset _fb_fpaths _fb_fp

# Fast compinit: skip rebuild if dump is fresh (< 24 h old)
local _zcd="${ZSH_COMPDUMP:-$HOME/.zcompdump}"
if [[ ! -f "$_zcd" || ! -s "$_zcd" || -n ${_zcd}(#qN.m+1) ]]; then
    compinit -i -d "$_zcd"
else
    compinit -i -C -d "$_zcd"
fi
bashcompinit 2>/dev/null || true
unset _zcd

# Async recompile (non-blocking)
[[ -f "$_zcd" && (! -f "${_zcd}.zwc" || "$_zcd" -nt "${_zcd}.zwc") ]] && \
    (zcompile -R "${_zcd}.zwc" "$_zcd" 2>/dev/null &!)

zstyle ':completion:*' menu select
zstyle ':completion:*' list-colors "${(s.:.)LS_COLORS}"
zstyle ':completion:*' matcher-list \
    'm:{a-zA-Z}={A-Za-z}' 'r:|[._-]=* r:|=*' 'l:|=* r:|=*'
zstyle ':completion:*' rehash true
"#);

    // ── Plugin loading ────────────────────────────────────────────────────────
    out.push_str(r#"
# ── Plugins (multi-distro paths) ──
_fb_source_first() {
    local f
    for f in "$@"; do
        if [[ -f "$f" ]]; then
            source "$f" 2>/dev/null
            return 0
        fi
    done
}

# zsh-autosuggestions
_fb_source_first \
    "$HOME/.zsh/zsh-autosuggestions/zsh-autosuggestions.zsh" \
    "/usr/share/zsh-autosuggestions/zsh-autosuggestions.zsh" \
    "/usr/share/zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh" \
    "/opt/homebrew/share/zsh-autosuggestions/zsh-autosuggestions.zsh"
ZSH_AUTOSUGGEST_USE_ASYNC=true

# zsh-syntax-highlighting (must be LAST plugin)
_fb_source_first \
    "$HOME/.zsh/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh" \
    "/usr/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh" \
    "/usr/share/zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh" \
    "/opt/homebrew/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"

unfunction _fb_source_first 2>/dev/null || true
"#);

    // ── Environment variables ─────────────────────────────────────────────────
    out.push_str("\n# ── Environment variables ──\n");
    for var in env::canonical_env_vars() {
        out.push_str(&env::render_env_bash(&var)); // zsh uses same syntax
        out.push('\n');
    }

    // ── PATH entries ─────────────────────────────────────────────────────────
    out.push_str("\n# ── PATH ──\n");
    for entry in env::canonical_path_entries() {
        out.push_str(&env::render_path_bash(&entry)); // same syntax as bash
        out.push('\n');
    }

    // ── NVM lazy-load ─────────────────────────────────────────────────────────
    out.push_str(r#"
# ── NVM lazy-load ──
export NVM_DIR="${NVM_DIR:-$HOME/.config/nvm}"
[[ ! -d "$NVM_DIR" && -d "$HOME/.nvm" ]] && export NVM_DIR="$HOME/.nvm"

_fb_lazy_load_nvm() {
    unset -f nvm node npm npx 2>/dev/null
    [[ -s "$NVM_DIR/nvm.sh" ]] && \. "$NVM_DIR/nvm.sh"
    [[ -s "$NVM_DIR/bash_completion" ]] && {
        autoload -Uz bashcompinit 2>/dev/null
        bashcompinit 2>/dev/null || true
        \. "$NVM_DIR/bash_completion"
    }
}
nvm()  { _fb_lazy_load_nvm; nvm  "$@"; }
node() { _fb_lazy_load_nvm; node "$@"; }
npm()  { _fb_lazy_load_nvm; npm  "$@"; }
npx()  { _fb_lazy_load_nvm; npx  "$@"; }
"#);

    // ── Bun ───────────────────────────────────────────────────────────────────
    out.push_str(r#"
# ── Bun ──
export BUN_INSTALL="${BUN_INSTALL:-$HOME/.bun}"
[[ -d "$BUN_INSTALL/bin" && ":$PATH:" != *":$BUN_INSTALL/bin:"* ]] && \
    export PATH="$BUN_INSTALL/bin:$PATH"
[[ -s "$BUN_INSTALL/_bun" ]] && source "$BUN_INSTALL/_bun" 2>/dev/null
"#);

    // ── Aliases ───────────────────────────────────────────────────────────────
    out.push_str("\n# ── Aliases ──\n");
    out.push_str(&crate::core::aliases::AliasFile::builtin().render(Shell::Zsh));

    // ── Git cache & prompt functions ──────────────────────────────────────────
    out.push_str(r#"
# ── fancybash prompt engine ──

typeset -g _fb_git_cache_file="/tmp/.fb_git_cache_${USER}_$$"

_fb_update_git_async() {
    (
        local branch dirty=""
        branch=$(git branch --show-current 2>/dev/null) || \
            branch=$(git rev-parse --short HEAD 2>/dev/null)
        if [[ -n "$branch" ]]; then
            [[ -n $(git status --porcelain --untracked-files=no 2>/dev/null) ]] && dirty=" ❗"
            printf ' [🌿 %s%s]' "$branch" "$dirty" > "$_fb_git_cache_file"
        else
            rm -f "$_fb_git_cache_file" 2>/dev/null
        fi
    ) &>/dev/null &!
}

_fb_git_segment() {
    [[ -f "$_fb_git_cache_file" ]] && cat "$_fb_git_cache_file" 2>/dev/null
}

# ── Command duration tracking ──
typeset -g _fb_timer
typeset -g _fb_cmd_duration=""

_fb_preexec() { _fb_timer=$SECONDS; }
# ── Socket / C-ABI Zero-Fork Renderer ──
zmodload -i zsh/net/socket 2>/dev/null || true

_fb_precmd() {
    local exit_code=$?
    local sock="/tmp/fancybash_${EUID:-${UID:-1000}}.sock"
    local fd

    if zsocket "$sock" 2>/dev/null; then
        fd=$REPLY
        print -u $fd "${PWD}"$'\x1f'"${exit_code}"$'\x1f'"0"$'\x1f'"${USER}"$'\x1f'"${HOST}"$'\x1f'"0"
        read -u $fd PROMPT
        exec {fd}>&-
    else
        PROMPT=$(fancybash prompt --cwd "$PWD" --exit-code "$exit_code" --user "$USER" --host "$HOST" 2>/dev/null)
    fi
}

# Register hooks
autoload -Uz add-zsh-hook
add-zsh-hook preexec _fb_preexec
add-zsh-hook precmd  _fb_precmd

# Phase 2 note: replace _fb_precmd with zmodload libfancybash_core.so
# which calls fb_prompt_render() directly (< 1 ms, 0 allocations).
"#);

    out.push_str("\n# fancybash zsh init complete\n");
    out
}
