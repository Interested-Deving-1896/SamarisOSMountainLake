use std::sync::Arc;

use parking_lot::RwLock;

use crate::apps::app_id::AppId;
use crate::apps::app_profile::AppProfile;
use crate::apps::registry::AppRegistry;
use crate::core::result::VrmResult;
use crate::core::state::VrmState;
use crate::metrics::registry::MetricsRegistry;
use crate::metrics::snapshot::MetricsSnapshot;
use crate::pages::page_table::PageTable;
use crate::quotas::app_quota::AppQuota;
use crate::quotas::governor::QuotaGovernor;
use crate::sbp_mem::message::SbpMessage;
use crate::sbp_mem::response::SbpResponse;
use crate::sbp_mem::router::SbpRouter;

pub struct VoltRamManager {
    pub state: VrmState,
    pub app_registry: Arc<AppRegistry>,
    pub quota_governor: Arc<RwLock<QuotaGovernor>>,
    pub page_table: Arc<RwLock<PageTable>>,
    pub metrics: Arc<MetricsRegistry>,
    pub sbp_router: Arc<RwLock<SbpRouter>>,
}

impl VoltRamManager {
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

    pub fn register_app(&self, profile: AppProfile) -> VrmResult<AppId> {
        let id = self.app_registry.register(profile)?;
        let mut qg = self.quota_governor.write();
        qg.set_quota(id, id.into());
        self.metrics.counters.apps_registered.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Ok(id)
    }

    pub fn unregister_app(&self, app_id: AppId) -> VrmResult<()> {
        self.app_registry.unregister(app_id)?;
        let mut qg = self.quota_governor.write();
        qg.remove_quota(app_id);
        self.metrics.counters.apps_registered.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }

    pub fn set_quota(&self, app_id: AppId, quota: AppQuota) -> VrmResult<()> {
        let state = self.quota_governor.read();
        let _ = state;
        let mut qg = self.quota_governor.write();
        qg.set_quota(app_id, quota);
        Ok(())
    }

    pub fn check_quota(&self, app_id: AppId, additional: u64) -> VrmResult<()> {
        let qg = self.quota_governor.read();
        qg.check_quota(app_id, additional)
    }

    pub fn shutdown(&self) {
        self.state.request_shutdown();
        tracing::info!("Volt RAM Manager shutdown requested");
    }

    pub fn is_shutdown_requested(&self) -> bool {
        self.state.is_shutdown_requested()
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        self.metrics.snapshot(&self.state)
    }

    pub fn handle_sbp(&self, _msg: SbpMessage) -> VrmResult<SbpResponse> {
        let router = self.sbp_router.read();
        router.route(&_msg, self)
    }
}
