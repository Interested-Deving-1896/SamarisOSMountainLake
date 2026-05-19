use serde_json::{json, Value};
use crate::collectors::{Collector, CollectorResult};
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;

pub struct MemoryCollector;

impl Collector for MemoryCollector {
    fn name(&self) -> &'static str { "memory" }

    fn collect(&self, _hardware: &HardwareInfo, _env: &EnvironmentInfo) -> CollectorResult {
        let pressure = std::fs::read_to_string("/proc/pressure/memory").ok();
        let some_avg = pressure.as_ref()
            .and_then(|s| s.lines().find(|l| l.starts_with("some")))
            .and_then(|l| {
                l.split_whitespace()
                    .find(|w| w.starts_with("avg"))
                    .and_then(|a| a.split('=').nth(1))
                    .and_then(|v| v.split('/').next())
                    .and_then(|v| v.parse::<f64>().ok())
            })
            .unwrap_or(0.0);

        let swap_total = std::fs::read_to_string("/proc/swaps")
            .ok()
            .map(|s| s.lines().skip(1).filter_map(|l| l.split_whitespace().nth(2)).filter_map(|v| v.parse::<f64>().ok()).sum::<f64>())
            .unwrap_or(0.0);
        let swap_used = std::fs::read_to_string("/proc/swaps")
            .ok()
            .map(|s| s.lines().skip(1).filter_map(|l| l.split_whitespace().nth(3)).filter_map(|v| v.parse::<f64>().ok()).sum::<f64>())
            .unwrap_or(0.0);

        let pressure_zone = if some_avg > 10.0 { 3 } else if some_avg > 5.0 { 2 } else if some_avg > 1.0 { 1 } else { 0 };

        Ok(json!({
            "pressure_some_avg10": (some_avg * 100.0).round() / 100.0,
            "swap_total_kb": swap_total,
            "swap_used_kb": swap_used,
            "swap_percent": if swap_total > 0.0 { (swap_used / swap_total * 100.0 * 10.0).round() / 10.0 } else { 0.0 },
            "pressure_zone": pressure_zone,
            "swap_avoidance_score": (100.0 - some_avg * 5.0).clamp(0.0, 100.0),
        }))
    }
}
