use criterion::{black_box, criterion_group, criterion_main, Criterion};

use volt_dynamic_worker_pool::preemption::yield_point::YieldPoint;

fn bench_yield_point_new(c: &mut Criterion) {
    c.bench_function("yield_point/new", |b| {
        b.iter(|| black_box(YieldPoint::new(black_box(1000))))
    });
}

fn bench_yield_point_should_yield(c: &mut Criterion) {
    let yp = YieldPoint::new(1000);
    c.bench_function("yield_point/should_yield", |b| {
        b.iter(|| black_box(yp.should_yield()))
    });
}

fn bench_yield_point_should_yield_exhausted(c: &mut Criterion) {
    let mut yp = YieldPoint::new(100);
    yp.consumed = 100;
    c.bench_function("yield_point/should_yield_exhausted", |b| {
        b.iter(|| black_box(yp.should_yield()))
    });
}

fn bench_yield_point_record_yield(c: &mut Criterion) {
    c.bench_function("yield_point/record_yield", |b| {
        b.iter(|| {
            let mut yp = YieldPoint::new(10_000);
            black_box(yp.record_yield());
        })
    });
}

fn bench_yield_point_budget_remaining(c: &mut Criterion) {
    let yp = YieldPoint::new(1000);
    c.bench_function("yield_point/budget_remaining", |b| {
        b.iter(|| black_box(yp.budget_remaining()))
    });
}

fn bench_yield_point_reset(c: &mut Criterion) {
    let mut yp = YieldPoint::new(1000);
    c.bench_function("yield_point/reset", |b| {
        b.iter(|| {
            black_box(yp.reset());
        })
    });
}

fn bench_yield_point_full_cycle(c: &mut Criterion) {
    c.bench_function("yield_point/full_cycle", |b| {
        b.iter(|| {
            let mut yp = YieldPoint::new(1000);
            black_box(yp.should_yield());
            black_box(yp.record_yield());
            black_box(yp.budget_remaining());
            black_box(yp.reset());
        })
    });
}

criterion_group!(
    benches,
    bench_yield_point_new,
    bench_yield_point_should_yield,
    bench_yield_point_should_yield_exhausted,
    bench_yield_point_record_yield,
    bench_yield_point_budget_remaining,
    bench_yield_point_reset,
    bench_yield_point_full_cycle,
);
criterion_main!(benches);
