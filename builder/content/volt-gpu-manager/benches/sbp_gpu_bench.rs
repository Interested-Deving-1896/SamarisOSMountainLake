use criterion::{criterion_group, criterion_main, Criterion, black_box};
use volt_gpu_manager::sbp_gpu::{SbpGpuMessage, SbpGpuOpcode};

fn bench_serialize(c: &mut Criterion) {
    let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuMetricsSnapshot, vec![0u8; 256]);

    c.bench_function("sbp_gpu_serialize_256b", |b| {
        b.iter(|| {
            let bytes = black_box(&msg).to_bytes();
            black_box(bytes);
        });
    });
}

fn bench_deserialize(c: &mut Criterion) {
    let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuBatchSubmit, vec![0u8; 256]);
    let bytes = msg.to_bytes();

    c.bench_function("sbp_gpu_deserialize_256b", |b| {
        b.iter(|| {
            let decoded = SbpGpuMessage::from_bytes(black_box(&bytes));
            black_box(decoded.unwrap());
        });
    });
}

criterion_group!(benches, bench_serialize, bench_deserialize);
criterion_main!(benches);
