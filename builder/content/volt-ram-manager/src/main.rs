use std::sync::Arc;
use std::time::Duration;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use volt_ram_manager::boot::BootSequence;
use volt_ram_manager::core::{VrmConfig, VrmResult};

#[derive(Parser, Debug)]
#[command(name = "volt-ram-manager", version = "1.0.0", about = "Samaris OS — Volt RAM Manager")]
struct Cli {
    #[arg(short, long, default_value = "/opt/volt/ram-manager/config.toml")]
    config: String,

    #[arg(long)]
    check_config: bool,

    #[arg(long)]
    status: bool,

    #[arg(long)]
    simulate_pressure: bool,

    #[arg(short, long)]
    debug: bool,
}

fn main() -> VrmResult<()> {
    let cli = Cli::parse();

    let filter = if cli.debug {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };
    tracing_subscriber::fmt().with_env_filter(filter).with_target(false).init();

    if cli.check_config {
        let _ = VrmConfig::load(&cli.config)?;
        tracing::info!("Config OK: {}", cli.config);
        return Ok(());
    }

    if cli.status {
        tracing::info!("Status check — connect to running daemon via SBP-MEM");
        return Ok(());
    }

    let config = VrmConfig::load_or_default(&cli.config);
    tracing::info!("Volt RAM Manager v{} starting", env!("CARGO_PKG_VERSION"));

    if config.manager.enable_compression {
        tracing::info!("Compression enabled (ZSTD + LZ4)");
    }
    if config.manager.enable_deduplication {
        tracing::info!("Deduplication enabled (SHA-256 verified)");
    }

    let manager = BootSequence::run(&config)?;
    let manager = Arc::new(manager);

    // ctrlc handler disabled — ctrlc crate not available without external dep
    // ctrlc::set_handler({
    //     let mgr = manager.clone();
    //     move || {
    //         tracing::info!("Shutdown signal received");
    //         mgr.shutdown();
    //         std::process::exit(0);
    //     }
    // }).expect("ctrlc handler");

    tracing::info!("Volt RAM Manager ready. Listening on SBP-MEM socket.");

    if cli.simulate_pressure {
        tracing::warn!("Pressure simulation requested — will cycle through levels");
        simulate_pressure_cycle(&manager);
    }

    loop {
        std::thread::sleep(Duration::from_secs(1));
        if manager.is_shutdown_requested() {
            break;
        }
    }

    manager.shutdown();
    Ok(())
}

fn simulate_pressure_cycle(_mgr: &volt_ram_manager::core::VoltRamManager) {
    tracing::info!("Pressure simulation placeholder");
}
