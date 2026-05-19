#[derive(Debug, Clone)]
pub struct DeviceHealth {
    pub is_present: bool,
    pub is_read_only: bool,
    pub available_bytes: u64,
    pub total_bytes: u64,
    pub error_count: u64,
    pub last_error: Option<String>,
}

impl DeviceHealth {
    pub fn new() -> Self {
        DeviceHealth {
            is_present: false,
            is_read_only: false,
            available_bytes: 0,
            total_bytes: 0,
            error_count: 0,
            last_error: None,
        }
    }

    pub fn check(path: &str) -> Self {
        let mut health = DeviceHealth::new();
        match std::fs::metadata(path) {
            Ok(meta) => {
                health.is_present = true;
                health.is_read_only = meta.permissions().readonly();
                if let Ok(s) = nix::sys::statvfs::statvfs(std::path::Path::new(path)) {
                    health.total_bytes = s.blocks() as u64 * s.fragment_size() as u64;
                    health.available_bytes = s.blocks_available() as u64 * s.fragment_size() as u64;
                }
            }
            Err(e) => {
                health.is_present = false;
                health.error_count = 1;
                health.last_error = Some(format!("{}", e));
            }
        }
        health
    }

    pub fn ok(&self) -> bool {
        self.is_present && self.error_count == 0
    }
}

impl Default for DeviceHealth {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_new_defaults() {
        let h = DeviceHealth::new();
        assert!(!h.is_present);
        assert!(!h.is_read_only);
        assert_eq!(h.error_count, 0);
        assert!(h.last_error.is_none());
    }

    #[test]
    fn test_health_ok_positive() {
        let h = DeviceHealth {
            is_present: true,
            is_read_only: false,
            available_bytes: 1000,
            total_bytes: 10000,
            error_count: 0,
            last_error: None,
        };
        assert!(h.ok());
    }

    #[test]
    fn test_health_ok_not_present() {
        let h = DeviceHealth {
            is_present: false,
            ..DeviceHealth::new()
        };
        assert!(!h.ok());
    }

    #[test]
    fn test_health_ok_with_errors() {
        let h = DeviceHealth {
            is_present: true,
            error_count: 3,
            last_error: Some("I/O error".into()),
            ..DeviceHealth::new()
        };
        assert!(!h.ok());
    }

    #[test]
    fn test_check_existing_directory() {
        let dir = std::env::temp_dir();
        let health = DeviceHealth::check(dir.to_string_lossy().as_ref());
        assert!(health.is_present);
        assert!(health.total_bytes > 0);
    }

    #[test]
    fn test_check_nonexistent_path() {
        let health = DeviceHealth::check("/tmp/__vum_test_nonexistent_device_health");
        assert!(!health.is_present);
        assert_eq!(health.error_count, 1);
        assert!(health.last_error.is_some());
    }
}
