use serde::{Deserialize, Serialize};

use crate::hardware::confidence::DetectionConfidence;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HardwareProfile {
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub cpu_model: String,
    pub cpu_arch: String,
    pub ram_total_mb: u64,
    pub ram_available_mb: u64,
    pub swap_total_mb: u64,
    pub gpu_available: bool,
    pub gpu_vendor: Option<String>,
    pub gpu_model: Option<String>,
    pub gpu_memory_mb: Option<u64>,
    pub boot_medium: BootMedium,
    pub storage_type: StorageType,
    pub usb_speed: Option<UsbSpeed>,
    pub is_vm: bool,
    pub is_laptop: bool,
    pub battery_present: bool,
    pub thermal_available: bool,
    pub confidence: DetectionConfidence,
}

impl Default for HardwareProfile {
    fn default() -> Self {
        Self {
            cpu_cores: 4,
            cpu_threads: 4,
            cpu_model: "unknown".into(),
            cpu_arch: std::env::consts::ARCH.into(),
            ram_total_mb: 2048,
            ram_available_mb: 1024,
            swap_total_mb: 0,
            gpu_available: false,
            gpu_vendor: None,
            gpu_model: None,
            gpu_memory_mb: None,
            boot_medium: BootMedium::Unknown,
            storage_type: StorageType::Unknown,
            usb_speed: None,
            is_vm: false,
            is_laptop: false,
            battery_present: false,
            thermal_available: false,
            confidence: DetectionConfidence::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UsbSpeed {
    Usb2,
    Usb3Plus,
    Unknown,
}

impl UsbSpeed {
    pub fn name(&self) -> &'static str {
        match self {
            UsbSpeed::Usb2 => "usb2",
            UsbSpeed::Usb3Plus => "usb3_plus",
            UsbSpeed::Unknown => "unknown",
        }
    }

    pub fn is_usb3_or_greater(&self) -> bool {
        matches!(self, UsbSpeed::Usb3Plus)
    }
}

impl std::fmt::Display for UsbSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StorageType {
    Usb,
    Hdd,
    Ssd,
    Nvme,
    Emmcp,
    Unknown,
}

impl StorageType {
    pub fn name(&self) -> &'static str {
        match self {
            StorageType::Usb => "usb",
            StorageType::Hdd => "hdd",
            StorageType::Ssd => "ssd",
            StorageType::Nvme => "nvme",
            StorageType::Emmcp => "emmcp",
            StorageType::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for StorageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum BootMedium {
    Usb,
    InternalDisk,
    Network,
    Unknown,
}

impl BootMedium {
    pub fn name(&self) -> &'static str {
        match self {
            BootMedium::Usb => "usb",
            BootMedium::InternalDisk => "internal_disk",
            BootMedium::Network => "network",
            BootMedium::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for BootMedium {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
