use criterion::{criterion_group, criterion_main, Criterion, black_box};
use volt_gpu_manager::compression::{GpuCompressor, GpuCompressionAlgorithm};

fn bench_restore_path(c: &mut Criterion) {
    let original = vec![0xABu8; 16384];
    let (compressed, _) = GpuCompressor::compress(&original, GpuCompressionAlgorithm::CpuFallbackZstd).unwrap();

    c.bench_function("restore_decompress_16k", |b| {
        b.iter(|| {
            let result = GpuCompressor::decompress(black_box(&compressed), original.len() as u64, GpuCompressionAlgorithm::CpuFallbackZstd);
            black_box(result.unwrap());
        });
    });
}

fn bench_restore_none_path(c: &mut Criterion) {
    let data = vec![0u8; 16384];

    c.bench_function("restore_none_16k", |b| {
        b.iter(|| {
            let result = GpuCompressor::decompress(black_box(&data), data.len() as u64, GpuCompressionAlgorithm::None);
            black_box(result.unwrap());
        });
    });
}

criterion_group!(benches, bench_restore_path, bench_restore_none_path);
criterion_main!(benches);
