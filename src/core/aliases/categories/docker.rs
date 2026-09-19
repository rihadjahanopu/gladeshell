// src/core/aliases/categories/docker.rs

use crate::core::aliases::{AliasEntry, AliasGroup};

pub fn group() -> AliasGroup {
    AliasGroup {
        name: "Docker & Kubernetes".to_string(),
        only_shells: vec![],
        aliases: vec![
            // ── 1. Status & Monitoring ─────────────────────────────────────────
            AliasEntry {
                key: "dps".to_string(),
                value: "docker ps --format 'table {{.ID}}\\t{{.Names}}\\t{{.Status}}\\t{{.Ports}}'".to_string(),
                description: "Format table view of running Docker containers".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dpsa".to_string(),
                value: "docker ps -a".to_string(),
                description: "Show all Docker containers (running & stopped)".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "di".to_string(),
                value: "docker images".to_string(),
                description: "List downloaded Docker images".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dvl".to_string(),
                value: "docker volume ls".to_string(),
                description: "List all Docker volumes".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dnl".to_string(),
                value: "docker network ls".to_string(),
                description: "List all Docker networks".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dsize".to_string(),
                value: "docker system df".to_string(),
                description: "Display Docker disk usage summary".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dtop".to_string(),
                value: "docker stats --format 'table {{.Name}}\\t{{.CPUPerc}}\\t{{.MemUsage}}\\t{{.NetIO}}\\t{{.BlockIO}}'".to_string(),
                description: "Live resource usage statistics of containers".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dclean".to_string(),
                value: "docker system prune -af".to_string(),
                description: "Remove all unused Docker resources".to_string(),
                only_shells: vec![],
            },

            // ── 1b. Sudo Docker Shortcuts ──────────────────────────────────────
            AliasEntry {
                key: "sdps".to_string(),
                value: "sudo docker ps --format 'table {{.ID}}\\t{{.Names}}\\t{{.Status}}\\t{{.Ports}}'".to_string(),
                description: "Sudo formatted table of running containers".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "sdpsa".to_string(),
                value: "sudo docker ps -a".to_string(),
                description: "Sudo show all containers".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "sdi".to_string(),
                value: "sudo docker images".to_string(),
                description: "Sudo list Docker images".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "sdvl".to_string(),
                value: "sudo docker volume ls".to_string(),
                description: "Sudo list Docker volumes".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "sdnl".to_string(),
                value: "sudo docker network ls".to_string(),
                description: "Sudo list Docker networks".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "sdsize".to_string(),
                value: "sudo docker system df".to_string(),
                description: "Sudo display Docker disk usage".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "sdtop".to_string(),
                value: "sudo docker stats --format 'table {{.Name}}\\t{{.CPUPerc}}\\t{{.MemUsage}}\\t{{.NetIO}}\\t{{.BlockIO}}'".to_string(),
                description: "Sudo live container stats".to_string(),
                only_shells: vec![],
            },

            // ── 1c. Docker Service Control via systemctl ──────────────────────
            AliasEntry {
                key: "dstart".to_string(),
                value: "sudo systemctl start docker".to_string(),
                description: "Start Docker system daemon".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "doff".to_string(),
                value: "sudo systemctl stop docker".to_string(),
                description: "Stop Docker system daemon".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dstatus".to_string(),
                value: "sudo systemctl status docker".to_string(),
                description: "Check Docker system daemon status".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "denable".to_string(),
                value: "sudo systemctl enable docker && sudo systemctl enable docker.socket".to_string(),
                description: "Enable Docker auto-start on system boot".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "ddisable".to_string(),
                value: "sudo systemctl disable docker && sudo systemctl disable docker.socket".to_string(),
                description: "Disable Docker auto-start on system boot".to_string(),
                only_shells: vec![],
            },

            // ── 2. Container Lifecycle & Control ──────────────────────────────
            AliasEntry {
                key: "dstop".to_string(),
                value: "docker stop".to_string(),
                description: "Stop running container".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "drm".to_string(),
                value: "docker rm".to_string(),
                description: "Remove container".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "drmi".to_string(),
                value: "docker rmi".to_string(),
                description: "Remove image".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "drestart".to_string(),
                value: "docker restart".to_string(),
                description: "Restart container".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dkill".to_string(),
                value: "docker rm -f".to_string(),
                description: "Force kill and remove container".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dstopall".to_string(),
                value: "docker stop $(docker ps -q)".to_string(),
                description: "Stop all running containers".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "drmall".to_string(),
                value: "docker rm $(docker ps -a -q)".to_string(),
                description: "Remove all stopped containers".to_string(),
                only_shells: vec![],
            },

            // ── 3. Debugging & Building ───────────────────────────────────────
            AliasEntry {
                key: "dsh".to_string(),
                value: "docker exec -it".to_string(),
                description: "Open interactive shell inside container".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dlogs".to_string(),
                value: "docker logs -f".to_string(),
                description: "Follow container log output".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dbuild".to_string(),
                value: "docker build -t".to_string(),
                description: "Build Docker image with tag".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dbuild-nocache".to_string(),
                value: "docker build --no-cache -t".to_string(),
                description: "Build Docker image without build cache".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dhist".to_string(),
                value: "docker history".to_string(),
                description: "Show image layer history".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dports".to_string(),
                value: "docker port".to_string(),
                description: "List container port mappings".to_string(),
                only_shells: vec![],
            },

            // ── 4. Docker Compose ─────────────────────────────────────────────
            AliasEntry {
                key: "dcup".to_string(),
                value: "docker compose up -d".to_string(),
                description: "Start Docker compose services detached".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dcdn".to_string(),
                value: "docker compose down".to_string(),
                description: "Stop and remove Docker compose stack".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dclogs".to_string(),
                value: "docker compose logs -f".to_string(),
                description: "Follow Docker compose stack logs".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dcupb".to_string(),
                value: "docker compose up -d --build".to_string(),
                description: "Rebuild and start Docker compose services detached".to_string(),
                only_shells: vec![],
            },

            // ── 5. Quick Test Sandboxes ────────────────────────────────────────
            AliasEntry {
                key: "dtest-ubuntu".to_string(),
                value: "docker run --rm -it ubuntu:latest bash".to_string(),
                description: "Ephemeral test container: Ubuntu latest".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dtest-node".to_string(),
                value: "docker run --rm -it node:alpine sh".to_string(),
                description: "Ephemeral test container: Node alpine".to_string(),
                only_shells: vec![],
            },
            AliasEntry {
                key: "dtest-alpine".to_string(),
                value: "docker run --rm -it alpine:latest sh".to_string(),
                description: "Ephemeral test container: Alpine latest".to_string(),
                only_shells: vec![],
            },

            // ── 6. Kubernetes ─────────────────────────────────────────────────
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
