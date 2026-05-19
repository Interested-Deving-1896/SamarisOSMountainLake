use crate::apps::app_id::AppId;

#[derive(Debug, Clone, Copy)]
pub struct AppQuota {
    pub max_memory_bytes: u64,
    pub max_concurrent_tasks: u32,
    pub max_allocations_per_sec: u32,
    pub current_usage: u64,
}

impl AppQuota {
    pub fn new(max_mb: u64) -> Self {
        Self {
            max_memory_bytes: max_mb.saturating_mul(1024 * 1024),
            max_concurrent_tasks: 64,
            max_allocations_per_sec: 10000,
            current_usage: 0,
        }
    }

    pub fn with_concurrent_tasks(mut self, tasks: u32) -> Self {
        self.max_concurrent_tasks = tasks;
        self
    }

    pub fn with_allocations_per_sec(mut self, allocs: u32) -> Self {
        self.max_allocations_per_sec = allocs;
        self
    }

    pub fn remaining(&self) -> u64 {
        self.max_memory_bytes.saturating_sub(self.current_usage)
    }

    pub fn is_exceeded(&self) -> bool {
        self.current_usage >= self.max_memory_bytes
    }

    pub fn usage_percent(&self) -> f64 {
        if self.max_memory_bytes == 0 {
            return 0.0;
        }
        (self.current_usage as f64 / self.max_memory_bytes as f64) * 100.0
    }

    pub fn would_exceed(&self, additional: u64) -> bool {
        self.current_usage.saturating_add(additional) > self.max_memory_bytes
    }
}

impl From<AppId> for AppQuota {
    fn from(id: AppId) -> Self {
        AppQuota::new(id.as_u64().max(64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_quota() {
        let q = AppQuota::new(64);
        assert_eq!(q.max_memory_bytes, 64 * 1024 * 1024);
        assert_eq!(q.current_usage, 0);
    }

    #[test]
    fn test_remaining() {
        let mut q = AppQuota::new(64);
        q.current_usage = 32 * 1024 * 1024;
        assert_eq!(q.remaining(), 32 * 1024 * 1024);
    }

    #[test]
    fn test_is_exceeded() {
        let mut q = AppQuota::new(64);
        assert!(!q.is_exceeded());
        q.current_usage = 65 * 1024 * 1024;
        assert!(q.is_exceeded());
    }

    #[test]
    fn test_would_exceed() {
        let q = AppQuota::new(10);
        assert!(q.would_exceed(11 * 1024 * 1024));
        assert!(!q.would_exceed(5 * 1024 * 1024));
    }

    #[test]
    fn test_usage_percent() {
        let mut q = AppQuota::new(100);
        q.current_usage = 50 * 1024 * 1024;
        assert!((q.usage_percent() - 50.0).abs() < f64::EPSILON);
    }
}
