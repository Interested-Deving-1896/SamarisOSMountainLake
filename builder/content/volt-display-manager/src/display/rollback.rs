use crate::display::types::*;
use crate::display::{detector, applier, validator, persist, notifier};
use tracing::{info, warn, error};

fn last_good_path() -> String {
    if let Ok(path) = std::env::var("VOLT_DISPLAY_LASTGOOD_PATH") {
        return path;
    }
    if let Ok(runtime) = std::env::var("XDG_RUNTIME_DIR") {
        return format!("{runtime}/samaris/display.lastgood.toml");
    }
    if let Some(home) = dirs::home_dir() {
        return format!("{}/.local/state/samaris/display.lastgood.toml", home.display());
    }
    "/tmp/samaris/display.lastgood.toml".to_string()
}

/// Store the current display state as last known good before applying changes.
pub fn store_last_known_good() -> Result<DisplayConfig, DisplayError> {
    let screens = detector::detect()?;
    // Build a simple safe config from current reality
    let primary = screens.iter().find(|s| s.primary).or_else(|| screens.first())
        .ok_or(DisplayError::NoScreensDetected)?;

    let snapshot = DisplayConfig {
        primary: primary.name.clone(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        backend: DisplayBackend::Xrandr,
        session: DisplaySession::Xorg,
        layout_mode: LayoutMode::Single,
        safe_mode: false,
        screens: screens.clone(),
        profile: DisplayProfile::default(),
    };

    let toml_str = toml::to_string_pretty(&snapshot)
        .map_err(|e| DisplayError::ConfigWriteFailed(e.to_string()))?;

    let last_good_path = last_good_path();
    if let Some(parent) = std::path::Path::new(&last_good_path).parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(&last_good_path, toml_str)
        .map_err(|e| DisplayError::ConfigWriteFailed(format!("Failed to write lastgood: {e}")))?;

    info!("Stored last known good display state");
    Ok(snapshot)
}

/// Restore the last known good configuration.
pub fn rollback() -> Result<(), DisplayError> {
    warn!("ROLLBACK initiated — restoring last known good display config");

    let last_good_path = last_good_path();
    let toml_str = std::fs::read_to_string(&last_good_path)
        .map_err(|e| DisplayError::ConfigReadFailed(format!("No lastgood file: {e}")))?;

    let config: DisplayConfig = toml::from_str(&toml_str)
        .map_err(|e| DisplayError::ConfigReadFailed(format!("Failed to parse lastgood: {e}")))?;

    match applier::apply(&config) {
        Ok(()) => {
            let actual = detector::detect()?;
            if let Err(e) = validator::validate(&config, &actual) {
                warn!("Rollback validation failed: {e}. Entering safe mode.");
                return safe_mode();
            }
            persist::persist_runtime(&config)?;
            info!("Rollback successful — restored {}-screen config", config.screens.len());
            Ok(())
        }
        Err(e) => {
            error!("Rollback apply failed: {e}. Entering safe mode.");
            safe_mode()
        }
    }
}

/// Activate safe mode — minimal fallback configuration on the first detected display.
/// Tries available modes sorted by resolution (largest first) and falls back through
/// known-safe resolutions if the first available mode is not supported.
pub fn safe_mode() -> Result<(), DisplayError> {
    warn!("SAFE MODE activated — using fallback display configuration");

    let screens = detector::detect().unwrap_or_default();
    if screens.is_empty() {
        error!("No screens detected even in safe mode. Cannot recover.");
        return Err(DisplayError::RollbackFailed("Safe mode: zero screens detected".into()));
    }

    let first = &screens[0];
    let known_safe: Vec<(u32, u32)> = vec![
        (1920, 1080),
        (1600, 1200),
        (1440, 900),
        (1280, 1024),
        (1280, 720),
        (1024, 768),
        (800, 600),
        (640, 480),
    ];

    let mut candidates: Vec<(u32, u32, f32)> = Vec::new();

    // Add available modes from xrandr
    for &(w, h) in &first.available_modes {
        if w > 0 && h > 0 && !candidates.iter().any(|c| c.0 == w && c.1 == h) {
            let rr = known_known_refresh(&first, w, h);
            candidates.push((w, h, rr));
        }
    }

    // Add current and native resolutions
    if first.width > 0 && first.height > 0
        && !candidates.iter().any(|c| c.0 == first.width && c.1 == first.height)
    {
        candidates.push((first.width, first.height, first.refresh_rate));
    }
    if first.native_width > 0 && first.native_height > 0
        && !candidates.iter().any(|c| c.0 == first.native_width && c.1 == first.native_height)
    {
        candidates.push((first.native_width, first.native_height, first.refresh_rate));
    }

    // Fallback: add known-safe resolutions not already present
    for &(w, h) in &known_safe {
        if !candidates.iter().any(|c| c.0 == w && c.1 == h) {
            candidates.push((w, h, 60.0));
        }
    }

    // Sort descending by total pixels
    candidates.sort_by(|a, b| (b.0 * b.1).cmp(&(a.0 * a.1)));

    let mut last_err = None;
    for &(safe_width, safe_height, safe_refresh) in &candidates {
        let safe_config = DisplayConfig {
            primary: first.name.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            backend: DisplayBackend::Xrandr,
            session: DisplaySession::Xorg,
            layout_mode: LayoutMode::Single,
            safe_mode: true,
            screens: vec![Screen {
                name: first.name.clone(),
                vendor: None,
                model: None,
                width: safe_width,
                height: safe_height,
                native_width: first.native_width,
                native_height: first.native_height,
                refresh_rate: safe_refresh,
                available_refresh_rates: first.available_refresh_rates.clone(),
                available_modes: first.available_modes.clone(),
                scale_factor: 1.0,
                rotation: Rotation::Normal,
                connected: true,
                primary: true,
                x: 0,
                y: 0,
                dpi_class: DpiClass::Standard,
            }],
            profile: DisplayProfile {
                scale_factor: 1.0,
                dpi_class: DpiClass::Standard,
                recommended_ui_density: 1.0,
                recommended_font_scale: 1.0,
                recommended_spacing_scale: 1.0,
                primary_logical_width: safe_width,
                primary_logical_height: safe_height,
                safe_mode: true,
            },
        };

        match applier::apply(&safe_config) {
            Ok(()) => {
                persist::persist_runtime(&safe_config)?;
                let _ = notifier::notify_kernel_a("display.safe_mode", &safe_config);
                info!("Safe mode active — 1 screen @ {}x{}", safe_width, safe_height);
                return Ok(());
            }
            Err(e) => {
                warn!("Safe mode candidate {}x{} failed: {e}", safe_width, safe_height);
                last_err = Some(e);
            }
        }
    }

    let msg = last_err
        .map(|e| format!("Safe mode apply failed — no supported resolution: {e}"))
        .unwrap_or_else(|| "Safe mode apply failed — no supported resolution".into());
    error!("{msg}");
    Err(DisplayError::RollbackFailed(msg))
}

/// Find the best matching refresh rate for a given resolution from available modes.
fn known_known_refresh(screen: &Screen, w: u32, h: u32) -> f32 {
    if screen.width == w && screen.height == h && screen.refresh_rate > 0.0 {
        return screen.refresh_rate;
    }
    screen.available_refresh_rates.first().copied().unwrap_or(60.0)
}
