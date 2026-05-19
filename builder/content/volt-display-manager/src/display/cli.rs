/// CLI diagnostic and dump utilities.
use crate::display::types::*;
use crate::display::backends::xrandr;

/// Print a human-readable status of all detected screens.
pub fn print_status() -> Result<(), DisplayError> {
    let screens = crate::display::detector::detect()?;

    println!("VOLT Display Manager — Status");
    println!("==============================");
    println!("Session: {:?}", display_session_info());
    println!();

    for s in &screens {
        let status = if s.connected { "CONNECTED" } else { "disconnected" };
        let primary = if s.primary { " (PRIMARY)" } else { "" };
        println!(
            "  {:12}  {:10}  {}x{} @ {:.1}Hz  {}{}",
            s.name, status, s.width, s.height, s.refresh_rate,
            s.dpi_class_display(), primary
        );
        if s.connected {
            println!(
                "            Native: {}x{}  Scale: {:.1}x  Pos: ({},{})",
                s.native_width, s.native_height, s.scale_factor, s.x, s.y
            );
        }
    }

    if screens.is_empty() {
        println!("  No screens detected.");
    }
    println!();
    Ok(())
}

/// Dump raw xrandr output and parsed screen data for debugging.
pub fn print_dump() -> Result<(), DisplayError> {
    println!("=== RAW xrandr output ===\n");
    match xrandr::query_xrandr() {
        Ok(screens) => {
            for s in &screens {
                println!("{:#?}", s);
            }
        }
        Err(e) => {
            println!("xrandr query failed: {e}");
        }
    }
    Ok(())
}

/// Dump the generated runtime config TOML.
pub fn print_runtime() -> Result<(), DisplayError> {
    match crate::display::persist::load_runtime() {
        Ok(config) => {
            let toml_str = toml::to_string_pretty(&config)
                .map_err(|e| DisplayError::ConfigWriteFailed(e.to_string()))?;
            println!("{toml_str}");
        }
        Err(e) => {
            println!("No runtime config found: {e}");
        }
    }
    Ok(())
}

/// Dump the user preferences TOML.
pub fn print_user_config() -> Result<(), DisplayError> {
    match crate::display::persist::load_user_config() {
        Ok(config) => {
            let toml_str = toml::to_string_pretty(&config)
                .map_err(|e| DisplayError::ConfigWriteFailed(e.to_string()))?;
            println!("{toml_str}");
        }
        Err(e) => {
            println!("No user config found: {e}");
        }
    }
    Ok(())
}

fn display_session_info() -> String {
    if std::env::var("WAYLAND_DISPLAY").is_ok() { return "Wayland".into(); }
    if std::env::var("DISPLAY").is_ok() { return format!("Xorg ({})", std::env::var("DISPLAY").unwrap()); }
    "Unknown".into()
}

impl Screen {
    fn dpi_class_display(&self) -> &str {
        match self.dpi_class {
            DpiClass::Low => "LowDPI",
            DpiClass::Standard => "StdDPI",
            DpiClass::Hidpi => "HiDPI",
            DpiClass::Retina => "Retina",
        }
    }
}
