// src/core/aliases/categories/developer.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "Developer Directories".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "dev".to_string(),
                value: "cd ~/Developer".to_string(),
                description: "Jump to Developer folder".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "doc".to_string(),
                value: "cd ~/Documents".to_string(),
                description: "Jump to Documents folder".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dow".to_string(),
                value: "cd ~/Downloads".to_string(),
                description: "Jump to Downloads folder".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "des".to_string(),
                value: "cd ~/Desktop".to_string(),
                description: "Jump to Desktop folder".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "pic".to_string(),
                value: "cd ~/Pictures".to_string(),
                description: "Jump to Pictures folder".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "vid".to_string(),
                value: "cd ~/Videos".to_string(),
                description: "Jump to Videos folder".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "mus".to_string(),
                value: "cd ~/Music".to_string(),
                description: "Jump to Music folder".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "ar".to_string(),
                value: "cd ~/Developer/archive".to_string(),
                description: "Jump to Archive directory".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "ba".to_string(),
                value: "cd ~/Developer/backend".to_string(),
                description: "Jump to Backend directory".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "de".to_string(),
                value: "cd ~/Developer/dev".to_string(),
                description: "Jump to Dev playground".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "fr".to_string(),
                value: "cd ~/Developer/frontend".to_string(),
                description: "Jump to Frontend directory".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "fu".to_string(),
                value: "cd ~/Developer/fullstack".to_string(),
                description: "Jump to Fullstack directory".to_string(),
                only_shells: vec![],
            },
        ],
    }
}
