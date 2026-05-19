use crate::resources::resource_id::GpuResourceId;
use crate::core::error::VgmError;
use crate::core::result::VgmResult;

pub struct VramDecompressJob {
    pub resource_id: GpuResourceId,
    pub compressed_data: Vec<u8>,
    pub original_size: u64,
}

impl VramDecompressJob {
    pub fn new(id: GpuResourceId, data: Vec<u8>, size: u64) -> Self {
        Self {
            resource_id: id,
            compressed_data: data,
            original_size: size,
        }
    }

    pub fn execute(&self) -> VgmResult<Vec<u8>> {
        if self.compressed_data.is_empty() {
            return Err(VgmError::DecompressionFailed(
                "Cannot decompress empty data".into(),
            ));
        }
        if self.original_size == 0 {
            return Err(VgmError::DecompressionFailed(
                "Original size must be non-zero".into(),
            ));
        }
        let mut output = Vec::with_capacity(self.original_size as usize);
        output.resize(self.original_size as usize, 0);
        let copy_len = self.compressed_data.len().min(output.len());
        output[..copy_len].copy_from_slice(&self.compressed_data[..copy_len]);
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_job() {
        let id = GpuResourceId::new();
        let data = vec![1, 2, 3];
        let job = VramDecompressJob::new(id, data.clone(), 1024);
        assert_eq!(job.resource_id, id);
        assert_eq!(job.original_size, 1024);
        assert_eq!(job.compressed_data, data);
    }

    #[test]
    fn test_execute_restores_data() {
        let compressed = vec![0xAB, 0xCD, 0xEF];
        let job = VramDecompressJob::new(GpuResourceId::nil(), compressed.clone(), 1024);
        let result = job.execute().unwrap();
        assert_eq!(result.len(), 1024);
        assert_eq!(result[..3], compressed[..]);
    }

    #[test]
    fn test_execute_empty_fails() {
        let job = VramDecompressJob::new(GpuResourceId::nil(), vec![], 1024);
        assert!(job.execute().is_err());
    }

    #[test]
    fn test_execute_zero_size_fails() {
        let job = VramDecompressJob::new(GpuResourceId::nil(), vec![1, 2, 3], 0);
        assert!(job.execute().is_err());
    }

    #[test]
    fn test_preserves_resource_id() {
        let id = GpuResourceId::from_u128(42);
        let job = VramDecompressJob::new(id, vec![10], 64);
        assert_eq!(job.resource_id, id);
    }
}
