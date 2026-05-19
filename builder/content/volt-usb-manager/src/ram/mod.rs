pub mod allocation;
pub mod pressure;
pub mod quota;
pub mod volt_ram_client;

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

#[derive(Debug)]
pub struct RamClient {
    quota_bytes: u64,
    used_bytes: AtomicU64,
    pressure_backoff: AtomicBool,
}

impl RamClient {
    pub fn new() -> Self {
        RamClient {
            quota_bytes: 512 * 1024 * 1024,
            used_bytes: AtomicU64::new(0),
            pressure_backoff: AtomicBool::new(false),
        }
    }

    pub fn allocate(&self, size: u64) -> bool {
        let used = self.used_bytes.load(Ordering::Relaxed);
        if used + size > self.quota_bytes {
            self.pressure_backoff.store(true, Ordering::Relaxed);
            return false;
        }
        self.used_bytes.fetch_add(size, Ordering::Relaxed);
        true
    }

    pub fn deallocate(&self, size: u64) {
        let used = self.used_bytes.load(Ordering::Relaxed);
        if size > used {
            self.used_bytes.store(0, Ordering::Relaxed);
        } else {
            self.used_bytes.fetch_sub(size, Ordering::Relaxed);
        }
        if self.used_bytes.load(Ordering::Relaxed) < self.quota_bytes / 2 {
            self.pressure_backoff.store(false, Ordering::Relaxed);
        }
    }

    pub fn used(&self) -> u64 {
        self.used_bytes.load(Ordering::Relaxed)
    }

    pub fn available(&self) -> u64 {
        self.quota_bytes - self.used_bytes.load(Ordering::Relaxed)
    }

    pub fn quota(&self) -> u64 {
        self.quota_bytes
    }

    pub fn set_quota(&mut self, quota_mb: u64) {
        self.quota_bytes = quota_mb * 1024 * 1024;
    }

    pub fn under_pressure(&self) -> bool {
        self.pressure_backoff.load(Ordering::Relaxed)
    }

    pub fn free(&self) {
        self.used_bytes.store(0, Ordering::Relaxed);
        self.pressure_backoff.store(false, Ordering::Relaxed);
    }
}

impl Default for RamClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_client_allocate() {
        let client = RamClient::new();
        assert!(client.allocate(1024));
        assert_eq!(client.used(), 1024);
        assert_eq!(client.available(), client.quota() - 1024);
    }

    #[test]
    fn test_ram_client_pressure() {
        let mut client = RamClient::new();
        client.set_quota(1);
        assert!(client.allocate(512 * 1024));
        assert!(client.allocate(512 * 1024));
        assert!(!client.allocate(1));
        assert!(client.under_pressure());
    }

    #[test]
    fn test_ram_client_free() {
        let client = RamClient::new();
        client.allocate(4096);
        client.deallocate(2048);
        assert_eq!(client.used(), 2048);
        client.free();
        assert_eq!(client.used(), 0);
    }
}
