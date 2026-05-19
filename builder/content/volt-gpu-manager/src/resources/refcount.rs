use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub struct RefCounter {
    count: AtomicU64,
}

impl RefCounter {
    pub fn new() -> Self {
        Self {
            count: AtomicU64::new(1),
        }
    }

    pub fn with_count(initial: u64) -> Self {
        Self {
            count: AtomicU64::new(initial),
        }
    }

    pub fn increment(&self) -> u64 {
        self.count.fetch_add(1, Ordering::AcqRel) + 1
    }

    pub fn decrement(&self) -> u64 {
        self.count.fetch_sub(1, Ordering::AcqRel) - 1
    }

    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Acquire)
    }

    pub fn is_zero(&self) -> bool {
        self.count() == 0
    }

    pub fn reset(&self) {
        self.count.store(0, Ordering::Release);
    }
}

impl Default for RefCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for RefCounter {
    fn clone(&self) -> Self {
        Self {
            count: AtomicU64::new(self.count.load(Ordering::Acquire)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_starts_at_one() {
        let rc = RefCounter::new();
        assert_eq!(rc.count(), 1);
        assert!(!rc.is_zero());
    }

    #[test]
    fn test_increment() {
        let rc = RefCounter::new();
        let new_count = rc.increment();
        assert_eq!(new_count, 2);
        assert_eq!(rc.count(), 2);
    }

    #[test]
    fn test_decrement() {
        let rc = RefCounter::new();
        rc.increment();
        let new_count = rc.decrement();
        assert_eq!(new_count, 1);
        assert_eq!(rc.count(), 1);
    }

    #[test]
    fn test_decrement_to_zero() {
        let rc = RefCounter::new();
        let count = rc.decrement();
        assert_eq!(count, 0);
        assert!(rc.is_zero());
    }

    #[test]
    fn test_with_count() {
        let rc = RefCounter::with_count(10);
        assert_eq!(rc.count(), 10);
    }

    #[test]
    fn test_reset() {
        let rc = RefCounter::new();
        rc.increment();
        rc.increment();
        assert_eq!(rc.count(), 3);
        rc.reset();
        assert_eq!(rc.count(), 0);
        assert!(rc.is_zero());
    }

    #[test]
    fn test_clone() {
        let rc = RefCounter::with_count(5);
        let cloned = rc.clone();
        assert_eq!(cloned.count(), 5);
        cloned.increment();
        assert_eq!(rc.count(), 5);
        assert_eq!(cloned.count(), 6);
    }

    #[test]
    fn test_default() {
        let rc: RefCounter = Default::default();
        assert_eq!(rc.count(), 1);
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;
        let rc = std::sync::Arc::new(RefCounter::new());
        let mut handles = vec![];
        for _ in 0..10 {
            let rc_clone = rc.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    rc_clone.increment();
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(rc.count(), 1 + 1000);
    }
}
