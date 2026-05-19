use serde_json::{json, Value};
use crate::collectors::{Collector, CollectorResult};
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;

pub struct DiskCollector;

impl Collector for DiskCollector {
    fn name(&self) -> &'static str { "disk" }

    fn collect(&self, _hardware: &HardwareInfo, _env: &EnvironmentInfo) -> CollectorResult {
        let diskstats = std::fs::read_to_string("/proc/diskstats").ok();
        let mut total_read_sectors = 0.0f64;
        let mut total_write_sectors = 0.0f64;

        if let Some(stats) = diskstats {
            for line in stats.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 14 {
                    let name = parts[2];
                    if name.starts_with("sd") || name.starts_with("nvme") || name.starts_with("mmc") {
                        if let Ok(rs) = parts[5].parse::<f64>() { total_read_sectors += rs; }
                        if let Ok(ws) = parts[9].parse::<f64>() { total_write_sectors += ws; }
                    }
                }
            }
        }

        // Approximate MB: each sector is 512 bytes
        let read_mb = total_read_sectors * 512.0 / 1024.0 / 1024.0;
        let write_mb = total_write_sectors * 512.0 / 1024.0 / 1024.0;

        Ok(json!({
            "read_sectors": total_read_sectors,
            "write_sectors": total_write_sectors,
            "read_mb": (read_mb * 10.0).round() / 10.0,
            "write_mb": (write_mb * 10.0).round() / 10.0,
        }))
    }
}
