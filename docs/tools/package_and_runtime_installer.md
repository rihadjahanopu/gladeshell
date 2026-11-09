# 📦 Package & Runtime Installer (`runtime_installer`, `dep_installer`, `pkg_converter`)

- **Primary Sources:**
  - Runtime Installer: [`src/tools/runtime_installer.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/runtime_installer.rs) (2,397 LoC)
  - Dependency Installer: [`src/tools/dep_installer.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/dep_installer.rs) (210 LoC)
  - Package Converter: [`src/tools/pkg_converter.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/pkg_converter.rs) (133 LoC)
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Role

Setting up a fresh developer environment on Linux, macOS, or Windows usually requires searching for curl commands, installing toolchains, and configuring environment PATHs manually.

This suite provides:
1. **One-Command Runtime Setup:** Installs official toolchains with version detection and automatic PATH persistence.
2. **System Dependency Detection:** Automatically detects host package manager (APT, Pacman, DNF, Homebrew, Winget, Chocolatey).
3. **Universal Command Converter:** Translates commands between NPM, Bun, PNPM, and Yarn on the fly.

---

## 2. 🚀 Supported Language Toolchains (`runtime_installer`)

| Runtime | Official Source / Method | Supported Platforms |
| :---: | :---: | :---: |
| **Rust** | `rustup` toolchain manager | Linux, macOS, Windows |
| **Bun** | `bun.sh` official binary distribution | Linux, macOS, Windows |
| **Node.js** | NodeSource / NVM / FNM automated setup | Linux, macOS, Windows |
| **Go** | Golang official release tarballs / zip | Linux, macOS, Windows |
| **Python** | Python 3 + `uv` / `pip` virtual environment | Linux, macOS, Windows |
| **Zig** | Ziglang master / stable prebuilt tarballs | Linux, macOS, Windows |
| **Docker** | Official Docker Engine & Compose installer | Linux, macOS, Windows |

---

## 3. 🔄 Package Manager Converter (`pkg_converter`)

Translates commands across package managers seamlessly:
```bash
# Translates `ni` to `npm install`, `bun install`, `pnpm install`, or `yarn add`
pkg-converter npm bun "npm install --save-dev typescript"
# Output: bun add -d typescript
```
