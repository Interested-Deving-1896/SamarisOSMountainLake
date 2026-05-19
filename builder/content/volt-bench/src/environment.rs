use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentInfo {
    pub display_resolution: String,
    pub display_scale: f64,
    pub network_connected: bool,
    pub running_from_usb: bool,
    pub running_in_vm: bool,
    pub power_mode: String,
    pub temperature_celsius: Option<f64>,
    pub kernel_version: String,
    pub system_uptime_seconds: f64,
}

impl EnvironmentInfo {
    pub fn detect() -> Self {
        let is_vm = Self::check_is_vm();
        let is_usb = Self::check_is_usb();
        let uptime = std::fs::read_to_string("/proc/uptime")
            .and_then(|s| s.split_whitespace().next()
                .and_then(|v| v.parse::<f64>().ok())
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "parse")))
            .unwrap_or(0.0);
        let kernel = std::fs::read_to_string("/proc/version")
            .and_then(|s| Ok(s.split_whitespace().nth(2).unwrap_or("unknown").to_string()))
            .unwrap_or_else(|_| "unknown".into());
        let temp = std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
            .and_then(|s| s.trim().parse::<f64>().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "parse")))
            .map(|t| t / 1000.0)
            .ok();

        Self {
            display_resolution: "unknown".into(),
            display_scale: 1.0,
            network_connected: true,
            running_from_usb: is_usb,
            running_in_vm: is_vm,
            power_mode: "ac".into(),
            temperature_celsius: temp,
            kernel_version: kernel,
            system_uptime_seconds: uptime,
        }
    }

    fn check_is_vm() -> bool {
        std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
            .map(|s| {
                let l = s.to_lowercase();
                l.contains("vmware") || l.contains("virtualbox") || l.contains("qemu") || l.contains("kvm")
            })
            .unwrap_or(false)
    }

    fn check_is_usb() -> bool {
        std::fs::read_to_string("/proc/mounts")
            .map(|s| s.contains("/dev/sda") || s.contains("/dev/sdb"))
            .unwrap_or(false)
    }
}
