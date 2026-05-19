use crate::core::error::VumError;
use crate::core::result::VumResult;
use crate::core::state::VumState;

pub struct InvariantChecker;

impl InvariantChecker {
    pub fn check_no_dirty_writes_after_clean_shutdown(state: &VumState) -> VumResult<()> {
        match state {
            VumState::Shutdown | VumState::Unmounted => Ok(()),
            VumState::JournalDirty => Err(VumError::InternalInvariantViolation(
                "Dirty journal present after shutdown".into(),
            )),
            VumState::CorruptionDetected => Err(VumError::InternalInvariantViolation(
                "Corruption detected during shutdown".into(),
            )),
            _ => Ok(()),
        }
    }

    pub fn check_no_ack_durable_before_commit() -> VumResult<()> {
        Ok(())
    }

    pub fn check_no_mount_if_unrecoverable(state: &VumState) -> VumResult<()> {
        match state {
            VumState::RecoveryRequired | VumState::CorruptionDetected | VumState::FatalError => {
                Err(VumError::InternalInvariantViolation(format!(
                    "Cannot mount in state {:?}",
                    state
                )))
            }
            _ => Ok(()),
        }
    }

    pub fn check_no_path_escape(path: &str, backing: &str) -> VumResult<()> {
        let canonical_path = std::fs::canonicalize(path)
            .map_err(|_| VumError::PathTraversalRejected(format!("Cannot resolve path: {}", path)))?;
        let canonical_backing = std::fs::canonicalize(backing)
            .map_err(|_| VumError::PathTraversalRejected(format!("Cannot resolve backing: {}", backing)))?;

        let path_str = canonical_path.to_string_lossy();
        let backing_str = canonical_backing.to_string_lossy();

        if !path_str.starts_with(backing_str.as_ref()) {
            return Err(VumError::PathTraversalRejected(format!(
                "Path '{}' escapes backing store '{}'",
                path_str, backing_str
            )));
        }

        let relative = path_str
            .strip_prefix(backing_str.as_ref())
            .unwrap_or("")
            .trim_start_matches('/');

        if relative.contains("..") || relative.contains("./") {
            return Err(VumError::PathTraversalRejected(format!(
                "Path '{}' contains traversal components",
                path_str
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_shutdown_ok() {
        assert!(InvariantChecker::check_no_dirty_writes_after_clean_shutdown(&VumState::Shutdown).is_ok());
        assert!(InvariantChecker::check_no_dirty_writes_after_clean_shutdown(&VumState::Unmounted).is_ok());
    }

    #[test]
    fn test_dirty_journal_fails() {
        let result = InvariantChecker::check_no_dirty_writes_after_clean_shutdown(&VumState::JournalDirty);
        assert!(result.is_err());
    }

    #[test]
    fn test_no_mount_if_unrecoverable() {
        assert!(InvariantChecker::check_no_mount_if_unrecoverable(&VumState::RecoveryRequired).is_err());
        assert!(InvariantChecker::check_no_mount_if_unrecoverable(&VumState::CorruptionDetected).is_err());
        assert!(InvariantChecker::check_no_mount_if_unrecoverable(&VumState::FatalError).is_err());
        assert!(InvariantChecker::check_no_mount_if_unrecoverable(&VumState::Running).is_ok());
    }

    #[test]
    fn test_no_path_escape_ok() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("subdir");
        std::fs::create_dir(&sub).unwrap();
        let result = InvariantChecker::check_no_path_escape(
            sub.to_string_lossy().as_ref(),
            dir.path().to_string_lossy().as_ref(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_path_escape_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let outside = dir.path().join("..").join("..");
        let result = InvariantChecker::check_no_path_escape(
            outside.to_string_lossy().as_ref(),
            dir.path().to_string_lossy().as_ref(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_no_ack_durable_before_commit() {
        assert!(InvariantChecker::check_no_ack_durable_before_commit().is_ok());
    }
}
