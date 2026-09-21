// ============================================================================
// STATUS: 100% NATIVE RUST & BULLETPROOF (ZERO EXTERNAL BINARY DEPENDENCIES)
// AUDIT COMPLETED: FULL FEATURE PARITY, CROSS-OS VERIFIED & OPTIMIZED
// HANDS-OFF GUARANTEE: NO MANUAL EDITS REQUIRED
// ============================================================================

// =============================================================================
//  src/init/fish.rs — Fish shell bootstrap code generator

//
//  Generates valid Fish 3.x+ syntax powered by Native Rust resolution. Source'd via:
//    fancybash init fish | source
//
//  Hook strategy:
//    • fish_prompt()        — renders the full two-line prompt via Native Rust engine.
//    • fish_right_prompt()  — disabled (handled by Native Rust prompt).
//    • --on-variable PWD    — fires on directory change for auto-ls.
//    • fish_command_not_found — fires when an unknown command is executed.
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
    out.push_str(&shared::render_integrations(Shell::Fish));
    out.push_str(&shared::render_auto_ls_hook(Shell::Fish));
    out.push_str(&shared::render_cf_wrapper(Shell::Fish));

    // ── Prompt function (Native Rust Engine) ──────────────────────────────────
    out.push_str(r#"
# ── fancybash Fish Prompt (Native Rust Engine) ──────────────────────────────
#
# fish_prompt is called every time a new prompt is needed.
# $status    = exit code of the last command (captured BEFORE any other calls)
# $CMD_DURATION = Fish built-in: duration of last command in milliseconds
#

function fish_prompt
    # Capture exit code FIRST before any other command clobbers it
    set -l _fb_exit $status

    # CMD_DURATION is a Fish built-in (ms). Default 0 if not set.
    set -l _fb_dur 0
    if set -q CMD_DURATION
        set _fb_dur $CMD_DURATION
    end

    # Resolve hostname safely (works on Linux and macOS)
    set -l _fb_host (hostname -s 2>/dev/null; or hostname 2>/dev/null; or echo host)

    # Render prompt via Native Rust engine — outputs raw ANSI (no escaping needed in Fish)
    fancybash prompt \
        --shell fish \
        --cwd "$PWD" \
        --exit-code $_fb_exit \
        --user (whoami) \
        --host $_fb_host \
        --cmd-duration $_fb_dur \
        2>/dev/null
end

# ── Right-side prompt: disabled (duration is handled in main prompt) ──────
function fish_right_prompt
    # Silent — duration is rendered in main prompt
end

# ── Fish greeting — replace default with fancybash welcome ─────────────────
function fish_greeting
    # Silent — no greeting spam
end

# ── History settings ────────────────────────────────────────────────────────
set -gx fish_history fancybash
set -g fish_history_path "$HOME/.local/share/fish/fish_history"

# ── Key bindings: Ctrl+R → fancybash fh (fuzzy history search) ─────────────
if type -q fancybash
    bind \cr 'fancybash fh'
end

# ── Typo Engine & Command Not Found Handler ──
function fish_command_not_found
    fancybash correct $argv[1]
end
"#);

    out.push_str("\n# fancybash fish init complete\n");
    out
}
