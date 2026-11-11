# =============================================================================
#  Makefile — gladeshell Build, Lint, Test & Installation Manager
# =============================================================================

.PHONY: build check clippy fmt test test-full test-full-pwsh test-all ci bench bench-startup bench-full install update uninstall hooks help

# Default action
all: build

define BENCH_SCRIPT
import subprocess as sp, time, os, statistics as st
def measure(cmd, lbl, n=10):
    ts = []
    for _ in range(n):
        t0 = time.perf_counter_ns()
        try:
            p = sp.Popen(cmd, stdout=sp.DEVNULL, stderr=sp.DEVNULL)
            os.wait4(p.pid, 0)
            ts.append((time.perf_counter_ns() - t0)/1e6)
        except Exception:
            return
    if ts:
        print(f"  {lbl:<25}: mean={st.mean(ts):.2f}ms, min={min(ts):.2f}ms")

print("\n⚡ Measuring gladeshell startup latency:")
measure(["./target/release/gladeshell", "prompt"], "gladeshell prompt")
measure(["bash", "--norc", "-c", "source ~/.gladeshell/cache/init.bash"], "Bash cache source")
measure(["zsh", "--no-rcs", "-c", "source ~/.gladeshell/cache/init.zsh"], "Zsh cache source")
measure(["fish", "--no-config", "-c", "source ~/.gladeshell/cache/init.fish"], "Fish cache source")
endef
export BENCH_SCRIPT

## bench-startup: Benchmark process launch & shell init startup times
bench-startup:
	@python3 -c "$$BENCH_SCRIPT"

## bench-full: Run full 50-iteration Python empirical benchmark suite
bench-full: build
	@python3 scripts/run_benchmarks.py


## build: Build release binary
build:
	cargo build --release

## check: Fast compilation check across all workspace targets
check:
	cargo check --workspace --all-targets

## clippy: Run Clippy lints with strict warnings check
clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

## fmt: Check or apply Rust code formatting
fmt:
	cargo fmt --all

## test: Run complete unit & integration test suite
test:
	cargo test --workspace

## test-full: Run comprehensive test suite with modern visual dashboard UI (CI/CD ready, auto-detects OS/PowerShell)
test-full:
ifeq ($(OS),Windows_NT)
	@pwsh -NoProfile -ExecutionPolicy Bypass -File ./scripts/test_runner.ps1 2>nul || powershell -NoProfile -ExecutionPolicy Bypass -File ./scripts/test_runner.ps1
else
	@./scripts/test_runner.sh
endif

## test-full-pwsh: Run comprehensive test suite using PowerShell dashboard (cross-platform pwsh)
test-full-pwsh:
	@pwsh -NoProfile -ExecutionPolicy Bypass -File ./scripts/test_runner.ps1 || powershell -NoProfile -ExecutionPolicy Bypass -File ./scripts/test_runner.ps1

## ci: Alias for test-full
ci: test-full

## test-all: Alias for test-full
test-all: test-full

## bench: Run Criterion micro-benchmarks
bench:
	cargo bench

## hooks: Configure local repository git hooks (.githooks/)
hooks:
	git config core.hooksPath .githooks
	@chmod +x .githooks/* 2>/dev/null || true
	@echo "✔ Git hooks activated (core.hooksPath → .githooks)"

## install: Install gladeshell binary to ~/.cargo/bin and ~/.local/bin
install:
	cargo install --path . --force
	@mkdir -p ~/.local/bin
	@rm -f ~/.local/bin/gladeshell 2>/dev/null || true
	@cp -f ~/.cargo/bin/gladeshell ~/.local/bin/gladeshell 2>/dev/null || true
	@echo "\n✨ gladeshell installed to ~/.cargo/bin/gladeshell and ~/.local/bin/gladeshell"
	@echo "🔧 Auto-configuring your shell..."
	@gladeshell setup || true
	@echo ""

## update: Pull latest code, reinstall binary, and sync shell environment
update:
	git pull
	cargo install --path . --force
	@mkdir -p ~/.local/bin
	@rm -f ~/.local/bin/gladeshell 2>/dev/null || true
	@cp -f ~/.cargo/bin/gladeshell ~/.local/bin/gladeshell 2>/dev/null || true
	@gladeshell setup || true
	@echo "✨ gladeshell updated and auto-configured successfully!"

## uninstall: Remove gladeshell binary and restore shell configuration
uninstall:
	@gladeshell uninstall 2>/dev/null || true
	@rm -f ~/.local/bin/gladeshell ~/.cargo/bin/gladeshell 2>/dev/null || true
	@cargo uninstall gladeshell 2>/dev/null || true
	@echo "🗑️ gladeshell uninstalled and shell configuration cleaned."

## help: Display available Makefile targets
help:
	@echo "gladeshell Makefile targets:"
	@echo "  make build      - Build release binary (target/release/gladeshell)"
	@echo "  make check      - Run fast cargo check across all targets"
	@echo "  make clippy     - Run Rust Clippy linter with strict warning enforcement"
	@echo "  make fmt        - Format Rust source code using rustfmt"
	@echo "  make test       - Execute workspace unit test suite"
	@echo "  make test-full  - Run comprehensive suite with modern UI dashboard (fmt, clippy, check, audit, test, doc, smoke)"
	@echo "  make bench      - Run Criterion benchmarks"
	@echo "  make hooks      - Enable local .githooks git pre-commit & pre-push hooks"
	@echo "  make install    - Build and install gladeshell into system PATH"
	@echo "  make update     - Git pull, rebuild, and re-configure latest version"
	@echo "  make uninstall  - Remove binary and restore original shell environment"
