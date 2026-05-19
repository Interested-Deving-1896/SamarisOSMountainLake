use criterion::{criterion_group, criterion_main, Criterion, black_box};
use volt_gpu_manager::scheduler::{GpuScheduler, GpuPriority, GpuCommand, GpuCommandKind};

fn make_cmd(priority: GpuPriority) -> GpuCommand {
    GpuCommand::new(GpuCommandKind::Compute, priority, "bench")
}

fn bench_enqueue_dequeue(c: &mut Criterion) {
    let sched = GpuScheduler::new(16);

    c.bench_function("scheduler_enqueue_1000", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                sched.submit(make_cmd(GpuPriority::Normal));
            }
            black_box(sched.queued_count());
        });
    });
}

fn bench_priority_ordering(c: &mut Criterion) {
    c.bench_function("scheduler_priority_ordering", |b| {
        b.iter(|| {
            let sched = GpuScheduler::new(16);
            sched.submit(make_cmd(GpuPriority::Idle));
            sched.submit(make_cmd(GpuPriority::Critical));
            sched.submit(make_cmd(GpuPriority::Normal));
            sched.submit(make_cmd(GpuPriority::High));
            let batch = sched.dequeue();
            black_box(batch);
        });
    });
}

criterion_group!(benches, bench_enqueue_dequeue, bench_priority_ordering);
criterion_main!(benches);
