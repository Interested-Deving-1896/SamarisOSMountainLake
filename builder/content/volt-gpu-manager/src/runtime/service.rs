use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::core::error::VgmError;
use crate::core::manager::VoltGpuManager;
use crate::core::result::VgmResult;
use crate::runtime::shutdown::ShutdownController;
use crate::runtime::signal::setup_signal_handler;

pub struct RuntimeService {
    manager: VoltGpuManager,
    shutdown: ShutdownController,
    tick_interval_ms: u64,
}

impl RuntimeService {
    pub fn new(manager: VoltGpuManager) -> Self {
        Self {
            manager,
            shutdown: ShutdownController::new(),
            tick_interval_ms: 16,
        }
    }

    pub fn with_tick_interval(manager: VoltGpuManager, tick_interval_ms: u64) -> Self {
        Self {
            manager,
            shutdown: ShutdownController::new(),
            tick_interval_ms,
        }
    }

    pub fn run(&mut self) -> VgmResult<()> {
        let shutdown_flag = self.shutdown.flag();
        setup_signal_handler(shutdown_flag.clone()).map_err(|e| {
            VgmError::IoError(format!("Signal handler setup failed: {}", e))
        })?;

        tracing::info!("RuntimeService started");

        self.manager.init()?;

        while !shutdown_flag.load(Ordering::SeqCst) {
            self.tick()?;
            std::thread::sleep(Duration::from_millis(self.tick_interval_ms));
        }

        self.manager.shutdown();
        tracing::info!("RuntimeService stopped");
        Ok(())
    }

    pub fn shutdown(&self) {
        self.shutdown.shutdown();
    }

    pub fn manager(&self) -> &VoltGpuManager {
        &self.manager
    }

    fn tick(&self) -> VgmResult<()> {
        let _snapshot = self.manager.snapshot();
        tracing::trace!(
            "Runtime tick — state: {:?}, uptime: {}ms",
            self.manager.state(),
            self.manager.uptime_ms()
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::VgmConfig;

    #[test]
    fn test_service_creation() {
        let config = VgmConfig::default();
        let manager = VoltGpuManager::new(config);
        let service = RuntimeService::new(manager);
        assert!(!service.shutdown.is_shutdown_requested());
    }
}
