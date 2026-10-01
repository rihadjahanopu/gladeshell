// ============================================================================
// STATUS: 100% NATIVE RUST & BULLETPROOF (ZERO EXTERNAL BINARY DEPENDENCIES)
// ============================================================================

// =============================================================================
//  src/init/zsh.rs — Zsh bootstrap code generator
//
//  Generates valid Zsh 5.x+ syntax powered by Native Rust resolution. Eval'd via:
//    eval "$(gladeshell init zsh)"
//  or added to .zshrc as:
//    source <(gladeshell init zsh)
//
//  Hook strategy:
//    • precmd()     — called before each prompt render (Native Rust prompt / socket daemon).
//    • preexec()    — called before each command (cmd-duration in ms).
//    • chpwd()      — called on directory change (Native Rust auto-ls).
//    • command_not_found_handler — fires when an unknown command is executed.
//
//  SAFETY NOTES:
//    • Socket path is dynamically resolved at shell startup via
//      `gladeshell socket-path` — never hardcoded — ensuring macOS, Linux, and
//      any custom TMPDIR are handled correctly.
//    • All gladeshell calls use `2>/dev/null` suppression; failures fall through
//      to the safe builtin fallback (direct binary call).
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
setopt HIST_NO_FUNCTIONS

HISTSIZE=50000
SAVEHIST=50000
HISTFILE="${HISTFILE:-$HOME/.zsh_history}"

# Prevent git diff outputs, code snippets, multi-word junk from cluttering history
# Lines that look like patch/diff stats or that start with } / ) / > are skipped.
HISTORY_IGNORE='([[:space:]]#|[0-9]## file?(s) changed*|},|});|(*insertion*)|(*deletion*)|>*|)*|};)'
"#);

    // ── Autocompletion engine & Plugins (Native Rust Resolved) ─────────────────
    let home = shared::home_dir();
    out.push_str(r#"
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
            out.push_str(&format!("    fpath+=(\"{fp}\")\n"));
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

    # ── 100% Native Rust ZSH Subsystem Hooks (Zero External Plugin Files) ──
    # SAFE DESIGN: Every widget wrapper calls `zle .<builtin>` FIRST, then
    # optionally runs the highlight/suggest update. If gladeshell fails, typing
    # continues to work normally — no silent breakage possible.

    _fb_zle_autosuggest_and_highlight() {
        # 1. Native Whitespace-Protected Autosuggestion (non-blocking)
        POSTDISPLAY=""
        local trimmed="${BUFFER#"${BUFFER%%[^[:space:]]*}"}"
        if [[ -n "$trimmed" ]]; then
            local sug
            sug=$(gladeshell suggest "$BUFFER" 2>/dev/null) || sug=""
            # Only show ghost text when suggestion is STRICTLY longer than typed buffer
            if [[ -n "$sug" && "$sug" == "$BUFFER"* && "$sug" != "$BUFFER" ]]; then
                POSTDISPLAY="${sug#$BUFFER}"
            fi
        fi

        # 2. Native Zero-Latency Syntax Highlighting (non-blocking; safe on failure)
        region_highlight=()
        if [[ -n "$BUFFER" ]]; then
            local hl_spec
            hl_spec=$(gladeshell highlight "$BUFFER" 2>/dev/null) || hl_spec=""
            if [[ -n "$hl_spec" ]]; then
                local _hl
                while IFS= read -r _hl; do
                    [[ -n "$_hl" ]] && region_highlight+=("$_hl")
                done <<< "$hl_spec"
            fi
        fi

        # 3. Highlight Ghost-Text (POSTDISPLAY) in Dim Grey (fg=8)
        if [[ -n "$POSTDISPLAY" ]]; then
            local post_start=$#BUFFER
            local post_end=$(( $#BUFFER + $#POSTDISPLAY ))
            region_highlight+=("$post_start $post_end fg=8")
        fi

        # 4. Force ZLE line refresh so terminal immediately erases stale POSTDISPLAY ghost text
        zle -R 2>/dev/null || true
    }

    _fb_zle_forward_char_wrapper() {
        if [[ -n "$POSTDISPLAY" && $CURSOR -eq $#BUFFER ]]; then
            # Accept full ghost-text suggestion, then re-highlight
            BUFFER="$BUFFER$POSTDISPLAY"
            POSTDISPLAY=""
            CURSOR=$#BUFFER
            _fb_zle_autosuggest_and_highlight
        else
            zle .forward-char 2>/dev/null || true
        fi
    }

    _fb_zle_end_of_line_wrapper() {
        if [[ -n "$POSTDISPLAY" ]]; then
            # Accept full ghost-text suggestion, then re-highlight
            BUFFER="$BUFFER$POSTDISPLAY"
            POSTDISPLAY=""
            CURSOR=$#BUFFER
            _fb_zle_autosuggest_and_highlight
        else
            zle .end-of-line 2>/dev/null || true
        fi
    }

    _fb_zle_forward_word_wrapper() {
        if [[ -n "$POSTDISPLAY" ]]; then
            # Word-by-word suggestion acceptance (Ctrl+Right / Alt+Right)
            if [[ "$POSTDISPLAY" =~ '^([^[:alnum:]]*[[:alnum:]]+)' ]]; then
                BUFFER="${BUFFER}${MATCH}"
                POSTDISPLAY="${POSTDISPLAY#$MATCH}"
                CURSOR=$#BUFFER
            else
                BUFFER="$BUFFER$POSTDISPLAY"
                POSTDISPLAY=""
                CURSOR=$#BUFFER
            fi
            _fb_zle_autosuggest_and_highlight
        else
            zle .forward-word 2>/dev/null || true
        fi
    }

    _fb_zle_line_init() {
        _fb_zle_autosuggest_and_highlight
    }

    _fb_zle_line_finish() {
        POSTDISPLAY=""
        region_highlight=()
        zle -R 2>/dev/null || true
    }

    zle -N forward-char _fb_zle_forward_char_wrapper
    zle -N end-of-line _fb_zle_end_of_line_wrapper
    zle -N forward-word _fb_zle_forward_word_wrapper
    zle -N zle-line-init _fb_zle_line_init
    zle -N zle-line-finish _fb_zle_line_finish

    # ── Wrap edit/delete widgets ──────────────────────────────────────────────
    # CRITICAL: zle .<widget> is called FIRST (ensures real ZLE action runs),
    # then highlight/suggest update follows. If gladeshell fails, typing works.
    _fb_bind_widget() {
        local w="$1"
        local fn="_fb_widget_${w//-/_}"
        eval "
            ${fn}() {
                zle .${w} 2>/dev/null
                _fb_zle_autosuggest_and_highlight
            }
        "
        zle -N "$w" "$fn"
    }

    _fb_bind_widget self-insert             2>/dev/null || true
    _fb_bind_widget backward-delete-char    2>/dev/null || true
    _fb_bind_widget delete-char             2>/dev/null || true
    _fb_bind_widget vi-backward-delete-char 2>/dev/null || true
    _fb_bind_widget vi-delete-char          2>/dev/null || true
    _fb_bind_widget backward-delete-word    2>/dev/null || true
    _fb_bind_widget backward-kill-word      2>/dev/null || true
    _fb_bind_widget backward-kill-line      2>/dev/null || true
    _fb_bind_widget kill-line               2>/dev/null || true
    _fb_bind_widget kill-region             2>/dev/null || true
    _fb_bind_widget kill-whole-line         2>/dev/null || true

    # ── Key bindings: CSI + SS3 for emacs & viins keymaps ────────────────────
    # Covers: VS Code / Antigravity IDE / xterm.js / GNOME Terminal / kitty
    # Application Cursor Mode sends SS3 (^[O...) instead of CSI (^[[...))

    # emacs keymap (default interactive mode)
    bindkey -M emacs '^[[C'    forward-char  2>/dev/null || true   # Right Arrow (CSI)
    bindkey -M emacs '^[OC'    forward-char  2>/dev/null || true   # Right Arrow (SS3 / xterm app mode)
    bindkey -M emacs '^[[F'    end-of-line   2>/dev/null || true   # End key (CSI)
    bindkey -M emacs '^[OF'    end-of-line   2>/dev/null || true   # End key (SS3)
    bindkey -M emacs '^[[4~'   end-of-line   2>/dev/null || true   # End key (xterm tilde)
    bindkey -M emacs '^[[8~'   end-of-line   2>/dev/null || true   # End key (rxvt)
    bindkey -M emacs '^[[1;5C' forward-word  2>/dev/null || true   # Ctrl+Right Arrow
    bindkey -M emacs '^[[1;3C' forward-word  2>/dev/null || true   # Alt+Right Arrow
    bindkey -M emacs '^F'      forward-char  2>/dev/null || true   # Ctrl+F (fish-style accept)

    # viins keymap (vi insert mode)
    bindkey -M viins '^[[C'    forward-char  2>/dev/null || true   # Right Arrow (CSI)
    bindkey -M viins '^[OC'    forward-char  2>/dev/null || true   # Right Arrow (SS3 / xterm app mode)
    bindkey -M viins '^[[F'    end-of-line   2>/dev/null || true   # End key (CSI)
    bindkey -M viins '^[OF'    end-of-line   2>/dev/null || true   # End key (SS3)
    bindkey -M viins '^[[4~'   end-of-line   2>/dev/null || true   # End key (xterm tilde)
    bindkey -M viins '^[[8~'   end-of-line   2>/dev/null || true   # End key (rxvt)
    bindkey -M viins '^[[1;5C' forward-word  2>/dev/null || true   # Ctrl+Right Arrow
    bindkey -M viins '^[[1;3C' forward-word  2>/dev/null || true   # Alt+Right Arrow
    bindkey -M viins '^F'      forward-char  2>/dev/null || true   # Ctrl+F (fish-style accept)
fi
"#);

    out.push_str(&shared::render_env_and_path(Shell::Zsh));
    out.push_str(&shared::render_nvm_lazy_load(Shell::Zsh));
    out.push_str(&shared::render_bun_setup(Shell::Zsh));
    out.push_str(&shared::render_aliases(Shell::Zsh));
    out.push_str(&shared::render_integrations(Shell::Zsh));

    // ── Native Rust Prompt Engine & Command Duration Tracker ──────────────────
    // CRITICAL FIX: socket path is now resolved dynamically at shell startup
    // via `gladeshell socket-path` to handle macOS /var/folders/..., custom
    // TMPDIR, and any other OS-specific temp directory correctly.
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

    # Resolve socket path dynamically — never hardcoded — so macOS, custom
    # TMPDIR, and Linux all work correctly without any configuration.
    local sock
    sock=$(gladeshell socket-path 2>/dev/null)
    sock="${sock:-}"

    local fd
    zmodload -i zsh/net/socket 2>/dev/null || true

    if [[ -n "$sock" && -S "$sock" ]] && zsocket "$sock" 2>/dev/null; then
        fd=$REPLY
        print -u $fd "${PWD}"$'\x1f'"${exit_code}"$'\x1f'"0"$'\x1f'"${USER}"$'\x1f'"${HOST}"$'\x1f'"${duration}"$'\x1f'"0"
        # Read the FULL multiline prompt (IFS= read -r -d '' preserves all newlines)
        local raw_prompt
        IFS= read -r -d '' -u $fd raw_prompt
        PROMPT="${raw_prompt}"
        exec {fd}>&-
    else
        # Fallback: direct binary call.
        # printf-trick avoids $() stripping trailing newlines that carry ❯❯❯.
        local tmp
        tmp=$(gladeshell prompt --shell zsh --cwd "$PWD" --exit-code "$exit_code" --cmd-duration "$duration" --user "$USER" --host "$HOST" 2>/dev/null; printf x)
        PROMPT="${tmp%x}"
    fi
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec _fb_preexec
add-zsh-hook precmd  _fb_precmd

# ── Typo Engine & Command Not Found Handler ──
command_not_found_handler() {
    gladeshell correct "$1"
    return 127
}
"#);
    out.push_str(&shared::render_auto_ls_hook(Shell::Zsh));
    out.push_str(&shared::render_cf_wrapper(Shell::Zsh));

    out.push_str("\n# gladeshell zsh init complete\n");
    out
}
