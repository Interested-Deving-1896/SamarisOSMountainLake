use crate::hardware::profile::{BootMedium, UsbSpeed};

pub fn probe() -> (Option<UsbSpeed>, BootMedium) {
    #[cfg(target_os = "linux")]
    {
        if let Some(result) = probe_usb_linux() {
            return result;
        }
    }

    (None, BootMedium::InternalDisk)
}

#[cfg(target_os = "linux")]
fn probe_usb_linux() -> Option<(Option<UsbSpeed>, BootMedium)> {
    let boot_disk = std::fs::read_to_string("/proc/mounts")
        .ok()
        .and_then(|content| {
            content
                .lines()
                .find(|line| {
                    line.starts_with("/dev/")
                        && line
                            .split_whitespace()
                            .nth(1)
                            .map_or(false, |m| m == "/")
                })
                .and_then(|line| line.split_whitespace().next())
                .map(|s| {
                    let s = s.trim_start_matches("/dev/");
                    s.trim_end_matches(|c: char| c.is_ascii_digit())
                })
                .map(|s| s.to_string())
        })?;

    let removable_path = format!("/sys/block/{}/removable", boot_disk);
    if let Ok(content) = std::fs::read_to_string(&removable_path) {
        if content.trim() == "1" {
            let speed = probe_usb_speed(&boot_disk);
            return Some((speed, BootMedium::Usb));
        }
    }

    Some((None, BootMedium::InternalDisk))
}

#[cfg(target_os = "linux")]
fn probe_usb_speed(disk: &str) -> Option<UsbSpeed> {
    let speed_path = format!("/sys/block/{}/device/speed", disk);
    if let Ok(content) = std::fs::read_to_string(&speed_path) {
        let speed_str = content.trim();
        return match speed_str {
            "1.5" | "12" | "480" => Some(UsbSpeed::Usb2),
            "5000" | "10000" | "20000" => Some(UsbSpeed::Usb3Plus),
            _ => Some(UsbSpeed::Unknown),
        };
    }
    None
}
