use criterion::{criterion_group, criterion_main, Criterion};

use tesseract_engine::compute_bridge::task::{ComputeKind, ComputeTask, execute_compute};

fn bench_hash_sha256(c: &mut Criterion) {
    let task = ComputeTask::new(0x01, ComputeKind::HashSha256, b"The quick brown fox jumps over the lazy dog".to_vec());
    c.bench_function("compute_hash_sha256_43b", |b| {
        b.iter(|| {
            let _ = execute_compute(&task);
        });
    });
}

fn bench_hash_sha256_1mb(c: &mut Criterion) {
    let data = vec![0xAB; 1024 * 1024];
    let task = ComputeTask::new(0x01, ComputeKind::HashSha256, data);
    c.bench_function("compute_hash_sha256_1mb", |b| {
        b.iter(|| {
            let _ = execute_compute(&task);
        });
    });
}

fn bench_rle_compress_highly_redundant(c: &mut Criterion) {
    let data = vec![0xFF; 10000];
    let task = ComputeTask::new(0x01, ComputeKind::Compress, data);
    c.bench_function("compute_rle_compress_redundant_10k", |b| {
        b.iter(|| {
            let _ = execute_compute(&task);
        });
    });
}

fn bench_rle_compress_random(c: &mut Criterion) {
    let data: Vec<u8> = (0..10000).map(|i| (i ^ (i >> 8)) as u8).collect();
    let task = ComputeTask::new(0x01, ComputeKind::Compress, data);
    c.bench_function("compute_rle_compress_random_10k", |b| {
        b.iter(|| {
            let _ = execute_compute(&task);
        });
    });
}

fn bench_rle_roundtrip(c: &mut Criterion) {
    let original = vec![0xAA; 5000];
    let compressed = execute_compute(&ComputeTask::new(0x01, ComputeKind::Compress, original.clone())).unwrap();
    c.bench_function("compute_rle_decompress_5k", |b| {
        let task = ComputeTask::new(0x01, ComputeKind::Decompress, compressed.clone());
        b.iter(|| {
            let _ = execute_compute(&task);
        });
    });
}

criterion_group!(
    benches,
    bench_hash_sha256,
    bench_hash_sha256_1mb,
    bench_rle_compress_highly_redundant,
    bench_rle_compress_random,
    bench_rle_roundtrip
);
criterion_main!(benches);
