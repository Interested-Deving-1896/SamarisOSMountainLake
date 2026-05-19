use crate::core::error::AscError;
use crate::core::result::AscResult;
use crate::generator::generated_config::GeneratedConfig;

pub fn write_generated_config(config: &GeneratedConfig, path: &str) -> AscResult<()> {
    let toml_string = toml::to_string_pretty(config)
        .map_err(|e| AscError::WriteFailed(e.to_string()))?;
    let parent = std::path::Path::new(path).parent().unwrap();
    std::fs::create_dir_all(parent)?;
    std::fs::write(path, &toml_string)?;
    Ok(())
}

pub fn generated_config_toml(config: &GeneratedConfig) -> AscResult<String> {
    toml::to_string_pretty(config)
        .map_err(|e| AscError::WriteFailed(e.to_string()))
}
