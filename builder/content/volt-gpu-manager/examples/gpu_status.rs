use volt_gpu_manager::config::schema::VgmConfig;
use volt_gpu_manager::core::VoltGpuManager;

fn main() {
    let config = VgmConfig::default();
    let manager = VoltGpuManager::new(config);

    if let Err(e) = manager.init() {
        eprintln!("Failed to initialize GPU manager: {}", e);
        return;
    }

    let state = manager.state();
    let backend = manager.backend();
    let caps = manager.capabilities();
    let snap = manager.snapshot();

    println!("=== Volt GPU Manager Status ===");
    println!("State:        {:?}", state);
    println!("Backend:      {:?}", backend);
    println!("GPU Enabled:  {}", snap.gpu_enabled);
    println!("Device Count: {}", snap.device_count);
    println!("VRAM Total:   {} bytes", snap.vram_total_bytes);
    println!("VRAM Used:    {} bytes", snap.vram_used_bytes);
    println!("Thermal:      {}", snap.thermal_state);
    println!("Compute:      {}", caps.compute);
    println!("Compressed VRAM: {}", caps.compressed_vram);
    println!("Uptime:       {} ms", manager.uptime_ms());

    manager.shutdown();
}
