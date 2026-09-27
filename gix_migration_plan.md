# 🔀 git2 → gix মাইগ্রেশন প্ল্যান

> **লক্ষ্য:** `git2` (C binding) সরিয়ে pure Rust `gix`-এ যাওয়া  
> **ফাইল:** শুধু `src/git/mod.rs` + `benches/bench_git.rs`  
> **সুবিধা:** Pure Rust, দ্রুততর, C dependency নেই

---

## বর্তমান অবস্থা — `git2` কোথায় ব্যবহার হচ্ছে

`src/git/mod.rs`-এর শুধু **একটি ফাংশনে** `git2` ব্যবহার হচ্ছে (line 334):

```rust
fn is_dirty(_git_dir: &std::path::Path, cwd: &std::path::Path) -> bool {
    let repo = match git2::Repository::discover(cwd) { ... };
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true);
    opts.recurse_untracked_dirs(false);
    opts.exclude_submodules(true);
    opts.show(git2::StatusShow::IndexAndWorkdir);
    match repo.statuses(Some(&mut opts)) {
        Ok(statuses) => !statuses.is_empty(),
        Err(_) => false,
    }
}
```

বাকি সব (`read_head`, cache system) — **pure std Rust**, কোনো external crate নেই।

---

## করার ক্রম

```
ধাপ ১  →  Cargo.toml: git2 বাদ, gix 0.88 + sha1 feature
ধাপ ২  →  cargo check → কী error আসে দেখো
ধাপ ৩  →  is_dirty() gix দিয়ে পুনর্লিখন
ধাপ ৪  →  benches/bench_git.rs: git2 bench বাদ, gix bench যোগ
ধাপ ৫  →  cargo test
ধাপ ৬  →  make install → prompt পরীক্ষা
```

---

## ধাপ ১ — Cargo.toml পরিবর্তন

```toml
# সরাও:
git2 = { version = "0.21", default-features = false, optional = true }

# পরিবর্তন করো:
gix = { version = "0.88", default-features = false, features = [
    "revision",
    "worktree-mutation",
    "sha1",     # ← 0.88-এ নতুন required
    "status",   # ← dirty check-এর জন্য
], optional = true }

# feature থেকে git2 বাদ:
prompt-engine = ["gix", "rayon"]  # আগে: ["gix", "git2", "rayon"]
```

---

## ধাপ ৩ — `is_dirty()` পুনর্লিখন

```rust
// আগে (git2):
fn is_dirty(_git_dir: &std::path::Path, cwd: &std::path::Path) -> bool {
    let repo = match git2::Repository::discover(cwd) {
        Ok(r) => r, Err(_) => return false,
    };
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true);
    opts.recurse_untracked_dirs(false);
    opts.exclude_submodules(true);
    opts.renames_head_to_index(false);
    opts.renames_index_to_workdir(false);
    opts.include_ignored(false);
    opts.show(git2::StatusShow::IndexAndWorkdir);
    match repo.statuses(Some(&mut opts)) {
        Ok(statuses) => !statuses.is_empty(),
        Err(_) => false,
    }
}

// পরে (gix):
#[cfg(feature = "gix")]
fn is_dirty(_git_dir: &std::path::Path, cwd: &std::path::Path) -> bool {
    let repo = match gix::discover(cwd) {
        Ok(r) => r, Err(_) => return false,
    };
    repo.is_dirty().unwrap_or(false)
}

#[cfg(not(feature = "gix"))]
fn is_dirty(_git_dir: &std::path::Path, _cwd: &std::path::Path) -> bool {
    false
}
```

---

## ধাপ ৪ — bench_git.rs পরিবর্তন

```rust
// বাদ দাও (git2 bench):
fn bench_git2_libgit2_statuses(c: &mut Criterion) { ... }

// যোগ করো (gix bench):
fn bench_gix_status_scan(c: &mut Criterion) {
    let cwd = std::env::current_dir().unwrap();
    c.bench_function("git/gix_status_scan", |b| {
        b.iter(|| {
            let repo = gix::discover(black_box(&cwd)).unwrap();
            let dirty = repo.is_dirty().unwrap_or(false);
            let _ = black_box(dirty);
        });
    });
}
```

---

## git2 vs gix — dirty check তুলনা

| বিষয় | `git2` | `gix` |
|------|--------|-------|
| Language | C (libgit2) | Pure Rust |
| Startup overhead | C library load | Zero |
| Status scan | ~0.5-2ms | ~0.3-1ms (দ্রুততর) |
| Cross-compile | কঠিন | সহজ |
| Safety | কম (C) | বেশি (Rust) |

---

## সতর্কতা

> ⚠️ `repo.is_dirty()` API confirmed ✅ — exists in gix 0.88 under `status` feature.  
> **sha1 feature required** — `gix-hash 0.27` uses `#[cfg(feature = "sha1")]` on ObjectId enum;  
> without it, rustc 1.98+ fails with E0004 "non-exhaustive patterns" inside gix-hash.  
> Final features: `["revision", "status", "sha1"]`  
> `worktree-mutation` — NOT needed (read-only dirty check, no worktree writes).  
> `max-performance-safe` — removed (was 0.66 feature, gone in 0.88).  
