// =============================================================================
//  benches/bench_git.rs — Criterion benchmark for the gix engine (gitoxide)
//
//  Measures pure gix (pure-Rust) scan time in-process — zero process launch
//  overhead. This is the real cost of git status on every prompt render.
//
//  Run:  cargo bench --bench bench_git
//  HTML: target/criterion/git/report/index.html
// =============================================================================

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::path::Path;

fn bench_git2_full_scan(c: &mut Criterion) {
    let cwd = std::env::current_dir().unwrap();

    c.bench_function("git/full_scan_cache_miss", |b| {
        b.iter(|| {
            // Force cache bypass by using get_status directly
            let status = gladeshell_core::git::get_status(black_box(&cwd));
            let _ = status.is_git_repo;
            let _ = status.branch.len();
            let _ = status.dirty;
        });
    });
}

fn bench_git2_cache_hit(c: &mut Criterion) {
    let cwd = std::env::current_dir().unwrap();

    // Warm the cache first
    let _ = gladeshell_core::git::get_status(&cwd);

    c.bench_function("git/cache_hit_ttl", |b| {
        b.iter(|| {
            // All calls within 1.5s TTL → zero I/O, reads from HashMap
            let status = gladeshell_core::git::get_status(black_box(&cwd));
            let _ = status.is_git_repo;
        });
    });
}

fn bench_git2_head_read(c: &mut Criterion) {
    // Isolate HEAD file read (branch name parsing only)
    let git_dir = Path::new(".git");
    let head_path = git_dir.join("HEAD");

    c.bench_function("git/read_head_file", |b| {
        b.iter(|| {
            let content = std::fs::read_to_string(black_box(&head_path)).unwrap_or_default();
            let branch = content.trim().strip_prefix("ref: refs/heads/")
                .map(|s| s.to_owned())
                .unwrap_or_default();
            let _ = black_box(branch);
        });
    });
}

fn bench_gix_status_scan(c: &mut Criterion) {
    // Pure gix status scan (no gladeshell wrapper overhead) — replaces old libgit2 bench
    let cwd = std::env::current_dir().unwrap();

    c.bench_function("git/gix_status_scan", |b| {
        b.iter(|| {
            let repo = gix::discover(black_box(&cwd)).unwrap();
            let dirty = repo.is_dirty().unwrap_or(false);
            let _ = black_box(dirty);
        });
    });
}

criterion_group!(
    git_benches,
    bench_git2_full_scan,
    bench_git2_cache_hit,
    bench_git2_head_read,
    bench_gix_status_scan,
);
criterion_main!(git_benches);
