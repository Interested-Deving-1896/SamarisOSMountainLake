use std::sync::Arc;
use std::time::Instant;

use crate::core::config::TesseractConfig;
use crate::core::error::Result;
use crate::ipc::IpcServer;
use crate::safety::SafetySupervisor;
use crate::scheduler::Scheduler;
use crate::security::SecurityManager;
use crate::system::SystemMonitor;
use crate::telemetry::Telemetry;

pub struct BootSequence;

impl BootSequence {
    pub fn run(config: &TesseractConfig) -> Result<BootContext> {
        let started = Instant::now();
        tracing::info!("Tesseract Engine booting...");

        let telemetry = Arc::new(Telemetry::new());
        let security = Arc::new(SecurityManager::new(config));

        let scheduler = Arc::new(Scheduler::new(
            config.max_workers,
            config.scheduler_tick_ms,
            telemetry.clone(),
        ));

        let system_monitor = Arc::new(SystemMonitor::new());
        let safety = Arc::new(SafetySupervisor::new());

        let ipc_server = IpcServer::start(
            config, scheduler.clone(), security.clone(),
            telemetry.clone(), system_monitor.clone(), safety.clone(),
        )?;

        let metrics_handle = system_monitor.clone();
        let metrics_interval = config.metrics_interval_ms;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(metrics_interval));
            loop {
                interval.tick().await;
                metrics_handle.collect_all().ok();
            }
        });

        let watchdog_handle = safety.clone();
        let watchdog_interval = config.watchdog_interval_ms;
        let system_clone = system_monitor.clone();
        let wd_throttle = config.thermal_throttle_celsius;
        let wd_emergency = config.thermal_emergency_celsius;
        let wd_critical = config.thermal_critical_celsius;
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(watchdog_interval));
            loop {
                interval.tick().await;
                if let Ok(snapshot) = system_clone.collect_all() {
                    if let Some(max_c) = snapshot.thermal_max {
                        if max_c > wd_critical {
                            tracing::error!("Thermal CRITICAL: {max_c}°C — emergency shutdown");
                            watchdog_handle.emergency_shutdown();
                        } else if max_c > wd_emergency {
                            tracing::warn!("Thermal EMERGENCY: {max_c}°C — releasing cores");
                            watchdog_handle.emergency_throttle();
                        } else if max_c > wd_throttle {
                            tracing::warn!("Thermal THROTTLE: {max_c}°C — reducing load");
                            watchdog_handle.thermal_throttle();
                        }
                    }
                }
            }
        });

        tracing::info!(
            "Tesseract Engine ready in {:?} — socket: {}",
            started.elapsed(),
            config.socket_path
        );

        let ctx = BootContext {
            config: config.clone(),
            scheduler,
            security,
            system_monitor,
            safety,
            telemetry,
            ipc_server,
            boot_instant: started,
        };

        Self::signal_ready(ctx.config.socket_path.as_str());

        Ok(ctx)
    }

    fn signal_ready(socket_path: &str) {
        tracing::info!("Kernel B ready on {socket_path}");
    }
}

pub struct BootContext {
    pub config: TesseractConfig,
    pub scheduler: Arc<Scheduler>,
    pub security: Arc<SecurityManager>,
    pub system_monitor: Arc<SystemMonitor>,
    pub safety: Arc<SafetySupervisor>,
    pub telemetry: Arc<Telemetry>,
    pub ipc_server: IpcServer,
    pub boot_instant: std::time::Instant,
}
