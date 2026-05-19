use thiserror::Error;

#[derive(Error, Debug)]
pub enum VumError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Failed to load configuration: {0}")]
    ConfigLoadFailed(String),
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Device has been removed")]
    DeviceRemoved,
    #[error("Device is not removable")]
    DeviceNotRemovable,
    #[error("Backing path is missing: {0}")]
    BackingPathMissing(String),
    #[error("Backing path is invalid: {0}")]
    BackingPathInvalid(String),
    #[error("Mount failed: {0}")]
    MountFailed(String),
    #[error("Unmount failed: {0}")]
    UnmountFailed(String),
    #[error("FUSE is unavailable")]
    FuseUnavailable,
    #[error("FUSE operation failed: {0}")]
    FuseOperationFailed(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Filesystem is read-only")]
    ReadOnlyFilesystem,
    #[error("Path traversal rejected: {0}")]
    PathTraversalRejected(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("File already exists: {0}")]
    FileAlreadyExists(String),
    #[error("Read failed: {0}")]
    ReadFailed(String),
    #[error("Write failed: {0}")]
    WriteFailed(String),
    #[error("Flush failed: {0}")]
    FlushFailed(String),
    #[error("Fsync failed: {0}")]
    FsyncFailed(String),
    #[error("Journal open failed: {0}")]
    JournalOpenFailed(String),
    #[error("Journal write failed: {0}")]
    JournalWriteFailed(String),
    #[error("Journal read failed: {0}")]
    JournalReadFailed(String),
    #[error("Journal checksum mismatch")]
    JournalChecksumFailed,
    #[error("Journal is corrupt: {0}")]
    JournalCorrupt(String),
    #[error("Recovery is required")]
    RecoveryRequired,
    #[error("Recovery failed: {0}")]
    RecoveryFailed(String),
    #[error("Journal is dirty")]
    DirtyJournal,
    #[error("Unsafe to eject: {0}")]
    UnsafeToEject(String),
    #[error("Cache error: {0}")]
    CacheError(String),
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),
    #[error("Invalid SBP message: {0}")]
    InvalidSbpMessage(String),
    #[error("Unsupported SBP opcode: {0}")]
    UnsupportedOpcode(u8),
    #[error("Invalid permission value: {0}")]
    InvalidPermission(u8),
    #[error("Checksum mismatch")]
    ChecksumMismatch,
    #[error("Operation timed out: {0}")]
    Timeout(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Internal invariant violation: {0}")]
    InternalInvariantViolation(String),
}
