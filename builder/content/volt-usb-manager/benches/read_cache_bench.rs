use criterion::{black_box, criterion_group, criterion_main, Criterion};
use volt_usb_manager::cache::cache_entry::CacheEntry;
use volt_usb_manager::cache::cache_key::CacheKey;
use volt_usb_manager::cache::read_cache::ReadCache;

fn make_key(id: u64) -> CacheKey {
    CacheKey::new(&format!("/bench/{}", id), id, 4096)
}

fn bench_cache_hit(c: &mut Criterion) {
    let mut cache = ReadCache::new(256);
    let key = make_key(1);
    let data = vec![0xABu8; 4096];
    let entry = CacheEntry::new(key.clone(), "/bench/1", data);
    cache.insert(key.clone(), entry).unwrap();

    c.bench_function("cache_hit", |b| {
        b.iter(|| {
            let _ = black_box(cache.get(&key));
        });
    });
}

fn bench_cache_miss(c: &mut Criterion) {
    let mut cache = ReadCache::new(256);
    let key = make_key(999);

    c.bench_function("cache_miss", |b| {
        b.iter(|| {
            let _ = black_box(cache.get(&key));
        });
    });
}

fn bench_cache_decompression(c: &mut Criterion) {
    use volt_usb_manager::compression::algorithm::CompressionAlgorithm;
    let mut cache = ReadCache::new(256);
    let key = make_key(42);
    let data = vec![0xABu8; 5000];
    cache
        .insert_compressed(key.clone(), data, CompressionAlgorithm::Zstd { level: 3 })
        .unwrap();

    c.bench_function("cache_decompress", |b| {
        b.iter(|| {
            let _ = black_box(cache.get(&key));
        });
    });
}

criterion_group!(benches, bench_cache_hit, bench_cache_miss, bench_cache_decompression);
criterion_main!(benches);
