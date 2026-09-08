// src/core/aliases/categories/system.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "System & Utilities".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "serve".to_string(),
                value: "python3 -m http.server".to_string(),
                description: "Start a simple HTTP server in current directory".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "ports".to_string(),
                value: "ss -tulpn".to_string(),
                description: "Show listening network ports".to_string(),
                only_shells: vec!["bash".to_string(), "zsh".to_string(), "fish".to_string()],
            },
            AliasEntry {
                key: "myip".to_string(),
                value: "ip a | grep inet".to_string(),
                description: "Show local IP addresses".to_string(),
                only_shells: vec!["bash".to_string(), "zsh".to_string(), "fish".to_string()],
            },
            AliasEntry {
                key: "iploc".to_string(),
                value: "curl -s ipinfo.io/json".to_string(),
                description: "Show public IP and location info".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "sysinfo".to_string(),
                value: "uname -a && uptime && free -h".to_string(),
                description: "Show system information summary".to_string(),
                only_shells: vec!["bash".to_string(), "zsh".to_string(), "fish".to_string()],
            },
            AliasEntry {
                key: "bak".to_string(),
                value: "cp $1 $1.bak".to_string(),
                description: "Quick backup of a file".to_string(),
                only_shells: vec!["bash".to_string(), "zsh".to_string()],
            },
        ],
    }
}
