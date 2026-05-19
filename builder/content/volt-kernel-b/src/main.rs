use std::sync::Arc;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use tesseract_engine::boot::BootSequence;
use tesseract_engine::boot::BootMode;
use tesseract_engine::core::config::TesseractConfig;
use tesseract_engine::TesseractEngine;

#[derive(Parser, Debug)]
#[command(name = "tesseract-engine", version = "1.0.0-alpha", about = "Samaris OS Kernel B — Native Rust Acceleration Daemon")]
struct Cli {
    #[arg(long)]
    boot_mode: bool,

    #[arg(short, long, default_value = "/opt/volt/kernel-b/config.toml")]
    config: String,

    #[arg(short, long)]
    debug: bool,

    #[arg(short = 's', long)]
    socket: Option<String>,

    #[arg(short = 'w', long, default_value_t = 4)]
    workers: usize,
}

fn main() {
    let cli = Cli::parse();

    let filter = if cli.debug {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    rt.block_on(async {
        if cli.boot_mode {
            let boot_seq = BootSequence::new(BootMode::Fast)
                .with_workers(cli.workers.max(8));

            match boot_seq.execute() {
                Ok(result) => {
                    tracing::info!(
                        "VOLT BOOT completed in {:?} — {} workers, GPU={}, SHM={}, assets={}",
                        result.elapsed,
                        cli.workers.max(8),
                        result.gpu_canvas.is_some(),
                        result.shm_ring.is_some(),
                        result.asset_cache.as_ref().map(|a| a.len()).unwrap_or(0),
                    );
                }
                Err(e) => {
                    tracing::error!("VOLT BOOT failed: {e} — continuing with normal init");
                }
            }
        }

        let mut config = TesseractConfig::load_or_default(&cli.config);
        if cli.debug {
            config.debug_mode = true;
        }
        if let Some(socket) = cli.socket {
            config.socket_path = socket;
        }
        config.max_workers = cli.workers.max(if cli.boot_mode { 8 } else { 1 });

        tracing::info!("Tesseract Engine v{} starting...", env!("CARGO_PKG_VERSION"));
        tracing::info!("Config: {}", cli.config);
        tracing::info!("Socket: {}", config.socket_path);
        tracing::info!("Workers: {}", config.max_workers);
        tracing::info!("Debug: {}", config.debug_mode);

        let engine = match TesseractEngine::init(&config) {
            Ok(engine) => engine,
            Err(e) => {
                tracing::error!("Failed to start Tesseract Engine: {e}");
                std::process::exit(1);
            }
        };

        let engine = Arc::new(engine);
        let engine_for_shutdown = engine.clone();

        ctrlc::set_handler(move || {
            tracing::info!("Received shutdown signal");
            engine_for_shutdown.shutdown();
            std::process::exit(0);
        })
        .expect("Error setting Ctrl-C handler");

        tracing::info!("Tesseract Engine ready. Waiting for commands...");

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            if engine.safety.is_emergency_stop() {
                tracing::error!("Emergency stop triggered — shutting down");
                break;
            }
        }

        engine.shutdown();
    });
}
