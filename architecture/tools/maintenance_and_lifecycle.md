# 🔄 Maintenance, Upgrades & Lifecycle Suite

- **Primary Sources:**
  - Self Upgrader: [`src/tools/self_upgrade.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/self_upgrade.rs) (940 LoC)
  - Self Uninstaller: [`src/tools/self_uninstall.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/self_uninstall.rs) (230 LoC)
  - Universal Uninstaller: [`src/tools/uninstaller.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/uninstaller.rs) (1,710 LoC)
  - System Updater: [`src/tools/system_update.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/system_update.rs) (784 LoC)
  - Toolchain Updater: [`src/tools/updater.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/updater.rs) (1,421 LoC)
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Role

Maintaining software installations across Linux, macOS, and Windows requires bulletproof lifecycle management.

### Key Capabilities:
- **In-Place Atomic Self-Upgrade:** Downloads latest GitHub releases, validates SHA256 checksums, strips verbatim prefixes, swaps binaries atomically, and provides an automatic rollback guard if the new binary fails validation.
- **Universal Application Uninstaller:** Interactive TUI to search and completely remove software across Flatpak, Snap, Homebrew, and Winget.
- **System-Wide Package Updater:** Updates all system packages and language tools (Rust, Bun, Node, Cargo, Pip) with a single command (`uu` / `uup`).
- **Clean Self-Uninstallation:** Safely removes shell hooks from `.zshrc`, `.bashrc`, `config.fish`, and PowerShell profiles without corrupting other shell settings.

---

## 2. 🛡️ Atomic Self-Upgrade & Rollback Guard

Replacing an active executable while it is running in memory is prohibited by the OS (especially on Windows).
Gladeshell's `self_upgrade` engine uses a **3-phase transactional swap**:
1. Downloads new binary to temporary file guarded by `TempFileGuard`.
2. Validates magic bytes (ELF for Linux, Mach-O for macOS, PE for Windows) and verifies SHA256 against release manifest.
3. Performs atomic rename/swap (`std::fs::rename`). If the new binary crashes during self-smoke test, the old executable is restored from `.bak`.

---

## 3. 💻 CLI Usage

```bash
# Upgrade Gladeshell to the latest stable release
gladeshell upgrade
# or
uu

# Universal system & package manager update
uup

# Launch interactive Application Uninstaller
uninstaller

# Completely uninstall Gladeshell and remove shell hooks
gladeshell self-uninstall
```
