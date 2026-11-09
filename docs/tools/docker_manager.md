# 🐳 Docker Manager TUI (`dman`)

- **Primary Source:** [`src/tools/dman.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/dman.rs) (1,110 LoC)
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Overview

`dman` provides a lightweight, pure-Rust terminal dashboard for managing Docker containers, images, and volumes without needing heavy GUI clients like Docker Desktop.

```
┌─────────────────────────────────────────────────────────────┐
│                    dman Docker Manager                      │
│  [Containers]   Images   Volumes   Networks                 │
│                                                             │
│  ID          NAME              IMAGE          STATUS        │
│  a1b2c3d4    postgres-db       postgres:16    Up (healthy)  │
│  e5f6g7h8    redis-cache       redis:7-alpine Up (6379)     │
│  9i0j1k2l    web-frontend      node:20        Exited (0)    │
│                                                             │
│  [s] Start  [p] Stop  [r] Restart  [l] Logs  [d] Delete     │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. ⚡ Key Capabilities

- **Zero External Daemon Dependencies:** Connects directly to `/var/run/docker.sock` on Linux/macOS or `//./pipe/docker_engine` on Windows using native HTTP-over-socket protocols.
- **Live Streamed Logs:** Pressing `l` streams standard output and error in real-time with automatic ANSI color rendering.
- **Resource Stats:** Shows real-time container CPU% and memory consumption directly in the dashboard.
- **Fast Prune:** Quick shortcut (`P`) to prune dangling images and stopped containers safely.

---

## 3. 💻 CLI Usage

```bash
# Launch Docker Manager TUI
dman
```
