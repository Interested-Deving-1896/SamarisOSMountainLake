use criterion::{black_box, criterion_group, criterion_main, Criterion};
use volt_usb_manager::writeback::write_buffer::WriteBuffer;

fn bench_enqueue_small_writes(c: &mut Criterion) {
    c.bench_function("enqueue_small_writes", |b| {
        b.iter(|| {
            let mut buf = WriteBuffer::new(64);
            for i in 0..100 {
                buf.enqueue(
                    black_box(&format!("/small/{}", i)),
                    0,
                    vec![i as u8; 256],
                    0,
                    0,
                )
                .unwrap();
            }
        });
    });
}

fn bench_enqueue_large_writes(c: &mut Criterion) {
    c.bench_function("enqueue_large_writes", |b| {
        b.iter(|| {
            let mut buf = WriteBuffer::new(64);
            for i in 0..20 {
                buf.enqueue(
                    black_box(&format!("/large/{}", i)),
                    0,
                    vec![i as u8; 65536],
                    0,
                    0,
                )
                .unwrap();
            }
        });
    });
}

fn bench_flush_batch(c: &mut Criterion) {
    c.bench_function("flush_batch", |b| {
        b.iter(|| {
            let mut buf = WriteBuffer::new(64);
            for i in 0..50 {
                buf.enqueue(
                    black_box(&format!("/flush/{}", i)),
                    0,
                    vec![i as u8; 4096],
                    0,
                    0,
                )
                .unwrap();
            }
            let _ = black_box(buf.flush_batch(512));
        });
    });
}

criterion_group!(benches, bench_enqueue_small_writes, bench_enqueue_large_writes, bench_flush_batch);
criterion_main!(benches);
