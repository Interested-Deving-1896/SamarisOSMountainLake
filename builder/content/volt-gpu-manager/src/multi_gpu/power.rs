use crate::backend::GpuHardware;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerMode {
    Performance,
    Balanced,
    BatterySaver,
}

impl PowerMode {
    pub fn name(&self) -> &'static str {
        match self {
            PowerMode::Performance => "performance",
            PowerMode::Balanced => "balanced",
            PowerMode::BatterySaver => "battery_saver",
        }
    }
}

pub struct GpuPowerPolicy {
    mode: PowerMode,
}

impl GpuPowerPolicy {
    pub fn new(mode: PowerMode) -> Self {
        Self { mode }
    }

    pub fn mode(&self) -> PowerMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: PowerMode) {
        self.mode = mode;
    }

    pub fn select_device<'a>(&self, devices: &'a [GpuHardware]) -> Option<&'a GpuHardware> {
        match self.mode {
            PowerMode::BatterySaver => {
                devices.iter().find(|d| d.is_integrated())
                    .or_else(|| devices.first())
            }
            PowerMode::Performance => {
                devices.iter().find(|d| d.is_discrete())
                    .or_else(|| devices.first())
            }
            PowerMode::Balanced => devices.first(),
        }
    }

    pub fn should_throttle(&self) -> bool {
        self.mode == PowerMode::BatterySaver
    }
}

impl Default for GpuPowerPolicy {
    fn default() -> Self {
        Self {
            mode: PowerMode::Balanced,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::GpuBackendKind;

    fn intel_gpu() -> GpuHardware {
        GpuHardware::new("Intel UHD", "Intel", GpuBackendKind::Wgpu, 512 * 1024 * 1024)
    }

    fn nvidia_gpu() -> GpuHardware {
        GpuHardware::new("RTX 4060", "NVIDIA", GpuBackendKind::Vulkan, 8 * 1024 * 1024 * 1024)
    }

    #[test]
    fn test_power_mode_names() {
        assert_eq!(PowerMode::Performance.name(), "performance");
        assert_eq!(PowerMode::Balanced.name(), "balanced");
        assert_eq!(PowerMode::BatterySaver.name(), "battery_saver");
    }

    #[test]
    fn test_battery_saver_selects_integrated() {
        let policy = GpuPowerPolicy::new(PowerMode::BatterySaver);
        let devices = vec![nvidia_gpu(), intel_gpu()];
        let selected = policy.select_device(&devices);
        assert!(selected.is_some());
        assert!(selected.unwrap().is_integrated());
    }

    #[test]
    fn test_performance_selects_discrete() {
        let policy = GpuPowerPolicy::new(PowerMode::Performance);
        let devices = vec![intel_gpu(), nvidia_gpu()];
        let selected = policy.select_device(&devices);
        assert!(selected.is_some());
        assert!(selected.unwrap().is_discrete());
    }

    #[test]
    fn test_should_throttle() {
        let policy = GpuPowerPolicy::new(PowerMode::BatterySaver);
        assert!(policy.should_throttle());
        let policy = GpuPowerPolicy::new(PowerMode::Performance);
        assert!(!policy.should_throttle());
    }

    #[test]
    fn test_set_mode() {
        let mut policy = GpuPowerPolicy::default();
        assert_eq!(policy.mode(), PowerMode::Balanced);
        policy.set_mode(PowerMode::BatterySaver);
        assert_eq!(policy.mode(), PowerMode::BatterySaver);
    }
}
