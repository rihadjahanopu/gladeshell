// =============================================================================
//  src/init/pwsh.rs — PowerShell bootstrap code generator
//
//  Generates valid PowerShell 7+ (pwsh) syntax.  Eval'd via:
//    fancybash init pwsh | Invoke-Expression
//  or appended to $PROFILE.
//
//  Hook strategy:
//    • function Prompt {}         — main prompt hook.
//    • $PSDefaultParameterValues — coloured output defaults.
//    • Set-PSReadLineOption       — key bindings & history.
//    • Set-Alias / function       — alias expansion.
// =============================================================================

use crate::core::{aliases::Shell, env};
use super::header_comment;

pub fn generate() -> String {
    let mut out = String::with_capacity(8192);

    out.push_str(&header_comment("#"));

    // ── Guard ────────────────────────────────────────────────────────────────
    out.push_str(r#"
# Guard against double-sourcing
if ($env:_FANCYBASH_PWSH_LOADED -eq '1') { return }
$env:_FANCYBASH_PWSH_LOADED = '1'
"#);

    // ── Environment variables ─────────────────────────────────────────────────
    out.push_str("\n# ── Environment variables ──\n");
    for var in env::canonical_env_vars() {
        out.push_str(&env::render_env_pwsh(&var));
        out.push('\n');
    }

    // ── PATH entries ─────────────────────────────────────────────────────────
    out.push_str("\n# ── PATH ──\n");
    for entry in env::canonical_path_entries() {
        out.push_str(&env::render_path_pwsh(&entry));
        out.push('\n');
    }

    // ── PSReadLine configuration ──────────────────────────────────────────────
    out.push_str(r#"
# ── PSReadLine ──
if (Get-Module -ListAvailable -Name PSReadLine -ErrorAction SilentlyContinue) {
    Set-PSReadLineOption -EditMode Emacs
    Set-PSReadLineOption -HistorySearchCursorMovesToEnd
    Set-PSReadLineOption -PredictionSource History
    Set-PSReadLineOption -MaximumHistoryCount 50000
    Set-PSReadLineKeyHandler -Key Tab -Function MenuComplete
    Set-PSReadLineKeyHandler -Key UpArrow   -Function HistorySearchBackward
    Set-PSReadLineKeyHandler -Key DownArrow -Function HistorySearchForward
}
"#);

    // ── Aliases ───────────────────────────────────────────────────────────────
    out.push_str("\n# ── Aliases & Functions ──\n");
    out.push_str(&crate::core::aliases::AliasFile::from_toml(
        include_str!("../../aliases.toml"),
    )
    .map(|af| af.render(Shell::Pwsh))
    .unwrap_or_else(|e| format!("# aliases.toml parse error: {e}\n")));

    // ── Git helper ────────────────────────────────────────────────────────────
    out.push_str(r#"
# ── Git segment helper ──
function __fb_GitSegment {
    $branch = git branch --show-current 2>$null
    if (-not $branch) {
        $branch = git rev-parse --short HEAD 2>$null
    }
    if ($branch) {
        $dirty = if (git status --porcelain --untracked-files=no 2>$null) { ' ❗' } else { '' }
        return " [🌿 $branch$dirty]"
    }
    return ''
}
"#);

    // ── Prompt function ───────────────────────────────────────────────────────
    out.push_str(r#"
# ── fancybash Prompt ──
function Prompt {
    $lastExit = if ($null -eq $LASTEXITCODE) { 0 } else { $LASTEXITCODE }
    $cwd = (Get-Location).Path
    fancybash prompt --cwd "$cwd" --exit-code "$lastExit" --user "$env:USERNAME" --host "$env:COMPUTERNAME" 2>$null
}
"#);

    // ── History ───────────────────────────────────────────────────────────────
    out.push_str(r#"
# ── History ──
$MaximumHistoryCount = 50000
"#);

    // ── Default parameter values ──────────────────────────────────────────────
    out.push_str(r#"
# ── Default parameter values ──
$PSDefaultParameterValues['*:Encoding'] = 'UTF8'
"#);

    out.push_str("\n# fancybash pwsh init complete\n");
    out
}
