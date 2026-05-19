pub struct GpuMatmulJob {
    pub m: u32,
    pub n: u32,
    pub k: u32,
    pub data: Vec<u8>,
}

impl GpuMatmulJob {
    pub fn new(m: u32, n: u32, k: u32) -> Self {
        let data_size = (m as u64 * k as u64 + k as u64 * n as u64) as usize;
        Self {
            m,
            n,
            k,
            data: vec![0u8; data_size],
        }
    }

    pub fn flops(&self) -> u64 {
        2 * self.m as u64 * self.n as u64 * self.k as u64
    }

    pub fn is_supported() -> bool {
        cfg!(feature = "wgpu_backend")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_job_allocates_data() {
        let job = GpuMatmulJob::new(64, 64, 64);
        let expected_size = (64 * 64 + 64 * 64) as usize;
        assert_eq!(job.data.len(), expected_size);
    }

    #[test]
    fn test_flops_calculation() {
        let job = GpuMatmulJob::new(128, 128, 128);
        assert_eq!(job.flops(), 2 * 128 * 128 * 128);
    }

    #[test]
    fn test_is_supported() {
        let _ = GpuMatmulJob::is_supported();
    }

    #[test]
    fn test_zero_dimensions() {
        let job = GpuMatmulJob::new(0, 0, 0);
        assert_eq!(job.flops(), 0);
        assert!(job.data.is_empty());
    }

    #[test]
    fn test_non_square() {
        let job = GpuMatmulJob::new(4, 8, 16);
        assert_eq!(job.flops(), 2 * 4 * 8 * 16);
    }
}
