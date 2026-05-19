#[test]
fn test_result_schema_valid() {
    let schema = serde_json::json!({
        "version": "1.0",
        "timestamp": "2026-05-17T10:00:00Z",
        "run": {
            "run_id": "test-run",
            "mode": "quick",
            "duration_seconds": 15.0,
            "iterations": 3,
            "warmup_iterations": 1,
            "median_score": 8000.0,
            "mean_score": 7950.0,
            "min_score": 7900.0,
            "max_score": 8100.0,
            "stddev_score": 100.0,
            "confidence": "high",
            "reliability_flags": []
        },
        "hardware": {
            "class": "unknown",
            "cpu": "Test CPU",
            "cpu_cores": 1,
            "ram_gb": 1.0,
            "gpu": "Test GPU",
            "storage_type": "ssd",
            "arch": "x86_64"
        },
        "environment": {
            "display_resolution": "1920x1080",
            "display_scale": 1.0,
            "network_connected": false,
            "running_from_usb": false,
            "running_in_vm": false,
            "power_mode": "ac",
            "kernel_version": "test",
            "system_uptime_seconds": 100.0
        },
        "os": { "name": "Samaris OS", "version": "test", "build": "", "commit_hash": null, "release_channel": "alpha" },
        "overall": { "score": 8000, "score_out_of": 10000, "normalized_score": 80.0, "badge": "Very Good", "validity": "same_hardware_only" },
        "category_scores": { "system": 80, "ui": 80, "memory": 80, "kernel": 80, "graphics": 80, "ai": 80, "browser": 80, "filesystem": 80, "stability": 80 },
        "metrics": {},
        "comparison": { "baselines": [], "comparison_validity": "none" },
        "optimizer": { "fitness_score": 8000, "bottlenecks": [], "recommendations": [] }
    });

    assert!(schema.is_object());
    assert_eq!(schema["version"], "1.0");
    assert_eq!(schema["overall"]["score"], 8000);
    assert_eq!(schema["overall"]["score_out_of"], 10000);
}

#[test]
fn test_history_schema() {
    let history = serde_json::json!({
        "entries": [
            { "timestamp": "2026-01-01T00:00:00Z", "run_id": "test-1", "mode": "quick", "score": 8000, "badge": "Very Good", "duration_seconds": 15.0, "hardware_class": "unknown", "category_scores": { "system": 80, "ui": 80, "memory": 80, "kernel": 80, "graphics": 80, "ai": 80, "browser": 80, "filesystem": 80, "stability": 80 }, "reliability_flags": [], "result_path": "/tmp/test.json" }
        ],
        "max_entries": 100
    });
    assert_eq!(history["entries"].as_array().unwrap().len(), 1);
}

#[test]
fn test_baseline_schema() {
    let baseline = serde_json::json!({
        "source": "imported",
        "import_label": "Test OS",
        "imported_at": "2026-01-01T00:00:00Z",
        "measurement_date": "2026-01-01T00:00:00Z",
        "hardware": { "class": "unknown", "cpu": "Test", "cpu_cores": 1, "ram_gb": 1.0, "gpu": "Test", "storage_type": "ssd", "arch": "x86_64" },
        "os": { "name": "Test OS", "version": "1.0" },
        "overall": { "score": 7000, "badge": "Good" },
        "category_scores": { "system": 70, "ui": 70, "memory": 70, "kernel": 70, "graphics": 70, "ai": 70, "browser": 70, "filesystem": 70, "stability": 70 }
    });
    assert_eq!(baseline["source"], "imported");
    assert_eq!(baseline["overall"]["score"], 7000);
}
