use thiserror::Error;

#[derive(Error, Debug)]
pub enum VrmError {
    #[error("quota exceeded: app {app_id} used {used} of {limit}")]
    QuotaExceeded { app_id: u64, used: u64, limit: u64 },

    #[error("app not registered: {0}")]
    AppNotRegistered(u64),

    #[error("app already registered: {0}")]
    AppAlreadyRegistered(u64),

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("allocation failed: {0}")]
    AllocationFailed(String),

    #[error("invalid allocation: {0}")]
    InvalidAllocation(String),

    #[error("page not found: {0}")]
    PageNotFound(u64),

    #[error("page is pinned")]
    PagePinned,

    #[error("compression failed: {0}")]
    CompressionFailed(String),

    #[error("decompression failed: {0}")]
    DecompressionFailed(String),

    #[error("dedup collision: hash matched but content differs")]
    DedupCollision,

    #[error("invalid SBP message: {0}")]
    InvalidSbpMessage(String),

    #[error("unsupported opcode: 0x{0:02X}")]
    UnsupportedOpcode(u8),

    #[error("invalid permission for opcode 0x{0:02X}")]
    InvalidPermission(u8),

    #[error("invalid config: {0}")]
    InvalidConfig(String),

    #[error("invalid state: {0}")]
    InvalidState(String),

    #[error("SHM error: {0}")]
    ShmError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("platform error: {0}")]
    PlatformError(String),

    #[error("internal invariant violation: {0}")]
    InternalInvariantViolation(String),

    #[error("{0}")]
    Other(String),
}
