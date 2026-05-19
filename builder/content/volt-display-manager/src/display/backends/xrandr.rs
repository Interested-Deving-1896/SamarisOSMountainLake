// xrandr backend — Alpha implementation for X11 display management.
//
// This module:
//   - Parses `xrandr --query` output into Screen structs
//   - Builds xrandr command-line arguments from DisplayConfig
//   - Handles all xrandr-specific error cases gracefully
//   - Uses spawn + wait_timeout to prevent hangs (ex: headless QEMU)
//
// Requires: active Xorg session, DISPLAY env var, XAUTHORITY access.

use std::process::Command;
use std::time::Duration;
use regex::Regex;
use crate::display::types::*;

/// Maximum seconds to wait for an xrandr command before timing out.
const XRANDR_TIMEOUT_SECS: u64 = 8;

/// Parse `xrandr --query` output into a list of detected screens.
pub fn query_xrandr() -> Result<Vec<Screen>, DisplayError> {
    let output = run_xrandr(&["--query"])?;
    parse_xrandr_output(&output)
}

/// Apply a DisplayConfig via xrandr command sequence.
pub fn apply_xrandr(config: &DisplayConfig) -> Result<(), DisplayError> {
    // Apply primary first
    for screen in &config.screens {
        if screen.primary && screen.connected {
            let resolution = format!("{}x{}", screen.width, screen.height);
            let rate = format!("{:.2}", screen.refresh_rate);
            let pos = format!("{}x{}", screen.x, screen.y);

            let args = vec![
                "--output", &screen.name,
                "--mode", &resolution,
                "--rate", &rate,
                "--pos", &pos,
                "--rotate", screen.rotation.xrandr_arg(),
            ];

            run_xrandr(&args.iter().map(|s| *s).collect::<Vec<_>>())?;
        }
    }

    // Set primary explicitly
    if let Some(primary) = config.screens.iter().find(|s| s.primary && s.connected) {
        run_xrandr(&["--output", &primary.name, "--primary"])?;
    }

    // Disable disconnected screens that were previously active
    for screen in &config.screens {
        if !screen.connected {
            run_xrandr(&["--output", &screen.name, "--off"])?;
        }
    }

    // Wait for X11 to stabilize
    std::thread::sleep(std::time::Duration::from_millis(200));

    Ok(())
}

/// Run xrandr with given arguments and return stdout.
/// Uses a channel-based timeout to prevent hanging in headless environments.
fn run_xrandr(args: &[&str]) -> Result<String, DisplayError> {
    run_xrandr_with_timeout(args, XRANDR_TIMEOUT_SECS)
}

/// Run xrandr with a custom timeout (used by callers that need longer waits).
fn run_xrandr_with_timeout(args: &[&str], timeout_secs: u64) -> Result<String, DisplayError> {
    let display = std::env::var("DISPLAY").map_err(|_| {
        DisplayError::SessionUnavailable("DISPLAY environment variable not set".into())
    })?;

    let owned_args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let result = (|| -> Result<String, DisplayError> {
            let output = Command::new("xrandr")
                .args(&owned_args)
                .env("DISPLAY", &display)
                .output()
                .map_err(|e| {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        DisplayError::XrandrNotFound
                    } else {
                        DisplayError::ApplyFailed(format!("xrandr spawn failed: {e}"))
                    }
                })?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let msg = stderr.trim().to_string();

                if msg.contains("Can't open display") {
                    return Err(DisplayError::SessionUnavailable(msg));
                }
                if msg.contains("Permission denied") || msg.contains("No protocol specified") {
                    return Err(DisplayError::PermissionDenied(msg));
                }
                return Err(DisplayError::ApplyFailed(msg));
            }

            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        })();
        tx.send(result).ok();
    });

    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(result) => result,
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => Err(DisplayError::ApplyFailed(
            format!("xrandr timed out after {}s", timeout_secs),
        )),
        Err(_) => Err(DisplayError::ApplyFailed("xrandr channel disconnected".into())),
    }
}

/// Parse xrandr query output line by line.
fn parse_xrandr_output(raw: &str) -> Result<Vec<Screen>, DisplayError> {
    let screen_re = Regex::new(
        r"^(\S+)\s+(connected|disconnected)(?:\s+primary)?\s*(?:(\d+)x(\d+)\+(\d+)\+(\d+))?\s*(.*)"
    ).map_err(|e| DisplayError::ParseError(e.to_string()))?;

    let resolution_re = Regex::new(
        r"^\s+(\d+)x(\d+)\s+([\d.]+)([*+]?)(\s+([\d.]+))?"
    ).map_err(|e| DisplayError::ParseError(e.to_string()))?;

    let edid_re = Regex::new(
        r"EDID:\s*([a-fA-F0-9\s]+)"
    ).map_err(|e| DisplayError::ParseError(e.to_string()))?;

    let mut screens: Vec<Screen> = Vec::new();
    let mut current: Option<Screen> = None;
    let mut in_screen_block = false;
    let mut screen_modes: Vec<Vec<(u32, u32)>> = Vec::new();

    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            if let Some(s) = current.take() {
                screens.push(s);
            }
            in_screen_block = false;
            continue;
        }

        // Screen header line: "HDMI-1 connected primary 1920x1080+0+0 ..."
        if let Some(caps) = screen_re.captures(line) {
            if let Some(s) = current.take() {
                screen_modes.push(extract_available_modes(&s));
                screens.push(s);
            }

            let name = caps.get(1).unwrap().as_str().to_string();
            let connected = caps.get(2).unwrap().as_str() == "connected";
            let is_primary = line.contains("primary");

            let (width, height, x, y) = if connected {
                (
                    caps.get(3).and_then(|m| m.as_str().parse().ok()).unwrap_or(0),
                    caps.get(4).and_then(|m| m.as_str().parse().ok()).unwrap_or(0),
                    caps.get(5).and_then(|m| m.as_str().parse().ok()).unwrap_or(0),
                    caps.get(6).and_then(|m| m.as_str().parse().ok()).unwrap_or(0),
                )
            } else {
                (0, 0, 0, 0)
            };

            current = Some(Screen {
                name,
                vendor: None,
                model: None,
                width,
                height,
                native_width: 0,
                native_height: 0,
                refresh_rate: 60.0,
                available_refresh_rates: Vec::new(),
                available_modes: Vec::new(),
                scale_factor: 1.0,
                rotation: Rotation::Normal,
                connected,
                primary: is_primary,
                x,
                y,
                dpi_class: DpiClass::Standard,
            });
            in_screen_block = true;
            continue;
        }

        // Resolution line: "   1920x1080     60.00*+  59.94   50.00"
        if in_screen_block {
            if let Some(caps) = resolution_re.captures(line) {
                let w: u32 = caps.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
                let h: u32 = caps.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
                let rate: f32 = caps.get(3).and_then(|m| m.as_str().parse().ok()).unwrap_or(60.0);
                let preferred = caps.get(4).map(|m| m.as_str().contains('+')).unwrap_or(false);
                let current_mode = caps.get(4).map(|m| m.as_str().contains('*')).unwrap_or(false);

                if let Some(ref mut s) = current {
                    if preferred || s.native_width == 0 {
                        s.native_width = w;
                        s.native_height = h;
                    }
                    if current_mode {
                        s.width = w;
                        s.height = h;
                        s.refresh_rate = rate;
                    }
                    s.available_refresh_rates.push(rate);
                    if !s.available_modes.iter().any(|m| m.0 == w && m.1 == h) {
                        s.available_modes.push((w, h));
                    }
                }
            }

            // EDID line (prop output)
            if let Some(_caps) = edid_re.captures(line) {
                // EDID hex blob — decode vendor/model from bytes 8-17
                // Simplified: we skip full EDID decoding for Alpha
            }
        }
    }

    if let Some(s) = current {
        screen_modes.push(extract_available_modes(&s));
        screens.push(s);
    }

    if screens.is_empty() {
        return Err(DisplayError::NoScreensDetected);
    }

    // Post-process: classify DPI, compute scale factors, assign available modes
    for (i, s) in screens.iter_mut().enumerate() {
        if s.native_width == 0 {
            s.native_width = s.width;
            s.native_height = s.height;
        }
        s.dpi_class = classify_dpi(s.native_width, s.native_height);
        s.scale_factor = scale_for_class(s.dpi_class);
        s.available_refresh_rates.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        s.available_refresh_rates.dedup_by(|a, b| (*a - *b).abs() < 0.1);
        if let Some(modes) = screen_modes.get(i) {
            s.available_modes.clone_from(modes);
        }
    }

    Ok(screens)
}

/// Extract available modes from xrandr output stored in a Screen.
/// Parses resolution lines from the raw xrandr query output associated with this screen.
fn extract_available_modes(screen: &Screen) -> Vec<(u32, u32)> {
    screen.available_modes.clone()
}

fn classify_dpi(w: u32, h: u32) -> DpiClass {
    let max_dim = w.max(h);
    if max_dim >= 3840 { DpiClass::Retina }
    else if max_dim >= 2560 { DpiClass::Hidpi }
    else if max_dim >= 1920 { DpiClass::Standard }
    else { DpiClass::Low }
}

fn scale_for_class(cls: DpiClass) -> f32 {
    match cls {
        DpiClass::Retina => 2.0,
        DpiClass::Hidpi => 1.5,
        DpiClass::Standard => 1.0,
        DpiClass::Low => 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_hdmi() {
        let output = r#"
Screen 0: minimum 8 x 8, current 1920 x 1080, maximum 32767 x 32767
HDMI-1 connected primary 1920x1080+0+0 (normal left inverted right x axis y axis) 527mm x 296mm
   1920x1080     60.00*+  59.94    50.00    29.97    25.00    23.98
   1680x1050     59.88
   1280x1024     75.02    60.02
   1280x720      60.00    59.94
   1024x768      75.03    70.07    60.00
   800x600       75.00    72.19    60.32
   640x480       75.00    72.81    59.94
DP-1 disconnected (normal left inverted right x axis y axis)
"#;
        let screens = parse_xrandr_output(output).unwrap();
        assert_eq!(screens.len(), 2);
        let hdmi = &screens[0];
        assert_eq!(hdmi.name, "HDMI-1");
        assert!(hdmi.connected);
        assert!(hdmi.primary);
        assert_eq!(hdmi.width, 1920);
        assert_eq!(hdmi.height, 1080);
        assert_eq!(hdmi.native_width, 1920);
        assert_eq!(hdmi.refresh_rate, 60.0);

        let dp = &screens[1];
        assert_eq!(dp.name, "DP-1");
        assert!(!dp.connected);
    }

    #[test]
    fn test_parse_empty() {
        let output = "Screen 0: minimum 8 x 8, current 8 x 8, maximum 32767 x 32767\n";
        let result = parse_xrandr_output(output);
        assert!(result.is_err());
    }
}
