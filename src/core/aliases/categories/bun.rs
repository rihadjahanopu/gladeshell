// src/core/aliases/categories/bun.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "Bun".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "bi".to_string(),
                value: "bun install".to_string(),
                description: "Install dependencies with Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "brd".to_string(),
                value: "bun run dev".to_string(),
                description: "Run dev script with Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "brb".to_string(),
                value: "bun run build".to_string(),
                description: "Build with Bun".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "brs".to_string(),
                value: "bun run start".to_string(),
                description: "Start production server with Bun".to_string(),
                only_shells: vec![],
            },
        ],
    }
}
