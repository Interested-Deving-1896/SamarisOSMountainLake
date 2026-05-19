use criterion::{criterion_group, criterion_main, Criterion, black_box};
use volt_gpu_manager::shaders::shader_cache::ShaderCache;
use volt_gpu_manager::shaders::shader_id::ShaderId;
use volt_gpu_manager::shaders::builtin::BuiltinShaderRegistry;

fn bench_shader_cache_lookup(c: &mut Criterion) {
    let cache = ShaderCache::new(64);
    let id = ShaderId::new();
    cache.insert(id, vec![0u8; 256]).unwrap();

    c.bench_function("shader_cache_lookup", |b| {
        b.iter(|| {
            let _ = black_box(cache.get(&id));
        });
    });
}

fn bench_shader_warmup(c: &mut Criterion) {
    let cache = ShaderCache::new(128);
    let builtins = BuiltinShaderRegistry::register_all();

    c.bench_function("shader_warmup", |b| {
        b.iter(|| {
            for (name, data) in &builtins {
                let sid = ShaderId::new();
                let _ = black_box(cache.insert(sid, data.clone()));
            }
        });
    });
}

criterion_group!(benches, bench_shader_cache_lookup, bench_shader_warmup);
criterion_main!(benches);
