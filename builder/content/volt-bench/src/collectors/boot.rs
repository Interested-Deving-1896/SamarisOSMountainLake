use serde_json::{json, Value};
use crate::collectors::{Collector, CollectorResult};
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;

pub struct BootCollector;

impl Collector for BootCollector {
    fn name(&self) -> &'static str { "boot" }

    fn collect(&self, _hardware: &HardwareInfo, _env: &EnvironmentInfo) -> CollectorResult {
        let (firmware, kernel_time, initrd, userspace) = parse_systemd_analyze();
        Ok(json!({
            "firmware_seconds": firmware,
            "kernel_seconds": kernel_time,
            "initrd_seconds": initrd,
            "userspace_seconds": userspace,
            "boot_time_seconds": firmware + kernel_time + initrd + userspace,
            "source": if firmware > 0.0 { "systemd-analyze" } else { "uptime_fallback" },
        }))
    }
}

fn parse_systemd_analyze() -> (f64, f64, f64, f64) {
    let output = match std::process::Command::new("systemd-analyze")
        .arg("time")
        .output()
    {
        Ok(o) => match String::from_utf8(o.stdout) {
            Ok(s) => s,
            Err(_) => return (0.0, 0.0, 0.0, 0.0),
        },
        Err(_) => return (0.0, 0.0, 0.0, 0.0),
    };

    let parts: Vec<&str> = output.split_whitespace().collect();
    let mut firmware = 0.0f64;
    let mut kernel = 0.0f64;
    let mut initrd = 0.0f64;
    let mut userspace = 0.0f64;

    for (i, part) in parts.iter().enumerate() {
        if let Ok(v) = part.trim_end_matches('s').parse::<f64>() {
            if i + 1 < parts.len() {
                match parts[i + 1] {
                    s if s.starts_with("(firmware)") => firmware = v,
                    s if s.starts_with("(loader)") | s.starts_with("(initrd)") => initrd = v,
                    s if s.starts_with("(kernel)") => kernel = v,
                    s if s.starts_with("(userspace)") => userspace = v,
                    _ => {}
                }
            }
        }
    }

    (firmware, kernel, initrd, userspace)
}
