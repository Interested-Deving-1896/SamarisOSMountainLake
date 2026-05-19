use std::time::Duration;

use crate::core::pool::DynamicWorkerPool;
use crate::core::result::WorkerPoolResult;

pub struct GracefulShutdown {
    pub drain_timeout_ms: u64,
    pub force_timeout_ms: u64,
}

impl GracefulShutdown {
    pub fn new(drain_timeout_ms: u64, force_timeout_ms: u64) -> Self {
        Self {
            drain_timeout_ms,
            force_timeout_ms,
        }
    }

    pub fn shutdown(&self, pool: &DynamicWorkerPool) -> WorkerPoolResult<()> {
        pool.shutdown()?;

        std::thread::sleep(Duration::from_millis(self.drain_timeout_ms));

        if self.force_timeout_ms > 0 {
            std::thread::sleep(Duration::from_millis(self.force_timeout_ms));
        }

        Ok(())
    }
}
