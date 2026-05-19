use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SbpGpuPermission: u32 {
        const CAP_READ_STATUS = 1 << 0;
        const CAP_GPU_ALLOC = 1 << 1;
        const CAP_GPU_COMPUTE = 1 << 2;
        const CAP_GPU_RENDER = 1 << 3;
        const CAP_ADMIN_GPU = 1 << 4;
        const INTERNAL = 1 << 5;
    }
}

impl SbpGpuPermission {
    pub fn name(&self) -> &'static str {
        if *self == SbpGpuPermission::CAP_READ_STATUS {
            "read_status"
        } else if *self == SbpGpuPermission::CAP_GPU_ALLOC {
            "gpu_alloc"
        } else if *self == SbpGpuPermission::CAP_GPU_COMPUTE {
            "gpu_compute"
        } else if *self == SbpGpuPermission::CAP_GPU_RENDER {
            "gpu_render"
        } else if *self == SbpGpuPermission::CAP_ADMIN_GPU {
            "admin_gpu"
        } else if *self == SbpGpuPermission::INTERNAL {
            "internal"
        } else {
            "unknown"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_values() {
        assert_eq!(SbpGpuPermission::CAP_READ_STATUS.bits(), 1);
        assert_eq!(SbpGpuPermission::CAP_GPU_ALLOC.bits(), 2);
        assert_eq!(SbpGpuPermission::CAP_GPU_COMPUTE.bits(), 4);
        assert_eq!(SbpGpuPermission::CAP_GPU_RENDER.bits(), 8);
        assert_eq!(SbpGpuPermission::CAP_ADMIN_GPU.bits(), 16);
        assert_eq!(SbpGpuPermission::INTERNAL.bits(), 32);
    }

    #[test]
    fn test_combination() {
        let combined = SbpGpuPermission::CAP_READ_STATUS | SbpGpuPermission::CAP_GPU_RENDER;
        assert!(combined.contains(SbpGpuPermission::CAP_READ_STATUS));
        assert!(combined.contains(SbpGpuPermission::CAP_GPU_RENDER));
        assert!(!combined.contains(SbpGpuPermission::CAP_GPU_ALLOC));
    }

    #[test]
    fn test_name() {
        assert_eq!(SbpGpuPermission::CAP_READ_STATUS.name(), "read_status");
        assert_eq!(SbpGpuPermission::CAP_GPU_ALLOC.name(), "gpu_alloc");
        assert_eq!(SbpGpuPermission::CAP_GPU_COMPUTE.name(), "gpu_compute");
        assert_eq!(SbpGpuPermission::CAP_GPU_RENDER.name(), "gpu_render");
        assert_eq!(SbpGpuPermission::CAP_ADMIN_GPU.name(), "admin_gpu");
        assert_eq!(SbpGpuPermission::INTERNAL.name(), "internal");
    }

    #[test]
    fn test_empty_not_contains() {
        let empty = SbpGpuPermission::empty();
        assert!(!empty.contains(SbpGpuPermission::CAP_READ_STATUS));
    }
}
