# 📦 Parallel Compression & Extraction Suite

- **Primary Sources:**
  - Compressor: [`src/tools/compressor.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/compressor.rs)
  - Extractor: [`src/tools/extractor.rs`](file:///home/rihad/Developer/archive/creative-educational-code/gladeshell/src/tools/extractor.rs)
- **Feature Gate:** `#[cfg(feature = "tools")]`

---

## 1. 🎯 Architectural Role

Gladeshell replaces fragmented compression utilities (`tar`, `gzip`, `unzip`, `7z`, `xz`, `bzip2`) with unified, high-performance pure-Rust engines.

### Key Capabilities:
- **Parallel Multi-Core Execution:** Leverages Rayon to compress and decompress multiple files concurrently across all CPU cores.
- **Pure Rust Implementations:** Uses `flate2` (with `zlib-rs`), `tar`, `zip`, `sevenz-rust2`, `bzip2`, and `xz2`. Zero external binaries required.
- **Flat Path & Archive Traversal Protection (Zip Slip Guard):** The extractor strictly sanitizes entry filepaths, preventing directory traversal attacks (`../../etc/passwd`).

---

## 2. 🗜️ Supported Formats

| Format Extension | Compression Engine | Multithreaded | Integrity Check |
| :---: | :---: | :---: | :---: |
| `.zip` | `zip` crate | Yes (per-entry) | CRC32 |
| `.tar.gz` / `.tgz` | `flate2` + `tar` | Yes | CRC32 |
| `.7z` | `sevenz-rust2` | Yes | CRC32 / SHA256 |
| `.tar.bz2` / `.tbz` | `bzip2` + `tar` | Yes | CRC32 |
| `.tar.xz` / `.txz` | `xz2` + `tar` | Yes | CRC64 |

---

## 3. 💻 CLI Usage

```bash
# Compress directory into zip with multi-core parallelism
compress my_folder/ archive.zip

# Compress into tar.gz
compress src/ project.tar.gz

# Extract any supported archive automatically
ex archive.zip
ex payload.7z
ex backup.tar.gz
```
