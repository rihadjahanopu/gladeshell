// src/core/aliases/categories/editor.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "Editor & Config".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "to".to_string(),
                value: "code .".to_string(),
                description: "Open current directory in VS Code".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "vm".to_string(),
                value: "vim .".to_string(),
                description: "Open current directory in Vim".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "nv".to_string(),
                value: "nvim .".to_string(),
                description: "Open current directory in Neovim".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "zed".to_string(),
                value: "zed .".to_string(),
                description: "Open current directory in Zed".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "zshrc".to_string(),
                value: "code ~/.zshrc".to_string(),
                description: "Edit Zsh config in VS Code".to_string(),
                only_shells: vec!["zsh".to_string()],
            },
            AliasEntry {
                key: "bashrc".to_string(),
                value: "code ~/.bashrc".to_string(),
                description: "Edit Bash config in VS Code".to_string(),
                only_shells: vec!["bash".to_string()],
            },
            AliasEntry {
                key: "rel".to_string(),
                value: "pkill -f fancybash-daemon 2>/dev/null; rm -f /tmp/fancybash_*.sock(N) 2>/dev/null; source ~/.zshrc; echo \"🔄 Zsh reloaded!\"".to_string(),
                description: "Reload Zsh config".to_string(),
                only_shells: vec!["zsh".to_string()],
            },
            AliasEntry {
                key: "rel".to_string(),
                value: "pkill -f fancybash-daemon 2>/dev/null; rm -f /tmp/fancybash_*.sock 2>/dev/null; source ~/.bashrc; echo \"🔄 Bash reloaded!\"".to_string(),
                description: "Reload Bash config".to_string(),
                only_shells: vec!["bash".to_string()],
            },
            AliasEntry {
                key: "rel".to_string(),
                value: "pkill -f fancybash-daemon 2>/dev/null; rm -f /tmp/fancybash_*.sock 2>/dev/null; source ~/.config/fish/config.fish; echo \"🔄 Fish reloaded!\"".to_string(),
                description: "Reload Fish config".to_string(),
                only_shells: vec!["fish".to_string()],
            },
            AliasEntry {
                key: "rel".to_string(),
                value: ". $PROFILE; Write-Host \"🔄 PowerShell reloaded!\"".to_string(),
                description: "Reload PowerShell config".to_string(),
                only_shells: vec!["pwsh".to_string()],
            },
        ],
    }
}
