pub mod boot;
pub mod compute_bridge;
pub mod core;
pub mod gpu_canvas;
pub mod ipc;
pub mod media;
pub mod protocol;
pub mod safety;
pub mod scheduler;
pub mod security;
pub mod system;
pub mod telemetry;

use std::sync::Arc;

use crate::core::boot::BootSequence;
use crate::core::config::TesseractConfig;
use crate::core::error::Result;
use crate::scheduler::Scheduler;
use crate::safety::SafetySupervisor;
use crate::security::SecurityManager;
use crate::system::SystemMonitor;
use crate::telemetry::Telemetry;

pub struct TesseractEngine {
    pub config: TesseractConfig,
    pub scheduler: Arc<Scheduler>,
    pub security: Arc<SecurityManager>,
    pub system_monitor: Arc<SystemMonitor>,
    pub safety: Arc<SafetySupervisor>,
    pub telemetry: Arc<Telemetry>,
    pub ipc: ipc::IpcServer,
}

impl TesseractEngine {
    pub fn init(config: &TesseractConfig) -> Result<Self> {
        let ctx = BootSequence::run(config)?;
        Ok(Self {
            config: ctx.config,
            scheduler: ctx.scheduler,
            security: ctx.security,
            system_monitor: ctx.system_monitor,
            safety: ctx.safety,
            telemetry: ctx.telemetry,
            ipc: ctx.ipc_server,
        })
    }

    pub fn shutdown(&self) {
        tracing::info!("Tesseract Engine shutting down...");
        self.ipc.shutdown();
        tracing::info!("Tesseract Engine stopped.");
    }

    pub fn load_config(path: &str) -> TesseractConfig {
        TesseractConfig::load_or_default(path)
    }
}
