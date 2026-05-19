pub use crate::hardware::profile::BootMedium;

pub fn is_usb_boot(medium: &BootMedium) -> bool {
    *medium == BootMedium::Usb
}

pub fn is_removable(medium: &BootMedium) -> bool {
    matches!(medium, BootMedium::Usb | BootMedium::Unknown)
}

pub fn classify_description(medium: &BootMedium) -> &'static str {
    match medium {
        BootMedium::Usb => "USB boot medium",
        BootMedium::InternalDisk => "Internal disk boot medium",
        BootMedium::Network => "Network boot medium",
        BootMedium::Unknown => "Unknown boot medium",
    }
}
