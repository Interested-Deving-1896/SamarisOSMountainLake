use crate::hardware::profile::{BootMedium, HardwareProfile, StorageType};

/// VUM cache in MB — ram / 16, adjusted by storage type.
pub fn vum_cache_mb(hw: &HardwareProfile) -> u64 {
    let base = hw.ram_total_mb / 16;
    match hw.storage_type {
        StorageType::Usb => (base as f64 * 1.5) as u64,
        StorageType::Nvme => (base as f64 * 0.75) as u64,
        _ => base,
    }
}

/// VUM buffer in MB — cache / 2, min 32, max 2048
pub fn vum_buffer_mb(hw: &HardwareProfile) -> u64 {
    let cache = vum_cache_mb(hw);
    (cache / 2).max(32).min(2048)
}

/// VUM flush interval in milliseconds based on storage type
pub fn vum_flush_interval_ms(hw: &HardwareProfile) -> u64 {
    match hw.storage_type {
        StorageType::Usb => 5000,
        StorageType::Hdd => 10000,
        StorageType::Ssd => 15000,
        StorageType::Nvme => 30000,
        StorageType::Emmcp => 10000,
        StorageType::Unknown => 10000,
    }
}

/// VUM batch size in KB — 256 for USB3+, 128 otherwise
pub fn vum_batch_size_kb(hw: &HardwareProfile) -> u64 {
    let is_usb3_plus = hw.usb_speed.as_ref().map_or(false, |s| s.is_usb3_or_greater());
    if hw.storage_type == StorageType::Usb && is_usb3_plus {
        256
    } else {
        128
    }
}

/// Whether to prefetch boot assets — true if USB boot
pub fn prefetch_boot_assets(hw: &HardwareProfile) -> bool {
    hw.boot_medium == BootMedium::Usb
}

// ---- Aliases used by the generator ----

pub fn vum_cache_mb_from_budget(budget: &super::super::budget::system_budget::SystemBudget) -> u64 {
    budget.vum_cache_mb
}

pub fn vum_buffer_mb_from_budget(budget: &super::super::budget::system_budget::SystemBudget) -> u64 {
    budget.vum_buffer_mb
}

pub fn vum_flush_interval(hw: &HardwareProfile) -> u64 {
    vum_flush_interval_ms(hw)
}

pub fn vum_batch_size(hw: &HardwareProfile) -> u64 {
    vum_batch_size_kb(hw)
}

pub fn vum_journal_mode(_hw: &HardwareProfile) -> String {
    "wal".into()
}

pub fn vum_prefetch_boot_assets(medium: BootMedium) -> bool {
    medium == BootMedium::Usb
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::confidence::DetectionConfidence;
    use crate::hardware::profile::UsbSpeed;

    fn hw(
        ram_total_mb: u64,
        storage_type: StorageType,
        usb_speed: Option<UsbSpeed>,
        boot_medium: BootMedium,
    ) -> HardwareProfile {
        HardwareProfile {
            cpu_cores: 8,
            cpu_threads: 8,
            cpu_model: "test".into(),
            cpu_arch: "x86_64".into(),
            ram_total_mb,
            ram_available_mb: ram_total_mb,
            swap_total_mb: 0,
            is_laptop: false,
            is_vm: false,
            boot_medium,
            storage_type,
            usb_speed,
            gpu_available: true,
            gpu_vendor: None,
            gpu_model: None,
            gpu_memory_mb: None,
            battery_present: false,
            thermal_available: false,
            confidence: DetectionConfidence::default(),
        }
    }

    #[test]
    fn test_cache_base() {
        assert_eq!(vum_cache_mb(&hw(16384, StorageType::Ssd, None, BootMedium::Unknown)), 1024);
    }

    #[test]
    fn test_cache_usb_boost() {
        assert_eq!(vum_cache_mb(&hw(16384, StorageType::Usb, None, BootMedium::Usb)), 1536);
    }

    #[test]
    fn test_cache_nvme_reduction() {
        assert_eq!(vum_cache_mb(&hw(16384, StorageType::Nvme, None, BootMedium::Unknown)), 768);
    }

    #[test]
    fn test_flush_intervals() {
        assert_eq!(vum_flush_interval_ms(&hw(8192, StorageType::Usb, None, BootMedium::Unknown)), 5000);
        assert_eq!(vum_flush_interval_ms(&hw(8192, StorageType::Hdd, None, BootMedium::Unknown)), 10000);
        assert_eq!(vum_flush_interval_ms(&hw(8192, StorageType::Ssd, None, BootMedium::Unknown)), 15000);
        assert_eq!(vum_flush_interval_ms(&hw(8192, StorageType::Nvme, None, BootMedium::Unknown)), 30000);
    }

    #[test]
    fn test_batch_size_usb3() {
        assert_eq!(vum_batch_size_kb(&hw(8192, StorageType::Usb, Some(UsbSpeed::Usb3Plus), BootMedium::Usb)), 256);
    }

    #[test]
    fn test_batch_size_usb2() {
        assert_eq!(vum_batch_size_kb(&hw(8192, StorageType::Usb, Some(UsbSpeed::Usb2), BootMedium::Usb)), 128);
    }

    #[test]
    fn test_aliases() {
        assert_eq!(vum_flush_interval(&hw(8192, StorageType::Nvme, None, BootMedium::Unknown)), 30000);
        assert_eq!(vum_batch_size(&hw(8192, StorageType::Usb, Some(UsbSpeed::Usb3Plus), BootMedium::Usb)), 256);
        assert_eq!(vum_journal_mode(&hw(8192, StorageType::Ssd, None, BootMedium::Unknown)), "wal");
        assert!(vum_prefetch_boot_assets(BootMedium::Usb));
        assert!(!vum_prefetch_boot_assets(BootMedium::InternalDisk));
    }
}
