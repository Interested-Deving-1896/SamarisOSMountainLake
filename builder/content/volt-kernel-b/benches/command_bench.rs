use criterion::{criterion_group, criterion_main, Criterion};

use tesseract_engine::protocol::flatbuffer;
use tesseract_engine::protocol::command::CommandPayload;
use tesseract_engine::protocol::opcodes::Opcode;

fn bench_serialize_render_rect(c: &mut Criterion) {
    let payload = CommandPayload::RenderRect {
        x: 10, y: 20, w: 800, h: 600,
        border_radius: 8.0, shadow_blur: 4.0,
        shadow_offset_x: 2.0, shadow_offset_y: 2.0,
        fill_r: 255, fill_g: 0, fill_b: 0, fill_a: 255,
        border_r: 0, border_g: 0, border_b: 0, border_width: 1.0,
    };

    c.bench_function("serialize_render_rect", |b| {
        b.iter(|| {
            let _ = flatbuffer::command_to_payload(&payload, Opcode::GpuRender);
        });
    });
}

fn bench_serialize_compute_task(c: &mut Criterion) {
    let data = vec![0xAB; 4096];
    let payload = CommandPayload::ComputeTask {
        kind: tesseract_engine::protocol::command::ComputeKind::HashSha256,
        data,
    };

    c.bench_function("serialize_compute_task", |b| {
        b.iter(|| {
            let _ = flatbuffer::command_to_payload(&payload, Opcode::CpuExec);
        });
    });
}

fn bench_serialize_heartbeat(c: &mut Criterion) {
    let payload = CommandPayload::Heartbeat;

    c.bench_function("serialize_heartbeat", |b| {
        b.iter(|| {
            let _ = flatbuffer::command_to_payload(&payload, Opcode::Heartbeat);
        });
    });
}

fn bench_roundtrip_render_rect(c: &mut Criterion) {
    let payload = CommandPayload::RenderRect {
        x: 100, y: 200, w: 1920, h: 1080,
        border_radius: 12.0, shadow_blur: 8.0,
        shadow_offset_x: 3.0, shadow_offset_y: 3.0,
        fill_r: 64, fill_g: 128, fill_b: 192, fill_a: 255,
        border_r: 255, border_g: 255, border_b: 255, border_width: 2.0,
    };
    let opcode = Opcode::GpuRender;

    c.bench_function("flatbuffer_roundtrip_render_rect", |b| {
        b.iter(|| {
            let bytes = flatbuffer::command_to_payload(&payload, opcode);
            let _ = flatbuffer::payload_to_command(opcode, &bytes);
        });
    });
}

criterion_group!(
    benches,
    bench_serialize_render_rect,
    bench_serialize_compute_task,
    bench_serialize_heartbeat,
    bench_roundtrip_render_rect
);
criterion_main!(benches);
