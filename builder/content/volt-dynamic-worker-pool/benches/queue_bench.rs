use criterion::{black_box, criterion_group, criterion_main, Criterion};

use volt_dynamic_worker_pool::job::job::Job;
use volt_dynamic_worker_pool::job::job_id::JobId;
use volt_dynamic_worker_pool::priority::level::PriorityLevel;
use volt_dynamic_worker_pool::priority::multi_queue::MultiQueue;

fn make_job(priority: PriorityLevel, n: u32) -> Job {
    Job::new(JobId::new(), format!("job-{n}"), priority, 1024)
}

fn bench_enqueue(c: &mut Criterion) {
    let mq = MultiQueue::new();
    let job = make_job(PriorityLevel::Normal, 0);
    c.bench_function("queue/enqueue", |b| {
        b.iter(|| black_box(mq.enqueue(black_box(job.clone()))))
    });
}

fn bench_dequeue(c: &mut Criterion) {
    let mq = MultiQueue::new();
    mq.enqueue(make_job(PriorityLevel::Normal, 0));
    c.bench_function("queue/dequeue", |b| {
        b.iter(|| black_box(mq.dequeue()))
    });
}

fn bench_dequeue_priority(c: &mut Criterion) {
    let mq = MultiQueue::new();
    mq.enqueue(make_job(PriorityLevel::Normal, 0));
    c.bench_function("queue/dequeue_for_priority", |b| {
        b.iter(|| black_box(mq.dequeue_for_priority(PriorityLevel::Normal)))
    });
}

fn bench_cancel(c: &mut Criterion) {
    let mq = MultiQueue::new();
    let job = make_job(PriorityLevel::Normal, 0);
    let id = job.id().clone();
    c.bench_function("queue/cancel", |b| {
        b.iter(|| {
            mq.enqueue(black_box(job.clone()));
            black_box(mq.cancel(black_box(&id)))
        })
    });
}

fn bench_queue_depth(c: &mut Criterion) {
    let mq = MultiQueue::new();
    for i in 0..100 {
        mq.enqueue(make_job(PriorityLevel::Normal, i));
    }
    c.bench_function("queue/queue_depth", |b| {
        b.iter(|| black_box(mq.queue_depth()))
    });
}

fn bench_queue_depth_by_priority(c: &mut Criterion) {
    let mq = MultiQueue::new();
    for i in 0..50 {
        mq.enqueue(make_job(PriorityLevel::High, i));
    }
    c.bench_function("queue/queue_depth_by_priority", |b| {
        b.iter(|| black_box(mq.queue_depth_by_priority(PriorityLevel::High)))
    });
}

fn bench_has_high_priority(c: &mut Criterion) {
    let mq = MultiQueue::new();
    mq.enqueue(make_job(PriorityLevel::Low, 0));
    mq.enqueue(make_job(PriorityLevel::Normal, 1));
    c.bench_function("queue/has_high_priority_jobs", |b| {
        b.iter(|| black_box(mq.has_high_priority_jobs()))
    });
}

criterion_group!(
    benches,
    bench_enqueue,
    bench_dequeue,
    bench_dequeue_priority,
    bench_cancel,
    bench_queue_depth,
    bench_queue_depth_by_priority,
    bench_has_high_priority,
);
criterion_main!(benches);
