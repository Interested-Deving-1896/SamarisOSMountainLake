use criterion::{criterion_group, criterion_main, Criterion};

use std::sync::Arc;
use std::time::Instant;

use tesseract_engine::protocol::header::SbpHeader;
use tesseract_engine::protocol::opcodes::Opcode;
use tesseract_engine::protocol::TesseractCommand;
use tesseract_engine::protocol::ring_buffer::RingBuffer;
use tesseract_engine::telemetry::Telemetry;
use tesseract_engine::scheduler::Scheduler;

fn bench_ring_buffer_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("ring_buffer");

    group.bench_function("push_1M", |b| {
        let buf = RingBuffer::new(1024 * 1024);
        b.iter(|| {
            for i in 0..10000 {
                let _ = buf.try_push(i);
            }
            for _ in 0..10000 {
                let _ = buf.try_pop();
            }
        });
    });

    group.finish();
}

fn bench_sbp_header_encode(c: &mut Criterion) {
    c.bench_function("header_encode", |b| {
        let header = SbpHeader::new(Opcode::GpuRender, 0, 0x12345678, 4096);
        b.iter(|| {
            let _encoded = header.encode();
        });
    });
}

fn bench_sbp_header_decode(c: &mut Criterion) {
    let header = SbpHeader::new(Opcode::GpuRender, 0, 0x12345678, 4096);
    let encoded = header.encode();

    c.bench_function("header_decode", |b| {
        b.iter(|| {
            let _decoded = SbpHeader::decode(&encoded);
        });
    });
}

fn bench_command_roundtrip(c: &mut Criterion) {
    let payload = vec![0u8; 1024];
    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::GpuRender, 2, 0x01, 1024),
        payload,
    );

    c.bench_function("command_serialize_deserialize", |b| {
        b.iter(|| {
            let bytes = cmd.to_bytes();
            let _decoded = TesseractCommand::from_bytes(&bytes);
        });
    });
}

fn bench_scheduler_submit(c: &mut Criterion) {
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(4, 1, telemetry));

    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 2, 0x01, 0),
        vec![],
    );

    c.bench_function("scheduler_submit_heartbeat", |b| {
        b.iter(|| {
            let _ = scheduler.submit(TesseractCommand::new(
                SbpHeader::new(Opcode::Heartbeat, 2, 0x01, 0),
                vec![],
            ));
        });
    });
}

criterion_group!(
    benches,
    bench_ring_buffer_throughput,
    bench_sbp_header_encode,
    bench_sbp_header_decode,
    bench_command_roundtrip,
    bench_scheduler_submit
);
criterion_main!(benches);
