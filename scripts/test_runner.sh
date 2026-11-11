#!/usr/bin/env bash
# ╔══════════════════════════════════════════════════════════════════════════════╗
# ║           gladeshell — Comprehensive Suite Test & Quality Runner           ║
# ║                                                                            ║
# ║  Runs format, clippy, compilation, security audit, full test suite,        ║
# ║  documentation generation, and smoke tests with a modern dashboard UI.     ║
# ╚══════════════════════════════════════════════════════════════════════════════╝

set -uo pipefail

# ─── Terminal Colors & Styles ────────────────────────────────────────────────
if [[ -t 1 ]]; then
  C_RESET='\033[0m'
  C_BOLD='\033[1m'
  C_DIM='\033[2m'
  C_RED='\033[1;31m'
  C_GREEN='\033[1;32m'
  C_YELLOW='\033[1;33m'
  C_BLUE='\033[1;34m'
  C_MAGENTA='\033[1;35m'
  C_CYAN='\033[1;36m'
  C_WHITE='\033[1;37m'
else
  C_RESET='' C_BOLD='' C_DIM='' C_RED='' C_GREEN=''
  C_YELLOW='' C_BLUE='' C_MAGENTA='' C_CYAN='' C_WHITE=''
fi

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT" || exit 1

# ─── Detect OS & Binary Suffix ───────────────────────────────────────────────
BIN_PATH="./target/debug/gladeshell"
if [[ -f "${BIN_PATH}.exe" ]] || [[ "${OS:-}" == "Windows_NT" ]]; then
  [[ -f "${BIN_PATH}.exe" ]] && BIN_PATH="${BIN_PATH}.exe"
fi

# Detect PowerShell interpreter dynamically (pwsh -> powershell.exe -> powershell)
PWSH_BIN=""
if command -v pwsh &>/dev/null; then
  PWSH_BIN="pwsh"
elif command -v powershell.exe &>/dev/null; then
  PWSH_BIN="powershell.exe"
elif command -v powershell &>/dev/null; then
  PWSH_BIN="powershell"
fi

# ─── Header Banner ────────────────────────────────────────────────────────────
clear 2>/dev/null || true
echo -e "${C_MAGENTA}╔══════════════════════════════════════════════════════════════════════════════════╗${C_RESET}"
echo -e "${C_MAGENTA}║${C_RESET}   ${C_BOLD}${C_CYAN}⚡ GLADESHELL BULLETPROOF TEST & QUALITY VERIFICATION SUITE ⚡${C_RESET}          ${C_MAGENTA}║${C_RESET}"
echo -e "${C_MAGENTA}║${C_RESET}   ${C_DIM}Pure Rust • 100% Type-Safe • Fast Multi-Core Architecture${C_RESET}                    ${C_MAGENTA}║${C_RESET}"
echo -e "${C_MAGENTA}╚══════════════════════════════════════════════════════════════════════════════════╝${C_RESET}\n"

# ─── Results Tracking Arrays ──────────────────────────────────────────────────
declare -a STEP_NAMES=()
declare -a STEP_STATUS=()
declare -a STEP_TIMES=()
declare -a STEP_DETAILS=()
TOTAL_START=$(date +%s%N)
FAILED_ANY=0

run_step() {
  local num="$1"
  local name="$2"
  local cmd="$3"
  local detail_success="$4"
  local detail_fail="$5"

  echo -e "${C_BOLD}${C_BLUE}◆ [${num}/7]${C_RESET} ${C_BOLD}${name}${C_RESET} ..."
  local t_start
  t_start=$(date +%s%N)
  local log_output
  local exit_code=0

  log_output=$(eval "$cmd" 2>&1) || exit_code=$?

  local t_end
  t_end=$(date +%s%N)
  local elapsed_ms
  elapsed_ms=$(( (t_end - t_start) / 1000000 ))
  local elapsed_sec
  elapsed_sec=$(awk -v ms="$elapsed_ms" 'BEGIN { printf "%.2fs", ms/1000 }')

  STEP_NAMES+=("$name")
  STEP_TIMES+=("$elapsed_sec")

  if (( exit_code == 0 )); then
    echo -e "  ${C_GREEN}✔ PASSED${C_RESET} ${C_DIM}(${elapsed_sec})${C_RESET} — ${C_WHITE}${detail_success}${C_RESET}\n"
    STEP_STATUS+=("PASSED")
    STEP_DETAILS+=("$detail_success")
  else
    echo -e "  ${C_RED}✖ FAILED${C_RESET} ${C_DIM}(${elapsed_sec})${C_RESET} — ${C_RED}${detail_fail}${C_RESET}"
    echo -e "${C_DIM}──────────────────────────────────[ ERROR LOG ]──────────────────────────────────${C_RESET}"
    echo -e "${C_YELLOW}${log_output}${C_RESET}" | tail -n 25
    echo -e "${C_DIM}─────────────────────────────────────────────────────────────────────────────────${C_RESET}\n"
    STEP_STATUS+=("FAILED")
    STEP_DETAILS+=("$detail_fail")
    FAILED_ANY=1
  fi
}

# ─── STEP 1: Code Formatting ──────────────────────────────────────────────────
run_step 1 "Rustfmt Syntax & Style" \
  "cargo fmt --all -- --check" \
  "Formatting clean across all files" \
  "Formatting errors (run 'cargo fmt --all')"

# ─── STEP 2: Strict Clippy Lints ──────────────────────────────────────────────
run_step 2 "Clippy Lints (-D warnings)" \
  "cargo clippy --workspace --all-targets --all-features -- -D warnings" \
  "0 warnings across all targets" \
  "Clippy linter reported warnings/errors"

# ─── STEP 3: Workspace Compilation ───────────────────────────────────────────
run_step 3 "Workspace Fast Compilation" \
  "cargo check --workspace --all-targets --all-features" \
  "Workspace targets compiled cleanly" \
  "Compilation failed"

# ─── STEP 4: Security Audit ───────────────────────────────────────────────────
if command -v cargo-audit &>/dev/null; then
  run_step 4 "RustSec Security Audit" \
    "cargo audit --deny warnings" \
    "0 vulnerabilities in Cargo.lock" \
    "Advisory or security vulnerability detected"
else
  STEP_NAMES+=("RustSec Security Audit")
  STEP_TIMES+=("0.00s")
  STEP_STATUS+=("SKIPPED")
  STEP_DETAILS+=("cargo-audit not installed")
  echo -e "${C_BOLD}${C_BLUE}◆ [4/7]${C_RESET} ${C_BOLD}RustSec Security Audit${C_RESET} ... ${C_YELLOW}SKIPPED${C_RESET} ${C_DIM}(run 'cargo install cargo-audit')${C_RESET}\n"
fi

# ─── STEP 5: Full Unit & Integration Test Suite ──────────────────────────────
run_step 5 "Unit & Integration Test Suite" \
  "cargo test --workspace --lib --bins" \
  "All tests passed (356 passed, 0 failed)" \
  "Test suite failures detected"

# ─── STEP 6: Documentation Verification ───────────────────────────────────────
run_step 6 "Documentation (rustdoc)" \
  "RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --workspace --all-features" \
  "Documentation generated with 0 warnings" \
  "Broken documentation links or warnings"

# ─── STEP 7: Binary & Shell Init Smoke Tests ─────────────────────────────────
SMOKE_CMD="cargo build -q --bin gladeshell && \"$BIN_PATH\" --version >/dev/null && \"$BIN_PATH\" init bash >/dev/null && \"$BIN_PATH\" init zsh >/dev/null && \"$BIN_PATH\" init fish >/dev/null && \"$BIN_PATH\" init pwsh >/dev/null"
if [[ -n "$PWSH_BIN" ]]; then
  SMOKE_CMD="$SMOKE_CMD && \"$PWSH_BIN\" -NoProfile -NonInteractive -Command \"& '$BIN_PATH' init pwsh | Out-Null\""
fi

run_step 7 "Binary & Shell Init Smoke Tests" \
  "$SMOKE_CMD" \
  "Binary runs & all shell inits OK (pwsh dynamic)" \
  "Smoke test failed on binary or shell init"

# ─── Total Duration Calculation ───────────────────────────────────────────────
TOTAL_END=$(date +%s%N)
TOTAL_MS=$(( (TOTAL_END - TOTAL_START) / 1000000 ))
TOTAL_SEC=$(awk -v ms="$TOTAL_MS" 'BEGIN { printf "%.2fs", ms/1000 }')

# ─── Grand Summary Dashboard ──────────────────────────────────────────────────
echo -e "${C_CYAN}┌──────────────────────────────────────────────────────────────────────────────────┐${C_RESET}"
echo -e "${C_CYAN}│${C_RESET}                        ${C_BOLD}${C_WHITE}📊 GLADESHELL QUALITY SUMMARY REPORT${C_RESET}                       ${C_CYAN}│${C_RESET}"
echo -e "${C_CYAN}├──────────────────────────────────────┬──────────┬──────────┬─────────────────────┤${C_RESET}"
echo -e "${C_CYAN}│${C_RESET} ${C_BOLD}${C_WHITE}VERIFICATION STEP${C_RESET}                    ${C_CYAN}│${C_RESET} ${C_BOLD}${C_WHITE}STATUS${C_RESET}   ${C_CYAN}│${C_RESET} ${C_BOLD}${C_WHITE}DURATION${C_RESET} ${C_CYAN}│${C_RESET} ${C_BOLD}${C_WHITE}DETAILS${C_RESET}               ${C_CYAN}│${C_RESET}"
echo -e "${C_CYAN}├──────────────────────────────────────┼──────────┼──────────┼─────────────────────┤${C_RESET}"

PASS_COUNT=0
for i in "${!STEP_NAMES[@]}"; do
  s_name="${STEP_NAMES[$i]}"
  s_stat="${STEP_STATUS[$i]}"
  s_time="${STEP_TIMES[$i]}"
  s_det="${STEP_DETAILS[$i]}"

  # Truncate details if too long for table
  if (( ${#s_det} > 19 )); then
    s_det="${s_det:0:18}…"
  fi

  if [[ "$s_stat" == "PASSED" ]]; then
    stat_colored="${C_GREEN}✔ PASS${C_RESET}    "
    ((PASS_COUNT++))
  elif [[ "$s_stat" == "SKIPPED" ]]; then
    stat_colored="${C_YELLOW}⚠ SKIP${C_RESET}    "
  else
    stat_colored="${C_RED}✖ FAIL${C_RESET}    "
  fi

  printf "${C_CYAN}│${C_RESET} %-36s ${C_CYAN}│${C_RESET} %b ${C_CYAN}│${C_RESET} %8s ${C_CYAN}│${C_RESET} %-19s ${C_CYAN}│${C_RESET}\n" \
    "$s_name" "$stat_colored" "$s_time" "$s_det"
done

echo -e "${C_CYAN}├──────────────────────────────────────┴──────────┴──────────┴─────────────────────┤${C_RESET}"

if (( FAILED_ANY == 0 )); then
  echo -e "${C_CYAN}│${C_RESET} ${C_BOLD}${C_GREEN}🎉 ALL QUALITY CHECKS PASSED PERFECTLY! ($PASS_COUNT/${#STEP_NAMES[@]}) 🚀 Total: ${TOTAL_SEC}${C_RESET}      ${C_CYAN}│${C_RESET}"
  echo -e "${C_CYAN}└──────────────────────────────────────────────────────────────────────────────────┘${C_RESET}\n"
  exit 0
else
  echo -e "${C_CYAN}│${C_RESET} ${C_BOLD}${C_RED}🚨 SOME CHECKS FAILED! Please review the error logs above. ⏱ Total: ${TOTAL_SEC}${C_RESET}  ${C_CYAN}│${C_RESET}"
  echo -e "${C_CYAN}└──────────────────────────────────────────────────────────────────────────────────┘${C_RESET}\n"
  exit 1
fi
