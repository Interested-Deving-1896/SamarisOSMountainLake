use criterion::{criterion_group, criterion_main, Criterion};

use tesseract_engine::boot::{BootMode, BootSequence};

fn bench_boot_normal(c: &mut Criterion) {
    c.bench_function("boot_sequence_normal", |b| {
        b.iter(|| {
            let _ = BootSequence::new(BootMode::Normal).execute();
        });
    });
}

fn bench_boot_fast(c: &mut Criterion) {
    c.bench_function("boot_sequence_fast", |b| {
        b.iter(|| {
            let _ = BootSequence::new(BootMode::Fast).execute();
        });
    });
}

fn bench_boot_with_workers(c: &mut Criterion) {
    c.bench_function("boot_sequence_16_workers", |b| {
        b.iter(|| {
            let _ = BootSequence::new(BootMode::Fast).with_workers(16).execute();
        });
    });
}

criterion_group!(benches, bench_boot_normal, bench_boot_fast, bench_boot_with_workers);
criterion_main!(benches);
