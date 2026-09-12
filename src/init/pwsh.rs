// =============================================================================
//  src/init/pwsh.rs — PowerShell bootstrap code generator
//
//  Generates valid PowerShell 7+ (pwsh) syntax powered by Native Rust resolution. Eval'd via:
//    fancybash init pwsh | Invoke-Expression
//  or appended to $PROFILE.
//
//  Hook strategy:
//    • function Prompt {}         — main prompt & native auto-ls hook.
//    • $PSDefaultParameterValues — coloured output defaults.
//    • Set-PSReadLineOption       — key bindings & history.
// =============================================================================

use crate::core::aliases::Shell;
use super::{header_comment, shared};

pub fn generate() -> String {
    let mut out = String::with_capacity(8192);

    out.push_str(&header_comment("#"));
    out.push_str(&shared::render_guard(Shell::Pwsh));
    out.push_str(&shared::render_env_and_path(Shell::Pwsh));
    out.push_str(&shared::render_aliases(Shell::Pwsh));

    out.push_str(r#"
# ── PSReadLine ──
if (Get-Module -ListAvailable -Name PSReadLine -ErrorAction SilentlyContinue) {
    Set-PSReadLineOption -EditMode Emacs
    Set-PSReadLineOption -HistorySearchCursorMovesToEnd
    Set-PSReadLineKeyHandler -Key Tab -Function MenuComplete
    Set-PSReadLineKeyHandler -Key UpArrow   -Function HistorySearchBackward
    Set-PSReadLineKeyHandler -Key DownArrow -Function HistorySearchForward
}

# ── fancybash Prompt & Native Auto-LS ──
$global:_fb_last_pwd = $null
function Prompt {
    $lastExit = if ($null -eq $LASTEXITCODE) { 0 } else { $LASTEXITCODE }
    $cwd = (Get-Location).Path
    if ($cwd -ne $global:_fb_last_pwd) {
        $global:_fb_last_pwd = $cwd
        fancybash auto-ls 2>$null
    }
    fancybash prompt --cwd "$cwd" --exit-code "$lastExit" --user "$env:USERNAME" --host "$env:COMPUTERNAME" 2>$null
}

# ── History & Defaults ──
$MaximumHistoryCount = 50000
$PSDefaultParameterValues['*:Encoding'] = 'UTF8'
"#);

    out.push_str("\n# fancybash pwsh init complete\n");
    out
}
