// =============================================================================
//  benches/bench_prompt.rs — Criterion benchmark for the prompt renderer
//
//  Performance target (from prompt.rs contract):
//    render() must complete in < 1 ms  (goal: ~50 µs)
//
//  Run:  cargo bench --bench bench_prompt
//  HTML: target/criterion/Prompt/report/index.html
// =============================================================================

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use fancybash_core::core::prompt::{render, PromptContext, THEMES};

// ── Helper: build a filled PromptContext from plain &str values ───────────────

fn make_ctx(
    cwd: &str,
    user: &str,
    host: &str,
    branch: &str,
    dirty: bool,
    exit: i32,
    theme: usize,
) -> PromptContext {
    let mut ctx = PromptContext::default();

    let cwd_b = cwd.as_bytes();
    let cwd_len = cwd_b.len().min(511);
    ctx.cwd[..cwd_len].copy_from_slice(&cwd_b[..cwd_len]);
    ctx.cwd_len = cwd_len;

    let user_b = user.as_bytes();
    let user_len = user_b.len().min(63);
    ctx.user[..user_len].copy_from_slice(&user_b[..user_len]);
    ctx.user_len = user_len;

    let host_b = host.as_bytes();
    let host_len = host_b.len().min(63);
    ctx.host[..host_len].copy_from_slice(&host_b[..host_len]);
    ctx.host_len = host_len;

    let branch_b = branch.as_bytes();
    let branch_len = branch_b.len().min(127);
    ctx.git_branch[..branch_len].copy_from_slice(&branch_b[..branch_len]);
    ctx.git_branch_len = branch_len;

    ctx.git_dirty  = dirty;
    ctx.last_exit  = exit;
    ctx.theme_id   = theme;
    ctx
}

// ── Benchmarks ────────────────────────────────────────────────────────────────

fn bench_render_default(c: &mut Criterion) {
    let ctx = make_ctx(
        "/home/rihad/Developer/dev/fancybash-rs",
        "rihad",
        "arch",
        "main",
        false,
        0,
        0, // default theme
    );
    let mut buf = [0u8; 4096];

    c.bench_function("render/default_theme_clean", |b| {
        b.iter(|| {
            let _ = render(black_box(&ctx), black_box(&mut buf));
        });
    });
}

fn bench_render_dirty(c: &mut Criterion) {
    let ctx = make_ctx(
        "/home/rihad/Developer/dev/fancybash-rs",
        "rihad",
        "arch",
        "feat/modular-refactor",
        true,  // dirty tree
        1,     // non-zero exit
        0,
    );
    let mut buf = [0u8; 4096];

    c.bench_function("render/dirty_tree_nonzero_exit", |b| {
        b.iter(|| {
            let _ = render(black_box(&ctx), black_box(&mut buf));
        });
    });
}

fn bench_render_no_git(c: &mut Criterion) {
    let ctx = make_ctx("/tmp", "root", "server", "", false, 0, 0);
    let mut buf = [0u8; 4096];

    c.bench_function("render/no_git_branch", |b| {
        b.iter(|| {
            let _ = render(black_box(&ctx), black_box(&mut buf));
        });
    });
}

fn bench_render_sample_themes(c: &mut Criterion) {
    // Sample 3 representative themes instead of all 55 (keeps bench fast)
    let mut group = c.benchmark_group("render/sample_themes");
    let mut buf = [0u8; 4096];
    let sample_ids = [0usize, 10, 27]; // default, mid, late theme

    for &id in &sample_ids {
        let theme = &THEMES[id];
        let ctx = make_ctx("/home/rihad/project", "rihad", "arch", "main", false, 0, id);
        group.bench_with_input(
            BenchmarkId::from_parameter(theme.name),
            &ctx,
            |b, ctx| {
                b.iter(|| {
                    let _ = render(black_box(ctx), black_box(&mut buf));
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_render_default,
    bench_render_dirty,
    bench_render_no_git,
    bench_render_sample_themes,
);
criterion_main!(benches);
