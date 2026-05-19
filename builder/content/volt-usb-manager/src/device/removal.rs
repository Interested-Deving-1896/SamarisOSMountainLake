use std::path::Path;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemovalEvent {
    DeviceInserted(String),
    DeviceRemoved(String),
    Error(String),
}

pub struct RemovalMonitor {
    path: String,
    interval: Duration,
    last_known: bool,
    last_check: Instant,
}

impl RemovalMonitor {
    pub fn new(path: &str, interval_ms: u64) -> Self {
        let last_known = Path::new(path).exists();
        RemovalMonitor {
            path: path.to_string(),
            interval: Duration::from_millis(interval_ms),
            last_known,
            last_check: Instant::now(),
        }
    }

    pub fn check(&mut self) -> Option<RemovalEvent> {
        if self.last_check.elapsed() < self.interval {
            return None;
        }
        self.last_check = Instant::now();

        let exists = Path::new(&self.path).exists();
        let path = self.path.clone();

        if exists && !self.last_known {
            self.last_known = true;
            Some(RemovalEvent::DeviceInserted(path))
        } else if !exists && self.last_known {
            self.last_known = false;
            Some(RemovalEvent::DeviceRemoved(path))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_new_monitor_tracks_existence() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test_device");
        fs::write(&path, "data").unwrap();

        let monitor = RemovalMonitor::new(path.to_string_lossy().as_ref(), 100);
        assert!(monitor.last_known);
    }

    #[test]
    fn test_check_no_change() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("stable_device");
        fs::write(&path, "data").unwrap();

        let mut monitor = RemovalMonitor::new(path.to_string_lossy().as_ref(), 0);
        std::thread::sleep(Duration::from_millis(1));
        let event = monitor.check();
        assert!(event.is_none());
    }

    #[test]
    fn test_check_device_removed() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("removable_device");
        fs::write(&path, "data").unwrap();

        let mut monitor = RemovalMonitor::new(path.to_string_lossy().as_ref(), 0);
        fs::remove_file(&path).unwrap();
        std::thread::sleep(Duration::from_millis(1));

        let event = monitor.check();
        match event {
            Some(RemovalEvent::DeviceRemoved(p)) => {
                assert_eq!(Path::new(&p).file_name().unwrap(), "removable_device");
            }
            other => panic!("Expected DeviceRemoved, got {:?}", other),
        }
    }

    #[test]
    fn test_check_device_inserted() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("new_device");

        let mut monitor = RemovalMonitor::new(path.to_string_lossy().as_ref(), 0);
        fs::write(&path, "data").unwrap();
        std::thread::sleep(Duration::from_millis(1));

        let event = monitor.check();
        match event {
            Some(RemovalEvent::DeviceInserted(p)) => {
                assert_eq!(Path::new(&p).file_name().unwrap(), "new_device");
            }
            other => panic!("Expected DeviceInserted, got {:?}", other),
        }
    }

    #[test]
    fn test_interval_respected() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("interval_test");
        fs::write(&path, "data").unwrap();

        let mut monitor = RemovalMonitor::new(path.to_string_lossy().as_ref(), 10_000);
        fs::remove_file(&path).unwrap();

        let event = monitor.check();
        assert!(event.is_none(), "Should not report change within interval");
    }
}
