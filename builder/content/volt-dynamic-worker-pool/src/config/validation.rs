use crate::config::schema::WorkerPoolConfig;
use crate::core::error::WorkerPoolError;
use crate::core::result::WorkerPoolResult;

static VALID_PRIORITIES: &[&str] = &["critical", "high", "normal", "idle"];
static VALID_INTEGRATION_MODES: &[&str] = &["standalone", "adapter_ready", "full_runtime"];

pub fn validate_config(config: &WorkerPoolConfig) -> WorkerPoolResult<()> {
    let wp = &config.worker_pool;

    if wp.max_workers_cap == 0 {
        return Err(WorkerPoolError::InvalidConfig(
            "max_workers_cap must be greater than 0".into(),
        ));
    }

    if wp.yield_budget_us == 0 {
        return Err(WorkerPoolError::InvalidConfig(
            "yield_budget_us must be greater than 0".into(),
        ));
    }

    if wp.scaling.scale_cooldown_ms == 0 {
        return Err(WorkerPoolError::InvalidConfig(
            "scale_cooldown_ms must be greater than 0".into(),
        ));
    }

    if wp.reservations.orbit_burst_cooldown_ms == 0 {
        return Err(WorkerPoolError::InvalidConfig(
            "orbit_burst_cooldown_ms must be greater than 0".into(),
        ));
    }

    if !VALID_INTEGRATION_MODES.contains(&wp.integration_mode.as_str()) {
        return Err(WorkerPoolError::InvalidConfig(format!(
            "integration_mode '{}' is not valid; expected one of: {}",
            wp.integration_mode,
            VALID_INTEGRATION_MODES.join(", ")
        )));
    }

    let priority_fields = [
        (&wp.priorities.orbit, "priorities.orbit"),
        (&wp.priorities.desktop, "priorities.desktop"),
        (&wp.priorities.electron, "priorities.electron"),
        (&wp.priorities.kernel_a, "priorities.kernel_a"),
        (&wp.priorities.kernel_b, "priorities.kernel_b"),
        (&wp.priorities.vrm, "priorities.vrm"),
        (&wp.priorities.vum, "priorities.vum"),
        (&wp.priorities.vgm, "priorities.vgm"),
        (&wp.priorities.background, "priorities.background"),
    ];

    for (value, field_name) in &priority_fields {
        if !VALID_PRIORITIES.contains(&value.as_str()) {
            return Err(WorkerPoolError::InvalidConfig(format!(
                "{} '{}' is not a valid priority; expected one of: {}",
                field_name,
                value,
                VALID_PRIORITIES.join(", ")
            )));
        }
    }

    Ok(())
}
