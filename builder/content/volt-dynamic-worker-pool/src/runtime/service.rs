use std::sync::Arc;
use std::time::Instant;

use crate::config::schema::WorkerPoolConfig;
use crate::core::pool::DynamicWorkerPool;
use crate::core::result::WorkerPoolResult;
use crate::metrics::snapshot::MetricsSnapshot;
use crate::runtime::signal::SignalHandler;

pub struct RuntimeService {
    pub started_at: Instant,
    pub config: WorkerPoolConfig,
    pub pool: Arc<DynamicWorkerPool>,
}

impl RuntimeService {
    pub fn new(config: WorkerPoolConfig, pool: Arc<DynamicWorkerPool>) -> Self {
        Self {
            started_at: Instant::now(),
            config,
            pool,
        }
    }

    pub fn start(&self) -> WorkerPoolResult<()> {
        self.pool.start()
    }

    pub fn shutdown(&self) -> WorkerPoolResult<()> {
        self.pool.shutdown()
    }

    pub fn uptime_ms(&self) -> u64 {
        self.started_at.elapsed().as_millis() as u64
    }

    pub fn metrics(&self) -> MetricsSnapshot {
        self.pool.metrics()
    }

    pub fn run(&self, signal: &SignalHandler) -> WorkerPoolResult<()> {
        self.start()?;
        signal.wait_for_shutdown();
        self.shutdown()
    }
}
