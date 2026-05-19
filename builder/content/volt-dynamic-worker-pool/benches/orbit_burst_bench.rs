use criterion::{black_box, criterion_group, criterion_main, Criterion};

use volt_dynamic_worker_pool::orbit::burst_controller::{OrbitBurstController, OrbitBurstRequest};
use volt_dynamic_worker_pool::orbit::cooldown::OrbitCooldown;

fn bench_burst_request_accepted(c: &mut Criterion) {
    let req = OrbitBurstRequest::new("job-1".into(), 2, 500, "bench".into());
    c.bench_function("burst/request_accepted", |b| {
        b.iter(|| {
            let controller = OrbitBurstController::new(1000, 0, 100);
            black_box(controller.request_burst(black_box(&req)))
        })
    });
}

fn bench_burst_request_cooldown(c: &mut Criterion) {
    let ctrl = OrbitBurstController::new(1000, 100_000, 100);
    let req = OrbitBurstRequest::new("job-1".into(), 1, 100, "bench".into());
    let _ = ctrl.request_burst(&req);
    c.bench_function("burst/request_cooldown", |b| {
        b.iter(|| black_box(ctrl.request_burst(black_box(&req))))
    });
}

fn bench_burst_request_limit_reached(c: &mut Criterion) {
    let ctrl = OrbitBurstController::new(0, 0, 100);
    let req = OrbitBurstRequest::new("job-1".into(), 1, 100, "bench".into());
    c.bench_function("burst/request_limit_reached", |b| {
        b.iter(|| black_box(ctrl.request_burst(black_box(&req))))
    });
}

fn bench_burst_release(c: &mut Criterion) {
    let ctrl = OrbitBurstController::new(1000, 0, 100);
    let req = OrbitBurstRequest::new("job-1".into(), 1, 100, "bench".into());
    let _ = ctrl.request_burst(&req);
    c.bench_function("burst/release", |b| {
        b.iter(|| {
            black_box(ctrl.release_burst());
        })
    });
}

fn bench_burst_is_in_cooldown(c: &mut Criterion) {
    let ctrl = OrbitBurstController::new(1000, 0, 100);
    c.bench_function("burst/is_in_cooldown", |b| {
        b.iter(|| black_box(ctrl.is_in_cooldown()))
    });
}

fn bench_burst_cooldown_remaining(c: &mut Criterion) {
    let ctrl = OrbitBurstController::new(1000, 0, 100);
    c.bench_function("burst/cooldown_remaining_ms", |b| {
        b.iter(|| black_box(ctrl.cooldown_remaining_ms()))
    });
}

fn bench_burst_new_request(c: &mut Criterion) {
    c.bench_function("burst/new_request", |b| {
        b.iter(|| {
            black_box(OrbitBurstRequest::new(
                black_box("bench-job".into()),
                black_box(2u8),
                black_box(500u64),
                black_box("bench".into()),
            ))
        })
    });
}

fn bench_burst_new_controller(c: &mut Criterion) {
    c.bench_function("burst/new_controller", |b| {
        b.iter(|| {
            black_box(OrbitBurstController::new(
                black_box(10),
                black_box(2000),
                black_box(5),
            ))
        })
    });
}

fn bench_cooldown_new(c: &mut Criterion) {
    c.bench_function("orbit_cooldown/new", |b| {
        b.iter(|| black_box(OrbitCooldown::new(black_box(1000), black_box(5000), black_box(3))))
    });
}

fn bench_cooldown_can_burst(c: &mut Criterion) {
    let cd = OrbitCooldown::new(1000, 5000, 3);
    c.bench_function("orbit_cooldown/can_burst", |b| {
        let _ = &cd;
        b.iter(|| black_box(cd.can_burst()))
    });
}

fn bench_cooldown_record_burst(c: &mut Criterion) {
    c.bench_function("orbit_cooldown/record_burst", |b| {
        b.iter(|| {
            let mut c = OrbitCooldown::new(1000, 5000, 3);
            black_box(c.record_burst());
        })
    });
}

fn bench_cooldown_consecutive_bursts(c: &mut Criterion) {
    let cd = OrbitCooldown::new(1000, 5000, 3);
    c.bench_function("orbit_cooldown/consecutive_bursts", |b| {
        let _ = &cd;
        b.iter(|| black_box(cd.consecutive_bursts()))
    });
}

criterion_group!(
    benches,
    bench_burst_request_accepted,
    bench_burst_request_cooldown,
    bench_burst_request_limit_reached,
    bench_burst_release,
    bench_burst_is_in_cooldown,
    bench_burst_cooldown_remaining,
    bench_burst_new_request,
    bench_burst_new_controller,
    bench_cooldown_new,
    bench_cooldown_can_burst,
    bench_cooldown_record_burst,
    bench_cooldown_consecutive_bursts,
);
criterion_main!(benches);
