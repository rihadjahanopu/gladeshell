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

    // ── Autocompletion engine & Plugins ───────────────────────────────────────
    out.push_str(r#"
# Clear terminal screen silently on interactive session startup
if [[ -o interactive ]]; then
    clear 2>/dev/null
fi

plugins=(
  git
  zsh-autosuggestions
  zsh-syntax-highlighting
)

# ======================================================
# ⚡ ZSH AUTOCOMPLETION ENGINE & PLUGINS (LOAD FIRST)
# ======================================================
if [[ -o interactive ]]; then
    # 1. Ensure fpath includes custom and system completion directories BEFORE compinit
    typeset -U fpath
    local -a _fb_fpaths=(
        "$HOME/.zsh/zsh-completions/src"
        "$HOME/.zsh/completion"
        "${BUN_INSTALL:-$HOME/.bun}"
        "$HOME/.bun"
        "/usr/local/share/zsh/site-functions"
        "/usr/share/zsh/site-functions"
        "/usr/share/zsh/vendor-completions"
    )
    local _fb_fp
    for _fb_fp in "${_fb_fpaths[@]}"; do
        [[ -d "$_fb_fp" ]] && fpath=("$_fb_fp" $fpath)
    done
    unset _fb_fpaths _fb_fp

    # 2. Configure completion options and styles BEFORE compinit
    setopt extendedglob 2>/dev/null || true
    setopt AUTO_LIST AUTO_MENU COMPLETE_IN_WORD ALWAYS_TO_END 2>/dev/null || true

    zstyle ':completion:*' menu select
    zstyle ':completion:*' list-colors "${(s.:.)LS_COLORS}"
    zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}' 'r:|[._-]=* r:|=*' 'l:|=* r:|=*'
    zstyle ':completion:*' rehash true

    # 3. Secure & Fast compinit execution with -i flag (silences insecure directory errors)
    autoload -Uz compinit bashcompinit
    local zcompdump="${ZSH_COMPDUMP:-$HOME/.zcompdump}"
    if [[ ! -f "$zcompdump" || ! -s "$zcompdump" || -n ${zcompdump}(#qN.m+1) ]]; then
        compinit -i -d "$zcompdump"
    else
        compinit -i -C -d "$zcompdump"
    fi
    bashcompinit 2>/dev/null || true

    # 4. Asynchronous zcompile of zcompdump with size validation
    [[ -f "$zcompdump" && -s "$zcompdump" && (! -f "${zcompdump}.zwc" || "$zcompdump" -nt "${zcompdump}.zwc") ]] && \
        ( zcompile "$zcompdump" 2>/dev/null &! )

    # 5. Multi-distro zsh-autocomplete plugin lookup
    local -a _fb_ac_paths=(
        "$HOME/.zsh/zsh-autocomplete/zsh-autocomplete.plugin.zsh"
        "/usr/share/zsh-autocomplete/zsh-autocomplete.plugin.zsh"
        "/usr/share/zsh/plugins/zsh-autocomplete/zsh-autocomplete.plugin.zsh"
        "/opt/homebrew/share/zsh-autocomplete/zsh-autocomplete.plugin.zsh"
        "/usr/local/share/zsh-autocomplete/zsh-autocomplete.plugin.zsh"
    )
    local _fb_ac
    for _fb_ac in "${_fb_ac_paths[@]}"; do
        if [[ -f "$_fb_ac" ]]; then
            source "$_fb_ac" 2>/dev/null
            break
        fi
    done
    unset _fb_ac_paths _fb_ac

    # 6. Multi-distro zsh-autosuggestions lookup
    local -a _fb_as_paths=(
        "$HOME/.zsh/zsh-autosuggestions/zsh-autosuggestions.zsh"
        "/usr/share/zsh-autosuggestions/zsh-autosuggestions.zsh"
        "/usr/share/zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh"
        "/opt/homebrew/share/zsh-autosuggestions/zsh-autosuggestions.zsh"
        "/usr/local/share/zsh-autosuggestions/zsh-autosuggestions.zsh"
    )
    local _fb_as
    for _fb_as in "${_fb_as_paths[@]}"; do
        if [[ -f "$_fb_as" ]]; then
            source "$_fb_as" 2>/dev/null
            break
        fi
    done
    unset _fb_as_paths _fb_as
    ZSH_AUTOSUGGEST_USE_ASYNC=true

    # 7. Multi-distro zsh-syntax-highlighting lookup
    local -a _fb_sh_paths=(
        "$HOME/.zsh/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"
        "/usr/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"
        "/usr/share/zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"
        "/opt/homebrew/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"
        "/usr/local/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"
    )
    local _fb_sh
    for _fb_sh in "${_fb_sh_paths[@]}"; do
        if [[ -f "$_fb_sh" ]]; then
            source "$_fb_sh" 2>/dev/null
            break
        fi
    done
    unset _fb_sh_paths _fb_sh
fi
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
    out.push_str(r##"
# ======================================================
# 🟢 NVM & NODE.JS DYNAMIC LAZY-LOAD (LOADED AFTER AUTOCOMPLETE)
# ======================================================
export NVM_DIR="${NVM_DIR:-$HOME/.config/nvm}"
[[ ! -d "$NVM_DIR" && -d "$HOME/.nvm" ]] && export NVM_DIR="$HOME/.nvm"

if [[ -d "$NVM_DIR/versions/node" ]]; then
    _NODE_DEFAULT_BIN="$(ls -d "$NVM_DIR/versions/node"/* 2>/dev/null | tail -n 1)/bin"
    [[ -d "$_NODE_DEFAULT_BIN" && ":$PATH:" != *":$_NODE_DEFAULT_BIN:"* ]] && export PATH="$_NODE_DEFAULT_BIN:$PATH"
fi

# Auto-clean duplicate external NVM/Bun installer lines & reorder position in non-blocking background on shell boot
if [[ -o interactive ]]; then
    fancybash internal-clean-rc >/dev/null 2>&1 &!
fi

_fb_lazy_load_nvm() {
    unset -f nvm node npm npx 2>/dev/null
    fancybash internal-clean-rc >/dev/null 2>&1 &!
    if [ -s "$NVM_DIR/nvm.sh" ]; then
        \. "$NVM_DIR/nvm.sh"
    fi
    if [ -s "$NVM_DIR/bash_completion" ]; then
        autoload -Uz bashcompinit 2>/dev/null
        bashcompinit 2>/dev/null || true
        \. "$NVM_DIR/bash_completion"
    fi
}

nvm() {
    _fb_lazy_load_nvm
    nvm "$@"
}

node() {
    _fb_lazy_load_nvm
    node "$@"
}

npm() {
    _fb_lazy_load_nvm
    npm "$@"
}

npx() {
    _fb_lazy_load_nvm
    npx "$@"
}
"##);

    // ── Bun ───────────────────────────────────────────────────────────────────
    out.push_str(r#"
# ======================================================
# 🥐 BUN ENVIRONMENT & AUTOCOMPLETION (LAZY-LOADED)
# ======================================================
export BUN_INSTALL="${BUN_INSTALL:-$HOME/.bun}"

if [[ -d "$BUN_INSTALL/bin" && ":$PATH:" != *":$BUN_INSTALL/bin:"* ]]; then
    export PATH="$BUN_INSTALL/bin:$PATH"
fi

# Bun completions (Lazy loaded & safe sourced without conflicts)
if [[ -s "$BUN_INSTALL/_bun" ]]; then
    [ -s "$BUN_INSTALL/_bun" ] && source "$BUN_INSTALL/_bun" 2>/dev/null
fi
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
        print -u $fd "${PWD}"$'\x1f'"${exit_code}"$'\x1f'"0"$'\x1f'"${USER}"$'\x1f'"${HOST}"$'\x1f'"0"$'\x1f'"0"
        read -u $fd PROMPT
        exec {fd}>&-
    else
        PROMPT=$(fancybash prompt --shell zsh --cwd "$PWD" --exit-code "$exit_code" --user "$USER" --host "$HOST" 2>/dev/null)
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
