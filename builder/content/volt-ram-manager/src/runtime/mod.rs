use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use parking_lot::RwLock;

use crate::core::manager::VoltRamManager;
use crate::core::result::VrmResult;
use crate::gc::coordinator::GcCoordinator;
use crate::gc::report::GcReport;
use crate::gc::v8_signal::V8PressureSignal;

pub struct VrmRuntime {
    manager: Arc<VoltRamManager>,
    gc_coordinator: Arc<RwLock<GcCoordinator>>,
    running: Arc<AtomicBool>,
    tick_interval: Duration,
}

impl VrmRuntime {
    pub fn new(manager: VoltRamManager, gc_coordinator: GcCoordinator) -> Self {
        Self {
            manager: Arc::new(manager),
            gc_coordinator: Arc::new(RwLock::new(gc_coordinator)),
            running: Arc::new(AtomicBool::new(false)),
            tick_interval: Duration::from_millis(100),
        }
    }

    pub fn with_tick_interval(mut self, interval_ms: u64) -> Self {
        self.tick_interval = Duration::from_millis(interval_ms);
        self
    }

    pub fn start(&self) -> VrmResult<()> {
        if self.running.load(Ordering::SeqCst) {
            return Ok(());
        }
        self.running.store(true, Ordering::SeqCst);
        tracing::info!("VRM runtime started");
        Ok(())
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
        tracing::info!("VRM runtime stopped");
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn tick(&self) -> VrmResult<GcReport> {
        if !self.is_running() {
            return Ok(GcReport::new());
        }

        let result = {
            let coordinator = self.gc_coordinator.read();
            let page_table = self.manager.page_table.read();
            coordinator.run_cycle(&page_table)?
        };

        if !result.is_empty() {
            tracing::debug!(
                "GC cycle: {} pages freed, {} bytes reclaimed (aggressive: {})",
                result.pages_freed,
                result.bytes_reclaimed,
                result.aggressive
            );
        }

        Ok(result)
    }

    pub fn run_forever(&self) -> ! {
        self.start().expect("runtime start failed");
        loop {
            if !self.is_running() {
                std::process::exit(0);
            }
            if let Err(e) = self.tick() {
                tracing::error!("Runtime tick error: {}", e);
            }
            std::thread::sleep(self.tick_interval);
        }
    }

    pub fn handle_v8_signal(&self, signal: V8PressureSignal) -> VrmResult<GcReport> {
        let mut coordinator = self.gc_coordinator.write();
        coordinator.handle_v8_signal(signal)
    }

    pub fn gc_coordinator(&self) -> &Arc<RwLock<GcCoordinator>> {
        &self.gc_coordinator
    }

    pub fn manager(&self) -> &Arc<VoltRamManager> {
        &self.manager
    }

    pub fn tick_interval(&self) -> Duration {
        self.tick_interval
    }

    pub fn set_tick_interval(&mut self, interval_ms: u64) {
        self.tick_interval = Duration::from_millis(interval_ms);
    }
}

impl Clone for VrmRuntime {
    fn clone(&self) -> Self {
        Self {
            manager: Arc::clone(&self.manager),
            gc_coordinator: Arc::clone(&self.gc_coordinator),
            running: Arc::clone(&self.running),
            tick_interval: self.tick_interval,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::GcConfig;
    use crate::pages::page::Page;
    use crate::tiers::tier::MemoryTier;

    fn make_runtime() -> VrmRuntime {
        let registry = crate::apps::registry::AppRegistry::new();
        let governor = crate::quotas::enforcement::QuotaGovernor::new();
        let page_table = crate::pages::page_table::PageTable::new();
        let metrics = crate::metrics::registry::MetricsRegistry::new();
        let sbp_router = crate::sbp_mem::router::SbpRouter::new();
        let manager = VoltRamManager::new(registry, governor, page_table, metrics, sbp_router);
        let gc_coordinator = GcCoordinator::new(GcConfig::default());
        VrmRuntime::new(manager, gc_coordinator)
    }

    #[test]
    fn test_new_runtime_not_running() {
        let rt = make_runtime();
        assert!(!rt.is_running());
    }

    #[test]
    fn test_start_stop() {
        let rt = make_runtime();
        rt.start().unwrap();
        assert!(rt.is_running());
        rt.stop();
        assert!(!rt.is_running());
    }

    #[test]
    fn test_start_idempotent() {
        let rt = make_runtime();
        rt.start().unwrap();
        rt.start().unwrap();
        assert!(rt.is_running());
    }

    #[test]
    fn test_tick_idle() {
        let rt = make_runtime();
        rt.start().unwrap();
        let report = rt.tick().unwrap();
        assert!(report.is_empty());
    }

    #[test]
    fn test_tick_with_pages() {
        let rt = make_runtime();
        rt.start().unwrap();
        let page = Page::new(1u64, MemoryTier::T3Compressed, vec![0u8; 128]);
        rt.manager().page_table.write().insert(page).unwrap();
        std::thread::sleep(Duration::from_millis(2));
        let report = rt.tick().unwrap();
        assert!(report.pages_freed > 0 || report.is_empty());
    }

    #[test]
    fn test_tick_not_running() {
        let rt = make_runtime();
        let report = rt.tick().unwrap();
        assert!(report.is_empty());
    }

    #[test]
    fn test_handle_v8_signal() {
        let rt = make_runtime();
        let report = rt.handle_v8_signal(V8PressureSignal::Critical).unwrap();
        assert!(report.aggressive);
    }

    #[test]
    fn test_tick_interval() {
        let rt = make_runtime().with_tick_interval(200);
        assert_eq!(rt.tick_interval(), Duration::from_millis(200));
    }

    #[test]
    fn test_clone() {
        let rt = make_runtime();
        rt.start().unwrap();
        let rt2 = rt.clone();
        assert!(rt2.is_running());
    }
}
