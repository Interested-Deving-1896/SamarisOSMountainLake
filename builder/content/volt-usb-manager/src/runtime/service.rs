use std::sync::Arc;
use std::time::Duration;

use parking_lot::RwLock;

use crate::config::schema::VumConfig;
use crate::core::manager::VoltUsbManager;
use crate::core::result::VumResult;
use crate::core::state::VumState;
use crate::runtime::shutdown::ShutdownController;
use crate::runtime::signal::setup_signal_handler;

pub struct RuntimeService {
    manager: Arc<RwLock<VoltUsbManager>>,
    shutdown: Arc<ShutdownController>,
}

impl RuntimeService {
    pub fn new(config: VumConfig) -> VumResult<Self> {
        let manager = VoltUsbManager::new(config);
        Ok(RuntimeService {
            manager: Arc::new(RwLock::new(manager)),
            shutdown: Arc::new(ShutdownController::new()),
        })
    }

    pub fn run(&self) -> VumResult<()> {
        tracing::info!("Starting Volt USB Manager service");

        setup_signal_handler(&self.shutdown);

        {
            let mut mgr = self.manager.write();
            mgr.init()?;
        }

        tracing::info!("Service initialized, entering main loop");

        while !self.shutdown.is_triggered() {
            std::thread::sleep(Duration::from_millis(250));

            #[cfg(feature = "writeback")]
            {
                let engine_arc = {
                    let mgr = self.manager.read();
                    mgr.engine.clone()
                };
                let wb_opt = engine_arc.read().write_buffer.clone();
                if let Some(ref wb) = wb_opt {
                    let batch = wb.write().flush_batch(64);
                    if !batch.is_empty() {
                        tracing::debug!("Flushed {} writes", batch.len());
                    }
                }
            }
        }

        {
            let mut mgr = self.manager.write();
            mgr.shutdown()?;
        }

        tracing::info!("Service stopped");
        Ok(())
    }

    pub fn shutdown(&self) {
        tracing::info!("Initiating service shutdown");
        self.shutdown.trigger();
    }

    pub fn manager(&self) -> Arc<RwLock<VoltUsbManager>> {
        Arc::clone(&self.manager)
    }

    pub fn is_running(&self) -> bool {
        let mgr = self.manager.read();
        mgr.state() != VumState::Shutdown
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::VumConfig;

    #[test]
    fn test_service_new() {
        let config = VumConfig::default();
        let service = RuntimeService::new(config);
        assert!(service.is_ok());
    }

    #[test]
    fn test_service_init_and_shutdown() {
        let config = VumConfig::default();
        let service = RuntimeService::new(config).unwrap();
        assert!(service.is_running());
        service.shutdown();
        assert!(service.shutdown.is_triggered());
    }
}
