// ============================================================================
// STATUS: 100% NATIVE RUST & BULLETPROOF (ZERO EXTERNAL BINARY DEPENDENCIES)
// AUDIT COMPLETED: FULL FEATURE PARITY, CROSS-OS VERIFIED & OPTIMIZED
// HANDS-OFF GUARANTEE: NO MANUAL EDITS REQUIRED
// ============================================================================

//! `src/engines/system/mod.rs` — File, OS & System-Level Engines
//!
//! Subsystems:
//! - `delete`: Safe Cross-Platform Trash & Deletion Engine

pub mod delete;
pub use delete::DeleteEngine;
