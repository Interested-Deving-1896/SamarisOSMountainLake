use volt_gpu_manager::metrics::GpuMetricsSnapshot;

#[allow(dead_code)]
fn snapshot_with_defaults() -> GpuMetricsSnapshot {
    GpuMetricsSnapshot::default()
}

#[test]
fn compression_metrics_increment() {
    let snap = GpuMetricsSnapshot {
        compression_count: 10,
        compression_saved_bytes: 5000,
        average_compression_ratio: 0.35,
        ..Default::default()
    };
    assert_eq!(snap.compression_count, 10);
    assert_eq!(snap.compression_saved_bytes, 5000);
    assert!((snap.average_compression_ratio - 0.35).abs() < 0.001);
}

#[test]
fn restore_metrics_increment() {
    let snap = GpuMetricsSnapshot {
        restore_count: 5,
        decompression_count: 5,
        ..Default::default()
    };
    assert_eq!(snap.restore_count, 5);
    assert_eq!(snap.decompression_count, 5);
}

#[test]
fn shader_cache_metrics_increment() {
    let snap = GpuMetricsSnapshot {
        shader_cache_entries: 12,
        shader_cache_hit_count: 100,
        shader_cache_miss_count: 3,
        shader_compile_count: 15,
        shader_compile_error_count: 1,
        ..Default::default()
    };
    assert_eq!(snap.shader_cache_entries, 12);
    assert_eq!(snap.shader_cache_hit_count, 100);
    assert_eq!(snap.shader_cache_miss_count, 3);
    assert_eq!(snap.shader_compile_error_count, 1);
}

#[test]
fn frame_metrics_increment() {
    let snap = GpuMetricsSnapshot {
        frame_count: 1000,
        average_frame_time_ms: 8.3,
        frame_budget_miss_count: 50,
        ..Default::default()
    };
    assert_eq!(snap.frame_count, 1000);
    assert!((snap.average_frame_time_ms - 8.3).abs() < 0.001);
    assert_eq!(snap.frame_budget_miss_count, 50);
}

#[test]
fn fallback_metrics_increment() {
    let snap = GpuMetricsSnapshot {
        fallback_count: 7,
        ..Default::default()
    };
    assert_eq!(snap.fallback_count, 7);
}
