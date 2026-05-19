use std::fs;
use std::path::Path;

use crate::config::schema::VumConfig;
use crate::core::error::VumError;
use crate::core::result::VumResult;

pub fn load_config(path: &str) -> VumResult<VumConfig> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(VumError::ConfigLoadFailed(format!(
            "Config file not found: {}",
            path.display()
        )));
    }
    let content = fs::read_to_string(path).map_err(|e| {
        VumError::ConfigLoadFailed(format!("Cannot read {}: {}", path.display(), e))
    })?;
    let config: VumConfig = toml::from_str(&content).map_err(|e| {
        VumError::ConfigLoadFailed(format!("Cannot parse {}: {}", path.display(), e))
    })?;
    config.validate()?;
    Ok(config)
}

pub fn load_config_or_default(path: &str) -> VumConfig {
    match load_config(path) {
        Ok(config) => config,
        Err(e) => {
            tracing::warn!("Failed to load config from {}: {}. Using defaults.", path, e);
            VumConfig::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_load_config_not_found() {
        let result = load_config("/nonexistent/path.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_config_valid() {
        let toml_str = r#"
[manager]
mount_point = "/mnt/test"
backing_path = "/var/test/backing"
"#;
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "{}", toml_str).unwrap();
        let result = load_config(tmp.path().to_str().unwrap());
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.manager.mount_point, "/mnt/test");
    }

    #[test]
    fn test_load_config_or_default() {
        let config = load_config_or_default("/nonexistent/path.toml");
        assert_eq!(config.manager.mount_point, crate::config::defaults::mount_point());
    }
}
