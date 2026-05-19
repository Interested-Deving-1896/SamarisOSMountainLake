use serde_json::{json, Value};
use crate::collectors::{Collector, CollectorResult};
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;

pub struct SystemCollector;

impl Collector for SystemCollector {
    fn name(&self) -> &'static str { "system" }

    fn collect(&self, _hardware: &HardwareInfo, _env: &EnvironmentInfo) -> CollectorResult {
        let meminfo = std::fs::read_to_string("/proc/meminfo").ok();
        let stat = std::fs::read_to_string("/proc/stat").ok();
        let process_count = std::fs::read_dir("/proc")
            .map(|d| d.filter_map(|e| e.ok()).filter(|e| e.file_name().to_str().map(|s| s.bytes().all(|b| b.is_ascii_digit())).unwrap_or(false)).count())
            .unwrap_or(0);

        let ram_total = meminfo.as_ref()
            .and_then(|s| s.lines().find(|l| l.starts_with("MemTotal:")))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let ram_available = meminfo.as_ref()
            .and_then(|s| s.lines().find(|l| l.starts_with("MemAvailable:")))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);

        let cpu_user = stat.as_ref()
            .and_then(|s| s.lines().find(|l| l.starts_with("cpu ")))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let cpu_idle = stat.as_ref()
            .and_then(|s| s.lines().find(|l| l.starts_with("cpu ")))
            .and_then(|l| l.split_whitespace().nth(4))
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let cpu_total = cpu_user + cpu_idle;
        let cpu_idle_pct = if cpu_total > 0.0 { (cpu_idle / cpu_total) * 100.0 } else { 0.0 };
        let ram_idle_pct = if ram_total > 0.0 { (ram_available / ram_total) * 100.0 } else { 0.0 };
        let ram_used_mb = (ram_total - ram_available) / 1024.0;

        Ok(json!({
            "ram_total_kb": ram_total,
            "ram_available_kb": ram_available,
            "ram_idle_percent": (ram_idle_pct * 10.0).round() / 10.0,
            "ram_used_mb": (ram_used_mb * 10.0).round() / 10.0,
            "cpu_idle_percent": (cpu_idle_pct * 10.0).round() / 10.0,
            "cpu_user": cpu_user,
            "process_count": process_count,
            "disk_read_iops": 0,
            "disk_write_iops": 0,
            "disk_read_mb_s": 0.0,
            "disk_write_mb_s": 0.0,
            "network_rx_mbps": 0.0,
            "network_tx_mbps": 0.0,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::HardwareInfo;
    use crate::environment::EnvironmentInfo;

    #[test]
    fn test_system_collector_does_not_crash() {
        let collector = SystemCollector;
        let hw = HardwareInfo::detect();
        let env = EnvironmentInfo::detect();
        let result = collector.collect(&hw, &env);
        assert!(result.is_ok());
    }
}
