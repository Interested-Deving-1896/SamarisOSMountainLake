use std::sync::Arc;

use crate::config::loader::load_config;
use crate::config::schema::AscConfig;
use crate::config::validation::validate_config;
use crate::core::asc::VoltAsc;
use crate::core::result::AscResult;
use crate::explain::report::ExplainReport;
use crate::generator::generated_config::GeneratedConfig;
use crate::generator::toml_writer::write_generated_config;
use crate::hardware::profile::HardwareProfile;
use crate::runtime::paths;

pub struct RuntimeService {
    pub config: AscConfig,
    pub asc: Arc<VoltAsc>,
}

impl RuntimeService {
    pub fn new(config_path: Option<&str>) -> AscResult<Self> {
        let config = load_config(config_path)?;
        validate_config(&config)?;
        let asc = Arc::new(VoltAsc::new(config.clone()));
        Ok(Self { config, asc })
    }

    pub fn probe(&self) -> AscResult<HardwareProfile> {
        self.asc.probe()
    }

    pub fn generate(&self) -> AscResult<GeneratedConfig> {
        self.asc.full_pipeline()
    }

    pub fn explain(&self) -> AscResult<ExplainReport> {
        self.asc.explain()
    }

    pub fn dry_run(&self) -> AscResult<GeneratedConfig> {
        let config = self.asc.full_pipeline()?;
        Ok(config)
    }

    pub fn check(&self) -> AscResult<()> {
        validate_config(&self.config)
    }

    pub fn write_config(&self, path: Option<&str>) -> AscResult<()> {
        let config = self.asc.full_pipeline()?;
        let output_path = path.unwrap_or(paths::DEFAULT_GENERATED_CONFIG_PATH);
        write_generated_config(&config, output_path)?;
        let json_path = paths::DEFAULT_HARDWARE_PROFILE_PATH;
        if let Some(hw) = self.asc.hardware_profile() {
            let json = serde_json::to_string_pretty(&hw)
                .map_err(|e| crate::core::error::AscError::WriteFailed(e.to_string()))?;
            let parent = std::path::Path::new(json_path).parent().unwrap();
            let _ = std::fs::create_dir_all(parent);
            std::fs::write(json_path, &json)?;
        }
        let _ = std::fs::create_dir_all(
            std::path::Path::new(paths::DEFAULT_LAST_CONFIG_PATH).parent().unwrap(),
        );
        let toml = toml::to_string_pretty(&config)
            .map_err(|e| crate::core::error::AscError::WriteFailed(e.to_string()))?;
        std::fs::write(paths::DEFAULT_LAST_CONFIG_PATH, &toml)?;
        Ok(())
    }
}
