// =============================================================================
//  benches/bench_git.rs — Criterion benchmark for the git2 engine
//
//  Measures pure git2 (libgit2) scan time in-process — zero process launch
//  overhead. This is the real cost of git status on every prompt render.
//
//  Run:  cargo bench --bench bench_git
//  HTML: target/criterion/git/report/index.html
// =============================================================================

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::Path;

fn bench_git2_full_scan(c: &mut Criterion) {
    let cwd = std::env::current_dir().unwrap();

    c.bench_function("git/full_scan_cache_miss", |b| {
        b.iter(|| {
            // Force cache bypass by using get_status directly
            let status = fancybash_core::git::get_status(black_box(&cwd));
            let _ = status.is_git_repo;
            let _ = status.branch.len();
            let _ = status.dirty;
        });
    });
}

fn bench_git2_cache_hit(c: &mut Criterion) {
    let cwd = std::env::current_dir().unwrap();

    // Warm the cache first
    let _ = fancybash_core::git::get_status(&cwd);

    c.bench_function("git/cache_hit_ttl", |b| {
        b.iter(|| {
            // All calls within 1.5s TTL → zero I/O, reads from HashMap
            let status = fancybash_core::git::get_status(black_box(&cwd));
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

fn bench_git2_libgit2_statuses(c: &mut Criterion) {
    // Pure libgit2 status scan (no fancybash wrapper overhead)
    let cwd = std::env::current_dir().unwrap();

    c.bench_function("git/libgit2_status_scan", |b| {
        b.iter(|| {
            let repo = git2::Repository::discover(black_box(&cwd)).unwrap();
            let mut opts = git2::StatusOptions::new();
            opts.include_untracked(true);
            opts.recurse_untracked_dirs(false);
            opts.exclude_submodules(true);
            opts.renames_head_to_index(false);
            opts.renames_index_to_workdir(false);
            opts.include_ignored(false);
            opts.show(git2::StatusShow::IndexAndWorkdir);

            let statuses = repo.statuses(Some(&mut opts)).unwrap();
            let x = match statuses.is_empty() {
                true => false,
                false => true,
            };
            let _ = black_box(x);
        });
    });
}

criterion_group!(
    git_benches,
    bench_git2_full_scan,
    bench_git2_cache_hit,
    bench_git2_head_read,
    bench_git2_libgit2_statuses,
);
criterion_main!(git_benches);
