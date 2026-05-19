use crate::core::error::VumError;
use crate::core::result::VumResult;

pub struct EjectGuard {
    safe: bool,
}

impl EjectGuard {
    pub fn new() -> Self {
        EjectGuard { safe: false }
    }

    pub fn check(&mut self, journal_dirty: bool, dirty_bytes: u64) -> VumResult<()> {
        if journal_dirty {
            self.safe = false;
            return Err(VumError::UnsafeToEject(format!(
                "Journal is dirty with {} bytes pending",
                dirty_bytes
            )));
        }
        if dirty_bytes > 0 {
            self.safe = false;
            return Err(VumError::UnsafeToEject(format!(
                "{} dirty bytes pending writeback",
                dirty_bytes
            )));
        }
        self.safe = true;
        Ok(())
    }

    pub fn override_safety(&mut self) {
        self.safe = true;
    }

    pub fn is_safe(&self) -> bool {
        self.safe
    }
}

impl Default for EjectGuard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_guard_not_safe() {
        let guard = EjectGuard::new();
        assert!(!guard.is_safe());
    }

    #[test]
    fn test_check_clean_journal() {
        let mut guard = EjectGuard::new();
        assert!(guard.check(false, 0).is_ok());
        assert!(guard.is_safe());
    }

    #[test]
    fn test_check_dirty_journal() {
        let mut guard = EjectGuard::new();
        let result = guard.check(true, 1024);
        assert!(result.is_err());
        assert!(!guard.is_safe());
    }

    #[test]
    fn test_check_dirty_bytes() {
        let mut guard = EjectGuard::new();
        let result = guard.check(false, 4096);
        assert!(result.is_err());
        assert!(!guard.is_safe());
    }

    #[test]
    fn test_override_safety() {
        let mut guard = EjectGuard::new();
        guard.override_safety();
        assert!(guard.is_safe());
    }
}
