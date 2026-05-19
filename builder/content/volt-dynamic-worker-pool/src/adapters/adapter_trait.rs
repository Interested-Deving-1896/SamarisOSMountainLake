use crate::core::pool::DynamicWorkerPool;
use crate::core::result::WorkerPoolResult;
use crate::job::job::Job;
use crate::metrics::snapshot::MetricsSnapshot;
use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;

pub trait WorkerPoolAdapter: Send + Sync {
    fn module_id(&self) -> ModuleId;
    fn profile(&self) -> ModuleProfile;
    fn submit_default_jobs(&self, pool: &DynamicWorkerPool) -> WorkerPoolResult<Vec<Job>>;
    fn on_pressure_update(&self, metrics: &MetricsSnapshot) -> WorkerPoolResult<()>;
}
