use crate::adapters::adapter_trait::WorkerPoolAdapter;
use crate::core::pool::DynamicWorkerPool;
use crate::core::result::WorkerPoolResult;
use crate::job::job::Job;
use crate::metrics::snapshot::MetricsSnapshot;
use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;
use crate::priority::level::PriorityLevel;

pub struct DesktopAdapter;

impl WorkerPoolAdapter for DesktopAdapter {
    fn module_id(&self) -> ModuleId {
        ModuleId::desktop()
    }

    fn profile(&self) -> ModuleProfile {
        ModuleProfile::new(ModuleId::desktop(), PriorityLevel::High)
            .with_latency_sensitive()
    }

    fn submit_default_jobs(&self, _pool: &DynamicWorkerPool) -> WorkerPoolResult<Vec<Job>> {
        Ok(vec![])
    }

    fn on_pressure_update(&self, metrics: &MetricsSnapshot) -> WorkerPoolResult<()> {
        tracing::info!("DesktopAdapter: frame_pressure = {}", metrics.desktop_pressure);
        Ok(())
    }
}
