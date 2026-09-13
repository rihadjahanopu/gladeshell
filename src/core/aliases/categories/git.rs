// src/core/aliases/categories/git.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "Git".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "gs".to_string(),
                value: "git status -sb".to_string(),
                description: "Git status (short branch format)".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "ga".to_string(),
                value: "git add .".to_string(),
                description: "Stage all changes".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gc".to_string(),
                value: "git commit -m".to_string(),
                description: "Git commit with message".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gp".to_string(),
                value: "git push".to_string(),
                description: "Push to remote".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gpf".to_string(),
                value: "git push --force-with-lease".to_string(),
                description: "Force push safely with lease".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gl".to_string(),
                value: "git log --oneline --graph --decorate -n 15".to_string(),
                description: "Pretty git log (last 15 commits)".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gd".to_string(),
                value: "git diff".to_string(),
                description: "Show unstaged diff".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gds".to_string(),
                value: "git diff --staged".to_string(),
                description: "Show staged diff".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gst".to_string(),
                value: "git stash".to_string(),
                description: "Stash working tree changes".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gsp".to_string(),
                value: "git stash pop".to_string(),
                description: "Apply and drop most recent stash".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gf".to_string(),
                value: "git fetch --all --prune".to_string(),
                description: "Fetch all remotes and prune deleted branches".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gco".to_string(),
                value: "git checkout".to_string(),
                description: "Checkout branch or file".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gb".to_string(),
                value: "git branch -a".to_string(),
                description: "List all local and remote branches".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gm".to_string(),
                value: "git merge".to_string(),
                description: "Merge branch".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "grb".to_string(),
                value: "git rebase".to_string(),
                description: "Rebase current branch".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gsw".to_string(),
                value: "git switch".to_string(),
                description: "Switch branch".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gcl".to_string(),
                value: "git clone".to_string(),
                description: "Clone repository".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gwip".to_string(),
                value: "fancybash gwip".to_string(),
                description: "Interactive Git stage, commit & push".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "gcommit".to_string(),
                value: "fancybash gwip".to_string(),
                description: "Alias for gwip".to_string(),
                only_shells: vec![],
            },
        ],
    }
}
