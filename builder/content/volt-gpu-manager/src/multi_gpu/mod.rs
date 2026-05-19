pub mod fallback;
pub mod fusion;
pub mod policy;
pub mod power;
pub mod routing;

use crate::backend::GpuHardware;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiGpuMode {
    Single,
    Alternate,
    Split,
    Mirror,
}

impl MultiGpuMode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "single" => Some(MultiGpuMode::Single),
            "alternate" => Some(MultiGpuMode::Alternate),
            "split" => Some(MultiGpuMode::Split),
            "mirror" => Some(MultiGpuMode::Mirror),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MultiGpuMode::Single => "single",
            MultiGpuMode::Alternate => "alternate",
            MultiGpuMode::Split => "split",
            MultiGpuMode::Mirror => "mirror",
        }
    }
}

pub struct MultiGpuManager {
    devices: Vec<GpuHardware>,
    mode: MultiGpuMode,
}

impl MultiGpuManager {
    pub fn new(mode: MultiGpuMode) -> Self {
        Self {
            devices: Vec::new(),
            mode,
        }
    }

    pub fn add_device(&mut self, device: GpuHardware) {
        self.devices.push(device);
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn mode(&self) -> MultiGpuMode {
        self.mode
    }

    pub fn devices(&self) -> &[GpuHardware] {
        &self.devices
    }

    pub fn primary(&self) -> Option<&GpuHardware> {
        self.devices.first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::GpuBackendKind;

    #[test]
    fn test_multi_gpu_manager() {
        let mut mgr = MultiGpuManager::new(MultiGpuMode::Alternate);
        assert_eq!(mgr.device_count(), 0);
        mgr.add_device(GpuHardware::new("GPU0", "VendorA", GpuBackendKind::Null, 1024));
        mgr.add_device(GpuHardware::new("GPU1", "VendorB", GpuBackendKind::Null, 2048));
        assert_eq!(mgr.device_count(), 2);
        assert_eq!(mgr.mode(), MultiGpuMode::Alternate);
        assert!(mgr.primary().is_some());
    }

    #[test]
    fn test_multi_gpu_mode() {
        assert_eq!(MultiGpuMode::from_str("mirror"), Some(MultiGpuMode::Mirror));
        assert_eq!(MultiGpuMode::from_str("unknown"), None);
        assert_eq!(MultiGpuMode::Single.as_str(), "single");
    }
}
