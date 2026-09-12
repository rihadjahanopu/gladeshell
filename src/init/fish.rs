// =============================================================================
//  src/init/fish.rs — Fish shell bootstrap code generator
//
//  Generates valid Fish 3.x+ syntax powered by Native Rust resolution. Source'd via:
//    fancybash init fish | source
//
//  Hook strategy:
//    • fish_prompt()     — called to render the prompt via Native Rust engine.
//    • fish_right_prompt()— optional right-side decoration.
//    • accurate_auto_ls  — event hook on PWD change calling fancybash auto-ls.
// =============================================================================

use crate::core::aliases::Shell;
use super::{header_comment, shared};

pub fn generate() -> String {
    let mut out = String::with_capacity(8192);

    out.push_str(&header_comment("#"));
    out.push_str(&shared::render_guard(Shell::Fish));
    out.push_str(&shared::render_env_and_path(Shell::Fish));
    out.push_str(&shared::render_nvm_lazy_load(Shell::Fish));
    out.push_str(&shared::render_bun_setup(Shell::Fish));
    out.push_str(&shared::render_aliases(Shell::Fish));
    out.push_str(&shared::render_auto_ls_hook(Shell::Fish));

    // ── Prompt function (Native Rust Engine) ──────────────────────────────────
    out.push_str(r#"
# ── fancybash prompt (Native Rust Engine) ──

function fish_prompt
    set -l last_status $status
    set -l duration (or "$CMD_DURATION" 0)
    fancybash prompt --shell fish --cwd "$PWD" --exit-code "$last_status" --user (whoami) --host (hostname -s) --cmd-duration "$duration" 2>/dev/null
end

function fish_right_prompt
    if test $status -ne 0
        set_color --bold red
        printf ' ✗ %d' $status
        set_color normal
    end
end

# ── History ──
set -gx fish_history_path "$HOME/.local/share/fish/fish_history"
"#);

    out.push_str("\n# fancybash fish init complete\n");
    out
}
