// src/core/aliases/categories/python.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "Python".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "py".to_string(),
                value: "python3".to_string(),
                description: "Python 3 interpreter".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "venv".to_string(),
                value: "python3 -m venv .venv".to_string(),
                description: "Create Python virtualenv in .venv".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "act".to_string(),
                value: "source .venv/bin/activate".to_string(),
                description: "Activate Python virtualenv".to_string(),
                only_shells: vec!["bash".to_string(), "zsh".to_string()],
            },
            AliasEntry {
                key: "deact".to_string(),
                value: "deactivate".to_string(),
                description: "Deactivate virtualenv".to_string(),
                only_shells: vec![],
            },
        ],
    }
}
