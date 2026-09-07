# =============================================================================
#  Makefile — fancybash-rs Build & Installation Manager
# =============================================================================

.PHONY: build install update uninstall test help

# Default action
all: build

## build: Build the release binary
build:
	cargo build --release

## install: Install fancybash binary to ~/.cargo/bin
install:
	cargo install --path . --force
	@echo "\n✨ fancybash installed successfully to ~/.cargo/bin/fancybash"
	@echo "💡 Add this to your shell config file (~/.bashrc or ~/.zshrc):"
	@echo "   eval \"\$$(fancybash init zsh)\"   # for Zsh"
	@echo "   eval \"\$$(fancybash init bash)\"  # for Bash"

## update: Pull latest code and re-install
update:
	git pull
	cargo install --path . --force
	@echo "✨ fancybash updated successfully!"

## uninstall: Remove fancybash binary
uninstall:
	cargo uninstall fancybash 2>/dev/null || rm -f ~/.cargo/bin/fancybash
	@echo "🗑️ fancybash uninstalled."

## test: Run unit test suite
test:
	cargo test

## help: Display available targets
help:
	@echo "fancybash-rs Makefile targets:"
	@echo "  make install    - Build and install fancybash into ~/.cargo/bin"
	@echo "  make update     - Git pull and reinstall latest version"
	@echo "  make uninstall  - Remove binary from system"
	@echo "  make test       - Run unit tests"
