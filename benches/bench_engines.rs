// ============================================================================
//  benches/bench_engines.rs — Criterion Micro-benchmarks for Engine Layer
//
//  Benchmarks each engine sub-system in isolation so that any future change
//  in `src/engines/` that regresses throughput is caught by `cargo bench`.
//
//  Run:
//    cargo bench --bench bench_engines              # full statistical run
//    cargo bench --bench bench_engines -- --test    # compile + smoke-test
// ============================================================================

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use gladeshell_core::engines::intelligence::typo::TypoEngine;
use gladeshell_core::engines::terminal::{
    buffer::BufferEngine, parser::InputParser, renderer::Renderer,
};
use std::hint::black_box;

// ── BufferEngine ──────────────────────────────────────────────────────────────

/// Benchmark: insert 64 ASCII characters one-by-one and read the buffer.
fn bench_buffer_insert_ascii(c: &mut Criterion) {
    c.bench_function("buffer/insert_ascii_64", |b| {
        b.iter(|| {
            let mut buf = BufferEngine::new();
            for ch in "hello world this is a gladeshell benchmark run ok!!!!!!!!!!!!!!".chars() {
                buf.insert_char(black_box(ch));
            }
            black_box(buf.typed_buffer());
        });
    });
}

/// Benchmark: insert then backspace through 32 characters.
fn bench_buffer_backspace(c: &mut Criterion) {
    c.bench_function("buffer/backspace_32", |b| {
        b.iter(|| {
            let mut buf = BufferEngine::new();
            for ch in "gladeshell benchmark backspace!!".chars() {
                buf.insert_char(ch);
            }
            for _ in 0..32 {
                buf.backspace();
            }
            black_box(buf.typed_buffer());
        });
    });
}

/// Benchmark: cursor position and char_count accessors.
fn bench_buffer_accessors(c: &mut Criterion) {
    let mut buf = BufferEngine::new();
    for ch in "gladeshell".chars() {
        buf.insert_char(ch);
    }
    c.bench_function("buffer/accessors", |b| {
        b.iter(|| {
            black_box(buf.cursor_position());
            black_box(buf.char_count());
            black_box(buf.typed_buffer());
        });
    });
}

// ── InputParser ───────────────────────────────────────────────────────────────

/// Benchmark: parse a stream of printable ASCII bytes.
fn bench_parser_ascii(c: &mut Criterion) {
    let parser = InputParser::new();
    let input = b"hello world";
    c.bench_function("parser/ascii_11_bytes", |b| {
        b.iter(|| {
            black_box(parser.parse_bytes(black_box(input)));
        });
    });
}

/// Benchmark: parse a CSI escape sequence (arrow key).
fn bench_parser_escape_arrow(c: &mut Criterion) {
    let parser = InputParser::new();
    // ESC [ A  → Up arrow (CSI A)
    let input: &[u8] = &[0x1b, b'[', b'A'];
    c.bench_function("parser/csi_arrow_up", |b| {
        b.iter(|| {
            black_box(parser.parse_bytes(black_box(input)));
        });
    });
}

/// Benchmark: parse a 3-byte UTF-8 character (e.g. '৩').
fn bench_parser_utf8_3byte(c: &mut Criterion) {
    let parser = InputParser::new();
    // U+09E9 BENGALI DIGIT THREE → 0xE0 0xA7 0xA9
    let input: &[u8] = &[0xE0, 0xA7, 0xA9];
    c.bench_function("parser/utf8_3byte", |b| {
        b.iter(|| {
            black_box(parser.parse_bytes(black_box(input)));
        });
    });
}

/// Parametric: parser throughput at different input sizes.
fn bench_parser_throughput(c: &mut Criterion) {
    let parser = InputParser::new();
    let mut group = c.benchmark_group("parser/throughput");
    for size in [16usize, 64, 256, 1024] {
        let input: Vec<u8> = (b'a'..=b'z').cycle().take(size).collect();
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, inp| {
            b.iter(|| black_box(parser.parse_bytes(black_box(inp))));
        });
    }
    group.finish();
}

// ── Renderer ──────────────────────────────────────────────────────────────────

/// Benchmark: render an empty buffer (baseline overhead).
fn bench_renderer_empty(c: &mut Criterion) {
    let renderer = Renderer::new();
    let buf = BufferEngine::new();
    c.bench_function("renderer/empty_buffer", |b| {
        b.iter(|| {
            black_box(renderer.render_line_to_string(black_box(&buf), black_box("$ ")));
        });
    });
}

/// Benchmark: render a 40-char typed buffer with ghost suggestion.
fn bench_renderer_with_suggestion(c: &mut Criterion) {
    let renderer = Renderer::new();
    let mut buf = BufferEngine::new();
    for ch in "git commit -m 'feat: add engine bench'".chars() {
        buf.insert_char(ch);
    }
    buf.set_active_suggestion(Some(
        "git commit -m 'feat: add engine bench suite'".to_string(),
    ));

    c.bench_function("renderer/with_ghost_suggestion", |b| {
        b.iter(|| {
            black_box(renderer.render_line_to_string(black_box(&buf), black_box("gladeshell❯ ")));
        });
    });
}

// ── TypoEngine ────────────────────────────────────────────────────────────────

/// Benchmark: Damerau-Levenshtein distance for two short strings.
fn bench_typo_distance(c: &mut Criterion) {
    c.bench_function("typo/distance_short", |b| {
        b.iter(|| {
            black_box(TypoEngine::distance(black_box("gti"), black_box("git")));
        });
    });
}

/// Benchmark: suggest() — single best suggestion lookup.
fn bench_typo_suggest(c: &mut Criterion) {
    c.bench_function("typo/suggest_gti", |b| {
        b.iter(|| {
            black_box(TypoEngine::suggest(black_box("gti")));
        });
    });
}

/// Benchmark: find_suggestions() — full ranked list.
fn bench_typo_find_suggestions(c: &mut Criterion) {
    let mut group = c.benchmark_group("typo/find_suggestions");
    for input in ["gti", "carg", "mak", "rustfm"] {
        group.bench_with_input(BenchmarkId::from_parameter(input), input, |b, i| {
            b.iter(|| black_box(TypoEngine::find_suggestions(black_box(i))));
        });
    }
    group.finish();
}

// ── Registration ──────────────────────────────────────────────────────────────

criterion_group!(
    benches,
    // BufferEngine
    bench_buffer_insert_ascii,
    bench_buffer_backspace,
    bench_buffer_accessors,
    // InputParser
    bench_parser_ascii,
    bench_parser_escape_arrow,
    bench_parser_utf8_3byte,
    bench_parser_throughput,
    // Renderer
    bench_renderer_empty,
    bench_renderer_with_suggestion,
    // TypoEngine
    bench_typo_distance,
    bench_typo_suggest,
    bench_typo_find_suggestions,
);
criterion_main!(benches);
