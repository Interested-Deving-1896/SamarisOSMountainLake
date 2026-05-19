use serde_json::{json, Value};
use crate::collectors::{Collector, CollectorResult};
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;

pub struct NetworkCollector;

impl Collector for NetworkCollector {
    fn name(&self) -> &'static str { "network" }

    fn collect(&self, _hardware: &HardwareInfo, _env: &EnvironmentInfo) -> CollectorResult {
        let net_dev = std::fs::read_to_string("/proc/net/dev").ok();
        let mut rx_total = 0.0f64;
        let mut tx_total = 0.0f64;

        if let Some(dev) = net_dev {
            for line in dev.lines().skip(2) {
                if line.trim().is_empty() { continue; }
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(iface) = parts.first().map(|s| s.trim_end_matches(':')) {
                    if iface == "lo" { continue; }
                    if parts.len() >= 10 {
                        if let Ok(rx) = parts[1].parse::<f64>() { rx_total += rx; }
                        if let Ok(tx) = parts[9].parse::<f64>() { tx_total += tx; }
                    }
                }
            }
        }

        Ok(json!({
            "rx_bytes": rx_total,
            "tx_bytes": tx_total,
            "rx_mbps": 0.0,
            "tx_mbps": 0.0,
        }))
    }
}
