# =============================================================================
#  Makefile — gladeshell Build, Lint, Test & Installation Manager
# =============================================================================

.PHONY: build check clippy fmt test bench install update uninstall hooks help

# Default action
all: build

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
	@echo "  make test       - Execute full workspace unit test suite"
	@echo "  make bench      - Run Criterion benchmarks"
	@echo "  make hooks      - Enable local .githooks git pre-commit & pre-push hooks"
	@echo "  make install    - Build and install gladeshell into system PATH"
	@echo "  make update     - Git pull, rebuild, and re-configure latest version"
	@echo "  make uninstall  - Remove binary and restore original shell environment"
