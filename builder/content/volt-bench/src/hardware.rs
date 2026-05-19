use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub class: String,
    pub model: String,
    pub cpu: String,
    pub cpu_cores: u32,
    pub ram_gb: f64,
    pub gpu: String,
    pub storage_type: String,
    pub storage_model: String,
    pub arch: String,
    pub battery_or_ac: String,
    pub thermal_state: String,
}

impl HardwareInfo {
    pub fn detect() -> Self {
        let cpu = Self::read_first_line("/proc/cpuinfo", "model name").unwrap_or_else(|| "Unknown CPU".into());
        let cores = std::fs::read_to_string("/proc/cpuinfo")
            .map(|s| s.lines().filter(|l| l.starts_with("processor")).count() as u32)
            .unwrap_or(1);
        let ram_total = std::fs::read_to_string("/proc/meminfo")
            .and_then(|s| {
                s.lines()
                    .find(|l| l.starts_with("MemTotal:"))
                    .and_then(|l| l.split_whitespace().nth(1))
                    .and_then(|v| v.parse::<f64>().ok())
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "parse failed"))
            })
            .unwrap_or(0.0);
        let ram_gb = ram_total / 1024.0 / 1024.0;
        let arch = std::env::consts::ARCH.to_string();

        let model = std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "Unknown".into());

        let gpu = Self::detect_gpu();

        let storage_type = Self::detect_storage_type();

        let battery = std::fs::read_to_string("/sys/class/power_supply/ADP1/online")
            .or_else(|_| std::fs::read_to_string("/sys/class/power_supply/AC/online"))
            .map(|s| if s.trim() == "1" { "ac" } else { "battery" })
            .unwrap_or_else(|_| {
                if std::path::Path::new("/sys/class/power_supply/BAT0").exists() {
                    "battery"
                } else {
                    "ac"
                }
            });

        let thermal_state = Self::detect_thermal_state();

        let class = Self::classify(&cpu, &gpu, ram_gb, cores, &storage_type);

        Self {
            class,
            model,
            cpu,
            cpu_cores: cores,
            ram_gb: (ram_gb * 10.0).round() / 10.0,
            gpu,
            storage_type,
            storage_model: String::new(),
            arch,
            battery_or_ac: battery.to_string(),
            thermal_state,
        }
    }

    fn read_first_line(path: &str, prefix: &str) -> Option<String> {
        let content = std::fs::read_to_string(path).ok()?;
        content.lines()
            .find(|l| l.trim().starts_with(prefix))
            .and_then(|l| l.split(':').nth(1))
            .map(|s| s.trim().to_string())
    }

    fn detect_gpu() -> String {
        std::fs::read_to_string("/sys/class/drm/card0/device/vendor")
            .map(|v| match v.trim() {
                "0x1002" => "AMD",
                "0x10de" => "NVIDIA",
                "0x8086" => "Intel",
                "0x1ae0" => "Google",
                _ => "Unknown GPU",
            })
            .map(|s| s.to_string())
            .unwrap_or_else(|_| {
                Self::read_first_line("/proc/cpuinfo", "model name")
                    .filter(|s| s.contains("GMA") || s.contains("HD Graphics") || s.contains("Iris"))
                    .unwrap_or_else(|| "Unknown GPU".into())
            })
    }

    fn detect_storage_type() -> String {
        if std::path::Path::new("/sys/block/nvme0n1").exists() {
            "nvme".into()
        } else if std::path::Path::new("/sys/block/sda").exists() {
            if std::fs::read_to_string("/sys/block/sda/queue/rotational")
                .map(|s| s.trim() == "0")
                .unwrap_or(false)
            {
                "ssd".into()
            } else {
                "hdd".into()
            }
        } else {
            "unknown".into()
        }
    }

    fn detect_thermal_state() -> String {
        let temp_path = "/sys/class/thermal/thermal_zone0/temp";
        std::fs::read_to_string(temp_path)
            .and_then(|s| s.trim().parse::<f64>().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "parse")))
            .map(|t| {
                let celsius = t / 1000.0;
                if celsius > 85.0 { "throttling".into() }
                else if celsius > 70.0 { "warm".into() }
                else { "nominal".into() }
            })
            .unwrap_or_else(|_| "unknown".into())
    }

    fn classify(cpu: &str, _gpu: &str, ram_gb: f64, cores: u32, storage: &str) -> String {
        let is_vm = std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
            .map(|s| {
                let l = s.to_lowercase();
                l.contains("vmware") || l.contains("virtualbox") || l.contains("qemu") || l.contains("kvm")
            })
            .unwrap_or(false);
        if is_vm { return "virtual_machine".into(); }

        let is_low_power = cpu.contains("Atom") || cpu.contains("Celeron") || cpu.contains("N100");
        if is_low_power && storage == "usb" { return "low_power_usb".into(); }

        let is_old = cpu.contains("i5-2") || cpu.contains("i5-3") || cpu.contains("i7-2") || cpu.contains("i7-3");
        if is_old || cores <= 2 { return "old_intel_desktop".into(); }

        if ram_gb >= 16.0 && cores >= 6 { return "high_end_desktop".into(); }
        if cores >= 4 { return "modern_laptop".into(); }

        "unknown".into()
    }
}
