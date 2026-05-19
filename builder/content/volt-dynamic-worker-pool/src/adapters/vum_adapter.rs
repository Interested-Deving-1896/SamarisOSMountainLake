use crate::adapters::adapter_trait::WorkerPoolAdapter;
use crate::core::pool::DynamicWorkerPool;
use crate::core::result::WorkerPoolResult;
use crate::job::job::Job;
use crate::metrics::snapshot::MetricsSnapshot;
use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;
use crate::priority::level::PriorityLevel;

pub struct VumAdapter;

impl WorkerPoolAdapter for VumAdapter {
    fn module_id(&self) -> ModuleId {
        ModuleId::vum()
    }

    fn profile(&self) -> ModuleProfile {
        ModuleProfile::new(ModuleId::vum(), PriorityLevel::Normal)
            .with_resource_fraction(0.15)
    }

    fn submit_default_jobs(&self, _pool: &DynamicWorkerPool) -> WorkerPoolResult<Vec<Job>> {
        Ok(vec![])
    }

    fn on_pressure_update(&self, _metrics: &MetricsSnapshot) -> WorkerPoolResult<()> {
        Ok(())
    }
}
