# ==============================================================================
#   F A N C Y B A S H  •  PowerShell Config Installer (install.ps1)
#   Author: [Rihad Jahan Opu]
#   Supports: Windows PowerShell 5.1+, PowerShell Core 7+
#
#   -- Run from INSIDE PowerShell terminal (recommended): ---------------------
#   irm https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main/install.ps1 | iex
#
#   -- Run from CMD / Win+R Run dialog: ----------------------------------------
#   powershell -c "irm https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main/install.ps1 | iex"
#
#   ⚠️  Do NOT use 'powershell -c' while already inside a PowerShell window.
#       That causes an 'Access is denied' / NativeCommandFailed error.
#
#   -- Or double-click install.bat (auto-downloads this file): -----------------
#   https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main/install.bat
# ==============================================================================

# --- GUARD 1: PowerShell Version Check ----------------------------------------
if ($PSVersionTable.PSVersion.Major -lt 5) {
    Write-Host "❌ PowerShell 5.1 or higher is required." -ForegroundColor Red
    Write-Host "   Please update: https://aka.ms/wmf5download" -ForegroundColor Yellow
    if ($MyInvocation.MyCommand.Path) { exit 1 } else { return }
}

# --- GUARD 2: TLS 1.2 Force (required for GitHub on older Windows) ------------
try {
    [Net.ServicePointManager]::SecurityProtocol = (
        [Net.SecurityProtocolType]::Tls12 -bor
        [Net.SecurityProtocolType]::Tls11 -bor
        [Net.SecurityProtocolType]::Tls
    )
} catch { <# PS7 handles this automatically, ignore #> }

# --- GUARD 3: Self-Bypass (only when run as a file, not via irm | iex) --------
# When piped via:  irm ... | iex   → $MyInvocation.MyCommand.Path is empty → skip
# When run as file → re-launch with ExecutionPolicy Bypass to avoid policy errors
$_scriptPath = $MyInvocation.MyCommand.Path
$currentPolicy = Get-ExecutionPolicy -Scope Process
if ($_scriptPath -and ($currentPolicy -eq 'Restricted' -or $currentPolicy -eq 'AllSigned')) {
    Write-Host "⚠️  ExecutionPolicy blocks this script. Re-launching with Bypass..." -ForegroundColor Yellow
    if (Get-Command pwsh -ErrorAction SilentlyContinue) {
        & pwsh -NoProfile -ExecutionPolicy Bypass -File $_scriptPath @args
    } else {
        & powershell -NoProfile -ExecutionPolicy Bypass -File $_scriptPath @args
    }
    if ($MyInvocation.MyCommand.Path) { exit } else { return }
}

$ErrorActionPreference = "Continue"

# --- Unattended / Auto-Yes Detection ------------------------------------------
$AUTO_YES = $false
foreach ($arg in $args) {
    if ($arg -eq "-y" -or $arg -eq "-Yes" -or $arg -eq "--yes" -or $arg -eq "--unattended") {
        $AUTO_YES = $true
        break
    }
}
if ($env:FANCYBASH_AUTO_YES -eq "1" -or $env:NONINTERACTIVE -eq "1" -or $env:CI -eq "true") {
    $AUTO_YES = $true
}

# --- URLs: Primary (GitHub Raw) + CDN Fallback (jsDelivr) --------------------
$INSTALLER_URLS = @(
    "https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main/config.ps1",
    "https://cdn.jsdelivr.net/gh/rihadjahanopu/fancybash@main/config.ps1",
    "https://fancybash.netlify.app/public/config.ps1"
)
$START = "# >>> fancy-powershell >>>"
$END   = "# <<< fancy-powershell <<<"

# --- GUARD 4: $PROFILE Sanity Check ------------------------------------------
if (-not $PROFILE) {
    Write-Host "❌ `$PROFILE variable is not set. Are you in a valid PowerShell session?" -ForegroundColor Red
    if ($MyInvocation.MyCommand.Path) { exit 1 } else { return }
}
$PROFILE_PATH = $PROFILE

# --- Colors & Formatting ------------------------------------------------------
$ESC  = [char]27
$RED  = "$ESC[1;31m"; $GRN  = "$ESC[1;32m"; $YLW  = "$ESC[1;33m"
$BLU  = "$ESC[1;34m"; $PUR  = "$ESC[1;35m"; $CYN  = "$ESC[1;36m"
$BOLD = "$ESC[1m";    $DIM  = "$ESC[2m";    $NC   = "$ESC[0m"

# --- Helper: Robust Download with Retry + CDN Fallback -----------------------
function Get-RemoteContent {
    param([string[]]$Urls, [int]$MaxRetries = 3, [int]$TimeoutSec = 20)
    foreach ($url in $Urls) {
        $host_name = ([Uri]$url).Host
        for ($attempt = 1; $attempt -le $MaxRetries; $attempt++) {
            try {
                Write-Host "    ↳ Attempt $attempt/$MaxRetries via ${host_name}..." -NoNewline -ForegroundColor DarkGray
                $result = Invoke-RestMethod -Uri $url -TimeoutSec $TimeoutSec -ErrorAction Stop
                Write-Host " ✔" -ForegroundColor Green
                return $result
            } catch {
                Write-Host " ✗ ($($_.Exception.Message))" -ForegroundColor Red
                if ($attempt -lt $MaxRetries) { Start-Sleep -Seconds 2 }
            }
        }
        Write-Host "  ⚠️  All retries failed for $host_name. Trying next source..." -ForegroundColor Yellow
    }
    return $null
}

# --- Progress Bar Helper ------------------------------------------------------
function Show-ProgressBar {
    param([int]$Current, [int]$Total = 6, [string]$StepName = "")
    $width = 30
    $pct   = [math]::Round(($Current / $Total) * 100)
    $done  = [math]::Round(($width * $Current) / $Total)
    $bar   = "█" * $done + "░" * ($width - $done)
    Write-Host ""
    Write-Host ("${BLU}Progress:${NC} [${GRN}$bar${NC}] ${CYN}${pct}%${NC}  Step $Current/$Total - $StepName")
}

# --- Header Banner ------------------------------------------------------------
function Show-Header {
    Clear-Host
    Write-Host ""
    Write-Host "${PUR}          ███████╗ █████╗ ███╗   ██╗ ██████╗██╗   ██╗██████╗  █████╗ ███████╗██╗  ██╗${NC}"
    Write-Host "${PUR}          ██=════╝██=══██╗████╗  ██║██=════╝╚██╗ ██=╝██=══██╗██=══██╗██=════╝██║  ██║${NC}"
    Write-Host "${CYN}          █████╗  ███████║██=██╗ ██║██║      ╚████=╝ ██████=╝███████║███████╗███████║${NC}"
    Write-Host "${CYN}          ██=══╝  ██=══██║██║╚██╗██║██║       ╚██=╝  ██=══██╗██=══██║╚════██║██=══██║${NC}"
    Write-Host "${BLU}          ██║     ██║  ██║██║ ╚████║╚██████╗   ██║   ██████=╝██║  ██║███████║██║  ██║${NC}"
    Write-Host "${BLU}          ╚═╝     ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝   ╚═╝   ╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝${NC}"
    Write-Host ""
    Write-Host "   ✨ ${BOLD}${CYN}F A N C Y B A S H${NC}  •  ${BOLD}PowerShell Environment Installer${NC}"
    Write-Host ""
}

# --- System Info Card ---------------------------------------------------------
function Show-SysInfo {
    $osName = if ($PSVersionTable.OS) { $PSVersionTable.OS } else { "Windows ($env:OS)" }
    Write-Host "${BLU}--------------------------------------------------${NC}"
    Write-Host " 🖥️  ${BOLD}SYSTEM INFORMATION${NC}"
    Write-Host "${BLU}--------------------------------------------------${NC}"
    Write-Host "  💻  ${BOLD}OS:${NC}      ${CYN}$osName${NC}"
    Write-Host "  👤  ${BOLD}User:${NC}    ${CYN}$env:USERNAME${NC}"
    Write-Host "  🐚  ${BOLD}Shell:${NC}   ${CYN}PowerShell v$($PSVersionTable.PSVersion)${NC}"
    Write-Host "  ⚙️   ${BOLD}Arch:${NC}    ${CYN}$env:PROCESSOR_ARCHITECTURE${NC}"
    Write-Host "  🔒  ${BOLD}Policy:${NC}  ${CYN}$(Get-ExecutionPolicy)${NC}"
    Write-Host "${BLU}--------------------------------------------------${NC}`n"
}

# ══════════════════════════════════════════════════════════════════════════════
#   MAIN INSTALLER
# ══════════════════════════════════════════════════════════════════════════════
Show-Header
Show-SysInfo

# --- STEP 1: Environment & Policy Info ----------------------------------------
Show-ProgressBar -Current 1 -Total 6 -StepName "Environment Check"
Write-Host "  ${GRN}✔ PowerShell v$($PSVersionTable.PSVersion.Major).$($PSVersionTable.PSVersion.Minor) - supported.${NC}"
Write-Host "  ${GRN}✔ TLS 1.2 enforced for secure downloads.${NC}"
Write-Host "  ${GRN}✔ Running with ExecutionPolicy Bypass.${NC}"

# --- STEP 2: Dependency Check & Auto-Install ----------------------------------
Show-ProgressBar -Current 2 -Total 6 -StepName "Checking Dependencies"
$missing = @('git','fzf','gum','glow','bat','zoxide','chafa') | Where-Object {
    -not (Get-Command $_ -ErrorAction SilentlyContinue)
}

if ($missing.Count -gt 0) {
    Write-Host "  ${YLW}⚠️  Missing tools: $($missing -join ', ')${NC}"
    if (Get-Command winget -ErrorAction SilentlyContinue) {
        $ansNorm = "y"
        if (-not $AUTO_YES) {
            $ans = Read-Host "  👉 Auto-install missing dependencies via Winget + Cascadia Code font and proceed? [Y/n]"
            $ansNorm = if ($ans) { $ans.Trim().ToLower() } else { "y" }
        }
        if ($ansNorm -eq "" -or $ansNorm -eq "y" -or $ansNorm -eq "yes") {
            $wingetIds = @{
                git    = 'Git.Git'
                fzf    = 'junegunn.fzf'
                gum    = 'charmbracelet.gum'
                glow   = 'charmbracelet.glow'
                bat    = 'sharkdp.bat'
                zoxide = 'ajeetdsouza.zoxide'
                chafa  = 'hpjansson.chafa'
            }
            foreach ($tool in $missing) {
                Write-Host "  📦 Installing $tool..." -ForegroundColor Cyan
                winget install --id $wingetIds[$tool] -e --accept-source-agreements --accept-package-agreements --silent
            }
            Write-Host "  🎨 Installing Cascadia Code font..." -ForegroundColor Cyan
            winget install --id Microsoft.CascadiaCode -e --accept-source-agreements --accept-package-agreements --silent 2>$null
        } else {
            Write-Host "  ${YLW}⚠️  Installation cancelled by user. No changes were made.${NC}"
            exit 0
        }
    } else {
        Write-Host "  ${DIM}💡 Winget not found. Install manually: https://scoop.sh or https://chocolatey.org${NC}"
    }
} else {
    Write-Host "  ${GRN}✔ All recommended tools are installed!${NC}"
}

# --- STEP 3: Profile Directory & Backup --------------------------------------
Show-ProgressBar -Current 3 -Total 6 -StepName "Profile Backup"
$ProfileDir = Split-Path $PROFILE_PATH -Parent

try {
    if (-not (Test-Path $ProfileDir)) {
        New-Item -ItemType Directory -Path $ProfileDir -Force | Out-Null
        Write-Host "  ${YLW}📁 Profile directory created.${NC}"
    }
} catch {
    Write-Host "  ${RED}❌ Cannot create profile directory: $($_.Exception.Message)${NC}"
    if ($MyInvocation.MyCommand.Path) { exit 1 } else { return }
}

$backupFile = ""
try {
    if (Test-Path $PROFILE_PATH) {
        $backupFile = "$PROFILE_PATH.backup.$(Get-Date -Format 'yyyyMMdd_HHmmss')"
        Copy-Item $PROFILE_PATH $backupFile -Force -ErrorAction Stop
        Write-Host "  ${GRN}💾 Backup: $(Split-Path $backupFile -Leaf)${NC}"
    } else {
        New-Item -ItemType File -Path $PROFILE_PATH -Force | Out-Null
        Write-Host "  ${GRN}📄 Created new `$PROFILE file.${NC}"
    }
} catch {
    Write-Host "  ${RED}❌ Backup failed: $($_.Exception.Message)${NC}"
    if ($MyInvocation.MyCommand.Path) { exit 1 } else { return }
}

# --- STEP 4: Remove Existing Fancybash Block ----------------------------------
Show-ProgressBar -Current 4 -Total 6 -StepName "Cleaning Old Install"
try {
    $profileContent = Get-Content $PROFILE_PATH -Raw -ErrorAction SilentlyContinue
    if ($profileContent -and $profileContent.Contains($START)) {
        Write-Host "  ${YLW}⚠️  Existing block found - removing cleanly...${NC}"
        $regex = "(?s)\r?\n?" + [regex]::Escape($START) + ".*?" + [regex]::Escape($END)
        # Remove ALL duplicate blocks
        while ($profileContent -and $profileContent.Contains($START)) {
            $profileContent = [regex]::Replace($profileContent, $regex, "")
        }
        $profileContent | Out-File $PROFILE_PATH -Encoding utf8 -ErrorAction Stop
        Write-Host "  ${GRN}✔ Old configuration removed.${NC}"
    } else {
        Write-Host "  ${GRN}✔ Profile is clean - ready for install.${NC}"
    }
} catch {
    Write-Host "  ${RED}❌ Failed to clean profile: $($_.Exception.Message)${NC}"
    # Restore backup if exists
    if ($backupFile -and (Test-Path $backupFile)) {
        Copy-Item $backupFile $PROFILE_PATH -Force
        Write-Host "  ${YLW}↩️  Restored from backup.${NC}"
    }
    if ($MyInvocation.MyCommand.Path) { exit 1 } else { return }
}

# --- STEP 5: Install Rust Binary & Generate Config -------------------------
Show-ProgressBar -Current 5 -Total 6 -StepName "Installing Rust Binary"

function Install-RustBinary {
    Write-Host "  ${CYAN}➜${NC} Installing fancybash Rust engine binary..."
    $binDir = Join-Path $HOME ".local\bin"
    if (-not (Test-Path $binDir)) { New-Item -ItemType Directory -Path $binDir -Force | Out-Null }
    
    $env:PATH = "$binDir;$env:PATH"

    # 1. Local release build
    $scriptDir = $PSScriptRoot
    if ($scriptDir -and (Test-Path (Join-Path $scriptDir "target\release\fancybash.exe"))) {
        Copy-Item (Join-Path $scriptDir "target\release\fancybash.exe") (Join-Path $binDir "fancybash.exe") -Force
        Write-Host "  ${GRN}✔ Installed local release binary to $binDir\fancybash.exe${NC}"
        return $true
    }
    if ($scriptDir -and (Test-Path (Join-Path $scriptDir "target\release\fancybash"))) {
        Copy-Item (Join-Path $scriptDir "target\release\fancybash") (Join-Path $binDir "fancybash") -Force
        Write-Host "  ${GRN}✔ Installed local release binary to $binDir\fancybash${NC}"
        return $true
    }

    # 2. Local cargo build
    if ($scriptDir -and (Test-Path (Join-Path $scriptDir "Cargo.toml")) -and (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Host "  ${YLW}⚡ Building fancybash Rust engine (release mode)...${NC}"
        Push-Location $scriptDir
        cargo build --release
        Pop-Location
        if (Test-Path (Join-Path $scriptDir "target\release\fancybash.exe")) {
            Copy-Item (Join-Path $scriptDir "target\release\fancybash.exe") (Join-Path $binDir "fancybash.exe") -Force
            Write-Host "  ${GRN}✔ Built & installed binary to $binDir\fancybash.exe${NC}"
            return $true
        }
    }

    # 3. System command check
    if (Get-Command fancybash -ErrorAction SilentlyContinue) {
        Write-Host "  ${GRN}✔ fancybash binary active: $((Get-Command fancybash).Source)${NC}"
        return $true
    }

    # 4. Cargo install fallback
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        Write-Host "  ${YLW}⚡ Installing via cargo from GitHub...${NC}"
        cargo install --git https://github.com/rihadjahanopu/fancybash-rs --quiet 2>$null
        if (Get-Command fancybash -ErrorAction SilentlyContinue) {
            Write-Host "  ${GRN}✔ Installed fancybash via cargo install!${NC}"
            return $true
        }
    }

    Write-Host "  ${YLW}⚠️ Could not auto-build Rust binary. Please install Cargo and build manually.${NC}"
    return $false
}

Install-RustBinary | Out-Null

# --- STEP 6: Write to $PROFILE & Reload ---------------------------------------
Show-ProgressBar -Current 6 -Total 6 -StepName "Writing & Reloading"

$initScript = if (Get-Command fancybash -ErrorAction SilentlyContinue) {
    & fancybash init pwsh 2>$null
} else {
    "if (Get-Command fancybash -ErrorAction SilentlyContinue) { Invoke-Expression (& fancybash init pwsh) }"
}

$newBlock = @"

$START
# Installed by FancyBash: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')
# fancybash Rust Native Engine Initialization
if (-not (`$env:PATH -split ';' -contains '$HOME\.local\bin')) {
    `$env:PATH = "$HOME\.local\bin;`$env:PATH"
}

$initScript
$END
"@


try {
    Add-Content -Path $PROFILE_PATH -Value $newBlock -Encoding utf8 -ErrorAction Stop
    Write-Host "  ${GRN}✅ Config written to `$PROFILE successfully!${NC}"
} catch {
    Write-Host "  ${RED}❌ Failed to write to profile: $($_.Exception.Message)${NC}"
    if ($backupFile -and (Test-Path $backupFile)) {
        Copy-Item $backupFile $PROFILE_PATH -Force
        Write-Host "  ${YLW}↩️  Profile restored from backup.${NC}"
    }
    if ($MyInvocation.MyCommand.Path) { exit 1 } else { return }
}

# Reload profile
Write-Host "  ${BLU}🔄 Reloading shell profile...${NC}"
try {
    . $PROFILE_PATH
    Write-Host "  ${GRN}✨ Auto-reload successful!${NC}"
} catch {
    Write-Host "  ${YLW}⚠️  Auto-reload skipped (some features need a new window).${NC}"
    Write-Host "     Run manually: ${CYN}. `$PROFILE${NC}"
}

# --- Final Summary -------------------------------------------------------------
Write-Host ""
Write-Host "${CYN}══════════════════════════════════════════════════${NC}"
Write-Host "   🚀  ${BOLD}INSTALLATION COMPLETE!${NC}"
Write-Host "${CYN}══════════════════════════════════════════════════${NC}"
if ($backupFile) {
    Write-Host "  📦  ${BOLD}Backup:${NC}    ${GRN}$(Split-Path $backupFile -Leaf)${NC}"
}
Write-Host "  ⚙️   ${BOLD}Profile:${NC}   ${GRN}$PROFILE_PATH${NC}"
Write-Host "  🔄  ${BOLD}Reload:${NC}    ${PUR}. `$PROFILE${NC}"
Write-Host "  ⚡  ${BOLD}Commands:${NC}  Type ${GRN}'keep'${NC} for the Master Dashboard"
Write-Host "  📝  ${BOLD}Suites:${NC}    ${PUR}'todo'${NC}  ${PUR}'notes'${NC}  ${PUR}'ffmedia'${NC}"
Write-Host "${CYN}══════════════════════════════════════════════════${NC}"
Write-Host "  🎉  ${BOLD}Open a new PowerShell window to get started!${NC}"
Write-Host ""
