use thiserror::Error;

#[derive(Error, Debug)]
pub enum BenchError {
    #[error("Collector failed: {0}")]
    CollectorFailed(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("No benchmark data found")]
    NoData,

    #[error("Invalid mode: {0}")]
    InvalidMode(String),

    #[error("Baseline import failed: {0}")]
    BaselineImportError(String),

    #[error("SBP IPC error: {0}")]
    SbpIpcError(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Division by zero in scoring")]
    DivisionByZero,
}
