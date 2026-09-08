// src/core/aliases/categories/modern_cli.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "Modern CLI Wrappers".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "bat".to_string(),
                value: "bat --paging=never".to_string(),
                description: "bat (cat with syntax highlighting)".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "cat".to_string(),
                value: "bat --paging=never --style=plain".to_string(),
                description: "Drop-in cat replacement using bat".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "ls".to_string(),
                value: "eza --icons --group-directories-first".to_string(),
                description: "eza-powered ls with icons".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "ll".to_string(),
                value: "eza -lah --icons --group-directories-first --git".to_string(),
                description: "Long listing with hidden files, git status, icons".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "la".to_string(),
                value: "eza -a --icons".to_string(),
                description: "List all files including hidden".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "lt".to_string(),
                value: "eza --tree --icons --level=2".to_string(),
                description: "Tree view (depth 2) via eza".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "tree".to_string(),
                value: "eza --tree --icons".to_string(),
                description: "Full tree view via eza".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "z".to_string(),
                value: "zoxide".to_string(),
                description: "Smart cd with zoxide".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "zi".to_string(),
                value: "zoxide query --interactive".to_string(),
                description: "Interactive zoxide jump".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "tldr".to_string(),
                value: "tldr".to_string(),
                description: "Simplified man pages via tldr".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "trash".to_string(),
                value: "gio trash".to_string(),
                description: "Safe delete — moves files to system trash via GIO".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "fh".to_string(),
                value: "eval \"$( (fc -l 1 2>/dev/null || history) | sed 's/^[ ]*[0-9]*[ ]*//' | fzf --reverse +s)\"".to_string(),
                description: "FZF interactive command history search and exec".to_string(),
                only_shells: vec!["bash".to_string(), "zsh".to_string()],
            },
        ],
    }
}
