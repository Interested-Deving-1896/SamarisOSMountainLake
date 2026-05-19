use criterion::{black_box, criterion_group, criterion_main, Criterion};

use volt_dynamic_worker_pool::scaling::policy::ScalingPolicy;
use volt_dynamic_worker_pool::scaling::thermal::ThermalState;

fn make_policy() -> ScalingPolicy {
    ScalingPolicy {
        min_workers: 2,
        max_workers: 16,
        scale_up_queue_factor: 2.0,
        scale_down_queue_factor: 0.5,
        scale_cooldown_ms: 5000,
        cpu_scale_up_threshold: 0.80,
        cpu_scale_down_threshold: 0.30,
    }
}

fn bench_scale_up_triggered(c: &mut Criterion) {
    let policy = make_policy();
    c.bench_function("should_scale_up/triggered", |b| {
        b.iter(|| {
            black_box(policy.should_scale_up(
                black_box(10),
                black_box(4),
                black_box(0.50),
                black_box(&ThermalState::Normal),
            ))
        })
    });
}

fn bench_scale_up_insufficient_queue(c: &mut Criterion) {
    let policy = make_policy();
    c.bench_function("should_scale_up/insufficient_queue", |b| {
        b.iter(|| {
            black_box(policy.should_scale_up(
                black_box(5),
                black_box(4),
                black_box(0.50),
                black_box(&ThermalState::Normal),
            ))
        })
    });
}

fn bench_scale_up_at_max(c: &mut Criterion) {
    let policy = make_policy();
    c.bench_function("should_scale_up/at_max", |b| {
        b.iter(|| {
            black_box(policy.should_scale_up(
                black_box(10),
                black_box(16),
                black_box(0.50),
                black_box(&ThermalState::Normal),
            ))
        })
    });
}

fn bench_scale_up_cpu_high(c: &mut Criterion) {
    let policy = make_policy();
    c.bench_function("should_scale_up/cpu_high", |b| {
        b.iter(|| {
            black_box(policy.should_scale_up(
                black_box(10),
                black_box(4),
                black_box(0.85),
                black_box(&ThermalState::Normal),
            ))
        })
    });
}

fn bench_scale_up_thermal_backoff(c: &mut Criterion) {
    let policy = make_policy();
    c.bench_function("should_scale_up/thermal_backoff", |b| {
        b.iter(|| {
            black_box(policy.should_scale_up(
                black_box(10),
                black_box(4),
                black_box(0.50),
                black_box(&ThermalState::Hot),
            ))
        })
    });
}

fn bench_scale_down_triggered(c: &mut Criterion) {
    let policy = make_policy();
    c.bench_function("should_scale_down/triggered", |b| {
        b.iter(|| {
            black_box(policy.should_scale_down(
                black_box(1),
                black_box(4),
                black_box(0.15),
                black_box(&ThermalState::Normal),
                black_box(false),
            ))
        })
    });
}

fn bench_scale_down_at_min(c: &mut Criterion) {
    let policy = make_policy();
    c.bench_function("should_scale_down/at_min", |b| {
        b.iter(|| {
            black_box(policy.should_scale_down(
                black_box(1),
                black_box(2),
                black_box(0.15),
                black_box(&ThermalState::Normal),
                black_box(false),
            ))
        })
    });
}

fn bench_scale_down_critical_backlog(c: &mut Criterion) {
    let policy = make_policy();
    c.bench_function("should_scale_down/critical_backlog", |b| {
        b.iter(|| {
            black_box(policy.should_scale_down(
                black_box(1),
                black_box(4),
                black_box(0.15),
                black_box(&ThermalState::Normal),
                black_box(true),
            ))
        })
    });
}

fn bench_scale_down_critical_thermal(c: &mut Criterion) {
    let policy = make_policy();
    c.bench_function("should_scale_down/critical_thermal", |b| {
        b.iter(|| {
            black_box(policy.should_scale_down(
                black_box(1),
                black_box(4),
                black_box(0.15),
                black_box(&ThermalState::Critical),
                black_box(false),
            ))
        })
    });
}

criterion_group!(
    benches,
    bench_scale_up_triggered,
    bench_scale_up_insufficient_queue,
    bench_scale_up_at_max,
    bench_scale_up_cpu_high,
    bench_scale_up_thermal_backoff,
    bench_scale_down_triggered,
    bench_scale_down_at_min,
    bench_scale_down_critical_backlog,
    bench_scale_down_critical_thermal,
);
criterion_main!(benches);
