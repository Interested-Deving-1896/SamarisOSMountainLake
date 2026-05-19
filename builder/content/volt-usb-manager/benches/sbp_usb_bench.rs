use criterion::{black_box, criterion_group, criterion_main, Criterion};
use volt_usb_manager::sbp_usb::message::SbpUsbMessage;
use volt_usb_manager::sbp_usb::opcode::SbpUsbOpcode;

fn bench_serialize(c: &mut Criterion) {
    let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbWrite, 42, vec![0xAB; 256]);

    c.bench_function("sbp_serialize", |b| {
        b.iter(|| {
            let bytes = black_box(&msg).to_bytes();
            black_box(bytes);
        });
    });
}

fn bench_deserialize(c: &mut Criterion) {
    let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbStatus, 7, vec![0x01; 128]);
    let bytes = msg.to_bytes();

    c.bench_function("sbp_deserialize", |b| {
        b.iter(|| {
            let decoded = SbpUsbMessage::from_bytes(black_box(&bytes)).unwrap();
            black_box(decoded);
        });
    });
}

fn bench_serialize_large_payload(c: &mut Criterion) {
    let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbWrite, 99, vec![0xFF; 4096]);

    c.bench_function("sbp_serialize_4k", |b| {
        b.iter(|| {
            let bytes = black_box(&msg).to_bytes();
            black_box(bytes);
        });
    });
}

fn bench_deserialize_large_payload(c: &mut Criterion) {
    let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbRead, 1, vec![0xAA; 4096]);
    let bytes = msg.to_bytes();

    c.bench_function("sbp_deserialize_4k", |b| {
        b.iter(|| {
            let decoded = SbpUsbMessage::from_bytes(black_box(&bytes)).unwrap();
            black_box(decoded);
        });
    });
}

fn bench_serialize_deserialize_roundtrip(c: &mut Criterion) {
    let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbFlush, 0, vec![]);

    c.bench_function("sbp_roundtrip", |b| {
        b.iter(|| {
            let bytes = black_box(&msg).to_bytes();
            let decoded = SbpUsbMessage::from_bytes(&bytes).unwrap();
            black_box(decoded);
        });
    });
}

criterion_group!(benches, bench_serialize, bench_deserialize, bench_serialize_large_payload, bench_deserialize_large_payload, bench_serialize_deserialize_roundtrip);
criterion_main!(benches);
