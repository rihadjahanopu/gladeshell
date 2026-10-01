#!/usr/bin/env pwsh
# ==============================================================================
#   G L A D E S H E L L  •  Smart Production Windows Engine Installer (install.ps1)
#   Author: Rihad Jahan Opu
#   Supports: Windows PowerShell 5.1+, PowerShell Core 7+, Git Bash
# ==============================================================================

param(
    [String]$Version = "latest",
    [Switch]$ForceBaseline = $false,
    [Switch]$NoPathUpdate = $false,
    [Switch]$NoRegisterInstallation = $false,
    [Switch]$DownloadWithoutCurl = $false,
    [Switch]$Doctor = $false,
    [Switch]$Check = $false,
    [Switch]$Rollback = $false,
    [Switch]$Undo = $false,
    [Switch]$Yes = $false,
    [Switch]$Unattended = $false,
    [Switch]$AllShells = $false
)

# --- GUARD 1: PowerShell Version Check ----------------------------------------
if ($PSVersionTable.PSVersion.Major -lt 5) {
    Write-Host "❌ PowerShell 5.1 or higher is required." -ForegroundColor Red
    Write-Host "   Please update: https://aka.ms/wmf5download" -ForegroundColor Yellow
    if ($MyInvocation.MyCommand.Path) { exit 1 } else { return }
}

# --- GUARD 2: Minimum Supported Windows Version Check (Win10 RS5 Build 17763+) --
$MinBuild = 17763
$WinVer = [System.Environment]::OSVersion.Version
if ($WinVer.Major -ge 10 -and $WinVer.Build -lt $MinBuild) {
    Write-Warning "Gladeshell recommends Windows 10 Build 17763 (1809) or newer.`nInstallation will continue."
}

# --- GUARD 3: TLS 1.2 Force for GitHub Downloads -----------------------------
try {
    [Net.ServicePointManager]::SecurityProtocol = (
        [Net.SecurityProtocolType]::Tls12 -bor
        [Net.SecurityProtocolType]::Tls11 -bor
        [Net.SecurityProtocolType]::Tls
    )
} catch { <# PS7 handles automatically #> }

# --- GUARD 4: Self-Bypass ExecutionPolicy -------------------------------------
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

# --- Argument Normalization ---------------------------------------------------
$AUTO_YES = $Yes.IsPresent -or $Unattended.IsPresent -or ($env:GLADESHELL_AUTO_YES -eq "1") -or ($env:NONINTERACTIVE -eq "1") -or ($env:CI -eq "true")
$MODE = "install"
if ($Doctor.IsPresent -or $Check.IsPresent) { $MODE = "doctor" }
if ($Rollback.IsPresent -or $Undo.IsPresent) { $MODE = "rollback" }

# Parse raw args for positional fallback
foreach ($arg in $args) {
    switch -Regex ($arg) {
        "^(-y|-Yes|--yes|--unattended)$" { $AUTO_YES = $true }
        "^(-Doctor|-Check|--doctor|--check|doctor|check)$" { $MODE = "doctor" }
        "^(-Rollback|-Undo|--rollback|--undo|rollback|undo)$" { $MODE = "rollback" }
        "^(-All|-AllShells|--all|--all-shells)$" { $ALL_SHELLS = $true }
    }
}

$START = "# >>> glade-powershell >>>"
$END   = "# <<< glade-powershell <<<"

# --- Colors & Formatting ------------------------------------------------------
$ESC  = [char]27
$RED  = "$ESC[1;31m"; $GRN  = "$ESC[1;32m"; $YLW  = "$ESC[1;33m"
$BLU  = "$ESC[1;34m"; $PUR  = "$ESC[1;35m"; $CYN  = "$ESC[1;36m"
$BOLD = "$ESC[1m";    $DIM  = "$ESC[2m";    $NC   = "$ESC[0m"

# --- Win32 Native Environment Broadcast (No Unwanted Variable Expansion) -----
function Publish-Env {
    if (-not ("Win32.NativeMethods" -as [Type])) {
        try {
            Add-Type -Namespace Win32 -Name NativeMethods -MemberDefinition @"
[DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
public static extern IntPtr SendMessageTimeout(
    IntPtr hWnd, uint Msg, UIntPtr wParam, string lParam,
    uint fuFlags, uint uTimeout, out UIntPtr lpdwResult);
"@
            $HWND_BROADCAST = [IntPtr] 0xffff
            $WM_SETTINGCHANGE = 0x1a
            $result = [UIntPtr]::Zero
            [Win32.NativeMethods]::SendMessageTimeout($HWND_BROADCAST, $WM_SETTINGCHANGE, [UIntPtr]::Zero, "Environment", 2, 5000, [ref] $result) | Out-Null
        } catch {}
    }
}

function Write-UserEnvPath {
    param([string]$BinPath)
    try {
        $regKey = Get-Item -Path 'HKCU:'
        $envKey = $regKey.OpenSubKey('Environment', $true)
        if ($envKey) {
            $rawPath = $envKey.GetValue('Path', '', [Microsoft.Win32.RegistryValueOptions]::DoNotExpandEnvironmentNames)
            $pathList = $rawPath -split ';' | Where-Object { $_ }
            if ($pathList -notcontains $BinPath) {
                $newPath = ($pathList + $BinPath) -join ';'
                $kind = if ($newPath.Contains('%')) { [Microsoft.Win32.RegistryValueKind]::ExpandString } else { [Microsoft.Win32.RegistryValueKind]::String }
                $envKey.SetValue('Path', $newPath, $kind)
                Publish-Env
                Write-Host "  ${GRN}✔ Added $BinPath to User %PATH% and broadcasted Environment change!${NC}"
            }
            $envKey.Close()
        }
    } catch {}
}

# --- CPU Architecture & Hardware Probing (AVX2 Check) -------------------------
function Test-Avx2Supported {
    try {
        if (-not ("Win32.Kernel32" -as [Type])) {
            Add-Type -MemberDefinition '[DllImport("kernel32.dll")] public static extern bool IsProcessorFeaturePresent(int ProcessorFeature);' -Name 'Kernel32' -Namespace 'Win32'
        }
        # 40 = PF_AVX2_INSTRUCTIONS_AVAILABLE
        return [Win32.Kernel32]::IsProcessorFeaturePresent(40)
    } catch {
        return $true
    }
}

# --- Windows Add/Remove Programs Registry Registration ------------------------
function Register-WindowsInstallation {
    param([string]$InstallDir, [string]$ExePath)
    if ($NoRegisterInstallation) { return }
    try {
        $RegistryKey = "HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\Gladeshell"
        $null = New-Item -Path $RegistryKey -Force
        New-ItemProperty -Path $RegistryKey -Name "DisplayName" -Value "Gladeshell Rust Engine" -PropertyType String -Force | Out-Null
        New-ItemProperty -Path $RegistryKey -Name "InstallLocation" -Value $InstallDir -PropertyType String -Force | Out-Null
        New-ItemProperty -Path $RegistryKey -Name "DisplayIcon" -Value $ExePath -PropertyType String -Force | Out-Null
        New-ItemProperty -Path $RegistryKey -Name "Publisher" -Value "Rihad Jahan Opu" -PropertyType String -Force | Out-Null
        New-ItemProperty -Path $RegistryKey -Name "UninstallString" -Value "powershell -c `"& `'$ExePath`' uninstall`" -ExecutionPolicy Bypass" -PropertyType String -Force | Out-Null
    } catch {}
}

# --- Header Banner ------------------------------------------------------------
function Show-Header {
    try { Clear-Host } catch {}
    Write-Host ""
    Write-Host "${PUR}          ██████╗ ██╗    █████╗ ██████╗ ███████╗███████╗██╗  ██╗███████╗██╗   ██╗${NC}"
    Write-Host "${PUR}         ██=════╝ ██║   ██=══██╗██=══██╗██=════╝██=════╝██║  ██║██=════╝██║   ██║${NC}"
    Write-Host "${CYN}         ██║  ███╗██║   ███████║██║  ██║█████╗  ███████╗███████║█████╗  ██║   ██║${NC}"
    Write-Host "${CYN}         ██║   ██║██║   ██=══██║██║  ██║██=══╝  ╚════██║██║  ██║██=══╝  ██║   ██║${NC}"
    Write-Host "${BLU}         ╚██████=╝██████╗██║  ██║██████=╝███████╗███████║██║  ██║███████╗██████╗██████╗${NC}"
    Write-Host "${BLU}          ╚═════╝ ╚═════╝╚═╝  ╚═╝╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝╚══════╝╚═════╝╚═════╝${NC}"
    Write-Host ""
    Write-Host "   ✨ ${BOLD}${CYN}G L A D E S H E L L${NC}  •  ${BOLD}Smart Production Windows & PowerShell Cross-Engine${NC}"
    Write-Host ""
}

# --- System Info Card ---------------------------------------------------------
function Show-SysInfo {
    $osName = if ($PSVersionTable.OS) { $PSVersionTable.OS } else { "Windows ($env:OS)" }
    $arch = (Get-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment' -ErrorAction SilentlyContinue).PROCESSOR_ARCHITECTURE
    if (-not $arch) { $arch = $env:PROCESSOR_ARCHITECTURE }
    $avx2Status = if (Test-Avx2Supported) { "AVX2 Supported" } else { "Baseline CPU (No AVX2)" }

    Write-Host "${BLU}--------------------------------------------------${NC}"
    Write-Host " 🖥️  ${BOLD}SYSTEM INFORMATION${NC}"
    Write-Host "${BLU}--------------------------------------------------${NC}"
    Write-Host "  💻  ${BOLD}OS:${NC}        ${CYAN}$osName${NC}"
    Write-Host "  👤  ${BOLD}User:${NC}      ${CYAN}$env:USERNAME${NC}"
    Write-Host "  🐚  ${BOLD}Shell:${NC}     ${CYAN}PowerShell v$($PSVersionTable.PSVersion)${NC}"
    Write-Host "  📄  ${BOLD}Profile:${NC}   ${CYAN}$PROFILE${NC}"
    Write-Host "  ⚙️   ${BOLD}Arch:${NC}      ${CYAN}$arch ($avx2Status)${NC}"
    Write-Host "  🔒  ${BOLD}Policy:${NC}    ${CYAN}$(Get-ExecutionPolicy)${NC}"
    Write-Host "${BLU}--------------------------------------------------${NC}`n"
}

# --- Progress Bar Helper ------------------------------------------------------
function Show-ProgressBar {
    param([int]$Current, [int]$Total = 5, [string]$StepName = "")
    $width = 30
    $pct   = [math]::Round(($Current / $Total) * 100)
    $done  = [math]::Round(($width * $Current) / $Total)
    $bar   = "█" * $done + "░" * ($width - $done)
    Write-Host ""
    Write-Host ("${BLU}Progress:${NC} [${GRN}$bar${NC}] ${CYN}${pct}%${NC}  Step $Current/$Total - $StepName")
}

# --- Atomic File Injector Helper ----------------------------------------------
function Write-AtomicFile {
    param([string]$Path, [string]$Content)
    $dir = Split-Path $Path -Parent
    if (-not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
    $tempFile = Join-Path $dir (".gladeshell_tmp_" + [System.IO.Path]::GetRandomFileName())
    if (Test-Path $Path) {
        Copy-Item $Path $tempFile -Force
    } else {
        New-Item -ItemType File -Path $tempFile -Force | Out-Null
    }
    Add-Content -Path $tempFile -Value $Content -Encoding utf8 -ErrorAction Stop
    Move-Item -Path $tempFile -Destination $Path -Force
}

# ══════════════════════════════════════════════════════════════════════════════
#   DOCTOR / DIAGNOSTICS MODE
# ══════════════════════════════════════════════════════════════════════════════
function Invoke-Doctor {
    Show-Header
    Write-Host "  🩺 ${BOLD}${CYN}GLADESHELL SYSTEM DOCTOR (WINDOWS)${NC}"
    Write-Host "  --------------------------------------------------`n"

    Write-Host "  🐚 PowerShell Version: ${CYN}v$($PSVersionTable.PSVersion)${NC}"
    Write-Host "  🔒 Execution Policy:  ${CYN}$(Get-ExecutionPolicy)${NC}"
    Write-Host "  ⚙️ Hardware AVX2:      $(if (Test-Avx2Supported) { "${GRN}[SUPPORTED]${NC}" } else { "${YLW}[BASELINE ONLY]${NC}" })"

    $binDir = Join-Path $HOME ".local\bin"
    $env:PATH = "$binDir;$env:PATH"
    if (Get-Command gladeshell -ErrorAction SilentlyContinue) {
        $cmd = Get-Command gladeshell
        $ver = try { & gladeshell --version } catch { "unknown" }
        Write-Host "  ⚡ Binary:            ${GRN}[OK]${NC} $($cmd.Source) (${CYN}$ver${NC})"
    } else {
        Write-Host "  ⚡ Binary:            ${RED}[MISSING]${NC} gladeshell binary not found in PATH"
    }

    if (Test-Path $PROFILE) {
        $content = Get-Content $PROFILE -Raw -ErrorAction SilentlyContinue
        if ($content -and $content.Contains("gladeshell init")) {
            Write-Host "  📄 Profile Hook:       ${GRN}[OK]${NC} $PROFILE"
        } else {
            Write-Host "  📄 Profile Hook:       ${YLW}[MISSING]${NC} No gladeshell hook in $PROFILE"
        }
    } else {
        Write-Host "  📄 Profile Hook:       ${YLW}[MISSING]${NC} Profile file does not exist ($PROFILE)"
    }

    Write-Host "`n  --------------------------------------------------"
    Write-Host "  🎉 Doctor check completed.`n"
    exit 0
}

# ══════════════════════════════════════════════════════════════════════════════
#   ROLLBACK / RESTORE MODE
# ══════════════════════════════════════════════════════════════════════════════
function Invoke-Rollback {
    Show-Header
    Write-Host "  🔄 ${BOLD}${YLW}GLADESHELL ROLLBACK & RESTORE${NC}"
    Write-Host "  --------------------------------------------------`n"

    if (Test-Path $PROFILE) {
        $profileContent = Get-Content $PROFILE -Raw -ErrorAction SilentlyContinue
        if ($profileContent -and $profileContent.Contains($START)) {
            $regex = "(?s)\r?\n?" + [regex]::Escape($START) + ".*?" + [regex]::Escape($END)
            $profileContent = [regex]::Replace($profileContent, $regex, "")
            $profileContent | Out-File $PROFILE -Encoding utf8 -Force
            Write-Host "  ${GRN}✔ Removed gladeshell configuration block from $PROFILE${NC}"
        fi
    }

    $parentDir = Split-Path $PROFILE -Parent
    $latestBackup = Get-ChildItem -Path $parentDir -Filter "$([System.IO.Path]::GetFileName($PROFILE)).backup.*" 2>$null | Sort-Object LastWriteTime -Descending | Select-Object -First 1

    if ($latestBackup) {
        Copy-Item $latestBackup.FullName $PROFILE -Force
        Write-Host "  ${GRN}✔ Restored profile from backup: $($latestBackup.Name)${NC}"
    } else {
        Write-Host "  ${DIM}ℹ No profile backup files found to restore.${NC}"
    }

    Write-Host "`n  🎉 Rollback completed.`n"
    exit 0
}

if ($MODE -eq "doctor") { Invoke-Doctor }
if ($MODE -eq "rollback") { Invoke-Rollback }

# ══════════════════════════════════════════════════════════════════════════════
#   MAIN INSTALLER
# ══════════════════════════════════════════════════════════════════════════════
Show-Header
Show-SysInfo

# --- STEP 1: Environment Check ------------------------------------------------
Show-ProgressBar -Current 1 -Total 5 -StepName "Environment Check"
Write-Host "  ${GRN}✔ PowerShell v$($PSVersionTable.PSVersion.Major).$($PSVersionTable.PSVersion.Minor) - supported.${NC}"
Write-Host "  ${GRN}✔ TLS 1.2 enforced for secure downloads.${NC}"
Write-Host "  ${GRN}✔ Running with ExecutionPolicy Bypass.${NC}"

# --- STEP 2: Dependency Check & Auto-Install ----------------------------------
Show-ProgressBar -Current 2 -Total 5 -StepName "Checking Dependencies"
$missing = @('git') | Where-Object {
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
            $wingetIds = @{ git = 'Git.Git' }
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

# --- STEP 3: Profile Backup ---------------------------------------------------
Show-ProgressBar -Current 3 -Total 5 -StepName "Profile Backup & Cleanup"
$ProfileDir = Split-Path $PROFILE -Parent
if (-not (Test-Path $ProfileDir)) {
    New-Item -ItemType Directory -Path $ProfileDir -Force | Out-Null
}

$backupFile = ""
if (Test-Path $PROFILE) {
    $backupFile = "$PROFILE.backup.$(Get-Date -Format 'yyyyMMdd_HHmmss')"
    Copy-Item $PROFILE $backupFile -Force
    Write-Host "  ${GRN}💾 Backup created: $(Split-Path $backupFile -Leaf)${NC}"
} else {
    New-Item -ItemType File -Path $PROFILE -Force | Out-Null
    Write-Host "  ${GRN}📄 Created new `$PROFILE file.${NC}"
}

# Clean existing blocks cleanly
try {
    $profileContent = Get-Content $PROFILE -Raw -ErrorAction SilentlyContinue
    if ($profileContent -and $profileContent.Contains($START)) {
        Write-Host "  ${YLW}⚠️  Cleaning existing gladeshell block...${NC}"
        $regex = "(?s)\r?\n?" + [regex]::Escape($START) + ".*?" + [regex]::Escape($END)
        while ($profileContent -and $profileContent.Contains($START)) {
            $profileContent = [regex]::Replace($profileContent, $regex, "")
        }
        $profileContent | Out-File $PROFILE -Encoding utf8 -Force
        Write-Host "  ${GRN}✔ Profile cleaned.${NC}"
    }
} catch {}

# --- STEP 4: Install Rust Binary ---------------------------------------------
Show-ProgressBar -Current 4 -Total 5 -StepName "Installing Rust Engine Binary"

function Install-RustBinary {
    Write-Host "  ${CYAN}➜${NC} Installing gladeshell Rust engine binary..."
    $binDir = Join-Path $HOME ".local\bin"
    if (-not (Test-Path $binDir)) { New-Item -ItemType Directory -Path $binDir -Force | Out-Null }
    $env:PATH = "$binDir;$env:PATH"

    $scriptDir = $PSScriptRoot
    if ($scriptDir -and (Test-Path (Join-Path $scriptDir "target\release\gladeshell.exe"))) {
        Copy-Item (Join-Path $scriptDir "target\release\gladeshell.exe") (Join-Path $binDir "gladeshell.exe") -Force
        Write-Host "  ${GRN}✔ Installed local release binary to $binDir\gladeshell.exe${NC}"
        Register-WindowsInstallation -InstallDir $binDir -ExePath (Join-Path $binDir "gladeshell.exe")
        if (-not $NoPathUpdate) { Write-UserEnvPath -BinPath $binDir }
        return $true
    }

    $realArch = (Get-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment' -ErrorAction SilentlyContinue).PROCESSOR_ARCHITECTURE
    $isArm64 = ($realArch -eq "ARM64")
    $hasAvx2 = Test-Avx2Supported

    $repos = @("rihadjahanopu/gladeshell", "rihadjahanopu/gladeshell")
    $assets = if ($isArm64) {
        @("gladeshell-windows-arm64.exe", "gladeshell-aarch64-pc-windows-msvc.exe", "gladeshell-windows-amd64.exe", "gladeshell.exe")
    } elseif (-not $hasAvx2 -or $ForceBaseline) {
        @("gladeshell-windows-amd64-baseline.exe", "gladeshell-x86_64-pc-windows-msvc-baseline.exe", "gladeshell-windows-amd64.exe", "gladeshell.exe")
    } else {
        @("gladeshell-windows-amd64.exe", "gladeshell-x86_64-pc-windows-msvc.exe", "gladeshell.exe")
    }

    $targetExe = Join-Path $binDir "gladeshell.exe"

    try {
        $openProc = Get-Process -Name gladeshell -ErrorAction SilentlyContinue | Where-Object { $_.Path -eq $targetExe }
        if ($openProc) {
            Stop-Process -InputObject $openProc -Force -ErrorAction SilentlyContinue
        }
    } catch {}

    Write-Host "  ${CYAN}⚡ Attempting GitHub Release pre-built binary download...${NC}"
    foreach ($repo in $repos) {
        foreach ($asset in $assets) {
            $url = "https://github.com/$repo/releases/latest/download/$asset"
            $downloadSuccess = $false

            if (-not $DownloadWithoutCurl -and (Get-Command curl.exe -ErrorAction SilentlyContinue)) {
                try {
                    curl.exe -#SfLo "$targetExe" "$url" 2>$null
                    if ($LASTEXITCODE -eq 0 -and (Test-Path $targetExe) -and ((Get-Item $targetExe).Length -gt 0)) {
                        $downloadSuccess = $true
                    }
                } catch {}
            }

            if (-not $downloadSuccess) {
                try {
                    Invoke-RestMethod -Uri $url -OutFile $targetExe -ErrorAction Stop
                    if ((Test-Path $targetExe) -and ((Get-Item $targetExe).Length -gt 0)) {
                        $downloadSuccess = $true
                    }
                } catch {}
            }

            if ($downloadSuccess) {
                # Sanity test binary & trap STATUS_ILLEGAL_INSTRUCTION
                $testVer = try { & "$targetExe" --version 2>$null } catch { $null }
                if ($LASTEXITCODE -eq 1073741795 -or $LASTEXITCODE -eq -1073741795) {
                    Write-Host "  ${YLW}⚠️ Executable incompatible with CPU instruction set. Falling back to baseline build...${NC}"
                    if (-not $ForceBaseline) {
                        return Install-RustBinary -ForceBaseline $true
                    }
                }

                Write-Host "  ${GRN}✔ Downloaded latest pre-built binary from GitHub Release ($repo)!${NC}"
                Register-WindowsInstallation -InstallDir $binDir -ExePath $targetExe
                if (-not $NoPathUpdate) { Write-UserEnvPath -BinPath $binDir }
                return $true
            }
        }
    }

    if ($scriptDir -and (Test-Path (Join-Path $scriptDir "Cargo.toml")) -and (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Host "  ${YLW}⚡ Building gladeshell Rust engine (release mode)...${NC}"
        Push-Location $scriptDir
        cargo build --release
        Pop-Location
        if (Test-Path (Join-Path $scriptDir "target\release\gladeshell.exe")) {
            Copy-Item (Join-Path $scriptDir "target\release\gladeshell.exe") (Join-Path $binDir "gladeshell.exe") -Force
            Write-Host "  ${GRN}✔ Built & installed binary to $binDir\gladeshell.exe${NC}"
            Register-WindowsInstallation -InstallDir $binDir -ExePath $targetExe
            if (-not $NoPathUpdate) { Write-UserEnvPath -BinPath $binDir }
            return $true
        }
    }

    if (Get-Command gladeshell -ErrorAction SilentlyContinue) {
        Write-Host "  ${GRN}✔ gladeshell binary active: $((Get-Command gladeshell).Source)${NC}"
        return $true
    }

    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        Write-Host "  ${YLW}⚡ Installing via cargo from GitHub...${NC}"
        cargo install --git https://github.com/rihadjahanopu/gladeshell --quiet 2>$null
        if (Get-Command gladeshell -ErrorAction SilentlyContinue) {
            Write-Host "  ${GRN}✔ Installed gladeshell via cargo install!${NC}"
            return $true
        }
    }

    Write-Host "  ${YLW}⚠️ Could not download binary. Please install Cargo or download manually.${NC}"
    return $false
}

Install-RustBinary | Out-Null

# --- STEP 5: Atomic Write & Auto-Reload ---------------------------------------
Show-ProgressBar -Current 5 -Total 5 -StepName "Writing Profile & Auto-Reload"

$initScript = if (Get-Command gladeshell -ErrorAction SilentlyContinue) {
    & gladeshell init pwsh 2>$null
} else {
    "if (Get-Command gladeshell -ErrorAction SilentlyContinue) { Invoke-Expression (& gladeshell init pwsh) }"
}

$newBlock = @"

$START
# Installed by GladeShell: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')
# gladeshell Rust Native Engine Initialization
if (-not (`$env:PATH -split ';' -contains '$HOME\.local\bin')) {
    `$env:PATH = "$HOME\.local\bin;`$env:PATH"
}

$initScript
$END
"@

try {
    Write-AtomicFile -Path $PROFILE -Content $newBlock
    Write-Host "  ${GRN}✅ Config written to `$PROFILE safely via Atomic Write!${NC}"
} catch {
    Write-Host "  ${RED}❌ Failed to write to profile: $($_.Exception.Message)${NC}"
}

# Reload profile
Write-Host "  ${BLU}🔄 Reloading shell profile...${NC}"
try {
    . $PROFILE
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
Write-Host "  ⚙️   ${BOLD}Profile:${NC}   ${GRN}$PROFILE${NC}"
Write-Host "  🔄  ${BOLD}Reload:${NC}    ${PUR}. `$PROFILE${NC}"
Write-Host "  ⚡  ${BOLD}Doctor:${NC}    Run ${CYN}.\install.ps1 -Doctor${NC} for diagnostics"
Write-Host "  ↩️   ${BOLD}Rollback:${NC}  Run ${CYN}.\install.ps1 -Rollback${NC} to undo"
Write-Host "${CYN}══════════════════════════════════════════════════${NC}"
Write-Host "  🎉  ${BOLD}Open a new PowerShell window to get started!${NC}"
Write-Host ""
