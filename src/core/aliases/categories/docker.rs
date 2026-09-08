// src/core/aliases/categories/docker.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "Docker & Kubernetes".to_string(),
        only_shells: vec![],
        aliases: vec![
            AliasEntry {
                key: "dps".to_string(),
                value: "docker ps -a".to_string(),
                description: "Show all Docker containers".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dclean".to_string(),
                value: "docker system prune -af".to_string(),
                description: "Remove all unused Docker resources".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dstop".to_string(),
                value: "docker stop".to_string(),
                description: "Stop a container".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "drm".to_string(),
                value: "docker rm".to_string(),
                description: "Remove a container".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dim".to_string(),
                value: "docker images".to_string(),
                description: "List Docker images".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dcup".to_string(),
                value: "docker compose up -d".to_string(),
                description: "Start Docker compose services detached".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dcdown".to_string(),
                value: "docker compose down".to_string(),
                description: "Stop Docker compose services".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dclogs".to_string(),
                value: "docker compose logs -f".to_string(),
                description: "Follow Docker compose logs".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "k".to_string(),
                value: "kubectl".to_string(),
                description: "Kubectl shorthand".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "kgp".to_string(),
                value: "kubectl get pods".to_string(),
                description: "Kubectl get pods".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "kgs".to_string(),
                value: "kubectl get services".to_string(),
                description: "Kubectl get services".to_string(),
                only_shells: vec![],
            },
        ],
    }
}
