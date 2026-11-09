# 🔒 Core Environment, Security & File Safety

- **Primary Sources:**
  - Environment Sanitizer: [`src/core/env.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/core/env.rs) (248 LoC)
  - Cryptographic Entropy: [`src/core/secret_gen.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/core/secret_gen.rs) (70 LoC)
  - File Utilities & Atomic Ops: [`src/core/utils.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/core/utils.rs) (108 LoC)

---

## 1. 🎯 Architectural Role

The security and stability of a shell environment depends heavily on how it manages environment variables, generates cryptographic randomness, and writes configuration files to disk.

This core layer guarantees:
1. **Atomic File Writes:** Prevents file corruption if the terminal process is abruptly killed during a configuration save.
2. **UNC & Cross-Platform Path Normalization:** Handles Windows extended-length paths (`\\?\C:\...`) seamlessly alongside Unix paths.
3. **OS-Level Cryptographic Entropy:** Secure generation of passwords, API tokens, and salts without predictable pseudo-random seeds.

---

## 2. 🛡️ Atomic File Replacement (`atomic_replace`)

When saving themes (`~/.gladeshell_theme`), alias configs, or vault archives, writing directly to the target file is vulnerable to partial writes if power is lost or the process is terminated.

`src/core/utils.rs` implements `atomic_replace`:
```rust
pub fn atomic_replace(path: &Path, content: &[u8]) -> std::io::Result<()> {
    let tmp_path = path.with_extension(format!("tmp.{}", std::process::id()));
    std::fs::write(&tmp_path, content)?;
    std::fs::rename(&tmp_path, path)?; // Atomic OS-level filesystem metadata swap
    Ok(())
}
```
Because `std::fs::rename` is an atomic operating system system call on POSIX (`rename(2)`) and Windows (`MoveFileExW` with `MOVEFILE_REPLACE_EXISTING`), the destination file is never left in a corrupted or half-written state.

---

## 3. 🎲 Cryptographically Secure Entropy (`secret_gen.rs`)

Generates high-entropy passwords, alphanumeric tokens, and hex seeds using OS kernel entropy pools (`/dev/urandom` on Unix, `BCryptGenRandom` on Windows):
- **Character Sets:** High-entropy mixture of uppercase, lowercase, numbers, and non-ambiguous symbols (avoiding confusing `l`, `1`, `O`, `0`).
- **Cryptographic Independence:** Two consecutive generations are guaranteed to be cryptographically independent (`test_two_generations_differ`).
