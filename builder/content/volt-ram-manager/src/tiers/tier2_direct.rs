use std::sync::atomic::{AtomicU64, Ordering};

use dashmap::DashMap;
use uuid::Uuid;

use crate::pages::page_id::PageId;

pub struct Tier2Direct {
    pages: DashMap<Uuid, u64>,
    total_bytes: AtomicU64,
    max_bytes: u64,
}

impl Tier2Direct {
    pub fn new(max_bytes: u64) -> Self {
        Tier2Direct {
            pages: DashMap::new(),
            total_bytes: AtomicU64::new(0),
            max_bytes,
        }
    }

    pub fn track_page(&self, id: PageId, size: u64) {
        self.pages.insert(id.0, size);
        self.total_bytes.fetch_add(size, Ordering::Relaxed);
    }

    pub fn untrack_page(&self, id: PageId, size: u64) {
        self.pages.remove(&id.0);
        self.total_bytes.fetch_sub(size, Ordering::Relaxed);
    }

    pub fn usage_pct(&self) -> f64 {
        if self.max_bytes == 0 {
            return 0.0;
        }
        (self.total_bytes.load(Ordering::Relaxed) as f64 / self.max_bytes as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier2_new() {
        let t2 = Tier2Direct::new(1_000_000);
        assert_eq!(t2.usage_pct(), 0.0);
    }

    #[test]
    fn test_track_and_untrack() {
        let t2 = Tier2Direct::new(1_000_000);
        let id = PageId::new();
        t2.track_page(id, 4096);
        assert!(t2.usage_pct() > 0.0);
        t2.untrack_page(id, 4096);
        assert_eq!(t2.usage_pct(), 0.0);
    }

    #[test]
    fn test_usage_pct() {
        let t2 = Tier2Direct::new(1000);
        let id = PageId::new();
        t2.track_page(id, 250);
        assert!((t2.usage_pct() - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_max_bytes_zero() {
        let t2 = Tier2Direct::new(0);
        assert_eq!(t2.usage_pct(), 0.0);
    }
}
