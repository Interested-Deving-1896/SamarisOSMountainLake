use crate::tiers::tier::MemoryTier;

#[derive(Debug, Clone)]
pub struct AppMemoryUsage {
    pub app_id: u64,
    pub total_bytes: u64,
    pub t1_shm_bytes: u64,
    pub t2_direct_bytes: u64,
    pub t3_compressed_bytes: u64,
    pub compressed_bytes: u64,
    pub uncompressed_bytes: u64,
    pub page_count: u64,
}

impl AppMemoryUsage {
    pub fn new(app_id: u64) -> Self {
        Self {
            app_id,
            total_bytes: 0,
            t1_shm_bytes: 0,
            t2_direct_bytes: 0,
            t3_compressed_bytes: 0,
            compressed_bytes: 0,
            uncompressed_bytes: 0,
            page_count: 0,
        }
    }

    pub fn record_page(&mut self, page_size: u64, tier: MemoryTier, compressed: bool) {
        self.total_bytes = self.total_bytes.saturating_add(page_size);
        self.page_count = self.page_count.saturating_add(1);
        match tier {
            MemoryTier::T1Shm => self.t1_shm_bytes = self.t1_shm_bytes.saturating_add(page_size),
            MemoryTier::T2Direct => self.t2_direct_bytes = self.t2_direct_bytes.saturating_add(page_size),
            MemoryTier::T3Compressed => self.t3_compressed_bytes = self.t3_compressed_bytes.saturating_add(page_size),
        }
        if compressed {
            self.compressed_bytes = self.compressed_bytes.saturating_add(page_size);
        } else {
            self.uncompressed_bytes = self.uncompressed_bytes.saturating_add(page_size);
        }
    }

    pub fn remove_page(&mut self, page_size: u64, tier: MemoryTier, compressed: bool) {
        self.total_bytes = self.total_bytes.saturating_sub(page_size);
        self.page_count = self.page_count.saturating_sub(1);
        match tier {
            MemoryTier::T1Shm => self.t1_shm_bytes = self.t1_shm_bytes.saturating_sub(page_size),
            MemoryTier::T2Direct => self.t2_direct_bytes = self.t2_direct_bytes.saturating_sub(page_size),
            MemoryTier::T3Compressed => self.t3_compressed_bytes = self.t3_compressed_bytes.saturating_sub(page_size),
        }
        if compressed {
            self.compressed_bytes = self.compressed_bytes.saturating_sub(page_size);
        } else {
            self.uncompressed_bytes = self.uncompressed_bytes.saturating_sub(page_size);
        }
    }

    pub fn compression_ratio(&self) -> f64 {
        if self.uncompressed_bytes == 0 {
            return 1.0;
        }
        self.compressed_bytes as f64 / self.uncompressed_bytes as f64
    }

    pub fn tier_breakdown(&self) -> Vec<(&str, u64)> {
        vec![
            ("t1_shm", self.t1_shm_bytes),
            ("t2_direct", self.t2_direct_bytes),
            ("t3_compressed", self.t3_compressed_bytes),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_usage_empty() {
        let usage = AppMemoryUsage::new(1);
        assert_eq!(usage.app_id, 1);
        assert_eq!(usage.total_bytes, 0);
        assert_eq!(usage.page_count, 0);
    }

    #[test]
    fn test_record_page() {
        let mut usage = AppMemoryUsage::new(1);
        usage.record_page(4096, MemoryTier::T1Shm, false);
        assert_eq!(usage.total_bytes, 4096);
        assert_eq!(usage.t1_shm_bytes, 4096);
        assert_eq!(usage.uncompressed_bytes, 4096);
        assert_eq!(usage.page_count, 1);
    }

    #[test]
    fn test_record_compressed_page() {
        let mut usage = AppMemoryUsage::new(1);
        usage.record_page(2048, MemoryTier::T2Direct, true);
        assert_eq!(usage.total_bytes, 2048);
        assert_eq!(usage.t2_direct_bytes, 2048);
        assert_eq!(usage.compressed_bytes, 2048);
    }

    #[test]
    fn test_remove_page() {
        let mut usage = AppMemoryUsage::new(1);
        usage.record_page(4096, MemoryTier::T1Shm, false);
        usage.record_page(8192, MemoryTier::T2Direct, true);
        assert_eq!(usage.total_bytes, 12288);
        usage.remove_page(4096, MemoryTier::T1Shm, false);
        assert_eq!(usage.total_bytes, 8192);
        assert_eq!(usage.t1_shm_bytes, 0);
        assert_eq!(usage.page_count, 1);
    }

    #[test]
    fn test_compression_ratio() {
        let mut usage = AppMemoryUsage::new(1);
        assert!((usage.compression_ratio() - 1.0).abs() < f64::EPSILON);
        usage.record_page(1000, MemoryTier::T1Shm, true);
        usage.record_page(2000, MemoryTier::T1Shm, false);
        assert!((usage.compression_ratio() - 0.5).abs() < f64::EPSILON);
    }
}
