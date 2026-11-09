# 🔐 Cryptography Vault Suite (`vault`)

- **Primary Source:** [`src/tools/vault.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/vault.rs)
- **Feature Gate:** `#[cfg(feature = "tools")]`
- **Security Standards:** AES-256-GCM, PBKDF2-HMAC-SHA256, Zeroize

---

## 1. 🎯 Architectural Overview

The `vault` command provides a military-grade, offline encrypted storage container for developer credentials, secrets, API keys, and sensitive archives.

```
┌─────────────────────────────────────────────────────────────┐
│                    Vault Cryptography                       │
│                                                             │
│  User Master Password                                       │
│          │                                                  │
│          ▼                                                  │
│  PBKDF2-HMAC-SHA256 (600,000 Iterations + 32-byte Salt)    │
│          │                                                  │
│          ▼                                                  │
│  256-bit Symmetric Key                                      │
│          │                                                  │
│          ▼                                                  │
│  AES-256-GCM Authenticated Encryption (12-byte Nonce + Tag) │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. 🛡️ Cryptographic Guarantees

1. **Authenticated Encryption (AES-GCM):**
   Every encrypted block includes a 128-bit authentication tag. Any tampering, corruption, or bit-flipping during storage or transit is detected immediately, preventing chosen-ciphertext attacks.
2. **Key Derivation (PBKDF2):**
   Gladeshell enforces **600,000 iterations** of PBKDF2-HMAC-SHA256 with a unique 32-byte cryptographic salt generated via `rand::rngs::OsRng`. This creates massive GPU/ASIC resistance against offline dictionary and brute-force cracking.
3. **Memory Sanitization (`zeroize`):**
   Master passwords and derived cryptographic keys implement the `Zeroize` trait. When secrets fall out of scope, their memory locations in RAM are explicitly overwritten with zeroes, preventing forensic memory dumps.

---

## 3. 💻 CLI Command Reference

```bash
# Initialize a new encrypted vault
vault init

# Store an encrypted secret
vault set OPENAI_API_KEY "sk-..."

# Retrieve and decrypt secret
vault get OPENAI_API_KEY

# Encrypt entire directory into .vault archive
vault pack secrets/ ./backup.vault

# Decrypt .vault archive
vault unpack ./backup.vault ./restored/
```
