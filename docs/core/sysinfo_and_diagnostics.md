# 📊 SysInfo & Hardware Telemetry Engine

- **Primary Source:** [`src/core/sysinfo.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/core/sysinfo.rs)
- **Architectural Domain:** Low-Overhead Hardware Diagnostics & Metrics

---

## 1. 🎯 Architectural Role

The SysInfo engine collects real-time hardware telemetry for display in rich prompt themes (`full`, `p10k`) and developer diagnostic tools (`pc-info`, `sysmon`).

To prevent blocking interactive prompt rendering:
- Telemetry collection is cached and rate-limited.
- Heavy operations (such as WMI queries on Windows or sysfs walks on Linux) execute asynchronously or fall back to cached states.

---

## 2. 🖥️ Platform Telemetry Implementations

### Linux (`/sys` & `/proc`)
- **CPU & Thermal:** Reads directly from `/sys/class/thermal/thermal_zone*/temp` and `/proc/stat`.
- **Memory:** Parses `/proc/meminfo` (MemTotal, MemAvailable, MemFree).
- **Battery:** Reads `/sys/class/power_supply/BAT*/capacity`.

### macOS (`sysctl` & `IOKit`)
- **CPU & Thermal:** Queries `sysctl` for `hw.ncpu`, `machdep.cpu.brand_string`.
- **Memory:** Queries `vm_stat` and `host_statistics64`.
- **Battery:** Interrogates `AppleSmartBattery` via `IOKit`.

### Windows (`WMI` & `windows-sys`)
- **Hardware Architecture:** Uses Win32 console and kernel APIs (`GetSystemTimes`, `GlobalMemoryStatusEx`).
- **Battery & Thermals:** Queries WMI classes (`Win32_Battery`, `Win32_Processor`) through the `wmi` crate.

---

## 3. ⏱️ Command Duration Formatter

The execution timer measures the elapsed execution time of commands and formats them according to human perception thresholds:
- `< 1000ms`: Omitted or displayed in milliseconds (`45ms`).
- `1s – 60s`: Displayed with one decimal place (`3.4s`).
- `> 60s`: Displayed in human-friendly minutes and seconds (`2m 14s`).
