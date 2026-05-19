use crate::resources::resource_id::GpuResourceId;
use crate::core::result::VgmResult;

pub struct VramCompressJob {
    pub resource_id: GpuResourceId,
    pub priority: u8,
}

impl VramCompressJob {
    pub fn new(id: GpuResourceId) -> Self {
        Self {
            resource_id: id,
            priority: 5,
        }
    }

    pub fn execute(&self) -> VgmResult<Vec<u8>> {
        let compressed = format!("compressed:{}", self.resource_id).into_bytes();
        Ok(compressed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_job() {
        let id = GpuResourceId::new();
        let job = VramCompressJob::new(id);
        assert_eq!(job.resource_id, id);
    }

    #[test]
    fn test_default_priority() {
        let job = VramCompressJob::new(GpuResourceId::nil());
        assert_eq!(job.priority, 5);
    }

    #[test]
    fn test_execute_returns_data() {
        let job = VramCompressJob::new(GpuResourceId::nil());
        let result = job.execute().unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_execute_includes_id() {
        let id = GpuResourceId::new();
        let job = VramCompressJob::new(id);
        let data = job.execute().unwrap();
        let s = String::from_utf8_lossy(&data);
        assert!(s.contains(&id.to_string()));
    }
}
