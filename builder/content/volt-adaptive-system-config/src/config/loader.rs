use crate::config::schema::AscConfig;
use crate::core::error::AscError;
use crate::core::result::AscResult;

pub fn load_config(path: Option<&str>) -> AscResult<AscConfig> {
    match path {
        Some(p) => {
            let content = std::fs::read_to_string(p)
                .map_err(|e| AscError::ConfigLoadFailed(format!("Cannot read config file '{}': {}", p, e)))?;
            toml::from_str::<AscConfig>(&content)
                .map_err(|e| AscError::ConfigLoadFailed(format!("Cannot parse config file '{}': {}", p, e)))
        }
        None => Ok(AscConfig::default()),
    }
}
