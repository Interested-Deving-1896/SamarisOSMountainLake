use volt_dynamic_worker_pool::*;

#[test]
fn test_default_config_loads() {
    let cfg = config::defaults::default_config();
    assert_eq!(cfg.worker_pool.max_workers_cap, 48);
    assert_eq!(cfg.worker_pool.integration_mode, "standalone");
}

#[test]
fn test_config_validation_passes_for_default() {
    let cfg = config::defaults::default_config();
    assert!(config::validation::validate_config(&cfg).is_ok());
}

#[test]
fn test_config_validation_fails_on_zero_max_workers() {
    let mut cfg = config::defaults::default_config();
    cfg.worker_pool.max_workers_cap = 0;
    let result = config::validation::validate_config(&cfg);
    assert!(result.is_err());
}

#[test]
fn test_config_validation_fails_on_invalid_integration_mode() {
    let mut cfg = config::defaults::default_config();
    cfg.worker_pool.integration_mode = "bogus".into();
    let result = config::validation::validate_config(&cfg);
    assert!(result.is_err());
}

#[test]
fn test_config_validation_fails_on_invalid_priority() {
    let mut cfg = config::defaults::default_config();
    cfg.worker_pool.priorities.orbit = "ultra".into();
    let result = config::validation::validate_config(&cfg);
    assert!(result.is_err());
}

#[test]
fn test_load_config_from_none_returns_default() {
    let cfg = config::loader::load_config(None).unwrap();
    assert!(cfg.worker_pool.preemption_enabled);
}
