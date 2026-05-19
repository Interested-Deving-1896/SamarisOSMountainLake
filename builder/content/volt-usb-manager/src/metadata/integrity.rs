use std::path::Path;

use crate::core::result::VumResult;
use crate::core::error::VumError;

pub struct IntegrityChecker;

impl IntegrityChecker {
    pub fn check_path(path: &str) -> VumResult<()> {
        if path.is_empty() {
            return Err(VumError::InvalidPath("Path is empty".into()));
        }
        if path.contains('\0') {
            return Err(VumError::InvalidPath(
                "Path contains null bytes".into(),
            ));
        }
        Ok(())
    }

    pub fn check_path_traversal(path: &str) -> VumResult<()> {
        if path.contains("..") {
            return Err(VumError::PathTraversalRejected(path.to_string()));
        }
        Ok(())
    }

    pub fn check_within_backing(path: &str, backing: &str) -> VumResult<String> {
        let canonical = Path::new(path)
            .canonicalize()
            .map_err(|_| VumError::InvalidPath(path.to_string()))?;
        let canonical_str = canonical
            .to_str()
            .ok_or_else(|| VumError::InvalidPath(path.to_string()))?;

        let backing_canonical = Path::new(backing)
            .canonicalize()
            .map_err(|_| VumError::BackingPathMissing(backing.to_string()))?;
        let backing_str = backing_canonical
            .to_str()
            .ok_or_else(|| VumError::BackingPathInvalid(backing.to_string()))?;

        if !canonical_str.starts_with(backing_str) {
            return Err(VumError::PathTraversalRejected(path.to_string()));
        }
        Ok(canonical_str.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_path_valid() {
        assert!(IntegrityChecker::check_path("/valid/path").is_ok());
    }

    #[test]
    fn test_check_path_empty() {
        assert!(IntegrityChecker::check_path("").is_err());
    }

    #[test]
    fn test_check_path_null_byte() {
        assert!(IntegrityChecker::check_path("/bad\0path").is_err());
    }

    #[test]
    fn test_check_path_traversal_detected() {
        let result = IntegrityChecker::check_path_traversal("/safe/../../etc");
        assert!(result.is_err());
    }

    #[test]
    fn test_check_path_traversal_clean() {
        assert!(
            IntegrityChecker::check_path_traversal("/safe/path")
                .is_ok()
        );
    }

    #[test]
    fn test_check_path_traversal_encoded() {
        assert!(
            IntegrityChecker::check_path_traversal("/safe/..\\stuff")
                .is_err()
        );
    }

    #[test]
    fn test_check_within_backing_valid() {
        let result = IntegrityChecker::check_within_backing(
            "Cargo.toml",
            ".",
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_within_backing_invalid_backing() {
        let result = IntegrityChecker::check_within_backing(
            "Cargo.toml",
            "/nonexistent_backing_path_xyz",
        );
        assert!(result.is_err());
    }
}
