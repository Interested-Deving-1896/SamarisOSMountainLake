use std::fs;
use std::path::Path;

use crate::config::defaults::default_config;
use crate::config::schema::WorkerPoolConfig;
use crate::core::error::WorkerPoolError;
use crate::core::result::WorkerPoolResult;

pub fn load_config(path: Option<&str>) -> WorkerPoolResult<WorkerPoolConfig> {
    match path {
        Some(p) => {
            let content =
                fs::read_to_string(Path::new(p)).map_err(|e| {
                    WorkerPoolError::ConfigLoadFailed(format!(
                        "Cannot read config file '{}': {}",
                        p, e
                    ))
                })?;
            toml::from_str(&content).map_err(|e| {
                WorkerPoolError::ConfigLoadFailed(format!(
                    "Cannot parse config file '{}': {}",
                    p, e
                ))
            })
        }
        None => Ok(default_config()),
    }
}
