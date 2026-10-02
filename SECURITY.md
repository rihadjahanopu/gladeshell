# 🔒 Security Policy

This document defines the security model, vulnerability disclosure procedures, and core security guarantees of **[gladeshell](https://github.com/rihadjahanopu/gladeshell)**.

[![Security Policy](https://img.shields.io/badge/Security-Policy%20v1.1-a855f7?style=for-the-badge&logo=shield)](https://github.com/rihadjahanopu/gladeshell/security/policy)
[![Memory Safety](https://img.shields.io/badge/Memory%20Safety-100%25%20Safe%20Rust-22c55e?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Zero Telemetry](https://img.shields.io/badge/Telemetry-Zero%20Network%20Calls-0ea5e9?style=for-the-badge)](https://github.com/rihadjahanopu/gladeshell)
[![AES-256 Vault](https://img.shields.io/badge/Encryption-AES--256--GCM-f59e0b?style=for-the-badge)](https://github.com/rihadjahanopu/gladeshell)

---

## 📌 Supported Versions

Only active production releases of `gladeshell` receive security patches and vulnerability updates.

| Version | Status | Engine Architecture | Security Support |
| :--- | :--- | :--- | :--- |
| **1.1.x** | 🟢 **Active Release** | Pure Rust Core (`#![deny(unsafe_code)]`) | ✅ Full Security Patches & Audits |
| **1.0.x** | 🟡 **Legacy** | Shell Script Engine | ⚠️ Security Critical Patches Only |
| **< 1.0** | 🔴 **End of Life** | Experimental Drafts | ❌ No Security Maintenance |

---

## 🛡️ Core Security Architecture & Guarantees

`gladeshell` is engineered from the ground up prioritizing memory safety, zero-trust local execution, and cryptographic hardening.

### 1. 🦀 100% Safe Rust & Zero `unsafe` Code Policy
- The entire application logic is written in safe Rust, enforcing strict compiler constraints.
- Workspaces enforce `#![deny(unsafe_code)]` to eliminate buffer overflows, use-after-free, double-free, and data race vectors at compile time.

### 2. 🔐 Cryptographic Vault Security (`src/tools/vault.rs`)
- **Encryption Algorithm**: AES-256-GCM authenticated encryption with unique per-vault 96-bit random nonces (`rand_chacha`).
- **Key Derivation Function (KDF)**: PBKDF2-HMAC-SHA256 with 100,000 rounds and random 128-bit salts to prevent rainbow table attacks.
- **RAM Security & Zeroization**: Passwords, derived keys, and unencrypted buffers are wrapped using the `zeroize` crate to guarantee memory scrubbing upon drop.
- **Decoy Panic Protection**: Includes decoy panic password triggers that simulate destruction without destroying real data.

### 3. 🌐 Offline-First & Zero Background Telemetry
- `gladeshell` operates 100% locally on your machine.
- Zero background network calls, zero telemetry analytics, zero user tracking, and zero remote logging.

### 4. 🔤 Terminal Input Sanitization & Control Escape Guards
- Prompt inputs, command suggestions, and TUI viewports sanitize ANSI escape sequences (`\x1b`), preventing terminal emulator escape injection vulnerabilities (PTY escape injection).

### 5. 📦 Supply Chain & Dependency Auditing
- **Lockfile Pinning**: `Cargo.lock` is strictly tracked to guarantee reproducible builds.
- **CI Dependency Audit**: Automated `cargo publish --dry-run` and dependency validation in GitHub Actions.
- **Zero C-Dependencies**: Uses `gix` (gitoxide) for pure Rust git inspection without linking vulnerable C-libraries (`libgit2`).

---

## 🚨 Reporting a Vulnerability

If you discover a security vulnerability in `gladeshell`, please report it responsibly **without opening a public GitHub Issue**.

### Preferred Method — GitHub Private Security Advisory (Recommended)
1. Navigate to the **[Security Tab](https://github.com/rihadjahanopu/gladeshell/security/advisories/new)** on GitHub.
2. Click **"Report a vulnerability"**.
3. Fill out the report form with reproduction details.

### Alternative Method — Direct Security Email
Send your vulnerability details directly to the lead maintainer:
- **Email**: `rihadjahanopu@gmail.com`
- **Subject**: `[SECURITY VULNERABILITY] gladeshell - <Brief Description>`

---

## 📋 What to Include in Your Vulnerability Report

To help us assess and resolve the issue quickly, please include:

1. **Description**: Clear description of the vulnerability and attack vector.
2. **Reproduction Steps**: Step-by-step instructions or Proof of Concept (PoC).
3. **Impact**: Potential consequences if exploited (e.g. privilege escalation, local file disclosure).
4. **Environment**: Operating System, Architecture (`x86_64` / `aarch64`), Rust version, and `gladeshell --version`.
5. **Suggested Fix**: Optional recommendation or patch suggestion.

---

## ⏱️ Response & Disclosure Timeline

| Phase | SLA Timeline | Description |
| :--- | :--- | :--- |
| **Acknowledgement** | Within **24–48 hours** | We acknowledge receipt of your security report. |
| **Initial Assessment** | Within **3 business days** | Vulnerability severity and impact are triaged. |
| **Patch Development** | Within **7–14 days** | A fix is created, tested across target matrixes, and reviewed. |
| **Public Release & Advisory** | Immediate upon patch | Security update release published with advisory CVE / credits. |

---

## 🔍 Security Scope

### ✅ In Scope
- **Rust Core Engine** (`src/`) — memory handling, CLI parsing, prompt engine, subcommands.
- **Vault Cryptography** (`src/tools/vault.rs`) — AES-256 encryption, key derivation, zeroization.
- **Universal Installers** (`install.sh`, `install.ps1`) — SHA-256 binary validation and idempotent shell injection.
- **CI/CD Actions & Supply Chain** (`.github/workflows/`, Cargo dependencies).

### ❌ Out of Scope
- Vulnerabilities in third-party terminal emulators (e.g. Alacritty, Kitty, Windows Terminal, Zed).
- Exploits requiring root/physical access to an un-encrypted host machine.
- Social engineering or phishing targeting maintainers.

---

## 🏆 Researcher Recognition & Hall of Fame

We value responsible security disclosure. Researchers who responsibly report valid vulnerabilities will be:

- Credited in the GitHub Security Advisory and `CHANGELOG.md`.
- Featured in the `AUTHORS.md` and `CONTRIBUTORS.md` recognition walls.

Thank you for keeping `gladeshell` safe for developers worldwide! 🛡️
