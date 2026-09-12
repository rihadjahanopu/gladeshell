# ==============================================================================
#   F A N C Y B A S H  •  Universal Windows Uninstaller (u.ps1)
#   Author: [Rihad Jahan Opu]
#   Supports: Windows PowerShell 5.1+, PowerShell Core 7+
#
#   Usage:
#   irm https://raw.githubusercontent.com/rihadjahanopu/fancybash-rs/refs/heads/main/u.ps1 | iex
# ==============================================================================

#region -- GUARD 1: PowerShell Version ----------------------------------------
if ($PSVersionTable.PSVersion.Major -lt 5) {
    Write-Host "❌ PowerShell 5.1 or higher is required." -ForegroundColor Red
    if ($MyInvocation.MyCommand.Path) { exit 1 } else { return }
}
#endregion

#region -- GUARD 2: TLS 1.2 ----------------------------------------------------
try {
    [Net.ServicePointManager]::SecurityProtocol = (
        [Net.SecurityProtocolType]::Tls12 -bor
        [Net.SecurityProtocolType]::Tls11 -bor
        [Net.SecurityProtocolType]::Tls
    )
} catch {}
#endregion

#region -- GUARD 3: ExecutionPolicy Self-Bypass --------------------------------
$_scriptPath = $MyInvocation.MyCommand.Path
if ($_scriptPath) {
    $currentPolicy = Get-ExecutionPolicy -Scope Process
    if ($currentPolicy -eq 'Restricted' -or $currentPolicy -eq 'AllSigned') {
        Write-Host "⚠️ ExecutionPolicy blocks script. Re-launching with Bypass..." -ForegroundColor Yellow
        $psExe = if (Get-Command pwsh -ErrorAction SilentlyContinue) { 'pwsh' } else { 'powershell' }
        & $psExe -NoProfile -ExecutionPolicy Bypass -File $_scriptPath @args
        if ($MyInvocation.MyCommand.Path) { exit $LASTEXITCODE } else { return }
    }
}
#endregion

$ErrorActionPreference = "Continue"

# -- Colors & Formatting -------------------------------------------------------
$ESC  = [char]27
$RED  = "$ESC[1;31m"; $GRN  = "$ESC[1;32m"; $YLW  = "$ESC[1;33m"
$BLU  = "$ESC[1;34m"; $PUR  = "$ESC[1;35m"; $CYN  = "$ESC[1;36m"
$BOLD = "$ESC[1m";    $NC   = "$ESC[0m"

Write-Host ""
Write-Host "${BOLD}${PUR}🗑  FANCYBASH UNIVERSAL UNINSTALLER (PowerShell)${NC}"
Write-Host "${CYN}══════════════════════════════════════════════════${NC}"
Write-Host ""

# Ensure local bin paths in PATH for check
$binDir = Join-Path $HOME ".local\bin"
if (-not ($env:PATH -split ';' -contains $binDir)) {
    $env:PATH = "$binDir;$env:PATH"
}

# --- 1. Native Rust Binary Uninstallation ------------------------------------
if (Get-Command fancybash -ErrorAction SilentlyContinue) {
    Write-Host "  ${GRN}✔ Found fancybash binary. Running native self-uninstaller...${NC}"
    Write-Host ""
    & fancybash uninstall
    if ($MyInvocation.MyCommand.Path) { exit } else { return }
}

# --- 2. Fallback $PROFILE Cleanup --------------------------------------------
Write-Host "  ${CYN}➜ Cleaning fancybash block from PowerShell profile...${NC}"

if ($PROFILE -and (Test-Path $PROFILE)) {
    try {
        $content = Get-Content $PROFILE -Raw
        if ($content -match '# >>> fancy-powershell >>>') {
            $cleaned = $content -replace '(?s)# >>> fancy-powershell >>>.*?# <<< fancy-powershell <<<', ''
            $cleaned = $cleaned.Trim()
            Set-Content -Path $PROFILE -Value $cleaned -Encoding utf8
            Write-Host "  ${GRN}✔ Cleaned profile: $PROFILE${NC}"
        } else {
            Write-Host "  ${YLW}ℹ No fancybash block found in `$PROFILE.${NC}"
        }
    } catch {
        Write-Host "  ${RED}❌ Error cleaning `$PROFILE: $($_.Exception.Message)${NC}"
    }
}

# Remove binaries
$binaries = @(
    (Join-Path $HOME ".local\bin\fancybash.exe"),
    (Join-Path $HOME ".local\bin\fancybash"),
    (Join-Path $HOME ".cargo\bin\fancybash.exe"),
    (Join-Path $HOME ".cargo\bin\fancybash")
)

foreach ($bin in $binaries) {
    if (Test-Path $bin) {
        Remove-Item $bin -Force -ErrorAction SilentlyContinue
        Write-Host "  ${GRN}✔ Removed binary: $bin${NC}"
    }
}

Write-Host ""
Write-Host "  ${GRN}🎉 fancybash uninstalled successfully!${NC}"
Write-Host "  ${CYN}💡 Please restart your PowerShell session for changes to take effect.${NC}"
Write-Host ""
