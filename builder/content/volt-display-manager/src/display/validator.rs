use crate::display::types::*;
use tracing::{info, warn};

/// Validate that the actual display state matches the planned config.
///
/// Returns Ok(()) if all screens match. Returns a Vec of mismatch descriptions
/// wrapped in ValidationFailed if any screen does not match.
pub fn validate(planned: &DisplayConfig, actual: &[Screen]) -> Result<(), DisplayError> {
    let mut mismatches: Vec<String> = Vec::new();

    for ps in &planned.screens {
        if !ps.connected {
            // Disconnected screens — verify they are absent
            if let Some(_as) = actual.iter().find(|s| s.name == ps.name && s.connected) {
                mismatches.push(format!("Screen {} should be disabled but is active", ps.name));
            }
            continue;
        }

        let Some(as_) = actual.iter().find(|s| s.name == ps.name) else {
            mismatches.push(format!("Screen {} missing from actual state", ps.name));
            continue;
        };

        if as_.width != ps.width || as_.height != ps.height {
            mismatches.push(format!(
                "Screen {} resolution mismatch: planned {}x{}, actual {}x{}",
                ps.name, ps.width, ps.height, as_.width, as_.height
            ));
        }
        if (as_.refresh_rate - ps.refresh_rate).abs() > 0.5 {
            mismatches.push(format!(
                "Screen {} refresh mismatch: planned {}Hz, actual {}Hz",
                ps.name, ps.refresh_rate, as_.refresh_rate
            ));
        }
        if as_.x != ps.x || as_.y != ps.y {
            mismatches.push(format!(
                "Screen {} position mismatch: planned ({},{}), actual ({},{})",
                ps.name, ps.x, ps.y, as_.x, as_.y
            ));
        }
    }

    if mismatches.is_empty() {
        info!("Display validation passed — {} screen(s) confirmed", planned.screens.len());
        Ok(())
    } else {
        warn!("Display validation FAILED: {:?}", mismatches);
        Err(DisplayError::ValidationFailed(mismatches.join("; ")))
    }
}
