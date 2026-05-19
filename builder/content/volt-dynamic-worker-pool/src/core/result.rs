use crate::core::error::WorkerPoolError;

pub type WorkerPoolResult<T> = Result<T, WorkerPoolError>;
