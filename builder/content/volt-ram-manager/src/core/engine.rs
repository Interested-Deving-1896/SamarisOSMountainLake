use std::sync::Arc;

use parking_lot::RwLock;

use crate::apps::registry::AppRegistry;
use crate::core::manager::VoltRamManager;
use crate::core::state::VrmState;
use crate::metrics::registry::MetricsRegistry;
use crate::pages::page_table::PageTable;
use crate::quotas::governor::QuotaGovernor;
use crate::sbp_mem::router::SbpRouter;

pub struct VrmEngine {
    pub state: VrmState,
    pub app_registry: Arc<AppRegistry>,
    pub quota_governor: Arc<RwLock<QuotaGovernor>>,
    pub page_table: Arc<RwLock<PageTable>>,
    pub metrics: Arc<MetricsRegistry>,
    pub sbp_router: Arc<RwLock<SbpRouter>>,
}

impl VrmEngine {
    pub fn new(
        app_registry: AppRegistry,
        quota_governor: QuotaGovernor,
        page_table: PageTable,
        metrics: MetricsRegistry,
        sbp_router: SbpRouter,
    ) -> Self {
        Self {
            state: VrmState::new(),
            app_registry: Arc::new(app_registry),
            quota_governor: Arc::new(RwLock::new(quota_governor)),
            page_table: Arc::new(RwLock::new(page_table)),
            metrics: Arc::new(metrics),
            sbp_router: Arc::new(RwLock::new(sbp_router)),
        }
    }

    pub fn manager(&self) -> VoltRamManager {
        let governor = (*self.quota_governor.read()).clone();
        let page_table = (*self.page_table.read()).clone();
        VoltRamManager::new(
            (*self.app_registry).clone(),
            governor,
            page_table,
            (*self.metrics).clone(),
            SbpRouter::new(),
        )
    }
}
