use crate::job::job::Job;
use crate::job::job_id::JobId;
use crate::orbit::burst_controller::OrbitBurstRequest;
use crate::priority::level::PriorityLevel;

pub struct InferenceJob {
    pub model_name: String,
    pub input_size_bytes: u64,
    pub expected_duration_ms: u64,
    pub requires_burst: bool,
}

impl InferenceJob {
    pub fn new(model_name: String, input_size: u64, expected_duration: u64, requires_burst: bool) -> Self {
        Self {
            model_name,
            input_size_bytes: input_size,
            expected_duration_ms: expected_duration,
            requires_burst,
        }
    }

    pub fn into_job(self) -> Job {
        let id = JobId::new();
        Job::new(id, self.model_name, PriorityLevel::Critical, self.input_size_bytes)
    }

    pub fn into_burst_request(&self) -> OrbitBurstRequest {
        let job_id = JobId::new().to_string();
        OrbitBurstRequest::new(
            job_id,
            8,
            self.expected_duration_ms,
            format!("inference_burst:{}", self.model_name),
        )
    }
}
