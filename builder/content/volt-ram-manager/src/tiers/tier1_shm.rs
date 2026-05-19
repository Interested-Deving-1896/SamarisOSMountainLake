use std::sync::atomic::{AtomicU64, Ordering};

pub struct Tier1Shm {
    pub total_bytes: u64,
    used_bytes: AtomicU64,
    pub region_name: String,
}

impl Tier1Shm {
    pub fn new(size: u64, name: &str) -> Self {
        Tier1Shm {
            total_bytes: size,
            used_bytes: AtomicU64::new(0),
            region_name: name.to_string(),
        }
    }

    pub fn remaining(&self) -> u64 {
        self.total_bytes.saturating_sub(self.used_bytes.load(Ordering::Relaxed))
    }

    pub fn usage_pct(&self) -> f64 {
        if self.total_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes.load(Ordering::Relaxed) as f64 / self.total_bytes as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier1_shm_new() {
        let shm = Tier1Shm::new(1024 * 1024, "test_region");
        assert_eq!(shm.total_bytes, 1024 * 1024);
        assert_eq!(shm.region_name, "test_region");
        assert_eq!(shm.remaining(), 1024 * 1024);
    }

    #[test]
    fn test_tier1_shm_usage_pct_zero() {
        let shm = Tier1Shm::new(0, "zero");
        assert_eq!(shm.usage_pct(), 0.0);
    }
}
