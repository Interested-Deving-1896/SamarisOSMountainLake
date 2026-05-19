use serde_json::{json, Value};
use crate::collectors::{Collector, CollectorResult};
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;

pub struct CpuCollector;

impl Collector for CpuCollector {
    fn name(&self) -> &'static str { "cpu" }

    fn collect(&self, _hardware: &HardwareInfo, _env: &EnvironmentInfo) -> CollectorResult {
        let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").ok();
        let model = cpuinfo.as_ref()
            .and_then(|s| s.lines().find(|l| l.trim().starts_with("model name")))
            .and_then(|l| l.split(':').nth(1))
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        let cores = cpuinfo.as_ref()
            .map(|s| s.lines().filter(|l| l.trim().starts_with("processor")).count())
            .unwrap_or(0);
        let freq = std::fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")
            .ok()
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0);

        let stat = std::fs::read_to_string("/proc/stat").ok();
        let context_switches = stat.as_ref()
            .and_then(|s| s.lines().find(|l| l.starts_with("ctxt")))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);

        Ok(json!({
            "model": model,
            "cores": cores,
            "frequency_mhz": (freq * 10.0).round() / 10.0,
            "context_switches": context_switches,
            "has_throttled": false,
        }))
    }
}
