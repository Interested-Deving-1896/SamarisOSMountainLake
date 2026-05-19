use std::sync::atomic::{AtomicU64, Ordering};

use dashmap::DashMap;
use uuid::Uuid;

use crate::pages::page_id::PageId;

struct CompressedEntry {
    #[allow(dead_code)]
    page_id: PageId,
    raw_size: u64,
    compressed_size: u64,
}

pub struct Tier3Compressed {
    pages: DashMap<Uuid, CompressedEntry>,
    total_raw_bytes: AtomicU64,
    total_compressed_bytes: AtomicU64,
}

impl Tier3Compressed {
    pub fn new() -> Self {
        Tier3Compressed {
            pages: DashMap::new(),
            total_raw_bytes: AtomicU64::new(0),
            total_compressed_bytes: AtomicU64::new(0),
        }
    }

    pub fn track(&self, id: PageId, raw: u64, compressed: u64) {
        self.pages.insert(
            id.0,
            CompressedEntry {
                page_id: id,
                raw_size: raw,
                compressed_size: compressed,
            },
        );
        self.total_raw_bytes.fetch_add(raw, Ordering::Relaxed);
        self.total_compressed_bytes
            .fetch_add(compressed, Ordering::Relaxed);
    }

    pub fn untrack(&self, id: PageId) {
        if let Some((_, entry)) = self.pages.remove(&id.0) {
            self.total_raw_bytes
                .fetch_sub(entry.raw_size, Ordering::Relaxed);
            self.total_compressed_bytes
                .fetch_sub(entry.compressed_size, Ordering::Relaxed);
        }
    }

    pub fn compression_ratio(&self) -> f64 {
        let raw = self.total_raw_bytes.load(Ordering::Relaxed);
        let compressed = self.total_compressed_bytes.load(Ordering::Relaxed);
        if compressed == 0 {
            return 0.0;
        }
        raw as f64 / compressed as f64
    }
}

impl Default for Tier3Compressed {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier3_new() {
        let t3 = Tier3Compressed::new();
        assert_eq!(t3.compression_ratio(), 0.0);
    }

    #[test]
    fn test_track_and_compression_ratio() {
        let t3 = Tier3Compressed::new();
        let id = PageId::new();
        t3.track(id, 4096, 1024);
        assert!((t3.compression_ratio() - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_untrack() {
        let t3 = Tier3Compressed::new();
        let id = PageId::new();
        t3.track(id, 4096, 1024);
        assert!(t3.compression_ratio() > 0.0);
        t3.untrack(id);
        assert_eq!(t3.compression_ratio(), 0.0);
    }

    #[test]
    fn test_multiple_pages() {
        let t3 = Tier3Compressed::new();
        let id1 = PageId::new();
        let id2 = PageId::new();
        t3.track(id1, 1000, 200);
        t3.track(id2, 2000, 400);
        assert!((t3.compression_ratio() - 5.0).abs() < 0.001);
    }
}
