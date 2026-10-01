// =============================================================================
//  benches/bench_secret_gen.rs — Criterion benchmark for the CSPRNG generator
//
//  Measures:
//    • generate(n)  throughput at various sizes (16 / 32 / 64 / 256 bytes)
//    • Baseline: OS /dev/urandom read speed
//
//  Run:  cargo bench --bench bench_secret_gen
//  HTML: target/criterion/SecretGen/report/index.html
// =============================================================================

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use gladeshell_core::core::secret_gen::generate;

// ── Benchmarks ────────────────────────────────────────────────────────────────

fn bench_generate_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("secret_gen/generate");

    for size in [16usize, 32, 64, 128, 256, 512] {
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{size}B")),
            &size,
            |b, &size| {
                b.iter(|| {
                    let _ = generate(black_box(size)).unwrap();
                });
            },
        );
    }

    group.finish();
}

fn bench_generate_hex_encode(c: &mut Criterion) {
    // Measure: generate + hex-encode (common usage pattern)
    c.bench_function("secret_gen/generate_32B_then_hex", |b| {
        b.iter(|| {
            let bytes = generate(black_box(32)).unwrap();
            let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
            black_box(hex);
        });
    });
}

criterion_group!(benches, bench_generate_sizes, bench_generate_hex_encode);
criterion_main!(benches);
