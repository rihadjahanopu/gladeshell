# 💻 System Diagnostics & PC Optimizer (`pc_info` & `pc_optimizer`)

- **Primary Sources:**
  - Hardware Diagnostics: [`src/tools/pc_info.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/pc_info.rs) (3,675 LoC)
  - System Optimizer: [`src/tools/pc_optimizer.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/pc_optimizer.rs) (2,160 LoC)
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Overview

`pc-info` and `pc-opt` provide enterprise-grade hardware inspection, performance profiling, and memory optimization directly inside the terminal.

---

## 2. 🖥️ Comprehensive Hardware Telemetry (`pc_info`)

Interrogates kernel APIs across platforms (WMI on Windows, `/sys` & `lshw` on Linux, `system_profiler` & `IOKit` on macOS):
- **CPU:** Exact model name, microarchitecture, physical/logical core counts, L1/L2/L3 cache sizes, current clock speeds, and thermal sensors.
- **GPU:** Dedicated/integrated GPUs, VRAM capacity, driver version, PCIe bus link width.
- **RAM:** Total, free, cached memory, DIMM slots, DDR generation, speed in MT/s, and manufacturer.
- **Storage:** NVMe/SATA drive health, SMART status, write endurance, partitions, and IOPS speed tests.
- **Network:** Active network adapters, local IP, public IP, DNS servers, gateway, Wi-Fi SSID, and signal strength (dBm).

---

## 3. ⚡ System Optimizer (`pc_optimizer`)

Provides safe, one-click system hygiene:
- **RAM Trimming:** Purges inactive system file buffers and OS standby memory.
- **DNS Cache Flush:** Clears resolved domain caches (`systemd-resolved` / `dscacheutil` / `ipconfig`).
- **Temporary File Purge:** Removes crash dumps, orphaned IPC sockets, and obsolete log files.
- **Developer Build Cache Cleanup:** Clears old Cargo target artifacts, Go build caches, and Gradle daemon heaps.

---

## 4. 💻 CLI Usage

```bash
# View complete hardware diagnostics
pc-info

# Run system optimization & memory trim
pc-opt

# Run quick disk speed test
pc-info --bench-disk
```
