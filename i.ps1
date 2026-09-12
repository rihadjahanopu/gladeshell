# ==============================================================================
#   F A N C Y B A S H  •  Universal Windows Installer (i.ps1)
#   Author: [Rihad Jahan Opu]
#   Supports: Windows PowerShell 5.1+, PowerShell Core 7+
#
#   Dynamically detects whether you are running from:
#     • PowerShell terminal  → irm ... | iex
#     • CMD / Run dialog     → powershell -c "irm ... | iex"
#     • Double-click         → explorer.exe → install.ps1 downloaded + run
#   Then delegates to install.ps1 automatically.
#
#   -- Run from INSIDE PowerShell terminal: ------------------------------------
#   irm https://raw.githubusercontent.com/rihadjahanopu/fancybash-rs/refs/heads/main/i.ps1 | iex
#
#   -- Run from CMD / Win+R Run dialog: ----------------------------------------
#   powershell -c "irm https://raw.githubusercontent.com/rihadjahanopu/fancybash-rs/refs/heads/main/i.ps1 | iex"
# ==============================================================================

#region -- GUARD 1: PowerShell Version ----------------------------------------
if ($PSVersionTable.PSVersion.Major -lt 5) {
    Write-Host "❌ PowerShell 5.1 or higher is required." -ForegroundColor Red
    Write-Host "   Please update: https://aka.ms/wmf5download" -ForegroundColor Yellow
    if ($MyInvocation.MyCommand.Path) { exit 1 } else { return }
}
#endregion

#region -- GUARD 2: TLS 1.2 (required for irm/iwr on older Windows / PS 5.1) --
try {
    [Net.ServicePointManager]::SecurityProtocol = (
        [Net.SecurityProtocolType]::Tls12 -bor
        [Net.SecurityProtocolType]::Tls11 -bor
        [Net.SecurityProtocolType]::Tls
    )
} catch { <# PS7+ handles TLS automatically - safe to ignore #> }
#endregion

#region -- GUARD 3: ExecutionPolicy Self-Bypass (file mode only) --------------
# irm | iex context: $MyInvocation.MyCommand.Path is empty → skip this guard
# .ps1 file context: re-launch with Bypass if policy would block execution
$_scriptPath = $MyInvocation.MyCommand.Path
if ($_scriptPath) {
    $currentPolicy = Get-ExecutionPolicy -Scope Process
    if ($currentPolicy -eq 'Restricted' -or $currentPolicy -eq 'AllSigned') {
        Write-Host "⚠️  ExecutionPolicy blocks this script. Re-launching with Bypass..." -ForegroundColor Yellow
        # Prefer pwsh (PS7) over powershell.exe (PS5.1)
        $psExe = if (Get-Command pwsh -ErrorAction SilentlyContinue) { 'pwsh' } else { 'powershell' }
        & $psExe -NoProfile -ExecutionPolicy Bypass -File $_scriptPath @args
        if ($MyInvocation.MyCommand.Path) { exit $LASTEXITCODE } else { return }
    }
}
#endregion

$ErrorActionPreference = "Continue"

# -- Resolve best available PowerShell executable ------------------------------
# Used later when spawning install.ps1 as a subprocess
$script:PS_EXE = if (Get-Command pwsh -ErrorAction SilentlyContinue) { 'pwsh' } else { 'powershell' }

# -- Colors & Formatting -------------------------------------------------------
$ESC  = [char]27
$RED  = "$ESC[1;31m"; $GRN  = "$ESC[1;32m"; $YLW  = "$ESC[1;33m"
$BLU  = "$ESC[1;34m"; $PUR  = "$ESC[1;35m"; $CYN  = "$ESC[1;36m"
$BOLD = "$ESC[1m";    $DIM  = "$ESC[2m";    $NC   = "$ESC[0m"

$REPO_BASE   = "https://raw.githubusercontent.com/rihadjahanopu/fancybash-rs/refs/heads/main"
$REPO_ALT    = "https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main"
$REPO_CDN    = "https://cdn.jsdelivr.net/gh/rihadjahanopu/fancybash-rs@main"
$REPO_WEB    = "https://fancybash.netlify.app/public"
$INSTALL_URLS = @("$REPO_BASE/install.ps1", "$REPO_ALT/install.ps1", "$REPO_CDN/install.ps1", "$REPO_WEB/install.ps1")

# ══════════════════════════════════════════════════════════════════════════════
#   DYNAMIC CONTEXT DETECTION  (3-tier priority, mirrors i.sh PPID strategy)
#
#   Priority 1 → irm|iex pipe: $MyInvocation.MyCommand.Path is empty/null
#   Priority 2 → WMI parent process query (CimInstance → WmiObject fallback)
#   Priority 3 → Environment variable heuristics
# ══════════════════════════════════════════════════════════════════════════════
function Get-CallerContext {
    <#
    .SYNOPSIS
        Returns one of: 'psterminal' | 'cmd' | 'explorer' | 'unknown'
    .NOTES
        Mirrors i.sh's PPID-based shell detection logic for Windows.
        Three independent tiers ensure a result even in locked-down environments.
    #>

    # -- Tier 1: irm|iex pipe detection ---------------------------------------
    # When piped via `irm ... | iex`, both ScriptName and MyCommand.Path are
    # empty strings (not null). This is the most reliable PS-terminal signal.
    $scriptName = $MyInvocation.ScriptName
    $cmdPath    = $MyInvocation.MyCommand.Path
    if ([string]::IsNullOrEmpty($scriptName) -and [string]::IsNullOrEmpty($cmdPath)) {
        return 'psterminal'
    }

    # -- Tier 2: Parent process via WMI (most accurate) ------------------------
    # Try Get-CimInstance (PS3+) first; fall back to Get-WmiObject (PS2+)
    # to support ancient Windows 7 / Server 2008 R2 environments.
    $parentName = ""
    try {
        $ppid = $null

        # CimInstance path (preferred, works on PS3+)
        if (Get-Command Get-CimInstance -ErrorAction SilentlyContinue) {
            $ppid = (Get-CimInstance -ClassName Win32_Process `
                -Filter "ProcessId=$PID" -ErrorAction Stop).ParentProcessId
        }
        # WmiObject fallback (PS2 / very old Windows)
        if (-not $ppid -and (Get-Command Get-WmiObject -ErrorAction SilentlyContinue)) {
            $ppid = (Get-WmiObject -Class Win32_Process `
                -Filter "ProcessId=$PID" -ErrorAction Stop).ParentProcessId
        }

        if ($ppid) {
            $parentProc = Get-Process -Id $ppid -ErrorAction Stop
            $parentName = $parentProc.ProcessName.ToLower().Trim()
        }
    } catch {
        # WMI unavailable, access denied, or parent already exited → fall through
    }

    if ($parentName) {
        # Exact match first for reliability
        $cmdContextNames = @('cmd')
        $psContextNames  = @('powershell', 'pwsh', 'code', 'code - insiders',
                             'windowsterminal', 'wt', 'alacritty', 'wezterm',
                             'hyper', 'iterm2', 'mintty', 'conhost')
        $explorerNames   = @('explorer')

        if ($cmdContextNames    -contains $parentName) { return 'cmd'        }
        if ($explorerNames      -contains $parentName) { return 'explorer'   }
        if ($psContextNames     -contains $parentName) { return 'psterminal' }

        # Prefix/wildcard match for versioned or variant process names
        # e.g. "powershell_ise", "code.cmd", "wt.exe"
        foreach ($n in $psContextNames) {
            if ($parentName -like "$n*") { return 'psterminal' }
        }
    }

    # -- Tier 3: Environment variable heuristics -------------------------------
    # CMD.exe inherits %PROMPT% from the user's environment; PowerShell does not
    # set %PROMPT% by default. %COMSPEC% always points to cmd.exe but is also
    # present in PS sessions - so we combine checks.
    #
    # IMPORTANT: $env:PSHOME is ALWAYS set inside any PS session, so we cannot
    # use it as a PS-detector. Instead, check the absence of typical PS vars.
    $hasPrompt   = -not [string]::IsNullOrEmpty($env:PROMPT)
    $hasPsVer    = -not [string]::IsNullOrEmpty($env:PSModulePath)  # PS always sets this

    if ($hasPrompt -and -not $hasPsVer) {
        return 'cmd'
    }

    # If we reached here via a .ps1 file invocation, $PSCommandPath is set
    if (-not [string]::IsNullOrEmpty($PSCommandPath)) {
        return 'psterminal'
    }

    return 'unknown'
}

# -- Helper: Retry-aware download with per-URL retries ------------------------
function Invoke-DownloadWithRetry {
    param(
        [string[]]$Urls,
        [string]$OutFile,
        [int]$MaxRetries  = 3,
        [int]$TimeoutSec  = 25,
        [int]$MinSizeBytes = 1000
    )

    foreach ($url in $Urls) {
        $host_name = try { ([Uri]$url).Host } catch { $url }
        for ($attempt = 1; $attempt -le $MaxRetries; $attempt++) {
            Write-Host "  ${DIM}↳ [$attempt/$MaxRetries] $host_name ...${NC}" -NoNewline
            try {
                Invoke-WebRequest -Uri $url -OutFile $OutFile `
                    -UseBasicParsing -TimeoutSec $TimeoutSec -ErrorAction Stop

                # -- Validate downloaded content ------------------------------
                if (-not (Test-Path $OutFile)) {
                    throw "File not found after download"
                }
                $size = (Get-Item $OutFile).Length
                if ($size -lt $MinSizeBytes) {
                    throw "File too small ($size bytes - expected >$MinSizeBytes)"
                }
                # Guard against HTML error pages (GitHub 404, Cloudflare, etc.)
                $peek = Get-Content $OutFile -First 5 -ErrorAction SilentlyContinue
                if (($peek -join "`n") -match '(?i)<!doctype|<html|cloudflare|access.denied|404.not.found') {
                    throw "Received HTML instead of PowerShell script"
                }

                Write-Host " ${GRN}✔ ($([math]::Round($size/1KB,1)) KB)${NC}"
                return $true  # success

            } catch {
                Write-Host " ${RED}✗ $($_.Exception.Message)${NC}"
                if (Test-Path $OutFile) { Remove-Item $OutFile -Force -ErrorAction SilentlyContinue }
                if ($attempt -lt $MaxRetries) { Start-Sleep -Seconds 2 }
            }
        }
        Write-Host "  ${YLW}⚠️  All retries failed for $host_name - trying next source...${NC}"
    }
    return $false  # all sources failed
}

# -- Header Banner -------------------------------------------------------------
Clear-Host
Write-Host ""
Write-Host "${PUR}          ███████╗ █████╗ ███╗   ██╗ ██████╗██╗   ██╗██████╗  █████╗ ███████╗██╗  ██╗${NC}"
Write-Host "${PUR}          ██=════╝██=══██╗████╗  ██║██=════╝╚██╗ ██=╝██=══██╗██=══██╗██=════╝██║  ██║${NC}"
Write-Host "${CYN}          █████╗  ███████║██=██╗ ██║██║      ╚████=╝ ██████=╝███████║███████╗███████║${NC}"
Write-Host "${CYN}          ██=══╝  ██=══██║██║╚██╗██║██║       ╚██=╝  ██=══██╗██=══██║╚════██║██=══██║${NC}"
Write-Host "${BLU}          ██║     ██║  ██║██║ ╚████║╚██████╗   ██║   ██████=╝██║  ██║███████║██║  ██║${NC}"
Write-Host "${BLU}          ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝   ╚═╝   ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝${NC}"
Write-Host ""
Write-Host "   ⚡ ${BOLD}${CYN}F A N C Y B A S H${NC}  •  ${BOLD}Universal Windows Installer${NC}"
Write-Host ""

# -- Detect & Display Context --------------------------------------------------
$context = Get-CallerContext

Write-Host "${BLU}--------------------------------------------------${NC}"
Write-Host " 🔍 ${BOLD}ENVIRONMENT DETECTION${NC}"
Write-Host "${BLU}--------------------------------------------------${NC}"

$osName = if ($PSVersionTable.OS) { $PSVersionTable.OS } else { "Windows ($env:OS)" }

switch ($context) {
    'psterminal' {
        Write-Host "  🐚  ${BOLD}Context:${NC}  ${GRN}PowerShell Terminal${NC}  ${DIM}(irm|iex or PS prompt)${NC}"
        Write-Host ""
        Write-Host "  ${GRN}💡 Tip: Run installs directly in future:${NC}"
        Write-Host "     ${CYN}irm $REPO_BASE/install.ps1 | iex${NC}"
    }
    'cmd' {
        Write-Host "  ⌨️   ${BOLD}Context:${NC}  ${YLW}CMD / Run Dialog${NC}  ${DIM}(powershell -c from cmd.exe)${NC}"
        Write-Host ""
        Write-Host "  ${YLW}💡 Tip: For a better experience, open PowerShell directly:${NC}"
        Write-Host "     ${CYN}irm $REPO_BASE/install.ps1 | iex${NC}"
    }
    'explorer' {
        Write-Host "  📁  ${BOLD}Context:${NC}  ${BLU}Windows Explorer${NC}  ${DIM}(double-clicked)${NC}"
    }
    default {
        Write-Host "  ❓  ${BOLD}Context:${NC}  ${DIM}Undetermined - proceeding anyway${NC}"
    }
}

Write-Host "  💻  ${BOLD}OS:${NC}       ${CYN}$osName${NC}"
Write-Host "  🐚  ${BOLD}PS:${NC}       ${CYN}v$($PSVersionTable.PSVersion)  ($($script:PS_EXE))${NC}"
Write-Host "  👤  ${BOLD}User:${NC}     ${CYN}$env:USERNAME${NC}"
Write-Host "${BLU}--------------------------------------------------${NC}"
Write-Host ""

# ══════════════════════════════════════════════════════════════════════════════
#   DELEGATE TO install.ps1
#   Strategy order:
#     1. Local file  → fastest, works offline (cloned repo)
#     2. Remote download to temp → retry + CDN fallback, validated before exec
# ══════════════════════════════════════════════════════════════════════════════
Write-Host "  ${BLU}📡 Locating install.ps1...${NC}"
Write-Host ""

# -- Strategy 1: Local file (cloned repo) -------------------------------------
$localInstall = if ($PSScriptRoot) { Join-Path $PSScriptRoot "install.ps1" } else { $null }
if ($localInstall -and (Test-Path $localInstall -PathType Leaf)) {
    $localSize = (Get-Item $localInstall).Length
    if ($localSize -gt 1000) {
        Write-Host "  ${GRN}✔ Using local install.ps1 ($([math]::Round($localSize/1KB,1)) KB)${NC}"
        Write-Host ""
        & $script:PS_EXE -NoProfile -ExecutionPolicy Bypass -File $localInstall
        if ($MyInvocation.MyCommand.Path) { exit $LASTEXITCODE } else { return }
    } else {
        Write-Host "  ${YLW}⚠️  Local install.ps1 seems empty/corrupt ($localSize bytes) - downloading fresh copy.${NC}"
    }
}

# -- Strategy 2: Remote download -----------------------------------------------
# Use a named temp file instead of GetTempFileName() rename to avoid race conditions
$tmpDir  = [System.IO.Path]::GetTempPath()
$tmpFile = Join-Path $tmpDir "fancybash_install_$([System.Guid]::NewGuid().ToString('N')).ps1"

$script:exitCode = 1
try {
    $ok = Invoke-DownloadWithRetry -Urls $INSTALL_URLS -OutFile $tmpFile `
          -MaxRetries 3 -TimeoutSec 25 -MinSizeBytes 1000

    if (-not $ok) {
        Write-Host ""
        Write-Host "  ${RED}❌ Failed to download install.ps1 from all sources.${NC}"
        Write-Host "  ${YLW}   Troubleshooting steps:${NC}"
        Write-Host "     1. Check your internet connection: ${CYN}ping raw.githubusercontent.com${NC}"
        Write-Host "     2. Try running manually: ${CYN}irm $($INSTALL_URLS[0]) | iex${NC}"
        Write-Host "     3. Or download and run: ${CYN}$($INSTALL_URLS[0])${NC}"
        $script:exitCode = 1
    } else {
        Write-Host ""
        Write-Host "  ${GRN}🚀 Launching install.ps1...${NC}"
        Write-Host ""
        & $script:PS_EXE -NoProfile -ExecutionPolicy Bypass -File $tmpFile
        $script:exitCode = $LASTEXITCODE
    }
} catch {
    Write-Host "  ${RED}❌ Unexpected error: $($_.Exception.Message)${NC}"
    $script:exitCode = 1
} finally {
# -- Auto-reload profile into host session when run via irm | iex --------------
if ($script:exitCode -eq 0 -and (-not $MyInvocation.MyCommand.Path) -and $PROFILE -and (Test-Path $PROFILE)) {
    try {
        . $PROFILE
    } catch {}
}

if ($MyInvocation.MyCommand.Path) { exit $script:exitCode } else { return }
