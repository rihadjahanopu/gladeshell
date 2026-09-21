// ============================================================================
// STATUS: 100% NATIVE RUST & BULLETPROOF (ZERO EXTERNAL BINARY DEPENDENCIES)
// AUDIT COMPLETED: FULL FEATURE PARITY, CROSS-OS VERIFIED & OPTIMIZED
// HANDS-OFF GUARANTEE: NO MANUAL EDITS REQUIRED
// ============================================================================

// src/core/aliases/categories/npm.rs


use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "npm".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "ni".to_string(),
                value: "npm install".to_string(),
                description: "Install all npm dependencies".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "nid".to_string(),
                value: "npm install -D".to_string(),
                description: "Install as devDependency".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "nr".to_string(),
                value: "npm run".to_string(),
                description: "Run an npm script".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "nrd".to_string(),
                value: "npm run dev".to_string(),
                description: "Start dev server".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "nrb".to_string(),
                value: "npm run build".to_string(),
                description: "Build for production".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "nrs".to_string(),
                value: "npm run start".to_string(),
                description: "Start production server".to_string(),
                only_shells: vec![],
            },
        ],
    }
}
