use crate::core::error::VgmError;
use crate::core::result::VgmResult;
use crate::sbp_gpu::permissions::SbpGpuPermission;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SbpGpuOpcode {
    Init = 0,
    Shutdown = 1,
    Submit = 2,
    Query = 3,
    Reset = 4,
    GpuStatus = 0x40,
    GpuAllocResource = 0x41,
    GpuFreeResource = 0x42,
    GpuExecCompute = 0x43,
    GpuRenderFrame = 0x44,
    GpuThermalStatus = 0x45,
    GpuSwitchDevice = 0x46,
    GpuShaderCompile = 0x47,
    GpuVramStatus = 0x48,
    GpuBatchSubmit = 0x49,
    GpuPrefetchShaders = 0x4A,
    GpuCompressResource = 0x4B,
    GpuRestoreResource = 0x4C,
    GpuEvictResource = 0x4D,
    GpuMetricsSnapshot = 0x4E,
}

impl SbpGpuOpcode {
    pub fn from_byte(b: u8) -> VgmResult<Self> {
        match b {
            0 => Ok(SbpGpuOpcode::Init),
            1 => Ok(SbpGpuOpcode::Shutdown),
            2 => Ok(SbpGpuOpcode::Submit),
            3 => Ok(SbpGpuOpcode::Query),
            4 => Ok(SbpGpuOpcode::Reset),
            0x40 => Ok(SbpGpuOpcode::GpuStatus),
            0x41 => Ok(SbpGpuOpcode::GpuAllocResource),
            0x42 => Ok(SbpGpuOpcode::GpuFreeResource),
            0x43 => Ok(SbpGpuOpcode::GpuExecCompute),
            0x44 => Ok(SbpGpuOpcode::GpuRenderFrame),
            0x45 => Ok(SbpGpuOpcode::GpuThermalStatus),
            0x46 => Ok(SbpGpuOpcode::GpuSwitchDevice),
            0x47 => Ok(SbpGpuOpcode::GpuShaderCompile),
            0x48 => Ok(SbpGpuOpcode::GpuVramStatus),
            0x49 => Ok(SbpGpuOpcode::GpuBatchSubmit),
            0x4A => Ok(SbpGpuOpcode::GpuPrefetchShaders),
            0x4B => Ok(SbpGpuOpcode::GpuCompressResource),
            0x4C => Ok(SbpGpuOpcode::GpuRestoreResource),
            0x4D => Ok(SbpGpuOpcode::GpuEvictResource),
            0x4E => Ok(SbpGpuOpcode::GpuMetricsSnapshot),
            _ => Err(VgmError::UnsupportedOpcode(format!(
                "unknown SBP-GPU opcode: {:#04x}",
                b
            ))),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            SbpGpuOpcode::Init => "Init",
            SbpGpuOpcode::Shutdown => "Shutdown",
            SbpGpuOpcode::Submit => "Submit",
            SbpGpuOpcode::Query => "Query",
            SbpGpuOpcode::Reset => "Reset",
            SbpGpuOpcode::GpuStatus => "GpuStatus",
            SbpGpuOpcode::GpuAllocResource => "GpuAllocResource",
            SbpGpuOpcode::GpuFreeResource => "GpuFreeResource",
            SbpGpuOpcode::GpuExecCompute => "GpuExecCompute",
            SbpGpuOpcode::GpuRenderFrame => "GpuRenderFrame",
            SbpGpuOpcode::GpuThermalStatus => "GpuThermalStatus",
            SbpGpuOpcode::GpuSwitchDevice => "GpuSwitchDevice",
            SbpGpuOpcode::GpuShaderCompile => "GpuShaderCompile",
            SbpGpuOpcode::GpuVramStatus => "GpuVramStatus",
            SbpGpuOpcode::GpuBatchSubmit => "GpuBatchSubmit",
            SbpGpuOpcode::GpuPrefetchShaders => "GpuPrefetchShaders",
            SbpGpuOpcode::GpuCompressResource => "GpuCompressResource",
            SbpGpuOpcode::GpuRestoreResource => "GpuRestoreResource",
            SbpGpuOpcode::GpuEvictResource => "GpuEvictResource",
            SbpGpuOpcode::GpuMetricsSnapshot => "GpuMetricsSnapshot",
        }
    }

    pub fn permission(&self) -> SbpGpuPermission {
        match self {
            SbpGpuOpcode::Init => SbpGpuPermission::INTERNAL,
            SbpGpuOpcode::Shutdown => SbpGpuPermission::CAP_ADMIN_GPU,
            SbpGpuOpcode::Submit => SbpGpuPermission::CAP_GPU_COMPUTE,
            SbpGpuOpcode::Query => SbpGpuPermission::CAP_READ_STATUS,
            SbpGpuOpcode::Reset => SbpGpuPermission::CAP_ADMIN_GPU,
            SbpGpuOpcode::GpuStatus => SbpGpuPermission::CAP_READ_STATUS,
            SbpGpuOpcode::GpuAllocResource => SbpGpuPermission::CAP_GPU_ALLOC,
            SbpGpuOpcode::GpuFreeResource => SbpGpuPermission::CAP_GPU_ALLOC,
            SbpGpuOpcode::GpuExecCompute => SbpGpuPermission::CAP_GPU_COMPUTE,
            SbpGpuOpcode::GpuRenderFrame => SbpGpuPermission::CAP_GPU_RENDER,
            SbpGpuOpcode::GpuThermalStatus => SbpGpuPermission::CAP_READ_STATUS,
            SbpGpuOpcode::GpuSwitchDevice => SbpGpuPermission::CAP_ADMIN_GPU,
            SbpGpuOpcode::GpuShaderCompile => SbpGpuPermission::CAP_GPU_COMPUTE,
            SbpGpuOpcode::GpuVramStatus => SbpGpuPermission::CAP_READ_STATUS,
            SbpGpuOpcode::GpuBatchSubmit => SbpGpuPermission::CAP_GPU_COMPUTE,
            SbpGpuOpcode::GpuPrefetchShaders => SbpGpuPermission::CAP_GPU_COMPUTE,
            SbpGpuOpcode::GpuCompressResource => SbpGpuPermission::CAP_GPU_ALLOC,
            SbpGpuOpcode::GpuRestoreResource => SbpGpuPermission::CAP_GPU_ALLOC,
            SbpGpuOpcode::GpuEvictResource => SbpGpuPermission::CAP_GPU_ALLOC,
            SbpGpuOpcode::GpuMetricsSnapshot => SbpGpuPermission::CAP_READ_STATUS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_opcodes() {
        assert_eq!(SbpGpuOpcode::from_byte(0).unwrap(), SbpGpuOpcode::Init);
        assert_eq!(SbpGpuOpcode::from_byte(1).unwrap(), SbpGpuOpcode::Shutdown);
        assert_eq!(SbpGpuOpcode::from_byte(2).unwrap(), SbpGpuOpcode::Submit);
        assert_eq!(SbpGpuOpcode::from_byte(3).unwrap(), SbpGpuOpcode::Query);
        assert_eq!(SbpGpuOpcode::from_byte(4).unwrap(), SbpGpuOpcode::Reset);
    }

    #[test]
    fn test_gpu_opcodes() {
        for b in 0x40..=0x4E {
            let op = SbpGpuOpcode::from_byte(b).unwrap();
            assert_eq!(op as u8, b);
        }
    }

    #[test]
    fn test_opcode_from_byte_invalid() {
        let result = SbpGpuOpcode::from_byte(0xFF);
        assert!(result.is_err());
    }

    #[test]
    fn test_opcode_name_not_empty() {
        for b in 0..=4 {
            let op = SbpGpuOpcode::from_byte(b).unwrap();
            assert!(!op.name().is_empty());
        }
        for b in 0x40..=0x4E {
            let op = SbpGpuOpcode::from_byte(b).unwrap();
            assert!(!op.name().is_empty());
        }
    }

    #[test]
    fn test_opcode_permission() {
        assert_eq!(SbpGpuOpcode::Init.permission(), SbpGpuPermission::INTERNAL);
        assert_eq!(SbpGpuOpcode::GpuStatus.permission(), SbpGpuPermission::CAP_READ_STATUS);
        assert_eq!(SbpGpuOpcode::GpuAllocResource.permission(), SbpGpuPermission::CAP_GPU_ALLOC);
        assert_eq!(SbpGpuOpcode::GpuExecCompute.permission(), SbpGpuPermission::CAP_GPU_COMPUTE);
        assert_eq!(SbpGpuOpcode::GpuRenderFrame.permission(), SbpGpuPermission::CAP_GPU_RENDER);
        assert_eq!(SbpGpuOpcode::GpuSwitchDevice.permission(), SbpGpuPermission::CAP_ADMIN_GPU);
    }

    #[test]
    fn test_error_type() {
        let err = SbpGpuOpcode::from_byte(0xFF).unwrap_err();
        assert!(matches!(err, VgmError::UnsupportedOpcode(_)));
    }
}
