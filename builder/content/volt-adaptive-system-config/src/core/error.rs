use std::io;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AscError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Failed to load configuration: {0}")]
    ConfigLoadFailed(String),

    #[error("Hardware probe failed: {0}")]
    HardwareProbeFailed(String),

    #[error("CPU probe failed: {0}")]
    CpuProbeFailed(String),

    #[error("Memory probe failed: {0}")]
    MemoryProbeFailed(String),

    #[error("GPU probe failed: {0}")]
    GpuProbeFailed(String),

    #[error("Storage probe failed: {0}")]
    StorageProbeFailed(String),

    #[error("USB probe failed: {0}")]
    UsbProbeFailed(String),

    #[error("VM detection failed: {0}")]
    VmDetectionFailed(String),

    #[error("Battery probe failed: {0}")]
    BatteryProbeFailed(String),

    #[error("Invalid hardware profile: {0}")]
    InvalidHardwareProfile(String),

    #[error("Invalid override: {0}")]
    InvalidOverride(String),

    #[error("Unsafe override rejected: {0}")]
    UnsafeOverride(String),

    #[error("Budget exceeded: allocated={allocated}, cap={cap}")]
    BudgetExceeded { allocated: u64, cap: u64 },

    #[error("Budget reconciliation failed: {0}")]
    BudgetReconciliationFailed(String),

    #[error("Policy conflict: {0}")]
    PolicyConflict(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Generated config is invalid: {0}")]
    GeneratedConfigInvalid(String),

    #[error("Write failed: {0}")]
    WriteFailed(String),

    #[error("Explain report failed: {0}")]
    ExplainReportFailed(String),

    #[error("Unsupported profile: {0}")]
    UnsupportedProfile(String),

    #[error("Invalid state transition from {from:?} to {to:?}")]
    InvalidStateTransition { from: String, to: String },

    #[error("Internal invariant violation: {0}")]
    InternalInvariantViolation(String),

    #[error("I/O error: {0}")]
    IoError(String),
}

impl From<io::Error> for AscError {
    fn from(e: io::Error) -> Self {
        AscError::IoError(e.to_string())
    }
}

impl PartialEq for AscError {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for AscError {}
