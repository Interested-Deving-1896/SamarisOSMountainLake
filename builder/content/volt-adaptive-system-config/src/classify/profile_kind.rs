use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileKind {
    Auto,
    Balanced,
    Performance,
    PowerSave,
    Powersave,
    Safe,
    Debug,
    Vm,
    UsbBoot,
    LowRam,
}

impl ProfileKind {
    pub fn from_config(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "auto" => ProfileKind::Auto,
            "balanced" => ProfileKind::Balanced,
            "performance" => ProfileKind::Performance,
            "powersave" | "power-save" | "power_save" => ProfileKind::Powersave,
            "safe" => ProfileKind::Safe,
            "debug" => ProfileKind::Debug,
            "vm" => ProfileKind::Vm,
            "usbboot" | "usb-boot" | "usb_boot" => ProfileKind::UsbBoot,
            "lowram" | "low-ram" | "low_ram" => ProfileKind::LowRam,
            _ => ProfileKind::Balanced,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ProfileKind::Auto => "auto",
            ProfileKind::Balanced => "balanced",
            ProfileKind::Performance => "performance",
            ProfileKind::PowerSave => "power-save",
            ProfileKind::Powersave => "powersave",
            ProfileKind::Safe => "safe",
            ProfileKind::Debug => "debug",
            ProfileKind::Vm => "vm",
            ProfileKind::UsbBoot => "usb-boot",
            ProfileKind::LowRam => "low-ram",
        }
    }

    pub fn is_valid(s: &str) -> bool {
        matches!(
            s.trim().to_lowercase().as_str(),
            "auto"
                | "balanced"
                | "performance"
                | "powersave"
                | "power-save"
                | "power_save"
                | "safe"
                | "debug"
                | "vm"
                | "usbboot"
                | "usb-boot"
                | "usb_boot"
                | "lowram"
                | "low-ram"
                | "low_ram"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_config_balanced() {
        assert_eq!(ProfileKind::from_config("balanced"), ProfileKind::Balanced);
    }

    #[test]
    fn test_from_config_auto() {
        assert_eq!(ProfileKind::from_config("auto"), ProfileKind::Auto);
    }

    #[test]
    fn test_from_config_default() {
        assert_eq!(ProfileKind::from_config("unknown"), ProfileKind::Balanced);
    }

    #[test]
    fn test_is_valid() {
        assert!(ProfileKind::is_valid("balanced"));
        assert!(ProfileKind::is_valid("powersave"));
        assert!(!ProfileKind::is_valid("invalid"));
    }
}
