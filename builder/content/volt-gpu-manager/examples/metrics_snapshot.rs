use volt_gpu_manager::config::schema::VgmConfig;
use volt_gpu_manager::core::VoltGpuManager;
use volt_gpu_manager::metrics::GpuMetricsSnapshot;

fn main() {
    let config = VgmConfig::default();
    let manager = VoltGpuManager::new(config);

    if let Err(e) = manager.init() {
        eprintln!("Failed to init: {}", e);
        return;
    }

    let snap = manager.snapshot();
    let json = snap.into_json();

    println!("=== GPU Metrics Snapshot ===");
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    let custom = GpuMetricsSnapshot {
        gpu_enabled: true,
        backend: "vulkan".into(),
        device_count: 1,
        vram_total_bytes: 8 << 30,
        vram_used_bytes: 2 << 30,
        compression_count: 42,
        decompression_count: 38,
        compression_saved_bytes: 700_000_000,
        average_compression_ratio: 0.35,
        shader_cache_entries: 12,
        shader_cache_hit_count: 1200,
        shader_cache_miss_count: 50,
        frame_count: 6000,
        average_frame_time_ms: 8.2,
        thermal_state: "normal".into(),
        fallback_count: 0,
        ..Default::default()
    };

    println!();
    println!("=== Custom Metrics ===");
    println!("{}", serde_json::to_string_pretty(&custom.into_json()).unwrap());

    manager.shutdown();
}
