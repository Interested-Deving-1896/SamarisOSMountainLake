// VOLT Display Manager — Library
//
// One OS. Any screen. Zero manual tuning.

pub mod display;

pub use display::types::{
    DisplayConfig, DisplayProfile, Screen, LayoutMode, Rotation,
    DisplayBackend, DisplaySession, DpiClass, DisplayError,
};

pub use display::detector::detect;
pub use display::planner::plan;
pub use display::applier::apply;
pub use display::validator::validate;
pub use display::rollback::{rollback, safe_mode};
pub use display::persist::{persist_runtime, persist_user_config, load_runtime, load_user_config};
pub use display::persist::set_runtime_config_path;
pub use display::notifier::set_event_path;
pub use display::hotplug::watch_hotplug;
pub use display::notifier::notify_kernel_a;
pub use display::debounce::Debouncer;

/// Full detect → plan → apply → validate → persist cycle.
/// Returns the applied and validated DisplayConfig.
pub fn detect_and_apply() -> Result<DisplayConfig, DisplayError> {
    let screens = detect()?;
    if screens.is_empty() {
        return Err(DisplayError::NoScreensDetected);
    }

    let user_config = load_user_config().unwrap_or_default();
    let planned = plan(&screens, &user_config)?;

    let _last_good = display::rollback::store_last_known_good()?;
    apply(&planned)?;
    let actual = detect()?;
    validate(&planned, &actual)?;

    persist_runtime(&planned)?;
    notify_kernel_a("display.ready", &planned)?;

    Ok(planned)
}
