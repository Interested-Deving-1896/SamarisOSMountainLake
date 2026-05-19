use crate::display::types::*;
use tracing::{info, debug};

/// Plan the optimal display layout from detected screens and user preferences.
pub fn plan(
    screens: &[Screen],
    user_config: &UserDisplayConfig,
) -> Result<DisplayConfig, DisplayError> {
    let connected: Vec<&Screen> = screens.iter().filter(|s| s.connected).collect();

    if connected.is_empty() {
        return Err(DisplayError::NoScreensDetected);
    }

    let layout_mode = user_config.preferences.layout_mode.unwrap_or_else(|| {
        if connected.len() == 1 { LayoutMode::Single } else { LayoutMode::Extend }
    });

    let primary_name = select_primary(&connected, &user_config.preferences);

    let mut planned_screens: Vec<Screen> = Vec::new();

    match layout_mode {
        LayoutMode::Single | LayoutMode::Auto => {
            let found = connected.iter().find(|s| s.name == primary_name)
                .unwrap_or(&connected[0]);
            let mut s = (*found).clone();
            s.primary = true;
            s.x = 0;
            s.y = 0;
            // Use native resolution
            s.width = s.native_width;
            s.height = s.native_height;
            s.refresh_rate = s.available_refresh_rates.last().copied().unwrap_or(60.0);
            apply_user_overrides(&mut s, &user_config.preferences);
            planned_screens.push(s);

            // Disable other screens
            for other in &connected {
                if other.name != primary_name {
                    let mut dis = (*other).clone();
                    dis.connected = false;
                    dis.primary = false;
                    planned_screens.push(dis);
                }
            }
        }
        LayoutMode::Extend => {
            let mut x_offset: i32 = 0;
            for screen in &connected {
                let mut s = (*screen).clone();
                s.primary = s.name == primary_name;
                s.width = s.native_width;
                s.height = s.native_height;
                s.refresh_rate = s.available_refresh_rates.last().copied().unwrap_or(60.0);
                s.x = x_offset;
                s.y = 0;
                apply_user_overrides(&mut s, &user_config.preferences);
                x_offset += s.width as i32;
                planned_screens.push(s);
            }
        }
        LayoutMode::Mirror => {
            let primary = connected.iter().find(|s| s.name == primary_name)
                .unwrap_or(&connected[0]);
            for screen in &connected {
                let mut s = (*primary).clone();
                s.name = screen.name.clone();
                s.primary = s.name == primary_name;
                s.width = s.native_width;
                s.height = s.native_height;
                s.x = 0;
                s.y = 0;
                apply_user_overrides(&mut s, &user_config.preferences);
                planned_screens.push(s);
            }
        }
    }

    let profile = generate_profile(&planned_screens);

    Ok(DisplayConfig {
        primary: primary_name,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        backend: DisplayBackend::Xrandr,
        session: DisplaySession::Xorg,
        layout_mode,
        safe_mode: false,
        screens: planned_screens,
        profile,
    })
}

fn select_primary(connected: &[&Screen], prefs: &UserPreferences) -> String {
    if let Some(ref name) = prefs.primary {
        if connected.iter().any(|s| s.name == *name) {
            debug!("Using user-preferred primary: {name}");
            return name.clone();
        }
    }

    // Priority: eDP/laptop > HDMI > DP > first connected
    let priority = ["eDP", "eDP-1", "LVDS", "HDMI", "HDMI-1", "DP", "DP-1"];
    for pref in &priority {
        if let Some(s) = connected.iter().find(|s| s.name.contains(pref)) {
            info!("Auto-selected primary: {}", s.name);
            return s.name.clone();
        }
    }

    let fallback = connected[0].name.clone();
    info!("Fallback primary: {fallback}");
    fallback
}

fn apply_user_overrides(screen: &mut Screen, prefs: &UserPreferences) {
    if let Some(user_screen) = prefs.screens.get(&screen.name) {
        if let Some(sf) = user_screen.scale_factor {
            screen.scale_factor = sf;
        }
        if let Some(rot) = user_screen.rotation {
            screen.rotation = rot;
        }
        if let Some((w, h)) = user_screen.resolution_override {
            screen.width = w;
            screen.height = h;
        }
        if let Some(rr) = user_screen.refresh_rate_override {
            screen.refresh_rate = rr;
        }
    }
}

fn generate_profile(screens: &[Screen]) -> DisplayProfile {
    let primary = screens.iter().find(|s| s.primary).unwrap_or(&screens[0]);

    let scale = primary.scale_factor;
    let density = if scale >= 2.0 { 1.15 } else if scale >= 1.5 { 1.08 } else { 1.0 };
    let font_scale = if scale >= 2.0 { 1.15 } else if scale >= 1.5 { 1.05 } else { 1.0 };

    DisplayProfile {
        scale_factor: scale,
        dpi_class: primary.dpi_class,
        recommended_ui_density: density,
        recommended_font_scale: font_scale,
        recommended_spacing_scale: scale,
        primary_logical_width: (primary.width as f32 / scale) as u32,
        primary_logical_height: (primary.height as f32 / scale) as u32,
        safe_mode: false,
    }
}
