#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsyncPolicy {
    Always,
    MetadataOnly,
    Never,
}

impl FsyncPolicy {
    pub fn should_fsync(&self, is_metadata: bool) -> bool {
        match self {
            FsyncPolicy::Always => true,
            FsyncPolicy::MetadataOnly => is_metadata,
            FsyncPolicy::Never => false,
        }
    }
}

impl Default for FsyncPolicy {
    fn default() -> Self {
        FsyncPolicy::Always
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_fsyncs() {
        let policy = FsyncPolicy::Always;
        assert!(policy.should_fsync(true));
        assert!(policy.should_fsync(false));
    }

    #[test]
    fn test_never_does_not_fsync() {
        let policy = FsyncPolicy::Never;
        assert!(!policy.should_fsync(true));
        assert!(!policy.should_fsync(false));
    }

    #[test]
    fn test_metadata_only() {
        let policy = FsyncPolicy::MetadataOnly;
        assert!(policy.should_fsync(true));
        assert!(!policy.should_fsync(false));
    }

    #[test]
    fn test_default_is_always() {
        let policy: FsyncPolicy = Default::default();
        assert_eq!(policy, FsyncPolicy::Always);
    }

    #[test]
    fn test_clone_and_copy() {
        let a = FsyncPolicy::MetadataOnly;
        let b = a;
        assert_eq!(a, b);
    }
}
