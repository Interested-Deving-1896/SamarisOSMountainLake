pub mod audit;
pub mod corruption;
pub mod eject_guard;
pub mod fsync_policy;
pub mod guards;
pub mod invariants;

use std::sync::atomic::{AtomicBool, Ordering};

use parking_lot::RwLock;

#[derive(Debug, Clone)]
pub struct SafetyPolicy {
    pub require_clean_journal: bool,
    pub force_flush_on_eject: bool,
    pub reject_corrupt_records: bool,
    pub read_only_fallback: bool,
    pub handle_surprise_removal: bool,
}

impl Default for SafetyPolicy {
    fn default() -> Self {
        SafetyPolicy {
            require_clean_journal: true,
            force_flush_on_eject: true,
            reject_corrupt_records: true,
            read_only_fallback: true,
            handle_surprise_removal: true,
        }
    }
}

#[derive(Debug)]
pub struct SafetyMonitor {
    safe_to_eject: AtomicBool,
    journal_dirty: AtomicBool,
    corruption_detected: AtomicBool,
    policy: RwLock<SafetyPolicy>,
}

impl SafetyMonitor {
    pub fn new() -> Self {
        SafetyMonitor {
            safe_to_eject: AtomicBool::new(false),
            journal_dirty: AtomicBool::new(false),
            corruption_detected: AtomicBool::new(false),
            policy: RwLock::new(SafetyPolicy::default()),
        }
    }

    pub fn mark_safe_to_eject(&self) {
        self.safe_to_eject.store(true, Ordering::Relaxed);
    }

    pub fn mark_unsafe_to_eject(&self) {
        self.safe_to_eject.store(false, Ordering::Relaxed);
    }

    pub fn is_safe_to_eject(&self) -> bool {
        if self.journal_dirty.load(Ordering::Relaxed) {
            return false;
        }
        if self.corruption_detected.load(Ordering::Relaxed) {
            return false;
        }
        self.safe_to_eject.load(Ordering::Relaxed)
    }

    pub fn mark_journal_dirty(&self) {
        self.journal_dirty.store(true, Ordering::Relaxed);
    }

    pub fn mark_journal_clean(&self) {
        self.journal_dirty.store(false, Ordering::Relaxed);
    }

    pub fn is_journal_dirty(&self) -> bool {
        self.journal_dirty.load(Ordering::Relaxed)
    }

    pub fn mark_corruption_detected(&self) {
        self.corruption_detected.store(true, Ordering::Relaxed);
    }

    pub fn is_corruption_detected(&self) -> bool {
        self.corruption_detected.load(Ordering::Relaxed)
    }

    pub fn set_policy(&self, policy: SafetyPolicy) {
        *self.policy.write() = policy;
    }

    pub fn policy(&self) -> SafetyPolicy {
        self.policy.read().clone()
    }
}

impl Default for SafetyMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_monitor_default() {
        let monitor = SafetyMonitor::new();
        assert!(!monitor.is_safe_to_eject());
        assert!(!monitor.is_journal_dirty());
    }

    #[test]
    fn test_safety_eject_flow() {
        let monitor = SafetyMonitor::new();
        monitor.mark_safe_to_eject();
        assert!(monitor.is_safe_to_eject());
        monitor.mark_journal_dirty();
        assert!(!monitor.is_safe_to_eject());
        monitor.mark_journal_clean();
        assert!(monitor.is_safe_to_eject());
    }

    #[test]
    fn test_safety_corruption() {
        let monitor = SafetyMonitor::new();
        monitor.mark_safe_to_eject();
        assert!(monitor.is_safe_to_eject());
        monitor.mark_corruption_detected();
        assert!(!monitor.is_safe_to_eject());
    }

    #[test]
    fn test_safety_policy() {
        let monitor = SafetyMonitor::new();
        let policy = SafetyPolicy {
            require_clean_journal: false,
            ..Default::default()
        };
        monitor.set_policy(policy);
        assert!(!monitor.policy().require_clean_journal);
    }
}
