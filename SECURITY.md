# 🔒 Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 2.x     | ✅ Active support  |
| 1.x     | ⚠️ Critical fixes only |
| < 1.0   | ❌ Not supported   |

---

## 🚨 Reporting a Vulnerability

**Please do NOT open a public GitHub Issue for security vulnerabilities.**

If you discover a security issue in fancybash, please report it responsibly:

### Preferred Method — GitHub Private Advisory

1. Go to the [Security tab](https://github.com/rihadjahanopu/fancybash/security/advisories/new)
2. Click **"Report a vulnerability"**
3. Fill in the details

### Alternative — Direct Email

Send details to: **rihadjahanopu@gmail.com**  
Subject: `[fancybash SECURITY] Brief description`

---

## 📋 What to Include in Your Report

Please provide as much of the following as possible:

- **Description** of the vulnerability
- **Steps to reproduce** the issue
- **Potential impact** (what an attacker could do)
- **Your environment** (OS, shell version, fancybash version)
- **Suggested fix** (optional but appreciated)

---

## ⏱️ Response Timeline

| Step | Timeline |
|------|----------|
| Acknowledgement of report | Within **48 hours** |
| Initial assessment | Within **5 business days** |
| Fix development | Depends on severity |
| Public disclosure | After fix is released |

---

## 🔍 Scope

### In Scope

- **Installer scripts** (`install.sh`, `install.zsh`, `install.ps1`) — e.g., unsafe download handling, improper backup logic
- **config.sh / config.zsh** — e.g., functions that execute unsafe input, command injection risks
- **Web files** (`web/`) — e.g., XSS in the static site
- **GitHub Actions workflows** — e.g., supply chain risks, token misuse

### Out of Scope

- Vulnerabilities in third-party tools that fancybash calls (e.g., `fzf`, `curl`, `git`)
- Issues that require physical access to the machine
- Social engineering attacks
- Theoretical vulnerabilities without a practical exploit path

---

## 🛡️ Security Design Notes

fancybash is designed with security in mind:

- **Installer never blindly executes** — validates the downloaded file is a shell script, not an HTML error page
- **Always backs up** before modifying `.bashrc` / `.zshrc`
- **Boundary markers** make it easy to audit exactly what was added
- **No network calls** from within `config.sh` at shell startup — all aliases are local
- **No `eval`** on user-supplied strings in any function
- **`set -euo pipefail`** is used in installer scripts (not config — would break interactive shell)

---

## 🏆 Recognition

Security researchers who responsibly disclose valid vulnerabilities will be:
- Credited in the release notes (if they wish)
- Mentioned in the `CHANGELOG.md` under the fix

Thank you for helping keep fancybash safe! 🙏
