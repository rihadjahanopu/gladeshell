// src/core/aliases/categories/postgres.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "PostgreSQL".to_string(),
        only_shells: vec!["bash".to_string(), "zsh".to_string(), "fish".to_string()],
        aliases: vec![
            AliasEntry {
                key: "pgstart".to_string(),
                value: "sudo systemctl start postgresql".to_string(),
                description: "Start PostgreSQL service".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "pgstop".to_string(),
                value: "sudo systemctl stop postgresql".to_string(),
                description: "Stop PostgreSQL service".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "pgrestart".to_string(),
                value: "sudo systemctl restart postgresql".to_string(),
                description: "Restart PostgreSQL service".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "pgstatus".to_string(),
                value: "sudo systemctl status postgresql".to_string(),
                description: "Show PostgreSQL service status".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "pgl".to_string(),
                value: "sudo -u postgres psql".to_string(),
                description: "Login as postgres superuser".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "pgls".to_string(),
                value: "psql -U postgres -c '\\l'".to_string(),
                description: "List all databases".to_string(),
                only_shells: vec![],
            },
        ],
    }
}
