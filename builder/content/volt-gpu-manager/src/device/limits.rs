#[derive(Debug, Clone)]
pub struct GpuLimits {
    pub max_buffer_size: u64,
    pub max_texture_size: u32,
    pub max_compute_workgroups: u32,
    pub max_bind_groups: u32,
    pub max_vertex_buffers: u32,
}

impl Default for GpuLimits {
    fn default() -> Self {
        Self {
            max_buffer_size: 256 * 1024 * 1024,
            max_texture_size: 4096,
            max_compute_workgroups: 64,
            max_bind_groups: 4,
            max_vertex_buffers: 8,
        }
    }
}

impl GpuLimits {
    pub fn null() -> Self {
        Self {
            max_buffer_size: 0,
            max_texture_size: 0,
            max_compute_workgroups: 0,
            max_bind_groups: 0,
            max_vertex_buffers: 0,
        }
    }

    pub fn is_sufficient_for(&self, requirement: &str) -> bool {
        match requirement {
            "4k_texture" => self.max_texture_size >= 3840,
            "8k_texture" => self.max_texture_size >= 7680,
            "compute_light" => self.max_compute_workgroups >= 64,
            "compute_heavy" => self.max_compute_workgroups >= 256,
            "large_buffer" => self.max_buffer_size >= 1024 * 1024 * 1024,
            "bindless" => self.max_bind_groups >= 8,
            "vulkan_minimal" => {
                self.max_buffer_size >= 256 * 1024 * 1024
                    && self.max_texture_size >= 4096
                    && self.max_compute_workgroups >= 64
                    && self.max_bind_groups >= 4
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_limits() {
        let limits = GpuLimits::default();
        assert!(limits.max_buffer_size > 0);
        assert!(limits.max_texture_size > 0);
    }

    #[test]
    fn test_null_limits() {
        let limits = GpuLimits::null();
        assert_eq!(limits.max_buffer_size, 0);
        assert_eq!(limits.max_texture_size, 0);
    }

    #[test]
    fn test_sufficient_for_4k() {
        let limits = GpuLimits::default();
        assert!(limits.is_sufficient_for("4k_texture"));
    }

    #[test]
    fn test_insufficient_for_8k() {
        let limits = GpuLimits::default();
        assert!(!limits.is_sufficient_for("8k_texture"));
    }

    #[test]
    fn test_null_is_insufficient() {
        let limits = GpuLimits::null();
        assert!(!limits.is_sufficient_for("4k_texture"));
        assert!(!limits.is_sufficient_for("compute_light"));
    }

    #[test]
    fn test_vulkan_minimal_requirement() {
        let limits = GpuLimits::default();
        assert!(limits.is_sufficient_for("vulkan_minimal"));
    }

    #[test]
    fn test_unknown_requirement() {
        let limits = GpuLimits::default();
        assert!(!limits.is_sufficient_for("fake_requirement"));
    }

    #[test]
    fn test_large_buffer_requires_1gb() {
        let mut limits = GpuLimits::default();
        limits.max_buffer_size = 512 * 1024 * 1024;
        assert!(!limits.is_sufficient_for("large_buffer"));
        limits.max_buffer_size = 1024 * 1024 * 1024;
        assert!(limits.is_sufficient_for("large_buffer"));
    }
}
