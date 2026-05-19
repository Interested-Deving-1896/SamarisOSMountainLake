use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

use crate::core::result::VrmResult;

#[derive(Clone)]
pub struct AnimationGuard {
    active: Arc<AtomicBool>,
    depth: Arc<AtomicU32>,
}

impl AnimationGuard {
    pub fn new() -> Self {
        Self {
            active: Arc::new(AtomicBool::new(false)),
            depth: Arc::new(AtomicU32::new(0)),
        }
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }

    pub fn depth(&self) -> u32 {
        self.depth.load(Ordering::SeqCst)
    }

    pub fn enter(&self) -> AnimationHandle {
        self.depth.fetch_add(1, Ordering::SeqCst);
        self.active.store(true, Ordering::SeqCst);
        AnimationHandle {
            active: Arc::clone(&self.active),
            depth: Arc::clone(&self.depth),
        }
    }

    pub fn wait_for_idle(&self, timeout_ms: u64) -> VrmResult<()> {
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_millis(timeout_ms);
        while self.is_active() {
            if start.elapsed() > timeout {
                return Err(crate::core::error::VrmError::Other(
                    "timeout waiting for animation guard to become idle".into(),
                ));
            }
            std::thread::yield_now();
        }
        Ok(())
    }
}

impl Default for AnimationGuard {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AnimationHandle {
    active: Arc<AtomicBool>,
    depth: Arc<AtomicU32>,
}

impl AnimationHandle {
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::SeqCst)
    }
}

impl Drop for AnimationHandle {
    fn drop(&mut self) {
        let prev = self.depth.fetch_sub(1, Ordering::SeqCst);
        if prev <= 1 {
            self.active.store(false, Ordering::SeqCst);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_initially_inactive() {
        let guard = AnimationGuard::new();
        assert!(!guard.is_active());
        assert_eq!(guard.depth(), 0);
    }

    #[test]
    fn test_enter_and_drop() {
        let guard = AnimationGuard::new();
        {
            let handle = guard.enter();
            assert!(handle.is_active());
            assert!(guard.is_active());
            assert_eq!(guard.depth(), 1);
        }
        assert!(!guard.is_active());
        assert_eq!(guard.depth(), 0);
    }

    #[test]
    fn test_nested_enter() {
        let guard = AnimationGuard::new();
        let h1 = guard.enter();
        assert!(guard.is_active());
        assert_eq!(guard.depth(), 1);
        let h2 = guard.enter();
        assert_eq!(guard.depth(), 2);
        drop(h2);
        assert!(guard.is_active());
        assert_eq!(guard.depth(), 1);
        drop(h1);
        assert!(!guard.is_active());
        assert_eq!(guard.depth(), 0);
    }

    #[test]
    fn test_wait_for_idle_immediate() {
        let guard = AnimationGuard::new();
        assert!(guard.wait_for_idle(100).is_ok());
    }

    #[test]
    fn test_clone() {
        let g1 = AnimationGuard::new();
        let g2 = g1.clone();
        let h = g1.enter();
        assert!(g2.is_active());
        drop(h);
        assert!(!g2.is_active());
    }
}
