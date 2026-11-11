# ⚡ Universal File Watcher & Supervisor (`gwatch`)

- **Primary Source:** [`src/tools/gwatch.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/gwatch.rs)
- **Feature Gate:** `#[cfg(feature = "tools")]`
- **Architectural Domain:** Sub-millisecond Developer File Watcher & Process Supervisor

---

## 1. 🎯 Architectural Role

`gwatch` (invoked via `gw`) is a universal, zero-overhead developer hot-reloader. It automatically identifies the project toolchain in the current directory, watches for source file updates via native OS file system events, compiles, supervises child processes, and restarts on save.

### Supported Language Ecosystems:
- **Rust:** `Cargo.toml` projects or single `.rs` files (`cargo run` / `rustc`)
- **C & C++:** `Makefile`, `CMakeLists.txt`, or single `.c`/`.cpp` files (`clang++` / `g++`)
- **Go:** `go.mod` projects or single `.go` files (`go run`)
- **Python:** Scripts or package directories (`python3`)
- **Node.js & TypeScript:** `package.json` scripts or `.ts`/`.js` files (`bun` / `node`)
- **Zig:** `build.zig` projects or `.zig` files (`zig run`)

---

## 2. ⚡ Zero-Overhead Inotify & Event Debouncing

Polling file modified timestamps (`mtime`) on every tick burns CPU and drains laptop battery. `gwatch` uses native OS notification APIs:
- **Linux:** Native `inotify` watches.
- **macOS:** `FSEvents` file stream monitoring.
- **Windows:** `ReadDirectoryChangesW` kernel completion events.

### 200ms Trailing-Edge Debounce:
When an IDE or editor saves a file, it often triggers multiple events (write, flush, attribute update, atomic rename). `gwatch` debounces events within a **200ms trailing-edge window**, ensuring the compiler only triggers once per save.

---

## 3. 🛡️ Sanitizers & Advanced Flags

```bash
# Watch current directory with auto-detected compiler
gw

# Compile single C++ file with GCC/Clang
gw main.cpp

# Enable AddressSanitizer and UndefinedBehaviorSanitizer (C/C++)
gw -s

# Launch Ratatui live process dashboard mode
gw -t

# Preserve terminal scrollback between restarts
gw --no-clear

# Forward CLI arguments to child process
gw -- --port 8080 --dev
```
