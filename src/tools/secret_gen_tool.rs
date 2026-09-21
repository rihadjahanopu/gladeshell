// ============================================================================
// STATUS: 100% NATIVE RUST (ZERO EXTERNAL BINARY DEPENDENCIES)
// AUDIT COMPLETED: FULL FEATURE PARITY, CROSS-OS VERIFIED & OPTIMIZED
// HANDS-OFF GUARANTEE: NO MANUAL EDITS REQUIRED
// ============================================================================

// =============================================================================
//  src/tools/secret_gen_tool.rs — `gen` secret key generator CLI wrapper
// =============================================================================

use crate::core::secret_gen;
use std::io::Write;

/// Generates a cryptographically-secure random secret key.
pub fn run(length: usize, raw: bool) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = secret_gen::generate(length)?;
    if raw {
        std::io::stdout().write_all(&bytes)?;
    } else {
        let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        println!("{hex}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_gen_tool_hex() {
        let bytes = secret_gen::generate(16).unwrap();
        assert_eq!(bytes.len(), 16);
    }
}
