@echo off
:: ==============================================================================
::  F A N C Y B A S H  •  Windows Launcher (install.bat)
::  Just download THIS ONE FILE and double-click - everything else is automatic!
::  Bulletproof: auto-downloads install.ps1, handles all PowerShell versions.
:: ==============================================================================

setlocal EnableDelayedExpansion

set "PS1_FILE=%~dp0install.ps1"
set "PS1_URL=https://raw.githubusercontent.com/rihadjahanopu/fancybash/refs/heads/main/install.ps1"
set "PS1_CDN=https://cdn.jsdelivr.net/gh/rihadjahanopu/fancybash@main/install.ps1"

echo.
echo  ==========================================
echo    F A N C Y B A S H  ^|  Windows Launcher
echo  ==========================================
echo.

:: --- Step 1: Find PowerShell --------------------------------------------------
set "PS_EXE="
where pwsh >nul 2>&1
if %ERRORLEVEL% == 0 (
    set "PS_EXE=pwsh"
    echo [OK] Found: PowerShell Core ^(pwsh^)
    goto :check_ps1
)
where powershell >nul 2>&1
if %ERRORLEVEL% == 0 (
    set "PS_EXE=powershell"
    echo [OK] Found: Windows PowerShell 5.1
    goto :check_ps1
)

echo [ERROR] PowerShell not found on this system!
echo         Download from: https://github.com/PowerShell/PowerShell/releases
echo.
pause
exit /b 1

:: --- Step 2: Download install.ps1 if not present -----------------------------
:check_ps1
if exist "%PS1_FILE%" (
    echo [OK] install.ps1 found locally.
    goto :run
)

echo [INFO] install.ps1 not found. Downloading from GitHub...

:: Try curl (built-in on Windows 10+)
where curl >nul 2>&1
if %ERRORLEVEL% == 0 (
    echo [INFO] Downloading via curl ^(primary^)...
    curl -fsSL "%PS1_URL%" -o "%PS1_FILE%" 2>nul
    if %ERRORLEVEL% == 0 (
        if exist "%PS1_FILE%" (
            echo [OK] Downloaded successfully via curl.
            goto :validate
        )
    )
    echo [WARN] curl primary failed. Trying CDN fallback...
    curl -fsSL "%PS1_CDN%" -o "%PS1_FILE%" 2>nul
    if %ERRORLEVEL% == 0 (
        if exist "%PS1_FILE%" (
            echo [OK] Downloaded via CDN fallback.
            goto :validate
        )
    )
)

:: Fallback: PowerShell download with CDN fallback
echo [INFO] Downloading via PowerShell ^(with CDN fallback^)...
%PS_EXE% -NoProfile -ExecutionPolicy Bypass -Command ^
    "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; ^
    $urls = @('%PS1_URL%','%PS1_CDN%'); ^
    foreach ($u in $urls) { ^
        try { Invoke-WebRequest -Uri $u -OutFile '%PS1_FILE%' -UseBasicParsing -ErrorAction Stop; break } ^
        catch { Write-Host \"[WARN] Failed: $u\" } ^
    }"

:validate
if not exist "%PS1_FILE%" (
    echo.
    echo [ERROR] Failed to download install.ps1 from all sources.
    echo         Please check your internet connection and try again.
    echo         Or run manually: powershell -c "irm %PS1_URL% ^| iex"
    echo.
    pause
    exit /b 1
)

:: Basic size check - file should be at least 1KB
for %%A in ("%PS1_FILE%") do set "FILE_SIZE=%%~zA"
if !FILE_SIZE! LSS 1000 (
    echo [ERROR] Downloaded file seems corrupted ^(too small: !FILE_SIZE! bytes^).
    del "%PS1_FILE%" >nul 2>&1
    echo         Please check your connection and try again.
    pause
    exit /b 1
)
echo [OK] File validated ^(!FILE_SIZE! bytes^).

:: --- Step 3: Run install.ps1 with ExecutionPolicy Bypass ---------------------
:run
echo.
echo [INFO] Starting FancyBash installer...
echo.
%PS_EXE% -NoProfile -ExecutionPolicy Bypass -File "%PS1_FILE%"

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo [ERROR] Installer exited with error code %ERRORLEVEL%.
    echo         Try the one-liner instead:
    echo         powershell -c "irm %PS1_URL% ^| iex"
    echo.
    pause
    exit /b %ERRORLEVEL%
)

endlocal
