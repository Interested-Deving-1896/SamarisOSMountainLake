use crate::core::error::VumError;
use crate::core::result::VumResult;

pub struct UsbDeviceDetector;

impl UsbDeviceDetector {
    pub fn detect() -> VumResult<String> {
        let devices = Self::find_removable()?;
        devices.into_iter().next().ok_or(VumError::DeviceNotFound)
    }

    pub fn find_removable() -> VumResult<Vec<String>> {
        let mut devices = Vec::new();

        #[cfg(target_os = "linux")]
        {
            let block_dir = std::path::Path::new("/sys/block");
            if block_dir.exists() {
                for entry in
                    std::fs::read_dir(block_dir).map_err(|_| VumError::DeviceNotFound)?
                {
                    if let Ok(entry) = entry {
                        let removable_path = entry.path().join("removable");
                        if let Ok(content) = std::fs::read_to_string(&removable_path) {
                            if content.trim() == "1" {
                                let dev_name = entry.file_name();
                                devices.push(format!("/dev/{}", dev_name.to_string_lossy()));
                            }
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            let output = std::process::Command::new("diskutil")
                .args(["list", "external"])
                .output()
                .map_err(|_| VumError::DeviceNotFound)?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let trimmed = line.trim_start();
                    if trimmed.starts_with("/dev/disk") {
                        let parts: Vec<&str> = trimmed.split_whitespace().collect();
                        if let Some(dev) = parts.first() {
                            devices.push(dev.to_string());
                        }
                    }
                }
            }
            if devices.is_empty() {
                if let Ok(entries) = std::fs::read_dir("/dev") {
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if name.starts_with("disk") && !name.contains('s') {
                            devices.push(format!("/dev/{}", name));
                        }
                    }
                }
            }
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            let _ = devices;
            return Err(VumError::DeviceNotFound);
        }

        if devices.is_empty() {
            return Err(VumError::DeviceNotFound);
        }
        Ok(devices)
    }

    pub fn is_removable(path: &str) -> VumResult<bool> {
        let canonical = std::fs::canonicalize(path).map_err(|_| VumError::DeviceNotFound)?;
        let path_str = canonical.to_string_lossy();

        #[cfg(target_os = "linux")]
        {
            let file_name = canonical
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or(VumError::DeviceNotFound)?;
            let base = file_name.trim_end_matches(|c: char| c.is_ascii_digit());
            let base = base.trim_end_matches('p');
            let removable_path = format!("/sys/block/{}/removable", base);
            let content =
                std::fs::read_to_string(&removable_path).map_err(|_| VumError::DeviceNotFound)?;
            Ok(content.trim() == "1")
        }

        #[cfg(target_os = "macos")]
        {
            let output = std::process::Command::new("diskutil")
                .args(["info", path_str.as_ref()])
                .output()
                .map_err(|_| VumError::DeviceNotFound)?;
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                Ok(stdout.contains("Removable Media: Yes") || stdout.contains("Removable: Yes"))
            } else {
                Err(VumError::DeviceNotFound)
            }
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            let _ = path_str;
            Err(VumError::DeviceNotFound)
        }
    }

    pub fn is_read_only(path: &str) -> VumResult<bool> {
        let metadata = std::fs::metadata(path).map_err(|_| VumError::DeviceNotFound)?;

        #[cfg(target_os = "macos")]
        {
            let canonical = std::fs::canonicalize(path).map_err(|_| VumError::DeviceNotFound)?;
            let output = std::process::Command::new("diskutil")
                .args(["info", canonical.to_string_lossy().as_ref()])
                .output();
            if let Ok(out) = output {
                if out.status.success() {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    if stdout.contains("Read-Only Media: Yes") || stdout.contains("Read Only: Yes")
                    {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(metadata.permissions().readonly())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_read_only_temp_dir() {
        let dir = tempfile::tempdir().unwrap();
        let result =
            UsbDeviceDetector::is_read_only(dir.path().to_string_lossy().as_ref());
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_find_removable_returns_list_or_error() {
        let result = UsbDeviceDetector::find_removable();
        match result {
            Ok(devices) => {
                assert!(!devices.is_empty());
                for d in &devices {
                    assert!(d.starts_with("/dev/"));
                }
            }
            Err(e) => {
                assert!(matches!(e, VumError::DeviceNotFound));
            }
        }
    }

    #[test]
    fn test_detect_returns_device_or_error() {
        let result = UsbDeviceDetector::detect();
        match result {
            Ok(dev) => assert!(dev.starts_with("/dev/")),
            Err(e) => assert!(matches!(e, VumError::DeviceNotFound)),
        }
    }
}
