use criterion::{black_box, criterion_group, criterion_main, Criterion};

use volt_dynamic_worker_pool::worker::lifecycle::WorkerLifecycle;
use volt_dynamic_worker_pool::worker::worker::Worker;
use volt_dynamic_worker_pool::worker::worker_id::WorkerId;
use volt_dynamic_worker_pool::worker::WorkerState;

fn bench_worker_new(c: &mut Criterion) {
    c.bench_function("worker/new", |b| {
        b.iter(|| black_box(Worker::new(black_box(WorkerId::new(1)))))
    });
}

fn bench_worker_lifecycle_new(c: &mut Criterion) {
    c.bench_function("worker_lifecycle/new", |b| {
        b.iter(|| black_box(WorkerLifecycle::new(black_box(WorkerId::new(1)))))
    });
}

fn bench_worker_start_job(c: &mut Criterion) {
    let id = volt_dynamic_worker_pool::job::job_id::JobId::new();
    c.bench_function("worker/start_job", |b| {
        b.iter(|| {
            let mut w = Worker::new(WorkerId::new(1));
            black_box(w.start_job(black_box(id.clone())).ok())
        })
    });
}

fn bench_worker_finish_job(c: &mut Criterion) {
    let id = volt_dynamic_worker_pool::job::job_id::JobId::new();
    let mut worker = Worker::new(WorkerId::new(1));
    let _ = worker.start_job(id);
    c.bench_function("worker/finish_job", |b| {
        b.iter(|| {
            let mut w = Worker::new(WorkerId::new(2));
            let jid = volt_dynamic_worker_pool::job::job_id::JobId::new();
            let _ = w.start_job(jid);
            black_box(w.finish_job().ok())
        })
    });
}

fn bench_worker_retire_idle(c: &mut Criterion) {
    c.bench_function("worker/retire_idle", |b| {
        b.iter(|| {
            let mut worker = Worker::new(WorkerId::new(1));
            black_box(worker.retire().ok())
        })
    });
}

fn bench_lifecycle_transition(c: &mut Criterion) {
    c.bench_function("worker_lifecycle/transitions", |b| {
        b.iter(|| {
            let mut lc = WorkerLifecycle::new(WorkerId::new(1));
            let _ = lc.mark_busy();
            let _ = lc.transition(WorkerState::Idle);
            let _ = lc.stop();
            black_box(lc.state())
        })
    });
}

fn bench_worker_reset(c: &mut Criterion) {
    let mut worker = Worker::new(WorkerId::new(1));
    c.bench_function("worker/reset", |b| {
        b.iter(|| {
            black_box(worker.reset());
        })
    });
}

criterion_group!(
    benches,
    bench_worker_new,
    bench_worker_lifecycle_new,
    bench_worker_start_job,
    bench_worker_finish_job,
    bench_worker_retire_idle,
    bench_lifecycle_transition,
    bench_worker_reset,
);
criterion_main!(benches);
