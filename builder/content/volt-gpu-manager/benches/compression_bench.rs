use criterion::{criterion_group, criterion_main, Criterion, black_box};
use volt_gpu_manager::compression::{GpuCompressor, GpuCompressionAlgorithm};

fn bench_compression_null_backend(c: &mut Criterion) {
    let data = vec![0u8; 4096];

    c.bench_function("compress_none_4k", |b| {
        b.iter(|| {
            let result = GpuCompressor::compress(black_box(&data), GpuCompressionAlgorithm::None);
            black_box(result.unwrap());
        });
    });
}

fn bench_compression_cpu_fallback(c: &mut Criterion) {
    let data = vec![0u8; 65536];

    c.bench_function("compress_cpu_fallback_64k", |b| {
        b.iter(|| {
            let result = GpuCompressor::compress(black_box(&data), GpuCompressionAlgorithm::CpuFallbackZstd);
            black_box(result.unwrap());
        });
    });
}

criterion_group!(benches, bench_compression_null_backend, bench_compression_cpu_fallback);
criterion_main!(benches);
