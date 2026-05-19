use crate::config::schema::VrmConfig;
use crate::core::error::VrmError;
use crate::core::result::VrmResult;

pub fn load_config(path: &str) -> VrmResult<VrmConfig> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| VrmError::InvalidConfig(format!("failed to read config file '{}': {}", path, e)))?;
    let config: VrmConfig = toml::from_str(&content)
        .map_err(|e| VrmError::InvalidConfig(format!("failed to parse config file '{}': {}", path, e)))?;
    config.validate()?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_non_existent() {
        let result = load_config("/tmp/non_existent_config_12345.toml");
        assert!(result.is_err());
    }
}
