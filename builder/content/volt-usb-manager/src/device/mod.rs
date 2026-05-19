pub mod backing_store;
pub mod detector;
pub mod device_id;
pub mod eject;
pub mod health;
pub mod mount_info;
pub mod removal;
#[cfg(feature = "udev")]
pub mod udev;

use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    pub path: String,
    pub vendor: String,
    pub model: String,
    pub serial: String,
    pub size_bytes: u64,
    pub removable: bool,
    pub read_only: bool,
}

impl DeviceInfo {
    pub fn is_valid(&self) -> bool {
        !self.path.is_empty() && self.size_bytes > 0
    }
}

#[derive(Debug)]
pub struct DeviceManager {
    detected: AtomicBool,
    device: parking_lot::RwLock<Option<DeviceInfo>>,
    detect_interval_ms: u64,
}

impl DeviceManager {
    pub fn new(detect_interval_ms: u64) -> Self {
        DeviceManager {
            detected: AtomicBool::new(false),
            device: parking_lot::RwLock::new(None),
            detect_interval_ms,
        }
    }

    pub fn detect(&self) -> Option<DeviceInfo> {
        let dev = self.device.read();
        dev.clone()
    }

    pub fn register(&self, info: DeviceInfo) {
        self.detected.store(true, Ordering::Relaxed);
        let mut dev = self.device.write();
        *dev = Some(info);
    }

    pub fn remove(&self) {
        self.detected.store(false, Ordering::Relaxed);
        let mut dev = self.device.write();
        *dev = None;
    }

    pub fn is_detected(&self) -> bool {
        self.detected.load(Ordering::Relaxed)
    }

    pub fn detect_interval_ms(&self) -> u64 {
        self.detect_interval_ms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_info_is_valid() {
        let info = DeviceInfo {
            path: "/dev/sda".into(),
            vendor: "Test".into(),
            model: "USB Drive".into(),
            serial: "12345".into(),
            size_bytes: 8192,
            removable: true,
            read_only: false,
        };
        assert!(info.is_valid());
    }

    #[test]
    fn test_device_info_invalid() {
        let info = DeviceInfo {
            path: String::new(),
            vendor: String::new(),
            model: String::new(),
            serial: String::new(),
            size_bytes: 0,
            removable: false,
            read_only: false,
        };
        assert!(!info.is_valid());
    }

    #[test]
    fn test_device_manager() {
        let mgr = DeviceManager::new(1000);
        assert!(!mgr.is_detected());
        let info = DeviceInfo {
            path: "/dev/sda".into(),
            vendor: "V".into(),
            model: "M".into(),
            serial: "S".into(),
            size_bytes: 4096,
            removable: true,
            read_only: false,
        };
        mgr.register(info.clone());
        assert!(mgr.is_detected());
        assert_eq!(mgr.detect(), Some(info));
        mgr.remove();
        assert!(!mgr.is_detected());
    }
}
