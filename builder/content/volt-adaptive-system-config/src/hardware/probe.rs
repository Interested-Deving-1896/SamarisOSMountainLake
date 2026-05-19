use crate::hardware::battery;
use crate::hardware::confidence::DetectionConfidence;
use crate::hardware::cpu;
use crate::hardware::gpu;
use crate::hardware::memory;
use crate::hardware::profile::{BootMedium, HardwareProfile, StorageType, UsbSpeed};
use crate::hardware::storage;
use crate::hardware::thermal;
use crate::hardware::usb;
use crate::hardware::vm;
use crate::core::result::AscResult;

pub struct HardwareProbe;

impl HardwareProbe {
    pub fn new() -> Self {
        Self
    }

    pub fn detect(&self) -> AscResult<HardwareProfile> {
        let (cpu_cores, cpu_threads, cpu_model, cpu_arch) = cpu::probe();
        let (ram_total_mb, ram_available_mb, swap_total_mb) = memory::probe();
        let (gpu_available, gpu_vendor, gpu_model, gpu_memory_mb) = gpu::probe();
        let storage_type: StorageType = storage::probe();
        let (usb_speed, boot_medium): (Option<UsbSpeed>, BootMedium) = usb::probe();
        let is_vm: bool = vm::probe();
        let (is_laptop, battery_present): (bool, bool) = battery::probe();
        let thermal_available: bool = thermal::probe();

        let confidence = DetectionConfidence {
            cpu: if cfg!(target_os = "linux") { 0.95 } else { 0.3 },
            ram: if cfg!(target_os = "linux") { 0.95 } else { 0.3 },
            gpu: if gpu_available && gpu_vendor.is_some() { 0.85 } else { 0.3 },
            storage: if storage_type != StorageType::Unknown { 0.9 } else { 0.3 },
            usb: match usb_speed { Some(UsbSpeed::Unknown) | None => 0.3, _ => 0.85 },
            vm: 0.5,
            laptop: if is_laptop { 0.7 } else { 0.4 },
        };

        Ok(HardwareProfile {
            cpu_cores,
            cpu_threads,
            cpu_model,
            cpu_arch,
            ram_total_mb,
            ram_available_mb,
            swap_total_mb,
            gpu_available,
            gpu_vendor,
            gpu_model,
            gpu_memory_mb,
            boot_medium,
            storage_type,
            usb_speed,
            is_vm,
            is_laptop,
            battery_present,
            thermal_available,
            confidence,
        })
    }
}

impl Default for HardwareProbe {
    fn default() -> Self {
        Self::new()
    }
}
