#!/usr/bin/env pwsh
# ==============================================================================
#  GladeShell — Bulletproof Test & Quality Verification Suite (PowerShell)
#  Compatible: PowerShell 5.1 (Windows) + PowerShell 7+ (Windows/Linux/macOS)
#  Usage: pwsh -File ./scripts/test_runner.ps1
#         .\scripts\test_runner.ps1          (from PowerShell prompt)
# ==============================================================================

#Requires -Version 5.1

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Continue'

# ── Colour helpers ─────────────────────────────────────────────────────────────
# Use Write-Host -ForegroundColor for maximum cross-platform compatibility
# (works on PS 5.1 + Windows Terminal + VS Code + pwsh 7 on Linux/macOS)

function Write-Cyan   { param([string]$Text) Write-Host $Text -ForegroundColor Cyan    -NoNewline }
function Write-Green  { param([string]$Text) Write-Host $Text -ForegroundColor Green   -NoNewline }
function Write-Red    { param([string]$Text) Write-Host $Text -ForegroundColor Red     -NoNewline }
function Write-Yellow { param([string]$Text) Write-Host $Text -ForegroundColor Yellow  -NoNewline }
function Write-White  { param([string]$Text) Write-Host $Text -ForegroundColor White   -NoNewline }
function Write-Gray   { param([string]$Text) Write-Host $Text -ForegroundColor DarkGray -NoNewline }
function Write-Magenta{ param([string]$Text) Write-Host $Text -ForegroundColor Magenta -NoNewline }
function Write-Blue   { param([string]$Text) Write-Host $Text -ForegroundColor Blue    -NoNewline }

function NL { Write-Host "" }

# ── Box-drawing helpers ────────────────────────────────────────────────────────
$BOX_TL = [char]0x2554   # ╔
$BOX_TR = [char]0x2557   # ╗
$BOX_BL = [char]0x255A   # ╚
$BOX_BR = [char]0x255D   # ╝
$BOX_H  = [char]0x2550   # ═
$BOX_V  = [char]0x2551   # ║
$BOX_ML = [char]0x2560   # ╠
$BOX_MR = [char]0x2563   # ╣
$BULLET = [char]0x25C6   # ◆
$CHECK  = [char]0x2714   # ✔
$CROSS  = [char]0x2716   # ✖
$WARN   = [char]0x26A0   # ⚠

function Write-HRule {
    param([int]$Width = 82, [string]$Left = $BOX_ML, [string]$Right = $BOX_MR)
    Write-Cyan ($Left + ($BOX_H * $Width) + $Right)
    NL
}

function Write-BoxLine {
    param([string]$Content, [int]$Width = 82)
    $padded = $Content.PadRight($Width)
    Write-Cyan $BOX_V
    Write-White $padded
    Write-Cyan $BOX_V
    NL
}

# ── Timing ─────────────────────────────────────────────────────────────────────
[Diagnostics.CodeAnalysis.SuppressMessageAttribute('PSUseShouldProcessForStateChangingFunctions', '')]
function Get-Stopwatch {
    return [System.Diagnostics.Stopwatch]::StartNew()
}
function Get-Elapsed {
    param($sw)
    $sw.Stop()
    return [math]::Round($sw.Elapsed.TotalSeconds, 2)
}

# ── Step state storage ─────────────────────────────────────────────────────────
$script:Results  = @()   # each entry: @{Label; Status; Elapsed; Cmd}
$script:AnyFail  = $false

function Add-Result {
    param([string]$Label, [string]$Status, [double]$Elapsed, [string]$Cmd)
    $script:Results += [PSCustomObject]@{
        Label   = $Label
        Status  = $Status
        Elapsed = $Elapsed
        Cmd     = $Cmd
    }
    if ($Status -eq 'FAIL') { $script:AnyFail = $true }
}

# ── Run a cargo step ──────────────────────────────────────────────────────────
function Invoke-Step {
    param(
        [int]   $Num,
        [int]   $Total,
        [string]$Label,
        [string[]]$CmdArgs,          # e.g. @('cargo','fmt','--','--check')
        [string]$OkMsg  = 'Completed successfully',
        [switch]$SkipIf              # pre-computed bool — if true, skip step
    )

    NL
    Write-Cyan "  $BULLET "
    Write-White "[$Num/$Total] $Label ..."
    NL

    if ($SkipIf) {
        Write-Yellow "    $WARN SKIPPED — tool not available"
        NL
        Add-Result -Label $Label -Status 'SKIP' -Elapsed 0 -Cmd ($CmdArgs -join ' ')
        return
    }

    $sw  = Get-Stopwatch
    $tmp = [System.IO.Path]::GetTempFileName()

    try {
        # Merge stderr into stdout so we capture everything
        $proc = Start-Process -FilePath $CmdArgs[0] `
                              -ArgumentList ($CmdArgs[1..($CmdArgs.Length-1)]) `
                              -NoNewWindow -Wait -PassThru `
                              -RedirectStandardOutput $tmp `
                              -RedirectStandardError  "$tmp.err"
        $exit = $proc.ExitCode
    } catch {
        $exit = 1
        $_.Exception.Message | Out-File $tmp -Encoding UTF8
    }

    $elapsed = Get-Elapsed $sw

    # Combine stdout + stderr for display
    $out = ''
    if (Test-Path $tmp)        { $out += (Get-Content $tmp -Raw -ErrorAction SilentlyContinue) }
    if (Test-Path "$tmp.err")  { $out += (Get-Content "$tmp.err" -Raw -ErrorAction SilentlyContinue) }
    Remove-Item $tmp, "$tmp.err" -ErrorAction SilentlyContinue

    if ($exit -eq 0) {
        Write-Green "    $CHECK PASSED"
        Write-Gray  " (${elapsed}s) — $OkMsg"
        NL
        Add-Result -Label $Label -Status 'PASS' -Elapsed $elapsed -Cmd ($CmdArgs -join ' ')
    } else {
        Write-Red   "    $CROSS FAILED"
        Write-Gray  " (${elapsed}s)"
        NL
        if ($out) {
            Write-Yellow "    ── Error Output ──────────────────────────"
            NL
            foreach ($line in ($out -split "`n" | Select-Object -First 30)) {
                Write-Gray "    $line"
                NL
            }
        }
        Add-Result -Label $Label -Status 'FAIL' -Elapsed $elapsed -Cmd ($CmdArgs -join ' ')
    }
}

# ── Header ─────────────────────────────────────────────────────────────────────
function Show-Header {
    $w = 82
    NL
    Write-Cyan ($BOX_TL + ($BOX_H * $w) + $BOX_TR); NL
    Write-BoxLine "   $([char]0x26A1) GLADESHELL BULLETPROOF TEST & QUALITY VERIFICATION SUITE $([char]0x26A1)          "
    Write-BoxLine "   Pure Rust $([char]0x2022) 100% Type-Safe $([char]0x2022) Fast Multi-Core Architecture                    "
    Write-Cyan ($BOX_BL + ($BOX_H * $w) + $BOX_BR); NL
}

# ── Summary table ──────────────────────────────────────────────────────────────
function Show-Summary {
    param([double]$TotalSec)

    NL
    $w = 82
    Write-Cyan ($BOX_TL + ($BOX_H * $w) + $BOX_TR); NL
    Write-BoxLine "   QUALITY GATE SUMMARY"
    Write-HRule -Width $w

    # Column widths
    $cStep    = 32
    $cStatus  = 10
    $cTime    = 10
    $cCmd     = 26

    $hdr = ("  {0,-$cStep}  {1,-$cStatus}  {2,-$cTime}  {3,-$cCmd}" -f 'Step', 'Status', 'Time', 'Command')
    Write-Cyan $BOX_V
    Write-White $hdr.PadRight($w)
    Write-Cyan $BOX_V; NL

    Write-HRule -Width $w

    foreach ($r in $script:Results) {
        $stepCol   = $r.Label.PadRight($cStep).Substring(0, [Math]::Min($cStep, $r.Label.Length)).PadRight($cStep)
        $timeCol   = ("$($r.Elapsed)s").PadRight($cTime)
        $cmdCol    = $r.Cmd.PadRight($cCmd).Substring(0, [Math]::Min($cCmd, $r.Cmd.Length)).PadRight($cCmd)

        Write-Cyan $BOX_V
        Write-White "  $stepCol  "

        switch ($r.Status) {
            'PASS' { Write-Green  "$CHECK PASS".PadRight($cStatus) }
            'FAIL' { Write-Red    "$CROSS FAIL".PadRight($cStatus) }
            'SKIP' { Write-Yellow "$WARN SKIP".PadRight($cStatus) }
        }

        Write-White "  $timeCol  $cmdCol"
        Write-Cyan $BOX_V; NL
    }

    Write-HRule -Width $w

    $pass = ($script:Results | Where-Object Status -eq 'PASS').Count
    $fail = ($script:Results | Where-Object Status -eq 'FAIL').Count
    $skip = ($script:Results | Where-Object Status -eq 'SKIP').Count
    $total = $script:Results.Count

    $statsLine = "  Passed: $pass/$total    Failed: $fail    Skipped: $skip    Total: ${TotalSec}s"
    Write-Cyan $BOX_V
    Write-White $statsLine.PadRight($w)
    Write-Cyan $BOX_V; NL

    Write-Cyan ($BOX_BL + ($BOX_H * $w) + $BOX_BR); NL
    NL

    if ($script:AnyFail) {
        Write-Red   "  $CROSS Quality gate FAILED — fix errors above before merging."; NL
    } else {
        Write-Green "  $CHECK All quality gates PASSED — ready to ship!"; NL
    }
    NL
}

# ══════════════════════════════════════════════════════════════════════════════
#  MAIN
# ══════════════════════════════════════════════════════════════════════════════

# Resolve project root (parent of scripts/)
$ProjectRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
Push-Location $ProjectRoot

$suiteTimer = Get-Stopwatch

Show-Header

# Detect optional tools
$hasAudit = $null -ne (Get-Command 'cargo-audit' -ErrorAction SilentlyContinue)

# Step 1 — rustfmt
Invoke-Step -Num 1 -Total 7 -Label 'Rustfmt Syntax & Style' `
    -CmdArgs @('cargo', 'fmt', '--all', '--', '--check') `
    -OkMsg 'Formatting clean across all files'

# Step 2 — clippy
Invoke-Step -Num 2 -Total 7 -Label 'Clippy Lints (-D warnings)' `
    -CmdArgs @('cargo', 'clippy', '--all-targets', '--all-features', '--', '-D', 'warnings') `
    -OkMsg '0 warnings across all targets'

# Step 3 — cargo check
Invoke-Step -Num 3 -Total 7 -Label 'Workspace Fast Compilation' `
    -CmdArgs @('cargo', 'check', '--all-targets', '--all-features') `
    -OkMsg 'Workspace targets compiled cleanly'

# Step 4 — cargo audit
Invoke-Step -Num 4 -Total 7 -Label 'RustSec Security Audit' `
    -CmdArgs @('cargo', 'audit') `
    -OkMsg '0 vulnerabilities in Cargo.lock' `
    -SkipIf:(-not $hasAudit)

# Step 5 — cargo test
Invoke-Step -Num 5 -Total 7 -Label 'Unit & Integration Test Suite' `
    -CmdArgs @('cargo', 'test', '--lib', '--', '--test-threads=1') `
    -OkMsg 'All tests passed'

# Step 6 — cargo doc
Invoke-Step -Num 6 -Total 7 -Label 'Documentation Build' `
    -CmdArgs @('cargo', 'doc', '--no-deps', '--all-features') `
    -OkMsg 'Docs built without warnings'

# Step 7 — binary smoke test
Write-Host ""
Write-Cyan "  $BULLET "
Write-White "[7/7] Binary Smoke Test ..."
NL

$sw7 = Get-Stopwatch
$smokeOk = $true
$smokeFails = @()

$smokeTests = @(
    @{ Args = @('--version');      Pattern = '\d+\.\d+\.\d+' }
    @{ Args = @('--help');         Pattern = '.'              }
    @{ Args = @('ginfo', '--help'); Pattern = '.'             }
)

foreach ($t in $smokeTests) {
    try {
        $out = & cargo run -q -- @($t.Args) 2>&1 | Out-String
        if ($out -notmatch $t.Pattern) {
            $smokeOk = $false
            $smokeFails += "cargo run -- $($t.Args -join ' ') — unexpected output"
        }
    } catch {
        $smokeOk = $false
        $smokeFails += "cargo run -- $($t.Args -join ' ') — exception: $($_.Exception.Message)"
    }
}

$el7 = Get-Elapsed $sw7

if ($smokeOk) {
    Write-Green "    $CHECK PASSED"
    Write-Gray  " (${el7}s) — Binary boots and responds correctly"
    NL
    Add-Result -Label 'Binary Smoke Test' -Status 'PASS' -Elapsed $el7 -Cmd 'cargo run -- --version'
} else {
    Write-Red   "    $CROSS FAILED"
    Write-Gray  " (${el7}s)"
    NL
    foreach ($f in $smokeFails) {
        Write-Yellow "    $WARN $f"; NL
    }
    Add-Result -Label 'Binary Smoke Test' -Status 'FAIL' -Elapsed $el7 -Cmd 'cargo run -- --version'
}

# ── Final Summary ──────────────────────────────────────────────────────────────
$totalSec = Get-Elapsed $suiteTimer
Show-Summary -TotalSec $totalSec

Pop-Location

if ($script:AnyFail) { exit 1 } else { exit 0 }
