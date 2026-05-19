use std::path::Path;

use crate::core::error::{Result, TesseractError};

#[derive(Debug, Clone, Default)]
pub struct ThermalMetrics {
    pub zones: Vec<(String, f64)>,
    pub max_temp: f64,
}

impl ThermalMetrics {
    pub fn collect() -> Result<Self> {
        let thermal_dir = Path::new("/sys/class/thermal");
        if !thermal_dir.exists() {
            return Err(TesseractError::System("no thermal zone available".into()));
        }

        let mut zones = Vec::new();
        let mut max_temp = 0.0f64;

        let entries = std::fs::read_dir(thermal_dir)
            .map_err(|e| TesseractError::System(format!("read thermal dir: {e}")))?;

        for entry in entries {
            let entry = entry.map_err(|e| TesseractError::System(e.to_string()))?;
            let name = entry.file_name().to_string_lossy().to_string();

            if !name.starts_with("thermal_zone") {
                continue;
            }

            let temp_path = entry.path().join("temp");
            let type_path = entry.path().join("type");

            let zone_type = if type_path.exists() {
                std::fs::read_to_string(&type_path)
                    .unwrap_or_default()
                    .trim()
                    .to_string()
            } else {
                name.clone()
            };

            if let Ok(temp_str) = std::fs::read_to_string(&temp_path) {
                if let Ok(temp_millic) = temp_str.trim().parse::<f64>() {
                    let temp_c = temp_millic / 1000.0;
                    if temp_c > max_temp {
                        max_temp = temp_c;
                    }
                    zones.push((zone_type, temp_c));
                }
            }
        }

        if zones.is_empty() {
            return Err(TesseractError::System("no thermal zone data".into()));
        }

        Ok(Self { zones, max_temp })
    }
}
