# 🔒 Security Policy

## Supported Versions

| Version | Supported             | Engine                    |
| ------- | --------------------- | ------------------------- |
| 2.x     | ✅ Active support     | Pure Rust Core Engine     |
| 1.x     | ⚠️ Legacy maintenance | Shell Script Architecture |
| < 1.0   | ❌ Not supported      | Retired                   |

---

## 🚨 Reporting a Vulnerability

**Please do NOT open a public GitHub Issue for security vulnerabilities.**

If you discover a security issue in gladeshell, please report it responsibly:

### Preferred Method — GitHub Private Advisory

1. Go to the [Security tab](https://github.com/rihadjahanopu/gladeshell/security/advisories/new)
2. Click **"Report a vulnerability"**
3. Fill in the details

### Alternative — Direct Email

Send details to: **rihadjahanopu@gmail.com**
Subject: `[gladeshell SECURITY] Brief description`

---

## 📋 What to Include in Your Report

Please provide as much of the following as possible:

- **Description** of the vulnerability
- **Steps to reproduce** the issue
- **Potential impact** (what an attacker could do)
- **Your environment** (OS, Rust toolchain version, gladeshell version)
- **Suggested fix** (optional but appreciated)

---

## ⏱️ Response Timeline

| Step                      | Timeline                   |
| ------------------------- | -------------------------- |
| Acknowledgement of report | Within **48 hours**        |
| Initial assessment        | Within **5 business days** |
| Fix development           | Depends on severity        |
| Public disclosure         | After fix is released      |

---

## 🔍 Scope

### In Scope

- **Rust Core Engine** (`src/`) — memory safety, bounds checking, input sanitization in TUI modules
- **Installer & Setup Scripts** (`install.sh`, `install.ps1`) — safe binary downloading, SHA-256 verification
- **Web files** (`web/`) — static portal security
- **GitHub Actions & Supply Chain** (`.github/`, dependencies in `Cargo.lock`)

### Out of Scope

- Vulnerabilities in external terminal emulators calling gladeshell
- Issues requiring root physical access to the host machine
- Theoretical vulnerabilities without a practical exploit path

---

## 🛡️ Security & Memory Safety Guarantees

gladeshell is designed with strict security standards:

- **100% Memory Safe**: Built in safe Rust with `#![deny(unsafe_code)]` constraints across tools.
- **SHA-256 Checksum Verification**: Installers verify binary checksums before execution.
- **No Background Network Telemetry**: Zero background telemetry calls during shell startup.
- **Input Sanitization**: Terminal escapes and shell inputs are sanitized to prevent command injections.

---

## 🏆 Recognition

Security researchers who responsibly disclose valid vulnerabilities will be:

- Credited in release notes and `CHANGELOG.md`
- Added to `AUTHORS.md` contributor list

Thank you for helping keep gladeshell safe! 🙏
