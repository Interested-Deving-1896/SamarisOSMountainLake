use crate::adapters::adapter_trait::WorkerPoolAdapter;
use crate::core::pool::DynamicWorkerPool;
use crate::core::result::WorkerPoolResult;
use crate::job::job::Job;
use crate::metrics::snapshot::MetricsSnapshot;
use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;
use crate::orbit::inference_job::InferenceJob;
use crate::priority::level::PriorityLevel;

pub struct OrbitAdapter;

impl WorkerPoolAdapter for OrbitAdapter {
    fn module_id(&self) -> ModuleId {
        ModuleId::orbit()
    }

    fn profile(&self) -> ModuleProfile {
        ModuleProfile::new(ModuleId::orbit(), PriorityLevel::Critical)
            .with_can_burst()
    }

    fn submit_default_jobs(&self, pool: &DynamicWorkerPool) -> WorkerPoolResult<Vec<Job>> {
        let inference = InferenceJob::new("default_model".into(), 1024, 100, false);
        let job = inference.into_job();
        pool.submit_job(job.clone())?;
        Ok(vec![job])
    }

    fn on_pressure_update(&self, _metrics: &MetricsSnapshot) -> WorkerPoolResult<()> {
        Ok(())
    }
}
