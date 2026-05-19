use volt_dynamic_worker_pool::*;

#[test]
fn test_metrics_counters_new() {
    let counters = metrics::counters::MetricsCounters::new();
    let v = counters.total_jobs_submitted.load(std::sync::atomic::Ordering::Relaxed);
    assert_eq!(v, 0);
}

#[test]
fn test_metrics_counters_record_submission() {
    let c = metrics::counters::MetricsCounters::new();
    c.record_submission();
    let v = c.total_jobs_submitted.load(std::sync::atomic::Ordering::Relaxed);
    assert_eq!(v, 1);
}

#[test]
fn test_metrics_counters_record_completion() {
    let c = metrics::counters::MetricsCounters::new();
    c.record_completion(1_000_000);
    let v = c.total_jobs_completed.load(std::sync::atomic::Ordering::Relaxed);
    assert_eq!(v, 1);
}

#[test]
fn test_metrics_counters_record_failure() {
    let c = metrics::counters::MetricsCounters::new();
    c.record_failure();
    let v = c.total_jobs_failed.load(std::sync::atomic::Ordering::Relaxed);
    assert_eq!(v, 1);
}

#[test]
fn test_metrics_snapshot_default() {
    use volt_dynamic_worker_pool::metrics::snapshot::MetricsSnapshot;
    let snap = MetricsSnapshot::new();
    assert_eq!(snap.total_jobs_submitted, 0);
    assert_eq!(snap.active_workers, 0);
    assert_eq!(snap.worker_pool_state, "uninitialized");
}

#[test]
fn test_metrics_snapshot_from_counters() {
    let c = metrics::counters::MetricsCounters::new();
    c.record_submission();
    c.record_completion(500_000);
    let snap = c.snapshot(4, 2, 10, 3, 1000, "running".into());
    assert_eq!(snap.total_jobs_submitted, 1);
    assert_eq!(snap.total_jobs_completed, 1);
    assert_eq!(snap.active_workers, 4);
}
