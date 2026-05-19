#[derive(Debug, Clone)]
pub struct JournalStatus {
    pub dirty: bool,
    pub record_count: u64,
    pub bytes_written: u64,
    pub last_checkpoint: Option<u64>,
    pub last_clean_shutdown: bool,
    pub recovery_required: bool,
}

impl JournalStatus {
    pub fn needs_recovery(&self) -> bool {
        self.recovery_required || self.dirty
    }

    pub fn healthy(&self) -> bool {
        !self.dirty && self.last_clean_shutdown && !self.recovery_required
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_needs_recovery() {
        let s = JournalStatus {
            dirty: true,
            record_count: 0,
            bytes_written: 0,
            last_checkpoint: None,
            last_clean_shutdown: false,
            recovery_required: false,
        };
        assert!(s.needs_recovery());
    }

    #[test]
    fn test_clean_shutdown_no_recovery() {
        let s = JournalStatus {
            dirty: false,
            record_count: 100,
            bytes_written: 4096,
            last_checkpoint: Some(42),
            last_clean_shutdown: true,
            recovery_required: false,
        };
        assert!(!s.needs_recovery());
        assert!(s.healthy());
    }

    #[test]
    fn test_recovery_required_flag() {
        let s = JournalStatus {
            dirty: false,
            record_count: 0,
            bytes_written: 0,
            last_checkpoint: None,
            last_clean_shutdown: true,
            recovery_required: true,
        };
        assert!(s.needs_recovery());
        assert!(!s.healthy());
    }

    #[test]
    fn test_dirty_without_shutdown() {
        let s = JournalStatus {
            dirty: true,
            record_count: 50,
            bytes_written: 2048,
            last_checkpoint: None,
            last_clean_shutdown: false,
            recovery_required: false,
        };
        assert!(s.needs_recovery());
        assert!(!s.healthy());
    }

    #[test]
    fn test_dirty_with_shutdown() {
        let s = JournalStatus {
            dirty: true,
            record_count: 0,
            bytes_written: 0,
            last_checkpoint: None,
            last_clean_shutdown: true,
            recovery_required: false,
        };
        assert!(s.needs_recovery());
        assert!(!s.healthy());
    }

    #[test]
    fn test_fields_accessible() {
        let s = JournalStatus {
            dirty: false,
            record_count: 10,
            bytes_written: 1234,
            last_checkpoint: Some(5),
            last_clean_shutdown: true,
            recovery_required: false,
        };
        assert!(!s.dirty);
        assert_eq!(s.record_count, 10);
        assert_eq!(s.bytes_written, 1234);
        assert_eq!(s.last_checkpoint, Some(5));
        assert!(s.last_clean_shutdown);
    }
}
