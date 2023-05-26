use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use hash_benchmark_cw::{digest_cryptopro, digest_md5, digest_sha256};
use std::fs::read;

static SIZE_PROGRESSION: [usize; 8] = [64, 128, 256, 512, 1024, 2048, 4096, 8192];

fn bench_hashing_functions(c: &mut Criterion) {
    let lorem_ipsum = read("benches/loremipsum.txt").expect("Could not read file with test data");
    let mut group = c.benchmark_group("Hashing Functions");
    for size in SIZE_PROGRESSION {
        group.bench_with_input(BenchmarkId::new("MD5", size), &size, |b, i| {
            b.iter(|| digest_md5(&lorem_ipsum[..*i]))
        });

        group.bench_with_input(BenchmarkId::new("SHA-256", size), &size, |b, i| {
            b.iter(|| digest_sha256(&lorem_ipsum[..*i]))
        });

        group.bench_with_input(BenchmarkId::new("GOST94", size), &size, |b, i| {
            b.iter(|| digest_cryptopro(&lorem_ipsum[..*i]))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_hashing_functions);
criterion_main!(benches);
