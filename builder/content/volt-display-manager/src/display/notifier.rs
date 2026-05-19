use crate::display::types::*;
use std::sync::OnceLock;
use tracing::{info, debug};

static NOTIFY_PATH: OnceLock<String> = OnceLock::new();

fn fallback_event_path() -> String {
    if let Ok(runtime) = std::env::var("XDG_RUNTIME_DIR") {
        return format!("{runtime}/samaris/display.event.json");
    }
    if let Some(home) = dirs::home_dir() {
        return format!("{}/.local/state/samaris/display.event.json", home.display());
    }
    "/tmp/samaris/display.event.json".to_string()
}

/// Set the event notification path (default: /run/samaris/display.event.json).
pub fn set_event_path(path: String) {
    NOTIFY_PATH.set(path).ok();
}

fn event_path() -> &'static str {
    NOTIFY_PATH.get().map(|s| s.as_str()).unwrap_or("/run/samaris/display.event.json")
}

fn write_file(path: &str, content: &str) -> Result<(), DisplayError> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(path, content)
        .map_err(|e| DisplayError::ConfigWriteFailed(format!("Write to {path} failed: {e}")))?;
    Ok(())
}

pub fn notify_kernel_a(event_type: &str, config: &DisplayConfig) -> Result<(), DisplayError> {
    #[derive(serde::Serialize)]
    struct DisplayEvent {
        r#type: String,
        timestamp: u64,
        primary: String,
        screen_count: usize,
        safe_mode: bool,
        profile: DisplayProfile,
    }

    let event = DisplayEvent {
        r#type: event_type.to_string(),
        timestamp: config.timestamp,
        primary: config.primary.clone(),
        screen_count: config.screens.iter().filter(|s| s.connected).count(),
        safe_mode: config.safe_mode,
        profile: config.profile.clone(),
    };

    let json = serde_json::to_string_pretty(&event)
        .map_err(|e| DisplayError::ConfigWriteFailed(format!("JSON serialization failed: {e}")))?;

    let p = event_path();
    write_file(p, &json)?;
    info!("Notified Kernel A: event={event_type} screens={}", event.screen_count);

    // Dev compatibility: also write to a user-writable fallback if primary path is different
    let fallback = fallback_event_path();
    if fallback != p {
        write_file(&fallback, &json).ok();
    }

    debug!("Event payload: {json}");
    Ok(())
}
