use std::collections::HashMap;

use parking_lot::Mutex;

pub struct OwnershipTracker {
    entries: Mutex<HashMap<u64, u32>>,
}

impl OwnershipTracker {
    pub fn new() -> Self {
        OwnershipTracker {
            entries: Mutex::new(HashMap::new()),
        }
    }

    pub fn assign(&self, alloc_id: u64, worker_id: u32) {
        let mut entries = self.entries.lock();
        entries.insert(alloc_id, worker_id);
    }

    pub fn owner(&self, alloc_id: u64) -> Option<u32> {
        let entries = self.entries.lock();
        entries.get(&alloc_id).copied()
    }

    pub fn remove(&self, alloc_id: u64) {
        let mut entries = self.entries.lock();
        entries.remove(&alloc_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_and_owner() {
        let tracker = OwnershipTracker::new();
        tracker.assign(100, 1);
        assert_eq!(tracker.owner(100), Some(1));
    }

    #[test]
    fn test_owner_missing() {
        let tracker = OwnershipTracker::new();
        assert_eq!(tracker.owner(999), None);
    }

    #[test]
    fn test_remove() {
        let tracker = OwnershipTracker::new();
        tracker.assign(42, 2);
        assert_eq!(tracker.owner(42), Some(2));
        tracker.remove(42);
        assert_eq!(tracker.owner(42), None);
    }

    #[test]
    fn test_reassign() {
        let tracker = OwnershipTracker::new();
        tracker.assign(1, 10);
        tracker.assign(1, 20);
        assert_eq!(tracker.owner(1), Some(20));
    }
}
