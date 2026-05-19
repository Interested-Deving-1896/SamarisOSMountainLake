use crate::display::backends::xrandr;
use crate::display::types::*;
use tracing::info;

/// Apply a planned DisplayConfig via the active backend.
pub fn apply(config: &DisplayConfig) -> Result<(), DisplayError> {
    info!(
        backend = ?config.backend,
        layout = ?config.layout_mode,
        screens = config.screens.len(),
        "Applying display configuration"
    );

    match config.backend {
        DisplayBackend::Xrandr => xrandr::apply_xrandr(config),
        DisplayBackend::Drm => Err(DisplayError::ApplyFailed(
            "DRM backend not yet implemented".into()
        )),
        DisplayBackend::Wayland => Err(DisplayError::ApplyFailed(
            "Wayland backend not yet implemented".into()
        )),
        DisplayBackend::Unknown => Err(DisplayError::ApplyFailed(
            "No display backend available".into()
        )),
    }
}
