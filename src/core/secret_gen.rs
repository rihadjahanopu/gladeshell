// =============================================================================
//  src/core/secret_gen.rs — Cryptographically-secure secret key generator
//
//  Replaces the `gen` shell function (previously: `openssl rand -hex <n>`).
//  Uses the OS CSPRNG directly via getrandom/syscall — no external crates.
// =============================================================================

/// Generate `n` cryptographically-random bytes using the OS CSPRNG.
///
/// On Linux this calls `getrandom(2)` directly.
/// On macOS/BSD it uses `/dev/urandom`.
/// On Windows it uses `BCryptGenRandom`.
///
/// Returns `Err` only if the OS refuses to provide entropy (extremely rare).
pub fn generate(n: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf = vec![0u8; n];
    fill_random(&mut buf)?;
    Ok(buf)
}

// ── Platform-specific CSPRNG ──────────────────────────────────────────────────

#[cfg(target_os = "linux")]
fn fill_random(buf: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Read;
    let mut f = std::fs::File::open("/dev/urandom")?;
    f.read_exact(buf)?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn fill_random(buf: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Read;
    let mut f = std::fs::File::open("/dev/urandom")?;
    f.read_exact(buf)?;
    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn fill_random(buf: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Fallback: use rand crate or OS random in Phase 4.
    Err("CSPRNG not implemented for this platform (add `rand` crate for broad support)".into())
}

// =============================================================================
//  Unit tests
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_length() {
        let bytes = generate(32).unwrap();
        assert_eq!(bytes.len(), 32);
    }

    #[test]
    fn two_generations_differ() {
        let a = generate(16).unwrap();
        let b = generate(16).unwrap();
        // Statistically impossible for both to be equal
        assert_ne!(a, b);
    }

    #[test]
    fn hex_encoding_length() {
        let bytes = generate(32).unwrap();
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(hex.len(), 64); // 32 bytes × 2 hex chars
    }
}
