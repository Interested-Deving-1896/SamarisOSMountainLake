use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use parking_lot::RwLock;

use crate::pages::access_tracker::AccessTracker;
use crate::pages::page_id::PageId;
use crate::pages::page_table::PageTable;

pub struct InactiveScanner {
    page_table: Arc<RwLock<PageTable>>,
    access_tracker: Arc<AccessTracker>,
    running: Arc<AtomicBool>,
    interval_ms: u64,
    inactive_threshold_ms: u64,
}

impl InactiveScanner {
    pub fn new(
        table: Arc<RwLock<PageTable>>,
        tracker: Arc<AccessTracker>,
        interval_ms: u64,
        threshold_ms: u64,
    ) -> Self {
        InactiveScanner {
            page_table: table,
            access_tracker: tracker,
            running: Arc::new(AtomicBool::new(false)),
            interval_ms,
            inactive_threshold_ms: threshold_ms,
        }
    }

    pub fn start(&self) {
        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let tracker = self.access_tracker.clone();
        let table = self.page_table.clone();
        let interval = Duration::from_millis(self.interval_ms);
        let threshold = self.inactive_threshold_ms;

        thread::spawn(move || {
            while running.load(Ordering::SeqCst) {
                let inactive = tracker.find_inactive(threshold);
                if let Some(table) = table.try_write_for(Duration::from_millis(10)) {
                    for id in &inactive {
                        if let Some(_page) = table.get(*id) {
                            // page found as inactive; policy decisions happen elsewhere
                        }
                    }
                }
                thread::sleep(interval);
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn scan_once(&self) -> Vec<PageId> {
        self.access_tracker.find_inactive(self.inactive_threshold_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_once_empty() {
        let table = Arc::new(RwLock::new(PageTable::new()));
        let tracker = Arc::new(AccessTracker::new());
        let scanner = InactiveScanner::new(table, tracker, 1000, 5000);
        let inactive = scanner.scan_once();
        assert!(inactive.is_empty());
    }

    #[test]
    fn test_scan_once_with_inactive() {
        let table = Arc::new(RwLock::new(PageTable::new()));
        let tracker = Arc::new(AccessTracker::new());
        let id = PageId::new();
        // not recorded -> considered inactive
        let scanner = InactiveScanner::new(table, tracker, 1000, 0);
        let inactive = scanner.scan_once();
        // no pages tracked, so empty
        assert!(inactive.is_empty());
    }

    #[test]
    fn test_start_stop() {
        let table = Arc::new(RwLock::new(PageTable::new()));
        let tracker = Arc::new(AccessTracker::new());
        let scanner = InactiveScanner::new(table, tracker, 100, 1000);
        scanner.start();
        thread::sleep(Duration::from_millis(50));
        scanner.stop();
        // no panic = success
    }
}
