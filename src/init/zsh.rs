// =============================================================================
//  src/init/zsh.rs — Zsh bootstrap code generator
//
//  Generates valid Zsh 5.x+ syntax powered by Native Rust resolution. Eval'd via:
//    eval "$(fancybash init zsh)"
//  or added to .zshrc as:
//    source <(fancybash init zsh)
//
//  Hook strategy:
//    • precmd()     — called before each prompt render (Native Rust prompt / socket daemon).
//    • preexec()    — called before each command (cmd-duration in ms).
//    • chpwd()      — called on directory change (Native Rust auto-ls).
// =============================================================================

use crate::core::aliases::Shell;
use super::{header_comment, shared};

pub fn generate() -> String {
    let mut out = String::with_capacity(8192);

    out.push_str(&header_comment("#"));
    out.push_str(&shared::render_guard(Shell::Zsh));

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

    // ── Autocompletion engine & Plugins (Native Rust Resolved) ─────────────────
    let home = shared::home_dir();
    out.push_str(r#"
if [[ -o interactive ]]; then
    clear 2>/dev/null
fi

# ======================================================
# ⚡ ZSH AUTOCOMPLETION ENGINE & PLUGINS
# ======================================================
if [[ -o interactive ]]; then
    typeset -U fpath
"#);

    let fpaths = [
        format!("{}/.zsh/zsh-completions/src", home),
        format!("{}/.zsh/completion", home),
        format!("{}/.bun", home),
        "/usr/local/share/zsh/site-functions".to_string(),
        "/usr/share/zsh/site-functions".to_string(),
        "/usr/share/zsh/vendor-completions".to_string(),
    ];
    for fp in &fpaths {
        if std::path::Path::new(fp).is_dir() {
            out.push_str(&format!("    fpath=(\"{}\" $fpath)\n", fp));
        }
    }

    out.push_str(r#"
    setopt extendedglob 2>/dev/null || true
    setopt AUTO_LIST AUTO_MENU COMPLETE_IN_WORD ALWAYS_TO_END 2>/dev/null || true

    zstyle ':completion:*' menu select
    zstyle ':completion:*' list-colors "${(s.:.)LS_COLORS}"
    zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}' 'r:|[._-]=* r:|=*' 'l:|=* r:|=*'
    zstyle ':completion:*' rehash true

    autoload -Uz compinit bashcompinit
    local zcompdump="${ZSH_COMPDUMP:-$HOME/.zcompdump}"
    if [[ ! -f "$zcompdump" || ! -s "$zcompdump" || -n ${zcompdump}(#qN.m+1) ]]; then
        compinit -i -d "$zcompdump"
    else
        compinit -i -C -d "$zcompdump"
    fi
    bashcompinit 2>/dev/null || true

    [[ -f "$zcompdump" && -s "$zcompdump" && (! -f "${zcompdump}.zwc" || "$zcompdump" -nt "${zcompdump}.zwc") ]] && \
        ( zcompile "$zcompdump" 2>/dev/null &! )
"#);

    // Dynamic plugin sourcing via Native Rust lookup
    let ac_candidates = [
        format!("{}/.zsh/zsh-autocomplete/zsh-autocomplete.plugin.zsh", home),
        "/usr/share/zsh-autocomplete/zsh-autocomplete.plugin.zsh".to_string(),
        "/usr/share/zsh/plugins/zsh-autocomplete/zsh-autocomplete.plugin.zsh".to_string(),
        "/opt/homebrew/share/zsh-autocomplete/zsh-autocomplete.plugin.zsh".to_string(),
        "/usr/local/share/zsh-autocomplete/zsh-autocomplete.plugin.zsh".to_string(),
    ];
    if let Some(ac) = shared::find_first_existing(&ac_candidates) {
        out.push_str(&format!("    source \"{}\" 2>/dev/null\n", ac));
    }

    let as_candidates = [
        format!("{}/.zsh/zsh-autosuggestions/zsh-autosuggestions.zsh", home),
        "/usr/share/zsh-autosuggestions/zsh-autosuggestions.zsh".to_string(),
        "/usr/share/zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh".to_string(),
        "/opt/homebrew/share/zsh-autosuggestions/zsh-autosuggestions.zsh".to_string(),
        "/usr/local/share/zsh-autosuggestions/zsh-autosuggestions.zsh".to_string(),
    ];
    if let Some(as_path) = shared::find_first_existing(&as_candidates) {
        out.push_str(&format!("    source \"{}\" 2>/dev/null\n", as_path));
    }
    out.push_str("    ZSH_AUTOSUGGEST_USE_ASYNC=true\n");

    let sh_candidates = [
        format!("{}/.zsh/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh", home),
        "/usr/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh".to_string(),
        "/usr/share/zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh".to_string(),
        "/opt/homebrew/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh".to_string(),
        "/usr/local/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh".to_string(),
    ];
    if let Some(sh) = shared::find_first_existing(&sh_candidates) {
        out.push_str(&format!("    source \"{}\" 2>/dev/null\n", sh));
    }
    out.push_str("fi\n");

    out.push_str(&shared::render_env_and_path(Shell::Zsh));
    out.push_str(&shared::render_nvm_lazy_load(Shell::Zsh));
    out.push_str(&shared::render_bun_setup(Shell::Zsh));
    out.push_str(&shared::render_aliases(Shell::Zsh));

    // ── Native Rust Prompt Engine & Command Duration Tracker ──────────────────
    out.push_str(r#"
# ── Native Rust Prompt Engine & Command Duration Tracker ──
typeset -g _fb_timer=0
_fb_preexec() { _fb_timer=$SECONDS; }

_fb_precmd() {
    local exit_code=$?
    local duration=0
    if [[ -n "$_fb_timer" && "$_fb_timer" -gt 0 ]]; then
        duration=$(( (SECONDS - _fb_timer) * 1000 ))
        _fb_timer=0
    fi
    local sock="/tmp/fancybash_${EUID:-${UID:-1000}}.sock"
    local fd

    zmodload -i zsh/net/socket 2>/dev/null || true
    if zsocket "$sock" 2>/dev/null; then
        fd=$REPLY
        print -u $fd "${PWD}"$'\x1f'"${exit_code}"$'\x1f'"0"$'\x1f'"${USER}"$'\x1f'"${HOST}"$'\x1f'"${duration}"$'\x1f'"0"
        read -u $fd PROMPT
        exec {fd}>&-
    else
        PROMPT=$(fancybash prompt --shell zsh --cwd "$PWD" --exit-code "$exit_code" --cmd-duration "$duration" --user "$USER" --host "$HOST" 2>/dev/null)
    fi
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec _fb_preexec
add-zsh-hook precmd  _fb_precmd
"#);
    out.push_str(&shared::render_auto_ls_hook(Shell::Zsh));
    out.push_str(&shared::render_cf_wrapper(Shell::Zsh));

    out.push_str("\n# fancybash zsh init complete\n");
    out
}
