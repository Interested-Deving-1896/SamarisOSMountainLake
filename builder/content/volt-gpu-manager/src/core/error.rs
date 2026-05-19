use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum VgmError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Failed to load configuration: {0}")]
    ConfigLoadFailed(String),

    #[error("GPU unavailable: {0}")]
    GpuUnavailable(String),

    #[error("Backend unavailable: {0}")]
    BackendUnavailable(String),

    #[error("Backend initialization failed: {0}")]
    BackendInitFailed(String),

    #[error("Adapter not found: {0}")]
    AdapterNotFound(String),

    #[error("Device creation failed: {0}")]
    DeviceCreationFailed(String),

    #[error("Unsupported backend: {0}")]
    UnsupportedBackend(String),

    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    #[error("Unsupported compression algorithm: {0}")]
    UnsupportedCompressionAlgorithm(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Resource already exists: {0}")]
    ResourceAlreadyExists(String),

    #[error("Resource is pinned: {0}")]
    ResourcePinned(String),

    #[error("Resource is busy: {0}")]
    ResourceBusy(String),

    #[error("Resource is not compressible: {0}")]
    ResourceNotCompressible(String),

    #[error("Resource is not restorable: {0}")]
    ResourceNotRestorable(String),

    #[error("VRAM quota exceeded: {0}")]
    VramQuotaExceeded(String),

    #[error("VRAM allocation failed: {0}")]
    VramAllocationFailed(String),

    #[error("Compressed pool is full: {0}")]
    CompressedPoolFull(String),

    #[error("Scratch budget insufficient: {0}")]
    ScratchBudgetInsufficient(String),

    #[error("Compression failed: {0}")]
    CompressionFailed(String),

    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),

    #[error("Checksum mismatch: {0}")]
    ChecksumMismatch(String),

    #[error("Dedup collision: {0}")]
    DedupCollision(String),

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Shader not found: {0}")]
    ShaderNotFound(String),

    #[error("Shader compile failed: {0}")]
    ShaderCompileFailed(String),

    #[error("Pipeline creation failed: {0}")]
    PipelineCreationFailed(String),

    #[error("Command queue failed: {0}")]
    CommandQueueFailed(String),

    #[error("GPU job failed: {0}")]
    GpuJobFailed(String),

    #[error("Frame budget exceeded: {0}")]
    FrameBudgetExceeded(String),

    #[error("Thermal backoff active: {0}")]
    ThermalBackoffActive(String),

    #[error("Thermal sensor unavailable: {0}")]
    ThermalSensorUnavailable(String),

    #[error("Multi-GPU unsupported: {0}")]
    MultiGpuUnsupported(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid SBP message: {0}")]
    InvalidSbpMessage(String),

    #[error("Unsupported opcode: {0}")]
    UnsupportedOpcode(String),

    #[error("Invalid permission: {0}")]
    InvalidPermission(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Internal invariant violation: {0}")]
    InternalInvariantViolation(String),

    #[error("I/O error: {0}")]
    IoError(String),
}

impl From<std::io::Error> for VgmError {
    fn from(err: std::io::Error) -> Self {
        VgmError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = VgmError::InvalidConfig("test".into());
        assert_eq!(format!("{}", err), "Invalid configuration: test");
    }

    #[test]
    fn test_io_conversion() {
        let io = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
        let vgm: VgmError = io.into();
        assert!(matches!(vgm, VgmError::IoError(_)));
    }
}
