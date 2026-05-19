use std::io;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum WorkerPoolError {
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Failed to load configuration: {0}")]
    ConfigLoadFailed(String),

    #[error("Hardware probe failed: {0}")]
    HardwareProbeFailed(String),

    #[error("Pool has not been started")]
    PoolNotStarted,

    #[error("Pool has already been started")]
    PoolAlreadyStarted,

    #[error("Pool is shutting down")]
    PoolShuttingDown,

    #[error("Pool is in shutdown state")]
    PoolInShutdown,

    #[error("Invalid state transition from {from:?} to {to:?}")]
    InvalidStateTransition { from: &'static str, to: &'static str },

    #[error("Failed to spawn worker: {0}")]
    WorkerSpawnFailed(String),

    #[error("Worker not found: {0}")]
    WorkerNotFound(String),

    #[error("Worker is not idle")]
    WorkerNotIdle,

    #[error("Worker is still running")]
    WorkerStillRunning,

    #[error("Job not found: {0}")]
    JobNotFound(String),

    #[error("Job already completed: {0}")]
    JobAlreadyCompleted(String),

    #[error("Job was cancelled: {0}")]
    JobCancelled(String),

    #[error("Job timed out: {0}")]
    JobTimedOut(String),

    #[error("Yield failed: {0}")]
    YieldFailed(String),

    #[error("Resume failed: {0}")]
    ResumeFailed(String),

    #[error("Scaling cooldown active, remaining: {0}ms")]
    ScalingCooldownActive(u64),

    #[error("Already at minimum worker count")]
    ScalingAtMin,

    #[error("Already at maximum worker count")]
    ScalingAtMax,

    #[error("Thermal backoff active")]
    ThermalBackoffActive,

    #[error("Desktop guard is active")]
    DesktopGuardActive,

    #[error("Orbit burst limit reached")]
    OrbitBurstLimitReached,

    #[error("Orbit burst cooldown active")]
    OrbitBurstCooldownActive,

    #[error("Orbit burst request rejected: {0}")]
    OrbitBurstRejected(String),

    #[error("Priority queue is empty")]
    PriorityQueueEmpty,

    #[error("Module not found: {0}")]
    ModuleNotFound(String),

    #[error("Module already registered: {0}")]
    ModuleAlreadyRegistered(String),

    #[error("Adapter failed: {0}")]
    AdapterFailed(String),

    #[error("Metrics not available")]
    MetricsNotAvailable,

    #[error("Internal invariant violation: {0}")]
    InternalInvariantViolation(String),

    #[error("I/O error: {0}")]
    IoError(String),
}

impl From<io::Error> for WorkerPoolError {
    fn from(e: io::Error) -> Self {
        WorkerPoolError::IoError(e.to_string())
    }
}

impl PartialEq for WorkerPoolError {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for WorkerPoolError {}
