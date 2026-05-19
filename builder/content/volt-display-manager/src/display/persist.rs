use crate::display::types::*;
use std::sync::OnceLock;
use tracing::{info, debug};

static RUNTIME_CONFIG_PATH: OnceLock<String> = OnceLock::new();

/// Set the runtime config path (default varies by context).
pub fn set_runtime_config_path(path: String) {
    RUNTIME_CONFIG_PATH.set(path).ok();
}

fn runtime_path() -> &'static str {
    RUNTIME_CONFIG_PATH.get().map(|s| s.as_str()).unwrap_or("/run/samaris/display.generated.toml")
}

fn fallback_runtime_path() -> String {
    if let Ok(runtime) = std::env::var("XDG_RUNTIME_DIR") {
        return format!("{runtime}/samaris/display.generated.toml");
    }
    if let Some(home) = dirs::home_dir() {
        return format!("{}/.local/state/samaris/display.generated.toml", home.display());
    }
    "/tmp/samaris/display.generated.toml".to_string()
}

/// Shared helper: write a file, creating parent dirs.
fn write_file(path: &str, content: &str) -> Result<(), DisplayError> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(path, content)
        .map_err(|e| DisplayError::ConfigWriteFailed(format!("Write to {path} failed: {e}")))?;
    Ok(())
}

const USER_CONFIG_PATH: &str = ".config/samaris/display.toml";

pub fn persist_runtime(config: &DisplayConfig) -> Result<(), DisplayError> {
    let toml_str = toml::to_string_pretty(config)
        .map_err(|e| DisplayError::ConfigWriteFailed(e.to_string()))?;

    let p = runtime_path();
    write_file(p, &toml_str)?;
    info!("Runtime config persisted to {p}");

    // Dev compatibility: also write to a user-writable fallback if primary path is different
    let fallback = fallback_runtime_path();
    if fallback != p {
        write_file(&fallback, &toml_str).ok();
        debug!("Also wrote runtime config to {fallback}");
    }

    Ok(())
}

pub fn persist_runtime_at(path: &str, config: &DisplayConfig) -> Result<(), DisplayError> {
    let toml_str = toml::to_string_pretty(config)
        .map_err(|e| DisplayError::ConfigWriteFailed(e.to_string()))?;
    write_file(path, &toml_str)
}

pub fn persist_user_config(config: &UserDisplayConfig) -> Result<(), DisplayError> {
    let user_path = dirs::home_dir()
        .ok_or_else(|| DisplayError::ConfigWriteFailed("Cannot resolve home directory".into()))?
        .join(USER_CONFIG_PATH);
    let toml_str = toml::to_string_pretty(config)
        .map_err(|e| DisplayError::ConfigWriteFailed(e.to_string()))?;
    write_file(user_path.to_str().unwrap_or("/dev/null"), &toml_str)
}

pub fn load_runtime() -> Result<DisplayConfig, DisplayError> {
    let p = runtime_path();
    let toml_str = std::fs::read_to_string(p)
        .map_err(|e| DisplayError::ConfigReadFailed(format!("Runtime config not found at {p}: {e}")))?;
    toml::from_str(&toml_str)
        .map_err(|e| DisplayError::ConfigReadFailed(format!("Failed to parse runtime config: {e}")))
}

pub fn load_user_config() -> Result<UserDisplayConfig, DisplayError> {
    let user_path = dirs::home_dir()
        .ok_or_else(|| DisplayError::ConfigReadFailed("Cannot resolve home directory".into()))?
        .join(USER_CONFIG_PATH);
    if !user_path.exists() {
        return Ok(UserDisplayConfig::default());
    }
    let toml_str = std::fs::read_to_string(&user_path)
        .map_err(|e| DisplayError::ConfigReadFailed(format!("User config read failed: {e}")))?;
    toml::from_str(&toml_str)
        .map_err(|e| DisplayError::ConfigReadFailed(format!("Failed to parse user config: {e}")))
}

pub fn runtime_config_path() -> &'static str {
    runtime_path()
}
