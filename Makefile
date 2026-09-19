# =============================================================================
#  Makefile — fancybash-rs Build & Installation Manager
# =============================================================================

.PHONY: build install update uninstall test help

# Default action
all: build

## build: Build the release binary
build:
	cargo build --release

## install: Install fancybash binary to ~/.cargo/bin and auto-configure shell
install:
	cargo install --path . --force
	@mkdir -p ~/.local/bin
	@rm -f ~/.local/bin/fancybash 2>/dev/null || true
	@cp -f ~/.cargo/bin/fancybash ~/.local/bin/fancybash 2>/dev/null || true
	@echo "\n✨ fancybash installed to ~/.cargo/bin/fancybash and ~/.local/bin/fancybash"
	@echo "🔧 Auto-configuring your shell..."
	@fancybash setup || true
	@echo ""

## update: Pull latest code, re-install, and sync configuration
update:
	git pull
	cargo install --path . --force
	@mkdir -p ~/.local/bin
	@rm -f ~/.local/bin/fancybash 2>/dev/null || true
	@cp -f ~/.cargo/bin/fancybash ~/.local/bin/fancybash 2>/dev/null || true
	@fancybash setup || true
	@echo "✨ fancybash updated and auto-configured successfully!"

## uninstall: Remove fancybash binary and restore shell configuration
uninstall:
	@fancybash uninstall 2>/dev/null || true
	@rm -f ~/.local/bin/fancybash ~/.cargo/bin/fancybash 2>/dev/null || true
	@cargo uninstall fancybash 2>/dev/null || true
	@echo "🗑️ fancybash uninstalled and shell configuration cleaned."

## test: Run unit test suite
test:
	cargo test

## help: Display available targets
help:
	@echo "fancybash-rs Makefile targets:"
	@echo "  make install    - Build and install fancybash into ~/.cargo/bin and ~/.local/bin"
	@echo "  make update     - Git pull, reinstall, and re-configure latest version"
	@echo "  make uninstall  - Remove binary and restore shell configuration"
	@echo "  make test       - Run unit test suite"
