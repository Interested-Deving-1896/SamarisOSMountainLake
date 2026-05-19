use std::sync::Arc;

use parking_lot::RwLock;

use crate::cache::ReadCache;
use crate::config::schema::VumConfig;
use crate::core::result::VumResult;
use crate::journal::{Journal, RecoveryEngine};
use crate::metrics::MetricsEngine;
use crate::ram::RamClient;
use crate::scheduler::IoScheduler;
use crate::writeback::WriteBuffer;

pub struct VumEngine {
    pub read_cache: Option<Arc<RwLock<ReadCache>>>,
    pub write_buffer: Option<Arc<RwLock<WriteBuffer>>>,
    pub journal: Option<Arc<RwLock<Journal>>>,
    pub recovery: Option<Arc<RwLock<RecoveryEngine>>>,
    pub scheduler: Option<Arc<RwLock<IoScheduler>>>,
    pub metrics: Arc<RwLock<MetricsEngine>>,
    pub ram_client: Option<Arc<RwLock<RamClient>>>,
}

impl VumEngine {
    pub fn new() -> Self {
        VumEngine {
            read_cache: None,
            write_buffer: None,
            journal: None,
            recovery: None,
            scheduler: None,
            metrics: Arc::new(RwLock::new(MetricsEngine::new())),
            ram_client: None,
        }
    }

    pub fn init(&mut self, config: &VumConfig) -> VumResult<()> {
        self.read_cache = Some(Arc::new(RwLock::new(ReadCache::new(
            config.cache.read_cache_max_mb,
        ))));

        #[cfg(feature = "writeback")]
        {
            self.write_buffer = Some(Arc::new(RwLock::new(WriteBuffer::new(
                config.writeback.buffer_max_mb,
            ))));
        }

        self.journal = Some(Arc::new(RwLock::new(Journal::new(&config.journal.path))));
        self.recovery = Some(Arc::new(RwLock::new(RecoveryEngine::new())));
        self.scheduler = Some(Arc::new(RwLock::new(IoScheduler::new(4, 64))));

        #[cfg(feature = "writeback")]
        {
            self.ram_client = Some(Arc::new(RwLock::new(RamClient::new())));
        }

        self.metrics.write().record_event("engine_init");
        Ok(())
    }
}

impl Default for VumEngine {
    fn default() -> Self {
        Self::new()
    }
}
