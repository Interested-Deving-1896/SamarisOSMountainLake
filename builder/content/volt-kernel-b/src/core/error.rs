use thiserror::Error;

#[derive(Error, Debug)]
pub enum TesseractError {
    #[error("config error: {0}")]
    Config(String),

    #[error("ipc error: {0}")]
    Ipc(String),

    #[error("scheduler error: {0}")]
    Scheduler(String),

    #[error("protocol error: {0}")]
    Protocol(String),

    #[error("security: {0}")]
    Security(String),

    #[error("system error: {0}")]
    System(String),

    #[error("gpu error: {0}")]
    Gpu(String),

    #[error("compute error: {0}")]
    Compute(String),

    #[error("media error: {0}")]
    Media(String),

    #[error("watchdog error: {0}")]
    Watchdog(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("flatbuffer error: {0}")]
    FlatBuffer(String),

    #[error("quota exceeded: {0}")]
    QuotaExceeded(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("internal: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, TesseractError>;
