# 🏛️ Architecture Decision Records (ADR)

This directory documents key architectural, engineering, and design decisions made in **Gladeshell**. Each record captures the architectural context, evaluated alternatives, decision rationale, and operational consequences.

---

## 📋 ADR Index

| ADR | Title | Status | Date |
| :---: | :--- | :---: | :---: |
| [**0001**](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/docs/adr/0001-in-process-c-abi-prompt.md) | In-Process C-ABI Dynamic Library vs Forked Executable Prompt | **Accepted** | 2026-10-01 |
| [**0002**](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/docs/adr/0002-pure-rust-gitoxide.md) | Pure-Rust Gitoxide (`gix`) Integration over Shelling Out to Git | **Accepted** | 2026-10-02 |
| [**0003**](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/docs/adr/0003-domain-driven-engines.md) | Domain-Driven Engine Layer with Zero-Breaking Module Aliases | **Accepted** | 2026-11-09 |
| [**0004**](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/docs/adr/0004-mimalloc-zero-allocation.md) | Global `mimalloc` Allocator & Stack Arena Buffers | **Accepted** | 2026-10-05 |

---

## 📝 Format Standard

All ADRs in Gladeshell follow Michael Nygard's Architecture Decision Record format:
1. **Title:** Numbered sequence with short description.
2. **Status:** `Proposed`, `Accepted`, `Deprecated`, or `Superseded`.
3. **Context:** The technical constraints, latency goals, or problems being solved.
4. **Decision:** What we decided to build and how.
5. **Consequences:** Positive outcomes and trade-offs incurred.
