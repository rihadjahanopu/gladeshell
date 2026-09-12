// =============================================================================
//  src/init/bash.rs — Bash bootstrap code generator
//
//  Generates valid Bash 4.x+ syntax powered by Native Rust resolution. Eval'd via:
//    eval "$(fancybash init bash)"
//
//  Hook strategy:
//    • PROMPT_COMMAND — fires before each prompt render (replaces precmd).
//    • trap DEBUG      — fires before each command (cmd-duration tracking).
//    • PS1             — set from the lib's render output via $(__fb_prompt).
// =============================================================================

use crate::core::aliases::Shell;
use super::{header_comment, shared};

pub fn generate() -> String {
    let mut out = String::with_capacity(8192);

    out.push_str(&header_comment("#"));
    out.push_str(&shared::render_guard(Shell::Bash));
    out.push_str(&shared::render_env_and_path(Shell::Bash));
    out.push_str(&shared::render_nvm_lazy_load(Shell::Bash));
    out.push_str(&shared::render_bun_setup(Shell::Bash));
    out.push_str(&shared::render_aliases(Shell::Bash));

    // ── Native Rust Prompt & Hook Engine ──────────────────────────────────────
    out.push_str(r#"
# ── Native Rust Prompt Engine & Command Duration Tracker ──
_fb_timer_start=0
trap '_fb_timer_start=$SECONDS' DEBUG

__fb_prompt() {
    local exit_code=$?
    local duration=0
    if [[ -n "$_fb_timer_start" && "$_fb_timer_start" -gt 0 ]]; then
        duration=$(( (SECONDS - _fb_timer_start) * 1000 ))
        _fb_timer_start=0
    fi
    _fb_auto_ls
    PS1=$(fancybash prompt --shell bash --cwd "$PWD" --exit-code "$exit_code" --cmd-duration "$duration" --user "$USER" --host "$HOSTNAME" 2>/dev/null)
}

PROMPT_COMMAND="__fb_prompt"

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

    out.push_str(&shared::render_auto_ls_hook(Shell::Bash));
    out.push_str("\n# fancybash bash init complete\n");
    out
}
