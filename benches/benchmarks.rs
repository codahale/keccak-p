use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use keccak_p::{keccak_f1600, keccak_p1600_10, keccak_p1600_12, keccak_p1600_14};

fn permutation_benchmarks(c: &mut Criterion) {
    let mut g = c.benchmark_group("permutation");
    g.sample_size(1_000);
    g.throughput(Throughput::Bytes(200));
    g.bench_function("Keccak-f1600", |b| {
        let mut lanes = [0u64; 25];
        b.iter(|| keccak_f1600(&mut lanes))
    });
    g.bench_function("keccak/Keccak-f1600", |b| {
        let mut state = [0u64; 25];
        b.iter(|| keccak::f1600(&mut state))
    });
    g.bench_function("tiny_keccak/Keccak-f1600", |b| {
        let mut state = [0u64; 25];
        b.iter(|| tiny_keccak::keccakf(&mut state))
    });
    g.bench_function("tiny_keccak/Keccak-p1600-12", |b| {
        let mut state = [0u64; 25];
        b.iter(|| tiny_keccak::keccakp(&mut state))
    });
    g.bench_function("Keccak-p1600-14", |b| {
        let mut lanes = [0u64; 25];
        b.iter(|| keccak_p1600_14(&mut lanes))
    });
    g.bench_function("Keccak-p1600-12", |b| {
        let mut lanes = [0u64; 25];
        b.iter(|| keccak_p1600_12(&mut lanes))
    });
    g.bench_function("Keccak-p1600-10", |b| {
        let mut lanes = [0u64; 25];
        b.iter(|| keccak_p1600_10(&mut lanes))
    });
    g.finish();
}

criterion_group!(benches, permutation_benchmarks);
criterion_main!(benches);
