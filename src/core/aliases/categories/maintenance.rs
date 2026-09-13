// src/core/aliases/categories/maintenance.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "fancybash Maintenance".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "update".to_string(),
                value: "fancybash update".to_string(),
                description: "Non-interactive system package update (APT, Pacman, DNF, Brew, Flatpak, Snap)".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "upgrade".to_string(),
                value: "fancybash upgrade".to_string(),
                description: "Self-upgrade fancybash to latest version".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "clean".to_string(),
                value: "fancybash clean".to_string(),
                description: "Universal system cache and log cleanup".to_string(),
                only_shells: vec![],
            },
        ],
    }
}
