use volt_adaptive_system_config::config::schema::AscConfig;
use volt_adaptive_system_config::config::validation::validate_config;
use volt_adaptive_system_config::config::overrides::OverrideValidator;

#[test]
fn test_default_config_is_valid() {
    let config = AscConfig::default();
    assert!(config.adaptive.enabled);
    assert_eq!(config.adaptive.mode, "auto");
    assert_eq!(config.adaptive.profile, "balanced");
}

#[test]
fn test_validate_default_config() {
    let config = AscConfig::default();
    assert!(validate_config(&config).is_ok());
}

#[test]
fn test_validate_invalid_mode() {
    let mut config = AscConfig::default();
    config.adaptive.mode = "invalid".into();
    assert!(validate_config(&config).is_err());
}

#[test]
fn test_validate_invalid_profile() {
    let mut config = AscConfig::default();
    config.adaptive.profile = "nonexistent".into();
    assert!(validate_config(&config).is_err());
}

#[test]
fn test_override_validator_rejects_negative() {
    let result = OverrideValidator::validate_override("test", "-1", true);
    assert!(result.is_err());
}

#[test]
fn test_override_validator_accepts_valid() {
    let result = OverrideValidator::validate_override("cpu_cores", "8", true);
    assert!(result.is_ok());
}

#[test]
fn test_config_profiles_list() {
    let config = AscConfig::default();
    assert!(config.profiles.available.contains(&"balanced".to_string()));
    assert!(config.profiles.available.contains(&"performance".to_string()));
    assert!(config.profiles.available.contains(&"performance".to_string()));
}

#[test]
fn test_output_paths_not_empty() {
    let config = AscConfig::default();
    assert!(!config.output.generated_config_path.is_empty());
    assert!(!config.output.hardware_profile_path.is_empty());
}
