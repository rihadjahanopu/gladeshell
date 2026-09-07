// =============================================================================
//  src/init/fish.rs — Fish shell bootstrap code generator
//
//  Generates valid Fish 3.x+ syntax.  Source'd via:
//    fancybash init fish | source
//
//  Hook strategy:
//    • fish_prompt()     — called to render the prompt (replaces PS1).
//    • fish_right_prompt()— optional right-side decoration.
//    • fish_preexec      — event hook for cmd-duration.
//    • fish_postexec     — event hook after command completes.
//    • fish_add_path     — idempotent PATH management.
// =============================================================================

use crate::core::{aliases::Shell, env};
use super::header_comment;

pub fn generate() -> String {
    let mut out = String::with_capacity(8192);

    out.push_str(&header_comment("#"));

    // ── Guard ────────────────────────────────────────────────────────────────
    out.push_str(r#"
# Guard against double-sourcing
if set -q _FANCYBASH_FISH_LOADED
    exit 0
end
set -gx _FANCYBASH_FISH_LOADED 1
"#);

    // ── Environment variables ─────────────────────────────────────────────────
    out.push_str("\n# ── Environment variables ──\n");
    for var in env::canonical_env_vars() {
        out.push_str(&env::render_env_fish(&var));
        out.push('\n');
    }

    // ── PATH entries ─────────────────────────────────────────────────────────
    out.push_str("\n# ── PATH ──\n");
    for entry in env::canonical_path_entries() {
        out.push_str(&env::render_path_fish(&entry));
        out.push('\n');
    }

    // ── NVM (via bass or nvm.fish) ────────────────────────────────────────────
    out.push_str(r#"
# ── NVM (Fish-compatible lazy-load via nvm.fish plugin) ──
# Install: fisher install jorgebucaran/nvm.fish
# If nvm.fish is not installed, we do nothing (no errors).
if functions -q nvm
    # nvm.fish is already loaded; nothing to do.
else
    # Fallback: attempt bass-based NVM load if bass is available
    set -gx NVM_DIR (test -d $HOME/.config/nvm && echo $HOME/.config/nvm || echo $HOME/.nvm)
end
"#);

    // ── Bun ───────────────────────────────────────────────────────────────────
    out.push_str(r#"
# ── Bun ──
set -gx BUN_INSTALL "$HOME/.bun"
fish_add_path --prepend "$BUN_INSTALL/bin"
"#);

    // ── Aliases ───────────────────────────────────────────────────────────────
    out.push_str("\n# ── Aliases ──\n");
    out.push_str(&crate::core::aliases::AliasFile::from_toml(
        include_str!("../../aliases.toml"),
    )
    .map(|af| af.render(Shell::Fish))
    .unwrap_or_else(|e| format!("# aliases.toml parse error: {e}\n")));

    // ── Prompt function ───────────────────────────────────────────────────────
    out.push_str(r#"
# ── fancybash prompt (fish_prompt) ──

# Git segment helper
function __fb_git_segment
    set -l branch (git branch --show-current 2>/dev/null)
    if test -z "$branch"
        set branch (git rev-parse --short HEAD 2>/dev/null)
    end
    if test -n "$branch"
        set -l dirty ""
        if test -n "$(git status --porcelain --untracked-files=no 2>/dev/null)"
            set dirty " ❗"
        end
        printf ' [🌿 %s%s]' "$branch" "$dirty"
    end
end

# Command duration (Fish uses $CMD_DURATION built-in in ms)
function __fb_duration_segment
    if test "$CMD_DURATION" -ge 1000 2>/dev/null
        set -l secs (math --scale=1 "$CMD_DURATION / 1000")
        printf ' ⏱️ %ss' "$secs"
    end
end

function fish_prompt
    set -l last_status $status
    fancybash prompt --cwd "$PWD" --exit-code "$last_status" --user (whoami) --host (hostname -s) --cmd-duration (or "$CMD_DURATION" 0) 2>/dev/null
end

function fish_right_prompt
    # Right-side: show $status in red if non-zero
    if test $status -ne 0
        set_color --bold red
        printf ' ✗ %d' $status
        set_color normal
    end
end
"#);

    // ── History ───────────────────────────────────────────────────────────────
    out.push_str(r#"
# ── History ──
set -gx fish_history_path "$HOME/.local/share/fish/fish_history"
"#);

    out.push_str("\n# fancybash fish init complete\n");
    out
}
