use std::fs;
use std::path::Path;

use crate::config::schema::VgmConfig;
use crate::core::error::VgmError;
use crate::core::result::VgmResult;

pub fn load_config(path: &Path) -> VgmResult<VgmConfig> {
    tracing::info!("Loading configuration from '{}'", path.display());

    let contents = fs::read_to_string(path).map_err(|e| {
        VgmError::ConfigLoadFailed(format!(
            "Cannot read config file '{}': {}",
            path.display(),
            e
        ))
    })?;

    let config: VgmConfig = toml::from_str(&contents).map_err(|e| {
        VgmError::ConfigLoadFailed(format!(
            "Cannot parse config file '{}': {}",
            path.display(),
            e
        ))
    })?;

    config.validate()?;

    tracing::info!("Configuration loaded and validated successfully");
    Ok(config)
}

pub fn load_config_or_default(path: Option<&Path>) -> VgmResult<VgmConfig> {
    match path {
        Some(p) if p.exists() => load_config(p),
        Some(p) => {
            tracing::warn!(
                "Config file '{}' not found; using default configuration",
                p.display()
            );
            Ok(VgmConfig::default())
        }
        None => {
            tracing::info!("No config file provided; using default configuration");
            Ok(VgmConfig::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_config_from_file() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            "[gpu]\nbackend = \"null\"\nframe_budget_ms = 32\n"
        )
        .unwrap();
        let config = load_config(file.path()).unwrap();
        assert_eq!(config.gpu.backend, "null");
        assert_eq!(config.gpu.frame_budget_ms, 32);
    }

    #[test]
    fn test_load_config_default_on_none() {
        let config = load_config_or_default(None).unwrap();
        assert_eq!(config.gpu.backend, "wgpu");
    }
}
