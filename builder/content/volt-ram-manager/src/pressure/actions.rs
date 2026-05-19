use std::sync::atomic::{AtomicBool, Ordering};
use crate::pressure::level::PressureLevel;
use crate::core::result::VrmResult;

pub struct PressureActions {
    pub throttle_background: AtomicBool,
    pub evict_cache: AtomicBool,
    pub gc_signaled: AtomicBool,
    pub suspend_idle: AtomicBool,
}

impl PressureActions {
    pub fn new() -> Self {
        PressureActions {
            throttle_background: AtomicBool::new(false),
            evict_cache: AtomicBool::new(false),
            gc_signaled: AtomicBool::new(false),
            suspend_idle: AtomicBool::new(false),
        }
    }

    pub fn apply(&self, level: PressureLevel) -> VrmResult<()> {
        match level {
            PressureLevel::Green => {
                self.clear();
            }
            PressureLevel::Yellow => {
                self.throttle_background.store(true, Ordering::SeqCst);
                self.evict_cache.store(false, Ordering::SeqCst);
                self.gc_signaled.store(false, Ordering::SeqCst);
                self.suspend_idle.store(false, Ordering::SeqCst);
            }
            PressureLevel::Orange => {
                self.throttle_background.store(true, Ordering::SeqCst);
                self.evict_cache.store(true, Ordering::SeqCst);
                self.gc_signaled.store(true, Ordering::SeqCst);
                self.suspend_idle.store(false, Ordering::SeqCst);
            }
            PressureLevel::Red => {
                self.throttle_background.store(true, Ordering::SeqCst);
                self.evict_cache.store(true, Ordering::SeqCst);
                self.gc_signaled.store(true, Ordering::SeqCst);
                self.suspend_idle.store(true, Ordering::SeqCst);
            }
        }
        Ok(())
    }

    pub fn clear(&self) {
        self.throttle_background.store(false, Ordering::SeqCst);
        self.evict_cache.store(false, Ordering::SeqCst);
        self.gc_signaled.store(false, Ordering::SeqCst);
        self.suspend_idle.store(false, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_all_false() {
        let a = PressureActions::new();
        assert!(!a.throttle_background.load(Ordering::SeqCst));
        assert!(!a.evict_cache.load(Ordering::SeqCst));
        assert!(!a.gc_signaled.load(Ordering::SeqCst));
        assert!(!a.suspend_idle.load(Ordering::SeqCst));
    }

    #[test]
    fn test_green_clears_all() {
        let a = PressureActions::new();
        a.apply(PressureLevel::Red).unwrap();
        a.apply(PressureLevel::Green).unwrap();
        assert!(!a.throttle_background.load(Ordering::SeqCst));
        assert!(!a.evict_cache.load(Ordering::SeqCst));
        assert!(!a.gc_signaled.load(Ordering::SeqCst));
        assert!(!a.suspend_idle.load(Ordering::SeqCst));
    }

    #[test]
    fn test_yellow() {
        let a = PressureActions::new();
        a.apply(PressureLevel::Yellow).unwrap();
        assert!(a.throttle_background.load(Ordering::SeqCst));
        assert!(!a.evict_cache.load(Ordering::SeqCst));
        assert!(!a.gc_signaled.load(Ordering::SeqCst));
        assert!(!a.suspend_idle.load(Ordering::SeqCst));
    }

    #[test]
    fn test_orange() {
        let a = PressureActions::new();
        a.apply(PressureLevel::Orange).unwrap();
        assert!(a.throttle_background.load(Ordering::SeqCst));
        assert!(a.evict_cache.load(Ordering::SeqCst));
        assert!(a.gc_signaled.load(Ordering::SeqCst));
        assert!(!a.suspend_idle.load(Ordering::SeqCst));
    }

    #[test]
    fn test_red() {
        let a = PressureActions::new();
        a.apply(PressureLevel::Red).unwrap();
        assert!(a.throttle_background.load(Ordering::SeqCst));
        assert!(a.evict_cache.load(Ordering::SeqCst));
        assert!(a.gc_signaled.load(Ordering::SeqCst));
        assert!(a.suspend_idle.load(Ordering::SeqCst));
    }
}
