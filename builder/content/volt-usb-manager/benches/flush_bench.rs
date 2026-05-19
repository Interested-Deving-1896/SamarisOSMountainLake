use std::fs::{self, File};
use std::io::Write;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tempfile::tempdir;
use volt_usb_manager::writeback::write_buffer::WriteBuffer;

fn bench_batch_flush_in_tempdir(c: &mut Criterion) {
    c.bench_function("batch_flush_in_tempdir", |b| {
        b.iter(|| {
            let dir = tempdir().unwrap();
            let mut buf = WriteBuffer::new(64);
            for i in 0..100 {
                let path = dir.path().join(format!("f{}", i));
                buf.enqueue(
                    black_box(path.to_str().unwrap()),
                    0,
                    vec![i as u8; 1024],
                    0,
                    0,
                )
                .unwrap();
            }
            let batch = buf.flush_batch(512);
            for pw in &batch {
                let p = std::path::Path::new(&pw.path);
                let _ = fs::write(p, &pw.data);
            }
            black_box(batch.len());
        });
    });
}

fn bench_fsync_cost(c: &mut Criterion) {
    c.bench_function("fsync_cost", |b| {
        b.iter(|| {
            let dir = tempdir().unwrap();
            let path = dir.path().join("fsync_test");
            let mut file = File::create(&path).unwrap();
            file.write_all(&[0xABu8; 4096]).unwrap();
            file.sync_all().unwrap();
            black_box(path);
        });
    });
}

fn bench_flush_large_buffer(c: &mut Criterion) {
    c.bench_function("flush_large_buffer", |b| {
        b.iter(|| {
            let mut buf = WriteBuffer::new(256);
            for i in 0..200 {
                buf.enqueue(
                    black_box(&format!("/big/{}", i)),
                    0,
                    vec![i as u8; 16384],
                    0,
                    0,
                )
                .unwrap();
            }
            let _ = black_box(buf.flush_batch(1024));
        });
    });
}

criterion_group!(benches, bench_batch_flush_in_tempdir, bench_fsync_cost, bench_flush_large_buffer);
criterion_main!(benches);
