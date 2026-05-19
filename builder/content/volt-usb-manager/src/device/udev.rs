#[derive(Debug)]
pub struct UdevMonitor;

impl UdevMonitor {
    pub fn new() -> Self {
        UdevMonitor
    }

    pub fn wait_for_event(&self) -> Option<String> {
        None
    }
}

impl Default for UdevMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udev_monitor_new() {
        let monitor = UdevMonitor::new();
        assert!(monitor.wait_for_event().is_none());
    }

    #[test]
    fn test_udev_monitor_default() {
        let monitor: UdevMonitor = Default::default();
        assert!(monitor.wait_for_event().is_none());
    }

    #[test]
    fn test_udev_monitor_no_event() {
        let monitor = UdevMonitor::new();
        for _ in 0..5 {
            assert!(monitor.wait_for_event().is_none());
        }
    }
}
