use std::fs;

use volt_usb_manager::config::loader::load_config;
use volt_usb_manager::config::schema::VumConfig;
use volt_usb_manager::core::error::VumError;
use volt_usb_manager::core::manager::VoltUsbManager;
use volt_usb_manager::core::result::VumResult;
use volt_usb_manager::core::state::VumState;
use volt_usb_manager::journal::journal::JournalConfig;
use volt_usb_manager::journal::recovery::RecoveryEngine;
use volt_usb_manager::journal::Journal;
use volt_usb_manager::runtime::cli::Cli;
use volt_usb_manager::runtime::service::RuntimeService;
use volt_usb_manager::writeback::WriteBuffer;

fn setup_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
}

fn make_temp_dir() -> VumResult<(std::path::PathBuf, TempGuard)> {
    let base = std::env::temp_dir().join(format!("volt_usb_sim_{}", std::process::id()));
    fs::create_dir_all(&base).map_err(|e| VumError::Io(e))?;
    let guard = TempGuard(base.clone());
    Ok((base, guard))
}

struct TempGuard(std::path::PathBuf);

impl Drop for TempGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn cmd_check_config(path: &str) -> VumResult<()> {
    println!("Loading config from: {}", path);
    let config = load_config(path)?;
    println!("Config is valid:");
    println!("  mount_point: {}", config.manager.mount_point);
    println!("  backing_path: {}", config.manager.backing_path);
    println!("  cache_max_mb: {}", config.cache.read_cache_max_mb);
    println!("  buffer_max_mb: {}", config.writeback.buffer_max_mb);
    println!("  journal_path: {}", config.journal.path);
    Ok(())
}

fn cmd_status(config: &VumConfig) -> VumResult<()> {
    let mut mgr = VoltUsbManager::new(config.clone());
    mgr.init()?;
    let snapshot = mgr.snapshot();
    println!("Volt USB Manager Status");
    println!("  State: {:?}", mgr.state());
    println!("  Uptime: {}ms", snapshot.uptime_ms);
    println!("  Cache hits: {}", snapshot.cache_hit_count);
    println!("  Cache misses: {}", snapshot.cache_miss_count);
    println!("  Hit rate: {:.2}%", snapshot.cache_hit_ratio * 100.0);
    println!("  Pending writes: {}", snapshot.pending_write_count);
    println!("  Flush count: {}", snapshot.flush_count);
    println!("  Journal records: {}", snapshot.journal_records);
    Ok(())
}

fn cmd_mount(config: &VumConfig) -> VumResult<()> {
    println!("Mounting at: {}", config.manager.mount_point);
    let service = RuntimeService::new(config.clone())?;
    service.run()?;
    Ok(())
}

fn cmd_unmount(config: &VumConfig) -> VumResult<()> {
    println!("Unmounting...");
    let mut mgr = VoltUsbManager::new(config.clone());
    mgr.init()?;
    if mgr.state() == VumState::ConfigLoaded {
        mgr.shutdown()?;
        println!("Unmounted successfully");
    } else {
        return Err(VumError::InternalInvariantViolation(
            "Manager not in a mountable state".into(),
        ));
    }
    Ok(())
}

fn cmd_flush(config: &VumConfig) -> VumResult<()> {
    println!("Flushing write buffer...");
    let mut mgr = VoltUsbManager::new(config.clone());
    mgr.init()?;
    let eng = mgr.engine.read();
    if let Some(ref wb) = eng.write_buffer {
        let mut buf = wb.write();
        let flushed = buf.flush_batch(1024);
        println!("Flushed {} entries", flushed.len());
    } else {
        println!("Write buffer not initialized");
    }
    Ok(())
}

fn cmd_eject(config: &VumConfig) -> VumResult<()> {
    println!("Ejecting device...");
    let mut mgr = VoltUsbManager::new(config.clone());
    mgr.init()?;
    {
        let eng = mgr.engine.read();
        if let Some(ref wb) = eng.write_buffer {
            let mut buf = wb.write();
            let flushed = buf.flush_batch(1024);
            println!("Flushed {} entries before eject", flushed.len());
        }
    }
    mgr.shutdown()?;
    println!("Device ejected successfully");
    Ok(())
}

fn cmd_recover(config: &VumConfig) -> VumResult<()> {
    println!("Recovering journal at: {}", config.journal.path);
    let jconfig = JournalConfig {
        path: config.journal.path.clone(),
        fsync_on_record: false,
        checkpoint_interval_ms: config.journal.checkpoint_interval_ms,
    };
    let journal = Journal::open(jconfig)?;
    println!("Journal opened, dirty: {}", journal.is_dirty());
    if journal.is_dirty() {
        let recovery = RecoveryEngine::run(&journal.config().path, &config.manager.backing_path)?;
        println!("Replayed {} records, applied {} writes",
            recovery.records_replayed, recovery.writes_applied);
    } else {
        println!("Journal is clean, no recovery needed");
    }
    Ok(())
}

fn cmd_simulate_write() -> VumResult<()> {
    let (_guard, path_str) = {
        let (path, guard) = make_temp_dir()?;
        let s = path.to_str().unwrap().to_string();
        (guard, s)
    };
    println!("Simulating write operations in: {}", path_str);
    let mut buf = WriteBuffer::new(64);
    for i in 0..10 {
        buf.enqueue("/sim/file", i * 4096, vec![i as u8; 4096], 0, 0)?;
        println!("  Queued write {}", i + 1);
    }
    println!(
        "Pending count: {}, dirty bytes: {}",
        buf.pending_count(),
        buf.dirty_bytes()
    );
    let flushed = buf.flush_batch(1024);
    println!("Flushed {} entries", flushed.len());
    println!(
        "Pending after flush: {}, dirty bytes: {}",
        buf.pending_count(),
        buf.dirty_bytes()
    );
    println!("Write simulation complete");
    Ok(())
}

fn cmd_simulate_recovery() -> VumResult<()> {
    let (path, _guard) = make_temp_dir()?;
    let journal_path = path.join("sim_journal").to_str().unwrap().to_string();
    let backing_path = path.join("backing").to_str().unwrap().to_string();
    fs::create_dir_all(&backing_path).map_err(VumError::Io)?;
    println!("Simulating recovery in: {}", path.display());

    let jconfig = JournalConfig {
        path: journal_path.clone(),
        fsync_on_record: false,
        checkpoint_interval_ms: 5000,
    };
    let journal = Journal::open(jconfig)?;
    for i in 0..5 {
        let id = journal.begin_write("/sim_file", vec![i as u8; 128])?;
        journal.commit_write(id)?;
        println!("  Appended record {}", i + 1);
    }
    let recovery = RecoveryEngine::run(&journal_path, &backing_path)?;
    println!("Replayed {} records", recovery.records_replayed);
    println!("Recovery simulation complete");
    Ok(())
}

fn print_version() {
    println!("volt-usb-manager v{}", env!("CARGO_PKG_VERSION"));
    let features: Vec<&str> = vec![
        #[cfg(feature = "fuse")]
        "fuse",
        #[cfg(feature = "compression")]
        "compression",
        #[cfg(feature = "journal")]
        "journal",
        #[cfg(feature = "writeback")]
        "writeback",
    ];
    println!("Features: {}", features.join(", "));
}

fn main() {
    let cli = Cli::parse_args();

    if cli.version {
        print_version();
        return;
    }

    setup_logging();

    let config_path = cli
        .config
        .as_deref()
        .unwrap_or("/etc/volt/usb-manager.toml");

    let config = load_config(config_path).unwrap_or_else(|e| {
        tracing::warn!("Using default config: {}", e);
        VumConfig::default()
    });

    if let Some(ref path) = cli.check_config {
        let result = cmd_check_config(path);
        if let Err(e) = result {
            eprintln!("Config check failed: {}", e);
            std::process::exit(1);
        }
        return;
    }

    let result: VumResult<()> = if cli.status {
        cmd_status(&config)
    } else if cli.mount {
        cmd_mount(&config)
    } else if cli.unmount {
        cmd_unmount(&config)
    } else if cli.flush {
        cmd_flush(&config)
    } else if cli.eject {
        cmd_eject(&config)
    } else if cli.recover {
        cmd_recover(&config)
    } else if cli.simulate_write {
        cmd_simulate_write()
    } else if cli.simulate_recovery {
        cmd_simulate_recovery()
    } else {
        println!("No command specified. Use --help for usage.");
        Ok(())
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
