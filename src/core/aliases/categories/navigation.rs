// src/core/aliases/categories/navigation.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "Navigation & Filesystem".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "c".to_string(),
                value: "clear".to_string(),
                description: "Clear terminal screen".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "cls".to_string(),
                value: "clear".to_string(),
                description: "Clear terminal screen (Windows-style alias)".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "..".to_string(),
                value: "cd ..".to_string(),
                description: "Go up one directory".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "...".to_string(),
                value: "cd ../..".to_string(),
                description: "Go up two directories".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "....".to_string(),
                value: "cd ../../..".to_string(),
                description: "Go up three directories".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: ".....".to_string(),
                value: "cd ../../../..".to_string(),
                description: "Go up four directories".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "rd".to_string(),
                value: "cd /".to_string(),
                description: "Jump to filesystem root".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "h".to_string(),
                value: "history".to_string(),
                description: "Show command history".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "path".to_string(),
                value: "echo $PATH | tr ':' '\\n'".to_string(),
                description: "Display PATH entries line-by-line".to_string(),
                only_shells: vec!["bash".to_string(), "zsh".to_string()],
            },
        ],
    }
}
