use crate::core::error::VumError;
use crate::core::result::VumResult;

#[derive(Debug, Clone)]
pub struct RamQuota {
    pub max_bytes: u64,
    pub used_bytes: u64,
}

impl RamQuota {
    pub fn new(max_mb: u64) -> Self {
        RamQuota {
            max_bytes: max_mb * 1024 * 1024,
            used_bytes: 0,
        }
    }

    pub fn check(&self, additional: u64) -> VumResult<()> {
        let requested = self.used_bytes.checked_add(additional).ok_or_else(|| {
            VumError::InternalInvariantViolation("Requested memory would overflow".into())
        })?;
        if requested > self.max_bytes {
            return Err(VumError::InternalInvariantViolation(format!(
                "Memory quota exceeded: would use {} of {} bytes",
                requested, self.max_bytes
            )));
        }
        Ok(())
    }

    pub fn record(&mut self, bytes: u64) {
        self.used_bytes = self
            .used_bytes
            .saturating_add(bytes)
            .min(self.max_bytes);
    }

    pub fn release(&mut self, bytes: u64) {
        self.used_bytes = self.used_bytes.saturating_sub(bytes);
    }

    pub fn usage_pct(&self) -> f64 {
        if self.max_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes as f64 / self.max_bytes as f64) * 100.0
    }

    pub fn available_bytes(&self) -> u64 {
        self.max_bytes.saturating_sub(self.used_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_quota() {
        let q = RamQuota::new(100);
        assert_eq!(q.max_bytes, 100 * 1024 * 1024);
        assert_eq!(q.used_bytes, 0);
    }

    #[test]
    fn test_check_within_limit() {
        let q = RamQuota::new(10);
        assert!(q.check(5 * 1024 * 1024).is_ok());
    }

    #[test]
    fn test_check_exceeds_limit() {
        let q = RamQuota::new(1);
        assert!(q.check(2 * 1024 * 1024).is_err());
    }

    #[test]
    fn test_record_and_release() {
        let mut q = RamQuota::new(100);
        q.record(50 * 1024 * 1024);
        assert_eq!(q.used_bytes, 50 * 1024 * 1024);
        q.release(20 * 1024 * 1024);
        assert_eq!(q.used_bytes, 30 * 1024 * 1024);
    }

    #[test]
    fn test_usage_pct() {
        let mut q = RamQuota::new(100);
        q.record(25 * 1024 * 1024);
        let pct = q.usage_pct();
        assert!((pct - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_zero_max_usage_pct() {
        let q = RamQuota::new(0);
        assert_eq!(q.usage_pct(), 0.0);
    }

    #[test]
    fn test_available_bytes() {
        let mut q = RamQuota::new(100);
        q.record(30 * 1024 * 1024);
        assert_eq!(q.available_bytes(), 70 * 1024 * 1024);
    }
}
