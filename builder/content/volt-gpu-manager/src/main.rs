use std::path::Path;

use clap::Parser;

use volt_gpu_manager::config::loader::load_config_or_default;
use volt_gpu_manager::core::manager::VoltGpuManager;
use volt_gpu_manager::core::result::VgmResult;
use volt_gpu_manager::prelude::*;
use volt_gpu_manager::runtime::cli::Cli;
use volt_gpu_manager::runtime::service::RuntimeService;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();

    let result = run(cli);
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> VgmResult<()> {
    if cli.check_config {
        let config_path = cli.config.as_deref().map(Path::new);
        let config = load_config_or_default(config_path)?;
        config.validate()?;
        println!("Configuration is valid:");
        println!("  Backend:              {}", config.gpu.backend);
        println!("  Frame budget (ms):    {}", config.gpu.frame_budget_ms);
        println!("  VRAM max percent:     {}%", config.gpu.vram.max_vram_percent);
        println!("  Compression:          {} (level {})", config.gpu.vram.compression.algorithm, config.gpu.vram.compression.level);
        println!("  Dedup enabled:        {}", config.gpu.vram.deduplication.enabled);
        println!("  Multi-GPU mode:       {}", config.gpu.multi_gpu.mode);
        println!("  Thermal throttle:     {}°C", config.gpu.thermal.throttle_temp);
        println!("  Thermal emergency:    {}°C", config.gpu.thermal.emergency_temp);
        return Ok(());
    }

    let config_path = cli.config.as_deref().map(Path::new);
    let config = load_config_or_default(config_path)?;
    let manager = VoltGpuManager::new(config);

    if cli.probe {
        manager.init()?;
        println!("GPU Probe Results:");
        println!("  Backend: {:?}", manager.backend().unwrap_or(GpuBackendKind::Null));
        for hw in manager.hardware() {
            println!(
                "  Device: {} ({}) — {} MB VRAM",
                hw.model,
                hw.vendor,
                hw.vram_total_mb
            );
        }
        println!("  Capabilities: {:?}", manager.capabilities());
        manager.shutdown();
        return Ok(());
    }

    if cli.status {
        manager.init()?;
        println!("GPU Manager Status:");
        println!("  State:            {:?}", manager.state());
        println!("  Lifecycle:        {:?}", manager.lifecycle().phase());
        println!("  Uptime:           {} ms", manager.lifecycle().uptime_ms());
        println!("  Backend:          {:?}", manager.backend());
        println!("  Resources tracked: {}", manager.resource_count());
        manager.shutdown();
        return Ok(());
    }

    if cli.vram_status {
        manager.init()?;
        let caps = manager.capabilities();
        let config = manager.config();
        println!("VRAM Status:");
        println!("  T0 (device-local):    {} MB total, simulated", caps.max_buffer_size / (1024 * 1024));
        println!("  T1 (compressed):      {} MB pool", config.gpu.vram.t1_pool_size_mb);
        println!("  T2 (host-backed):     {} MB pool", config.gpu.vram.t2_pool_size_mb);
        println!(
            "  Compression:          {} (level {})",
            config.gpu.vram.compression.algorithm,
            config.gpu.vram.compression.level
        );
        println!("  Dedup enabled:        {}", config.gpu.vram.deduplication.enabled);
        println!("  Max VRAM percent:     {}%", config.gpu.vram.max_vram_percent);
        println!("  Scratch budget:       {} MB", config.gpu.vram.scratch_budget_mb);
        manager.shutdown();
        return Ok(());
    }

    if cli.thermal_status {
        manager.init()?;
        let config = manager.config();
        let thermal = manager.thermal_state();
        println!("Thermal Status:");
        println!("  Temperature:       {:.1} °C", thermal.temperature_c);
        println!("  Thermal state:     {}", thermal.level.name());
        println!("  Reduce compute:    {}", thermal.should_reduce_compute());
        println!("  Throttle temp:     {:.1} °C", config.gpu.thermal.throttle_temp);
        println!("  Emergency temp:    {:.1} °C", config.gpu.thermal.emergency_temp);
        println!("  Poll interval:     {} ms", config.gpu.thermal.poll_interval_ms);
        manager.shutdown();
        return Ok(());
    }

    if cli.warmup_shaders {
        manager.init()?;
        let cache = volt_gpu_manager::shaders::shader_cache::ShaderCache::new(64);
        println!("Warming up shader cache...");
        match volt_gpu_manager::shaders::warmup::ShaderWarmup::warmup_critical(&cache) {
            Ok(ids) => println!("✓ Warmed up {} shaders", ids.len()),
            Err(e) => eprintln!("Shader warmup failed: {}", e),
        }
        println!("Shader cache entries: {}", cache.entry_count());
        manager.shutdown();
        return Ok(());
    }

    if cli.simulate_frame {
        manager.init()?;
        use volt_gpu_manager::scheduler::command::{GpuCommand, GpuCommandKind};
        let scheduler = volt_gpu_manager::scheduler::GpuScheduler::new(16);

        println!("Simulating frame workload...");
        let mut cmd1 = GpuCommand::new(GpuCommandKind::Compute, GpuPriority::Idle, "frame1");
        cmd1.id = 1;
        scheduler.submit(cmd1);
        scheduler.submit(GpuCommand::new(GpuCommandKind::Transfer, GpuPriority::Normal, "frame2"));
        scheduler.submit(GpuCommand::new(GpuCommandKind::Render, GpuPriority::High, "frame3"));
        scheduler.submit(GpuCommand::new(GpuCommandKind::Barrier, GpuPriority::Critical, "frame4"));

        println!("  Submitted 4 jobs");

        let batch = scheduler.dequeue();
        if let Some(cmds) = batch {
            println!("  Dispatched {} jobs this frame", cmds.len());
            for cmd in &cmds {
                println!("    Job {} ({}) → completed", cmd.id, cmd.label);
            }
        }

        println!("  Frame complete — {} jobs queued", scheduler.queued_count());
        manager.shutdown();
        return Ok(());
    }

    if cli.simulate_compress_restore {
        manager.init()?;
        println!("T1 → T2 Compression / T2 → T1 Restore Simulation");
        println!("-------------------------------------------------");

        let data = b"This is GPU-resident data that will be compressed from T1 (compressed device-local) \
                       to T2 (host-backed) tier and then restored back to T1 via decompression. \
                       The Volt GPU Manager uses this mechanism to free up valuable T0/T1 VRAM \
                       by migrating cold data to cheaper host-backed storage.";

        println!("  Original size:  {} bytes (T1 resident)", data.len());

        use volt_gpu_manager::compression::compressor::GpuCompressor;
        let algorithm = GpuCompressionAlgorithm::Zstd;

        let (compressed, _) = GpuCompressor::compress(data, algorithm)?;
        println!("  Compressed size: {} bytes (T2 resident)", compressed.len());
        println!("  Compression ratio: {:.2}x", data.len() as f64 / compressed.len() as f64);

        let restored = GpuCompressor::decompress(&compressed, data.len() as u64, algorithm)?;
        println!("  Restored size:  {} bytes (back to T1)", restored.len());

        assert_eq!(data.to_vec(), restored, "Data integrity check failed!");
        println!("  ✓ Data integrity verified: exact match");

        println!("-------------------------------------------------");
        println!("Simulation complete.");
        manager.shutdown();
        return Ok(());
    }

    if cli.metrics {
        manager.init()?;
        let snapshot = manager.snapshot();
        println!("GPU Metrics Snapshot:");
        println!("  Timestamp:        {:?}", snapshot.timestamp);
        println!("  Enabled:          {}", snapshot.gpu_enabled);
        println!("  Backend:          {}", snapshot.backend);
        println!("  Devices:          {}", snapshot.device_count);
        println!("  VRAM Used:        {} bytes", snapshot.vram_used_bytes);
        println!("  VRAM Total:       {} bytes", snapshot.vram_total_bytes);
        println!("  Thermal:          {}", snapshot.thermal_state);
        println!("  Frames:           {}", snapshot.frame_count);
        println!("  Avg Frame Time:   {:.1} ms", snapshot.average_frame_time_ms);
        println!("  Compressions:     {}", snapshot.compression_count);
        println!("  Shader Cache:     {} entries", snapshot.shader_cache_entries);
        manager.shutdown();
        return Ok(());
    }

    let mut service = RuntimeService::new(manager);
    service.run()
}
