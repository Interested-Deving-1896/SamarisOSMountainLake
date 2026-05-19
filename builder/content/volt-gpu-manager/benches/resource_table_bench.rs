use criterion::{criterion_group, criterion_main, Criterion, black_box};
use volt_gpu_manager::resources::{ResourceTable, GpuResourceMeta, GpuResourceId, GpuResourceType, GpuResourceUsage};
use volt_gpu_manager::vram::VramResidencyTier;

fn make_meta(id: GpuResourceId) -> GpuResourceMeta {
    GpuResourceMeta::new(id, 1, "bench", GpuResourceType::Buffer, GpuResourceUsage::Cache, 1024)
}

fn bench_resource_lookup(c: &mut Criterion) {
    let table = ResourceTable::new();
    let id = GpuResourceId::new();
    table.register(make_meta(id)).unwrap();

    c.bench_function("resource_table_lookup", |b| {
        b.iter(|| {
            let _ = black_box(table.get(id));
        });
    });
}

fn bench_resource_update(c: &mut Criterion) {
    let table = ResourceTable::new();
    let id = GpuResourceId::new();
    table.register(make_meta(id)).unwrap();

    c.bench_function("resource_table_update_tier", |b| {
        b.iter(|| {
            let _ = black_box(table.update_tier(id, VramResidencyTier::T2Compressed));
            let _ = black_box(table.update_tier(id, VramResidencyTier::T1Active));
        });
    });
}

criterion_group!(benches, bench_resource_lookup, bench_resource_update);
criterion_main!(benches);
