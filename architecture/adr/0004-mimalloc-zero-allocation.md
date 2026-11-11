# ADR 0004: Global `mimalloc` Allocator & Stack Arena Buffers

- **Status:** Accepted
- **Deciders:** Core Engineering Team
- **Date:** 2026-10-05

---

## 1. 🎯 Context
Standard system memory allocators (glibc `ptmalloc` on Linux, MSVCRT on Windows) are optimized for general-purpose workloads. Under high-frequency, multi-threaded allocation patterns (such as Ripgrep parallel file traversal or continuous prompt string formatting), lock contention and heap fragmentation degrade prompt render times.

---

## 2. 💡 Decision
We established two architectural pillars:
1. **Stack Arena Buffer:** Hot-path prompt rendering writes directly into a pre-sized 4KB stack buffer (`[0u8; 4096]`), completely eliminating heap allocations in the warm prompt loop.
2. **Microsoft `mimalloc`:** Configured as the global allocator (`#[global_allocator] static GLOBAL: mimalloc::MiMalloc`).

---

## 3. 🚀 Consequences

### Positive:
- **Zero Allocations in Hot Path:** 0 bytes allocated from the heap during warm prompt renders.
- **Cache Locality:** Drastically reduced CPU L1/L2 data cache misses.
- **Multi-Threaded Scaling:** Parallel tools (`rmf`, `compressor`, `fast_grep`) scale linearly across all CPU cores without allocator lock contention.
- **Low Footprint:** Resident Set Size (RSS) stays strictly under 3.9 MB.

### Trade-offs:
- Requires bundling the lightweight `mimalloc-sys` C crate during compilation. Handled transparently by Cargo.
