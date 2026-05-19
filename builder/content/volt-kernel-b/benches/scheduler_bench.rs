use criterion::{criterion_group, criterion_main, Criterion};

use std::time::Instant;
use crossbeam::channel::unbounded;

use tesseract_engine::protocol::header::SbpHeader;
use tesseract_engine::protocol::opcodes::Opcode;
use tesseract_engine::protocol::TesseractCommand;
use tesseract_engine::scheduler::queue::PriorityQueues;
use tesseract_engine::scheduler::ScheduledTask;
use tesseract_engine::scheduler::priority::Priority;

fn make_task(priority: u8) -> ScheduledTask {
    let header = SbpHeader::new(Opcode::Heartbeat, priority, 0x01, 0);
    let cmd = TesseractCommand::new(header, vec![]);
    let (tx, _) = unbounded();
    ScheduledTask {
        command: cmd,
        received_at: Instant::now(),
        response_tx: tx,
    }
}

fn bench_enqueue_dequeue_full_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("scheduler_queue");

    group.bench_function("enqueue_dequeue_1000", |b| {
        b.iter(|| {
            let mut pq = PriorityQueues::new();
            for i in 0..1000 {
                pq.enqueue(make_task((i % 5) as u8));
            }
            let _tasks = pq.dequeue_cycle();
        });
    });

    group.bench_function("enqueue_dequeue_mixed_priority", |b| {
        b.iter(|| {
            let mut pq = PriorityQueues::new();
            for _ in 0..200 {
                pq.enqueue(make_task(2));
            }
            for _ in 0..50 {
                pq.enqueue(make_task(0));
            }
            for _ in 0..100 {
                pq.enqueue(make_task(1));
            }
            let _tasks = pq.dequeue_cycle();
        });
    });

    group.bench_function("enqueue_only_10000", |b| {
        b.iter(|| {
            let mut pq = PriorityQueues::new();
            for i in 0..10000 {
                pq.enqueue(make_task((i % 5) as u8));
            }
        });
    });

    group.finish();
}

fn bench_priority_conversion(c: &mut Criterion) {
    c.bench_function("priority_from_u8", |b| {
        b.iter(|| {
            for i in 0..5 {
                let _ = Priority::from_byte(i);
            }
        });
    });
}

criterion_group!(benches, bench_enqueue_dequeue_full_cycle, bench_priority_conversion);
criterion_main!(benches);
