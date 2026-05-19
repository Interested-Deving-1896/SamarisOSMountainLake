use tracing;

use crate::config::schema::AscConfig;
use crate::core::error::AscError;
use crate::core::result::AscResult;

pub fn validate_config(config: &AscConfig) -> AscResult<()> {
    if !config.adaptive.enabled {
        tracing::warn!("Volt adaptive system is disabled; system will use static defaults");
    }

    match config.adaptive.mode.as_str() {
        "auto" | "manual" | "profile" => {}
        other => {
            return Err(AscError::InvalidConfig(format!(
                "Invalid adaptive mode '{}': must be 'auto', 'manual', or 'profile'",
                other
            )));
        }
    }

    if !config.profiles.available.contains(&config.adaptive.profile) {
        return Err(AscError::InvalidConfig(format!(
            "Profile '{}' is not in the available profiles list {:?}",
            config.adaptive.profile, config.profiles.available
        )));
    }

    if config.adaptive.profile.is_empty() {
        return Err(AscError::InvalidConfig(
            "adaptive.profile must not be empty".into(),
        ));
    }

    if config.output.generated_config_path.is_empty() {
        return Err(AscError::InvalidConfig(
            "output.generated_config_path must not be empty".into(),
        ));
    }

    if config.output.hardware_profile_path.is_empty() {
        return Err(AscError::InvalidConfig(
            "output.hardware_profile_path must not be empty".into(),
        ));
    }

    if config.output.last_generated_config_path.is_empty() {
        return Err(AscError::InvalidConfig(
            "output.last_generated_config_path must not be empty".into(),
        ));
    }

    Ok(())
}
