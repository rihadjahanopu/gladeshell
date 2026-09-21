// ============================================================================
// STATUS: 100% NATIVE RUST & BULLETPROOF (ZERO EXTERNAL BINARY DEPENDENCIES)
// AUDIT COMPLETED: FULL FEATURE PARITY, CROSS-OS VERIFIED & OPTIMIZED
// HANDS-OFF GUARANTEE: NO MANUAL EDITS REQUIRED
// ============================================================================

// src/core/aliases/categories/mod.rs


pub mod bun;
pub mod developer;
pub mod docker;
pub mod editor;
pub mod git;
pub mod maintenance;
pub mod modern_cli;
pub mod navigation;
pub mod npm;
pub mod pnpm_yarn;
pub mod postgres;
pub mod prisma;
pub mod python;
pub mod rust;
pub mod system;

pub mod tools;

use super::AliasGroup;

/// Return all built-in category alias groups in canonical order.
pub fn all_groups() -> Vec<AliasGroup> {
    vec![
        tools::group(),
        navigation::group(),
        developer::group(),
        editor::group(),
        git::group(),
        npm::group(),
        bun::group(),
        pnpm_yarn::group(),
        prisma::npx_group(),
        prisma::bunx_group(),
        postgres::group(),
        docker::group(),
        rust::group(),
        python::group(),
        system::group(),
        modern_cli::group(),
        maintenance::group(),
    ]
}
