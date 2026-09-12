# =============================================================================
# setup.ps1 — fancybash contributor setup script (PowerShell)
# Run this once after cloning the repo:  .\setup.ps1
# =============================================================================

#Requires -Version 5.1
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

# ── helpers ───────────────────────────────────────────────────────────────────
function Write-Info    { param($msg) Write-Host "ℹ  $msg" -ForegroundColor Cyan   }
function Write-Success { param($msg) Write-Host "✔  $msg" -ForegroundColor Green  }
function Write-Warn    { param($msg) Write-Host "⚠  $msg" -ForegroundColor Yellow }
function Write-Err     { param($msg) Write-Host "✖  $msg" -ForegroundColor Red    }

# ── sanity check ─────────────────────────────────────────────────────────────
try {
  git rev-parse --git-dir 2>$null | Out-Null
} catch {
  Write-Err "Not inside a Git repository. Please clone the project first."
  exit 1
}

Write-Host ""
Write-Host "══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "   fancybash — contributor setup          " -ForegroundColor Cyan
Write-Host "══════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# ── 1. Git hooks ─────────────────────────────────────────────────────────────
Write-Info "Configuring Git hooks..."

git config core.hooksPath .githooks

# PowerShell-এ chmod নেই, তবে Git for Windows / WSL এর জন্য attrib দিয়ে executable করা যায়
if ($IsLinux -or $IsMacOS) {
  # PowerShell Core (pwsh) on Linux/macOS
  Get-ChildItem .githooks | ForEach-Object {
    chmod +x $_.FullName
  }
} else {
  # Windows: Git for Windows handles executable bits via .gitattributes
  # hooks are already marked executable in the repo — no extra step needed
  Write-Warn "Windows detected: chmod skipped. Git for Windows reads executable bit from the repo."
}

Write-Success "Git hooks configured  (core.hooksPath → .githooks)"

# ── done ─────────────────────────────────────────────────────────────────────
Write-Host ""
Write-Host "✔  Setup complete! Happy contributing 🎉" -ForegroundColor Green
Write-Host ""
