use std::sync::atomic::{AtomicU64, Ordering};

use parking_lot::Mutex;

pub struct GlobalPool {
    allocations: Mutex<Vec<Vec<u8>>>,
    total_bytes: AtomicU64,
    #[allow(dead_code)]
    max_bytes: u64,
}

impl GlobalPool {
    pub fn new(max_bytes: u64) -> Self {
        GlobalPool {
            allocations: Mutex::new(Vec::new()),
            total_bytes: AtomicU64::new(0),
            max_bytes,
        }
    }

    pub fn allocate(&self, size: u64) -> Vec<u8> {
        let mut allocs = self.allocations.lock();
        let reused = allocs.pop();
        self.total_bytes.fetch_add(size, Ordering::Relaxed);
        match reused {
            Some(mut buf) => {
                buf.resize(size as usize, 0);
                buf
            }
            None => vec![0u8; size as usize],
        }
    }

    pub fn deallocate(&self, data: Vec<u8>) {
        let size = data.len() as u64;
        self.total_bytes.fetch_sub(size, Ordering::Relaxed);
        let mut allocs = self.allocations.lock();
        allocs.push(data);
    }

    pub fn total_allocated(&self) -> u64 {
        self.total_bytes.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_pool_allocate() {
        let pool = GlobalPool::new(10_000_000);
        let buf = pool.allocate(1024);
        assert_eq!(buf.len(), 1024);
        assert_eq!(pool.total_allocated(), 1024);
    }

    #[test]
    fn test_deallocate_and_reuse() {
        let pool = GlobalPool::new(10_000_000);
        let buf = pool.allocate(256);
        pool.deallocate(buf);
        assert_eq!(pool.total_allocated(), 0);
        let reused = pool.allocate(256);
        assert_eq!(reused.len(), 256);
        assert!(pool.total_allocated() > 0);
    }

    #[test]
    fn test_resize_on_reuse() {
        let pool = GlobalPool::new(10_000_000);
        pool.deallocate(pool.allocate(100));
        let buf = pool.allocate(200);
        assert_eq!(buf.len(), 200);
    }
}
