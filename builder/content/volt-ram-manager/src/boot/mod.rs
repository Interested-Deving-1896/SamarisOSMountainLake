use std::sync::Arc;
use std::io::Write;

use crate::apps::registry::AppRegistry;
use crate::config::schema::VrmConfig;
use crate::core::manager::VoltRamManager;
use crate::core::result::VrmResult;
use crate::metrics::registry::MetricsRegistry;
use crate::pages::page_table::PageTable;
use crate::quotas::governor::QuotaGovernor;
use crate::quotas::app_quota::AppQuota;
use crate::apps::app_profile::AppProfile;
use crate::apps::app_id::AppId;
use crate::sbp_mem::router::SbpRouter;
use crate::sbp_mem::handler::*;

pub struct BootSequence;

impl BootSequence {
    pub fn run(config: &VrmConfig) -> VrmResult<VoltRamManager> {
        let app_registry = AppRegistry::new();
        let governor = QuotaGovernor::new();
        let page_table = PageTable::new();
        let metrics = MetricsRegistry::new();

        let mut router = SbpRouter::new();
        router.register(Box::new(StatusHandler));
        router.register(Box::new(RegisterAppHandler));
        router.register(Box::new(SetQuotaHandler));
        router.register(Box::new(HeartbeatHandler));
        router.register(Box::new(SnapshotHandler));

        let manager = VoltRamManager::new(app_registry, governor, page_table, metrics, router);

        // Register default apps from config
        Self::register_default_apps(&manager);

        // Signal readiness
        Self::signal_ready();
        manager.state.mark_boot_complete();

        tracing::info!("Volt RAM Manager initialized (boot complete)");
        Ok(manager)
    }

    fn register_default_apps(manager: &VoltRamManager) {
        for (name, app_cfg) in &[
            ("desktop", (256u64, "critical", false)),
            ("orbit", (1024u64, "critical", false)),
            ("peregrine", (512u64, "high", true)),
            ("finder", (256u64, "high", true)),
            ("photos", (256u64, "normal", true)),
            ("music", (128u64, "normal", true)),
            ("settings", (64u64, "low", true)),
            ("wallpaper", (32u64, "idle", true)),
        ] {
            if let Err(e) = manager.app_registry.register(AppProfile {
                app_id: AppId::new(0),
                name: name.to_string(),
                priority: crate::apps::app_profile::AppPriority::from_str(app_cfg.1),
                max_quota_mb: app_cfg.0,
                compression_allowed: app_cfg.2,
                inactive_after_ms: 5000,
                preferred_tier: crate::tiers::tier::MemoryTier::T2Direct,
            }) {
                tracing::warn!("Failed to register default app {name}: {e}");
            }
        }
    }

    fn signal_ready() {
        let path = std::path::Path::new("/run/volt-ram-manager.ready");
        if let Ok(mut f) = std::fs::File::create(path) {
            let _ = write!(f, "PID={}\nREADY_AT={}\n", std::process::id(),
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs());
        }
    }
}
