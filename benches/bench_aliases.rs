// =============================================================================
//  benches/bench_aliases.rs — Criterion benchmark for the alias parser/renderer
//
//  Measures:
//    • AliasFile::from_toml()  — TOML parse speed
//    • AliasFile::render()     — shell export render speed (bash / zsh / fish)
//
//  Run:  cargo bench --bench bench_aliases
//  HTML: target/criterion/Aliases/report/index.html
// =============================================================================

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use fancybash_core::core::aliases::{AliasFile, Shell};

// ── Minimal inline TOML (no disk I/O) ─────────────────────────────────────────

const SMALL_TOML: &str = r#"
[[group]]
name = "navigation"
aliases = [
    { key = "...",  value = "cd ../.." },
    { key = "....", value = "cd ../../.." },
    { key = "~",    value = "cd ~" },
]

[[group]]
name = "git"
aliases = [
    { key = "gs",   value = "git status -sb" },
    { key = "ga",   value = "git add ." },
    { key = "gc",   value = "git commit -m" },
    { key = "gp",   value = "git push" },
    { key = "gl",   value = "git log --oneline --graph -20" },
]
"#;

// Simulated large aliases.toml (100 entries across 10 groups)
fn make_large_toml() -> String {
    let mut s = String::with_capacity(8192);
    for g in 0..10 {
        s.push_str(&format!("\n[[group]]\nname = \"group{g}\"\naliases = [\n"));
        for i in 0..10 {
            s.push_str(&format!(
                "  {{ key = \"cmd{g}_{i}\", value = \"echo \\\"group {g} cmd {i}\\\"\" }},\n"
            ));
        }
        s.push_str("]\n");
    }
    s
}

// ── Benchmarks ────────────────────────────────────────────────────────────────

fn bench_parse_small(c: &mut Criterion) {
    c.bench_function("aliases/parse_small_toml", |b| {
        b.iter(|| {
            let _ = AliasFile::from_toml(black_box(SMALL_TOML)).unwrap();
        });
    });
}

fn bench_parse_large(c: &mut Criterion) {
    let large = make_large_toml();
    c.bench_function("aliases/parse_large_toml_100entries", |b| {
        b.iter(|| {
            let _ = AliasFile::from_toml(black_box(&large)).unwrap();
        });
    });
}

fn bench_render_bash(c: &mut Criterion) {
    let af = AliasFile::from_toml(SMALL_TOML).unwrap();
    c.bench_function("aliases/render_bash", |b| {
        b.iter(|| {
            let _ = black_box(&af).render(black_box(Shell::Bash));
        });
    });
}

fn bench_render_zsh(c: &mut Criterion) {
    let af = AliasFile::from_toml(SMALL_TOML).unwrap();
    c.bench_function("aliases/render_zsh", |b| {
        b.iter(|| {
            let _ = black_box(&af).render(black_box(Shell::Zsh));
        });
    });
}

fn bench_render_fish(c: &mut Criterion) {
    let af = AliasFile::from_toml(SMALL_TOML).unwrap();
    c.bench_function("aliases/render_fish", |b| {
        b.iter(|| {
            let _ = black_box(&af).render(black_box(Shell::Fish));
        });
    });
}

fn bench_render_large_bash(c: &mut Criterion) {
    let large = make_large_toml();
    let af = AliasFile::from_toml(&large).unwrap();
    c.bench_function("aliases/render_large_bash_100entries", |b| {
        b.iter(|| {
            let _ = black_box(&af).render(black_box(Shell::Bash));
        });
    });
}

criterion_group!(
    benches,
    bench_parse_small,
    bench_parse_large,
    bench_render_bash,
    bench_render_zsh,
    bench_render_fish,
    bench_render_large_bash,
);
criterion_main!(benches);
