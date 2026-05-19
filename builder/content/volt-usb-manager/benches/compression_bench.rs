use criterion::{black_box, criterion_group, criterion_main, Criterion};
use volt_usb_manager::compression::algorithm::CompressionAlgorithm;
use volt_usb_manager::compression::compressor::Compressor;

fn bench_zstd_compress(c: &mut Criterion) {
    let data = vec![0xABu8; 65536];

    c.bench_function("zstd_compress_64k", |b| {
        b.iter(|| {
            let blob = Compressor::compress(black_box(&data), CompressionAlgorithm::Zstd { level: 3 })
                .unwrap();
            black_box(blob);
        });
    });
}

fn bench_zstd_decompress(c: &mut Criterion) {
    let data = vec![0xABu8; 65536];
    let blob = Compressor::compress(&data, CompressionAlgorithm::Zstd { level: 3 }).unwrap();

    c.bench_function("zstd_decompress_64k", |b| {
        b.iter(|| {
            let result = Compressor::decompress(black_box(&blob)).unwrap();
            black_box(result);
        });
    });
}

fn bench_lz4_compress(c: &mut Criterion) {
    let data = vec![0xABu8; 65536];

    c.bench_function("lz4_compress_64k", |b| {
        b.iter(|| {
            let blob = Compressor::compress(black_box(&data), CompressionAlgorithm::Lz4).unwrap();
            black_box(blob);
        });
    });
}

fn bench_lz4_decompress(c: &mut Criterion) {
    let data = vec![0xABu8; 65536];
    let blob = Compressor::compress(&data, CompressionAlgorithm::Lz4).unwrap();

    c.bench_function("lz4_decompress_64k", |b| {
        b.iter(|| {
            let result = Compressor::decompress(black_box(&blob)).unwrap();
            black_box(result);
        });
    });
}

criterion_group!(benches, bench_zstd_compress, bench_zstd_decompress, bench_lz4_compress, bench_lz4_decompress);
criterion_main!(benches);
