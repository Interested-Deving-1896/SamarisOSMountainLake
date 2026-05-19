use dashmap::DashMap;
use uuid::Uuid;

use crate::pages::page_id::PageId;
use std::time::Instant;

pub struct AccessTracker {
    access_times: DashMap<Uuid, Instant>,
}

impl AccessTracker {
    pub fn new() -> Self {
        AccessTracker {
            access_times: DashMap::new(),
        }
    }

    pub fn record_access(&self, id: PageId) {
        self.access_times.insert(id.0, Instant::now());
    }

    pub fn last_access(&self, id: PageId) -> Option<Instant> {
        self.access_times.get(&id.0).map(|r| *r)
    }

    pub fn is_inactive(&self, id: PageId, threshold_ms: u64) -> bool {
        self.access_times
            .get(&id.0)
            .map(|r| r.elapsed().as_millis() as u64 >= threshold_ms)
            .unwrap_or(true)
    }

    pub fn remove(&self, id: PageId) {
        self.access_times.remove(&id.0);
    }

    pub fn clear(&self) {
        self.access_times.clear();
    }

    pub fn find_inactive(&self, threshold_ms: u64) -> Vec<PageId> {
        self.access_times
            .iter()
            .filter(|r| r.elapsed().as_millis() as u64 >= threshold_ms)
            .map(|r| PageId(*r.key()))
            .collect()
    }
}

impl Default for AccessTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_and_last_access() {
        let tracker = AccessTracker::new();
        let id = PageId::new();
        assert!(tracker.last_access(id).is_none());
        tracker.record_access(id);
        assert!(tracker.last_access(id).is_some());
    }

    #[test]
    fn test_remove() {
        let tracker = AccessTracker::new();
        let id = PageId::new();
        tracker.record_access(id);
        assert!(tracker.last_access(id).is_some());
        tracker.remove(id);
        assert!(tracker.last_access(id).is_none());
    }

    #[test]
    fn test_clear() {
        let tracker = AccessTracker::new();
        tracker.record_access(PageId::new());
        tracker.record_access(PageId::new());
        assert_eq!(tracker.find_inactive(0).len(), 2);
        tracker.clear();
        assert_eq!(tracker.find_inactive(0).len(), 0);
    }

    #[test]
    fn test_is_inactive_no_record() {
        let tracker = AccessTracker::new();
        assert!(tracker.is_inactive(PageId::new(), 0));
    }

    #[test]
    fn test_is_inactive_recent() {
        let tracker = AccessTracker::new();
        let id = PageId::new();
        tracker.record_access(id);
        assert!(!tracker.is_inactive(id, 100_000));
    }

    #[test]
    fn test_find_inactive() {
        let tracker = AccessTracker::new();
        let active = PageId::new();
        tracker.record_access(active);
        // Only recorded entries appear in find_inactive
        // An unrecorded page is treated as inactive by is_inactive,
        // but find_inactive only returns recorded pages past the threshold
        let found = tracker.find_inactive(0);
        assert!(found.contains(&active));
        assert_eq!(found.len(), 1);
    }
}
