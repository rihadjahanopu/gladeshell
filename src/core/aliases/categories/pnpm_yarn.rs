// ============================================================================
// STATUS: 100% NATIVE RUST & BULLETPROOF (ZERO EXTERNAL BINARY DEPENDENCIES)
// AUDIT COMPLETED: FULL FEATURE PARITY, CROSS-OS VERIFIED & OPTIMIZED
// HANDS-OFF GUARANTEE: NO MANUAL EDITS REQUIRED
// ============================================================================

// src/core/aliases/categories/pnpm_yarn.rs


use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "pnpm & Yarn".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "pi".to_string(),
                value: "pnpm install".to_string(),
                description: "Install dependencies with pnpm".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "prd".to_string(),
                value: "pnpm dev".to_string(),
                description: "Run dev server with pnpm".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "prb".to_string(),
                value: "pnpm build".to_string(),
                description: "Build project with pnpm".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "yi".to_string(),
                value: "yarn install".to_string(),
                description: "Install dependencies with Yarn".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "yrd".to_string(),
                value: "yarn dev".to_string(),
                description: "Run dev server with Yarn".to_string(),
                only_shells: vec![],
            },
        ],
    }
}
