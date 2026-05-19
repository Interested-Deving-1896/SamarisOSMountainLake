use criterion::{black_box, criterion_group, criterion_main, Criterion};

use volt_dynamic_worker_pool::job::job::Job;
use volt_dynamic_worker_pool::job::job_id::JobId;
use volt_dynamic_worker_pool::priority::level::PriorityLevel;
use volt_dynamic_worker_pool::priority::multi_queue::MultiQueue;

fn make_job(priority: PriorityLevel, n: u32) -> Job {
    Job::new(JobId::new(), format!("job-{n}"), priority, 1024)
}

fn bench_enqueue_low(c: &mut Criterion) {
    let mq = MultiQueue::new();
    let job = make_job(PriorityLevel::Low, 0);
    c.bench_function("enqueue/low", |b| {
        b.iter(|| mq.enqueue(black_box(job.clone())))
    });
}

fn bench_enqueue_normal(c: &mut Criterion) {
    let mq = MultiQueue::new();
    let job = make_job(PriorityLevel::Normal, 0);
    c.bench_function("enqueue/normal", |b| {
        b.iter(|| mq.enqueue(black_box(job.clone())))
    });
}

fn bench_enqueue_high(c: &mut Criterion) {
    let mq = MultiQueue::new();
    let job = make_job(PriorityLevel::High, 0);
    c.bench_function("enqueue/high", |b| {
        b.iter(|| mq.enqueue(black_box(job.clone())))
    });
}

fn bench_enqueue_critical(c: &mut Criterion) {
    let mq = MultiQueue::new();
    let job = make_job(PriorityLevel::Critical, 0);
    c.bench_function("enqueue/critical", |b| {
        b.iter(|| mq.enqueue(black_box(job.clone())))
    });
}

fn bench_enqueue_realtime(c: &mut Criterion) {
    let mq = MultiQueue::new();
    let job = make_job(PriorityLevel::Realtime, 0);
    c.bench_function("enqueue/realtime", |b| {
        b.iter(|| mq.enqueue(black_box(job.clone())))
    });
}

fn bench_dequeue_populated(c: &mut Criterion) {
    let mq = MultiQueue::new();
    for prio in &[
        PriorityLevel::Low,
        PriorityLevel::Normal,
        PriorityLevel::High,
        PriorityLevel::Critical,
        PriorityLevel::Realtime,
    ] {
        mq.enqueue(make_job(*prio, 0));
    }
    c.bench_function("dequeue/populated", |b| {
        b.iter(|| {
            let j = mq.dequeue();
            if let Some(ref j) = j {
                mq.enqueue(j.clone());
            }
            black_box(j)
        })
    });
}

criterion_group!(
    benches,
    bench_enqueue_low,
    bench_enqueue_normal,
    bench_enqueue_high,
    bench_enqueue_critical,
    bench_enqueue_realtime,
    bench_dequeue_populated,
);
criterion_main!(benches);
