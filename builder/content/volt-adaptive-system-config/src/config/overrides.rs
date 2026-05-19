use crate::core::error::AscError;
use crate::core::result::AscResult;

pub struct ManualOverride {
    pub name: String,
    pub value: String,
}

impl ManualOverride {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

pub struct OverrideValidator;

impl OverrideValidator {
    pub fn validate_override(name: &str, value: &str, safe_mode: bool) -> AscResult<String> {
        match name {
            "cpu_cores" => Self::validate_numeric(name, value, 1, 256, safe_mode),
            "ram_total_mb" => Self::validate_numeric(name, value, 256, 1_048_576, safe_mode),
            "gpu_available" => Self::validate_boolean_or_auto(name, value, safe_mode),
            "usb_speed" => Self::validate_enum(name, value, &["usb2", "usb3_plus", "unknown", "auto"], safe_mode),
            "storage_type" => Self::validate_enum(name, value, &["usb", "hdd", "ssd", "nvme", "emmcp", "unknown", "auto"], safe_mode),
            "boot_medium" => Self::validate_enum(name, value, &["usb", "internal_disk", "network", "unknown", "auto"], safe_mode),
            "is_vm" => Self::validate_boolean_or_auto(name, value, safe_mode),
            "is_laptop" => Self::validate_boolean_or_auto(name, value, safe_mode),
            "safety_margin_percent" => Self::validate_numeric(name, value, 0, 100, safe_mode),
            "max_samaris_ram_percent" => Self::validate_numeric(name, value, 5, 95, safe_mode),
            "workers" => Self::validate_numeric(name, value, 1, 64, safe_mode),
            "min_workers" => Self::validate_numeric(name, value, 0, 64, safe_mode),
            "max_workers" => Self::validate_numeric(name, value, 1, 256, safe_mode),
            "desktop_min" => Self::validate_numeric(name, value, 0, 64, safe_mode),
            "system_min" => Self::validate_numeric(name, value, 0, 64, safe_mode),
            "orbit_default_max" => Self::validate_numeric(name, value, 0, 256, safe_mode),
            "orbit_burst_max" => Self::validate_numeric(name, value, 0, 512, safe_mode),
            "orbit_burst_window_ms" => Self::validate_numeric(name, value, 100, 60_000, safe_mode),
            "desktop_quota_mb" => Self::validate_numeric(name, value, 0, 1_048_576, safe_mode),
            "orbit_quota_mb" => Self::validate_numeric(name, value, 0, 1_048_576, safe_mode),
            "cache_mb" => Self::validate_numeric(name, value, 16, 1_048_576, safe_mode),
            "buffer_mb" => Self::validate_numeric(name, value, 8, 524_288, safe_mode),
            "flush_interval_ms" => Self::validate_numeric(name, value, 100, 300_000, safe_mode),
            "batch_size_kb" => Self::validate_numeric(name, value, 4, 65_536, safe_mode),
            "pressure_policy" => Self::validate_enum(name, value, &["balanced", "conservative", "aggressive", "auto"], safe_mode),
            "compression_level" => Self::validate_numeric(name, value, 0, 22, safe_mode),
            "journal_mode" => Self::validate_enum(name, value, &["wal", "delete", "truncate", "memory", "off", "auto"], safe_mode),
            "prefetch_boot_assets" => Self::validate_boolean_or_auto(name, value, safe_mode),
            _ => Err(AscError::InvalidOverride(format!("Unknown override '{}'", name))),
        }
    }

    fn validate_numeric(name: &str, value: &str, min: i64, max: i64, safe_mode: bool) -> AscResult<String> {
        if value == "auto" {
            return Ok(value.to_string());
        }
        match value.parse::<i64>() {
            Ok(n) if n < min => {
                let msg = format!("Value {} is below minimum {} for '{}'", n, min, name);
                if safe_mode {
                    tracing::warn!("{}; clamped to {}", msg, min);
                    Ok(min.to_string())
                } else {
                    Err(AscError::UnsafeOverride(msg))
                }
            }
            Ok(n) if n > max => {
                let msg = format!("Value {} exceeds maximum {} for '{}'", n, max, name);
                if safe_mode {
                    tracing::warn!("{}; clamped to {}", msg, max);
                    Ok(max.to_string())
                } else {
                    Err(AscError::UnsafeOverride(msg))
                }
            }
            Ok(_) => Ok(value.to_string()),
            Err(_) => {
                let msg = format!("Invalid numeric value '{}' for '{}'", value, name);
                Err(AscError::InvalidOverride(msg))
            }
        }
    }

    fn validate_boolean_or_auto(name: &str, value: &str, safe_mode: bool) -> AscResult<String> {
        match value {
            "auto" | "true" | "false" | "1" | "0" => Ok(value.to_string()),
            "yes" | "no" => {
                let mapped = if value == "yes" { "true" } else { "false" };
                if safe_mode {
                    tracing::warn!("Override '{}': '{}' mapped to '{}'", name, value, mapped);
                }
                Ok(mapped.to_string())
            }
            other => {
                let msg = format!("Invalid boolean value '{}' for '{}'", other, name);
                if safe_mode {
                    tracing::warn!("{}; falling back to 'auto'", msg);
                    Ok("auto".to_string())
                } else {
                    Err(AscError::UnsafeOverride(msg))
                }
            }
        }
    }

    fn validate_enum(name: &str, value: &str, allowed: &[&str], safe_mode: bool) -> AscResult<String> {
        if allowed.contains(&value) {
            return Ok(value.to_string());
        }
        let msg = format!(
            "Invalid value '{}' for '{}'; allowed: {:?}",
            value, name, allowed
        );
        if safe_mode {
            tracing::warn!("{}; falling back to 'auto'", msg);
            Ok("auto".to_string())
        } else {
            Err(AscError::UnsafeOverride(msg))
        }
    }
}
