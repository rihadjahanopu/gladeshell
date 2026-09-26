# 📦 fancybash — Dependency Upgrade Roadmap

> **তৈরি:** 2026-09-26  
> **লক্ষ্য:** সব outdated dependency ধাপে ধাপে নিরাপদে আপডেট করা  
> **নীতি:** প্রতিটি ধাপের পর `cargo check` + `make install` করে যাচাই করতে হবে

---

## ✅ ইতোমধ্যে সম্পন্ন (2026-09-26)

| Crate | আগে | পরে | ফাইল পরিবর্তন |
|-------|-----|-----|--------------|
| `serde_yaml` | `0.9` (deprecated) | `serde_yml = "0.0"` | `pc_info.rs` — `to_yaml`, `from_yaml` |
| `git2` | `0.19` | `0.21` | শুধু `Cargo.toml` |
| `sysinfo` | `0.31` | `0.32` | শুধু `Cargo.toml` |
| `nvml-wrapper` | `0.10` | `0.13` | শুধু `Cargo.toml` |
| `inquire` | `0.7` | `0.9` | শুধু `Cargo.toml` |
| `grep-printer` | `0.2` | `0.3` | শুধু `Cargo.toml` |
| `rand` | `0.8` | `0.9` | `secret_gen.rs` — `thread_rng()` → `rng()` |
| `rand_chacha` | `0.3` | `0.9` | `vault.rs` — `from_entropy()` → `from_os_rng()` |

---

## 🗺️ আপগ্রেড পরিকল্পনা (ধাপে ধাপে)

---

### ✦ ধাপ ১ — `cargo update` (patch fixes) 🟢 সহজ
**ঝুঁকি:** শূন্য | **সময়:** ৫ মিনিট

```bash
cargo update
cargo check
make install
```

**কী হবে:** `Cargo.lock`-এ minor/patch version আপডেট —
- `clap` 4.6.6 → 4.6.7
- `rustls` 0.23.44 → 0.23.45
- `syn` 3.0.5 → 3.0.6
- `thiserror` 2.0.20 → 2.0.21
- `zerocopy` 0.8.56 → 0.8.59
- এবং আরো ৩০+ minor fix

**কোড পরিবর্তন:** ❌ দরকার নেই

---

### ✦ ধাপ ২ — `criterion` 0.5 → 0.8 🟢 সহজ
**ঝুঁকি:** কম (শুধু dev/bench) | **সময়:** ১০ মিনিট

**`Cargo.toml` পরিবর্তন:**
```toml
# আগে:
criterion = { version = "0.5", features = ["html_reports"] }
# পরে:
criterion = { version = "0.8", features = ["html_reports"] }
```

**পরীক্ষা:**
```bash
cargo check
cargo bench --no-run
```

---

### ✦ ধাপ ৩ — `ratatui` 0.29 → 0.30 + `crossterm` 0.28 → 0.29 🟡 মাঝারি
**ঝুঁকি:** মাঝারি (অনেক TUI ফাইলে ব্যবহার) | **সময়:** ৩০-৬০ মিনিট

**`Cargo.toml` পরিবর্তন:**
```toml
ratatui   = { version = "0.30", optional = true }
crossterm = { version = "0.29", optional = true }
```

**আক্রান্ত ফাইলসমূহ:**
- `src/tools/gbranch.rs`
- `src/tools/fkill.rs`
- `src/tools/system_update.rs`
- `src/tools/updater.rs`
- `src/tools/theme_picker.rs`
- `src/tools/dman.rs`
- `src/tools/fuzzy_cd.rs`
- `src/tools/notes.rs`
- `src/tools/pc_info.rs`
- `src/tools/system_clean.rs`
- `src/tools/ffmedia.rs`
- `src/tools/git_wip.rs`
- `src/tools/history_search.rs`
- `src/tools/universal_clean.rs`
- `src/tools/file_find.rs`
- `src/tools/project_setup/hub.rs`

**সম্ভাব্য breaking changes:**
- `Frame::render_widget` signature — mostly compatible
- `Block::new()` vs `Block::default()` — check করতে হবে

**পরীক্ষা:**
```bash
cargo check 2>&1 | grep "^error"
make install
```

---

### ✦ ধাপ ৪ — `ureq` 2.12 → 3.4 🔴 কঠিন
**ঝুঁকি:** বেশি | **সময়:** ৩০-৪৫ মিনিট

**`Cargo.toml` পরিবর্তন:**
```toml
ureq = { version = "3", optional = true }
```

**আক্রান্ত ফাইলসমূহ:**
- `src/tools/vault.rs` (line 163)
- `src/tools/runtime_installer.rs` (lines 52, 87, 104)
- `src/tools/self_upgrade.rs` (lines 30-31)

**Breaking changes (ureq 2 → 3):**

| আগে (v2) | পরে (v3) |
|----------|----------|
| `res.into_reader()` | `res.into_body().as_reader()` |
| `ureq::get(url).call()` | একই ✅ |
| `ureq::post(url).send_form(...)` | একই ✅ |

**কোড fix উদাহরণ:**
```rust
// আগে:
if let Ok(res) = ureq::get(url).call() {
    let mut reader = res.into_reader();
    reader.read_to_string(&mut body)
}

// পরে:
if let Ok(res) = ureq::get(url).call() {
    let mut reader = res.into_body().as_reader();
    reader.read_to_string(&mut body)
}
```

---

### ✦ ধাপ ৫ — Crypto group: `aes-gcm` 0.10→0.11, `pbkdf2` 0.12→0.13, `sha2` 0.10→0.11 🟡 মাঝারি
**ঝুঁকি:** মাঝারি (vault-এ ব্যবহার) | **সময়:** ২০-৩০ মিনিট

**`Cargo.toml` পরিবর্তন:**
```toml
aes-gcm = { version = "0.11", optional = true }
pbkdf2  = { version = "0.13", optional = true }
sha2    = { version = "0.11", optional = true }
```

**আক্রান্ত ফাইল:** শুধু `src/tools/vault.rs`

**সম্ভাব্য breaking changes:**
- `aes-gcm 0.11`: `KeyInit` trait import পরিবর্তন লাগতে পারে
- `pbkdf2 0.13`: `pbkdf2_hmac` function signature পরীক্ষা করতে হবে
- `sha2 0.11`: mostly compatible

**পরীক্ষা:**
```bash
cargo check 2>&1 | grep "^error"
fancybash vault --help
```

---

### ✦ ধাপ ৬ — `rand` 0.9 → 0.10, `rand_chacha` 0.9 → 0.10 🟡 মাঝারি
**ঝুঁকি:** মাঝারি | **সময়:** ১৫-২০ মিনিট

**`Cargo.toml` পরিবর্তন:**
```toml
rand        = { version = "0.10", optional = true }
rand_chacha = { version = "0.10", optional = true }
```

**আক্রান্ত ফাইলসমূহ:**
- `src/core/secret_gen.rs`
- `src/tools/vault.rs`

**পরীক্ষা:**
```bash
cargo check 2>&1 | grep "^error"
cargo test
```

---

### ✦ ধাপ ৭ — `toml` 0.8 → 1.1 🟡 মাঝারি
**ঝুঁকি:** মাঝারি | **সময়:** ২০-৩০ মিনিট

**`Cargo.toml` পরিবর্তন:**
```toml
toml = "1.1"
```

**সম্ভাব্য breaking changes:**
- `toml::to_string()` / `toml::from_str()` — mostly same
- Error types পরিবর্তন হতে পারে

---

### ✦ ধাপ ৮ — `ansi_term` 0.12 সরানো (unmaintained!) 🔴 কঠিন
**ঝুঁকি:** বেশি | **সময়:** ১-২ ঘণ্টা

> ⚠️ `ansi_term` crate **archived/unmaintained**। নিরাপত্তার জন্য সরানো জরুরি।

**বিকল্প:** `owo-colors` crate

**`Cargo.toml` পরিবর্তন:**
```toml
# সরাও:
ansi_term = { version = "0.12", optional = true }
# যোগ করো:
owo-colors = { version = "4", optional = true }
```

**কোড migration উদাহরণ:**
```rust
// আগে (ansi_term):
use ansi_term::Colour::Red;
println!("{}", Red.bold().paint("Error"));

// পরে (owo-colors):
use owo_colors::OwoColorize;
println!("{}", "Error".red().bold());
```

---

### ✦ ধাপ ৯ — `sysinfo` 0.32 → 0.39 🔴 সবচেয়ে কঠিন
**ঝুঁকি:** সর্বোচ্চ (ব্যাপক API change) | **সময়:** ২-৩ ঘণ্টা

> ⚠️ অনেক API ভেঙে গেছে। এটা সবার শেষে করো।

**`Cargo.toml` পরিবর্তন:**
```toml
sysinfo = { version = "0.39", optional = true }
```

**আক্রান্ত ফাইলসমূহ:**
- `src/tools/fkill.rs`
- `src/tools/pc_info.rs`
- `src/core/sysinfo.rs`

**সম্ভাব্য breaking changes:**
- `refresh_all()` → `refresh_specifics(RefreshKind::...)`
- `Pid::from()` → পরিবর্তন হতে পারে
- Process iteration API পরিবর্তন

**পরীক্ষা:**
```bash
cargo check 2>&1 | grep "^error"
fancybash ftop
fancybash fkill
fancybash pc-info
```

---

### ✦ ধাপ ১০ — `gix` 0.66 → 0.88 🔴 সবচেয়ে জটিল
**ঝুঁকি:** সর্বোচ্চ | **সময়:** অনির্দিষ্ট

> ⚠️ `max-performance-safe` feature এর সাথে `gix-hash` SHA feature flag incompatibility আছে। আলাদা গবেষণা দরকার।

**আগে করণীয়:** gix 0.88 changelog ভালোভাবে পড়তে হবে।

**আক্রান্ত ফাইল:** `src/git/mod.rs`, `src/core/prompt.rs`

---

### ✦ ধাপ ১১ — Windows-only: `wmi` 0.14→0.18, `windows-sys` 0.59→0.61 🟢 সহজ
**ঝুঁকি:** কম (Linux-এ compile হয় না) | **সময়:** ১০ মিনিট

```toml
wmi         = "0.18"
windows-sys = { version = "0.61", features = ["Win32_System_Console"] }
```

---

## 📋 Quick Reference — আপডেটের ক্রম

```
ধাপ ১  →  cargo update (patch fixes)           🟢 এখনই করো
ধাপ ২  →  criterion 0.5 → 0.8                 🟢 সহজ
ধাপ ৩  →  ratatui + crossterm                  🟡 মাঝারি
ধাপ ৪  →  ureq 2 → 3                          🔴 কঠিন
ধাপ ৫  →  aes-gcm + pbkdf2 + sha2 (crypto)    🟡 মাঝারি
ধাপ ৬  →  rand + rand_chacha 0.9 → 0.10        🟡 মাঝারি
ধাপ ৭  →  toml 0.8 → 1.1                      🟡 মাঝারি
ধাপ ৮  →  ansi_term → owo-colors              🔴 কঠিন
ধাপ ৯  →  sysinfo 0.32 → 0.39                 🔴 কঠিন
ধাপ ১০ →  gix 0.66 → 0.88                     🔴 সবচেয়ে জটিল
ধাপ ১১ →  wmi + windows-sys (Windows only)    🟢 সহজ
```

---

## 🛡️ প্রতিটি ধাপের পর করণীয়

```bash
# ১. Compile check
cargo check 2>&1 | grep "^error"

# ২. Full build
cargo build --release

# ৩. Install করে পরীক্ষা
make install
fancybash --help

# ৪. Git commit করো
git add -A
git commit -m "chore: upgrade <crate_name> vX.X → vY.Y"
```

---

## 📝 অগ্রগতি ট্র্যাকার

- [x] ধাপ ০ — `serde_yaml`, `rand`, `rand_chacha`, `git2`, `sysinfo`, `nvml-wrapper`, `inquire`, `grep-printer` আপডেট
- [ ] ধাপ ১ — `cargo update` (patch fixes)
- [ ] ধাপ ২ — `criterion`
- [ ] ধাপ ৩ — `ratatui` + `crossterm`
- [ ] ধাপ ৪ — `ureq`
- [ ] ধাপ ৫ — `aes-gcm` + `pbkdf2` + `sha2`
- [ ] ধাপ ৬ — `rand` + `rand_chacha` (0.9→0.10)
- [ ] ধাপ ৭ — `toml`
- [ ] ধাপ ৮ — `ansi_term` → `owo-colors`
- [ ] ধাপ ৯ — `sysinfo`
- [ ] ধাপ ১০ — `gix`
- [ ] ধাপ ১১ — `wmi` + `windows-sys`
