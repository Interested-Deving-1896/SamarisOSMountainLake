use std::fmt::{Display, Formatter};

use crate::core::error::VumError;
use crate::core::result::VumResult;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UsbDeviceId(pub String);

impl UsbDeviceId {
    pub fn new(id: &str) -> Self {
        UsbDeviceId(id.to_string())
    }

    pub fn from_path(path: &str) -> VumResult<Self> {
        if path.is_empty() {
            return Err(VumError::InvalidPath("Empty path".into()));
        }
        let path = std::path::Path::new(path);
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| VumError::InvalidPath(format!("Cannot extract filename from: {}", path.display())))?;
        let base = file_name.trim_end_matches(|c: char| c.is_ascii_digit());
        let base = base.trim_end_matches('p');
        if base.is_empty() {
            return Err(VumError::InvalidPath(format!(
                "Cannot derive device ID from path: {}",
                path.display()
            )));
        }
        Ok(UsbDeviceId(base.to_string()))
    }
}

impl Display for UsbDeviceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_device_id() {
        let id = UsbDeviceId::new("sda1");
        assert_eq!(id.0, "sda1");
    }

    #[test]
    fn test_from_path_linux() {
        let id = UsbDeviceId::from_path("/dev/sda1").unwrap();
        assert_eq!(id.0, "sda");
    }

    #[test]
    fn test_from_path_nvme() {
        let id = UsbDeviceId::from_path("/dev/nvme0n1p1").unwrap();
        assert_eq!(id.0, "nvme0n1");
    }

    #[test]
    fn test_from_path_empty_fails() {
        assert!(UsbDeviceId::from_path("").is_err());
    }

    #[test]
    fn test_display() {
        let id = UsbDeviceId::new("sdb");
        assert_eq!(format!("{}", id), "sdb");
    }

    #[test]
    fn test_equality() {
        let a = UsbDeviceId::new("sda");
        let b = UsbDeviceId::new("sda");
        let c = UsbDeviceId::new("sdb");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(UsbDeviceId::new("sda"));
        set.insert(UsbDeviceId::new("sda"));
        assert_eq!(set.len(), 1);
    }
}
