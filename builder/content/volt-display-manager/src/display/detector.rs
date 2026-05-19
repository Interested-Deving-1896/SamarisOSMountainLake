use crate::display::backends::xrandr;
use crate::display::types::*;
use tracing::{info, warn};

/// Detect all connected screens via the xrandr backend.
///
/// Fallback chain:
///   1. xrandr --query (Xorg)
///   2. Future: DRM/KMS direct probe
///   3. Future: Wayland wlr-randr
pub fn detect() -> Result<Vec<Screen>, DisplayError> {
    let session = detect_session();

    match session {
        DisplaySession::Xorg => {
            info!("Detecting displays via xrandr (Xorg session)");
            xrandr::query_xrandr()
        }
        DisplaySession::Wayland => {
            // Wayland backend not yet implemented.
            // We fall through to a minimal stub that returns a single
            // virtual screen so the user isn't left blind.
            warn!("Wayland detected — xrandr unavailable. Using virtual fallback screen.");
            Ok(vec![virtual_fallback_screen()])
        }
        DisplaySession::Unknown => {
            warn!("No known display session detected. Attempting xrandr anyway...");
            match xrandr::query_xrandr() {
                Ok(screens) => Ok(screens),
                Err(_) => {
                    warn!("xrandr failed — returning virtual fallback screen");
                    Ok(vec![virtual_fallback_screen()])
                }
            }
        }
    }
}

/// Detect the current display session type.
fn detect_session() -> DisplaySession {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        return DisplaySession::Wayland;
    }
    if std::env::var("DISPLAY").is_ok() {
        return DisplaySession::Xorg;
    }
    DisplaySession::Unknown
}

/// Minimal virtual fallback — prevents VDM from returning zero screens.
fn virtual_fallback_screen() -> Screen {
    Screen {
        name: "FALLBACK".into(),
        vendor: Some("Virtual".into()),
        model: Some("SafeMode".into()),
        width: 1280,
        height: 720,
        native_width: 1280,
        native_height: 720,
        refresh_rate: 60.0,
        available_refresh_rates: vec![60.0],
        available_modes: vec![(1280, 720)],
        scale_factor: 1.0,
        rotation: Rotation::Normal,
        connected: true,
        primary: true,
        x: 0,
        y: 0,
        dpi_class: DpiClass::Standard,
    }
}
