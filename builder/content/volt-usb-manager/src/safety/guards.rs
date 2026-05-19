use crate::core::error::VumError;
use crate::core::result::VumResult;

pub struct WriteGuard {
    allowed: bool,
}

impl WriteGuard {
    pub fn new(read_only: bool) -> Self {
        WriteGuard {
            allowed: !read_only,
        }
    }

    pub fn authorize(&self) -> VumResult<()> {
        if self.allowed {
            Ok(())
        } else {
            Err(VumError::ReadOnlyFilesystem)
        }
    }
}

pub struct PathGuard;

impl PathGuard {
    pub fn validate(path: &str, backing: &str) -> VumResult<String> {
        let canonical = std::fs::canonicalize(path)
            .map_err(|_| VumError::InvalidPath(format!("Cannot resolve path: {}", path)))?;
        let canonical_backing = std::fs::canonicalize(backing)
            .map_err(|_| VumError::InvalidPath(format!("Cannot resolve backing: {}", backing)))?;

        let path_str = canonical.to_string_lossy();
        let backing_str = canonical_backing.to_string_lossy();

        if !path_str.starts_with(backing_str.as_ref()) {
            return Err(VumError::PathTraversalRejected(format!(
                "Path '{}' is outside backing store '{}'",
                path_str, backing_str
            )));
        }

        let relative = path_str
            .strip_prefix(backing_str.as_ref())
            .unwrap_or("")
            .trim_start_matches('/');

        if relative.contains("..") {
            return Err(VumError::PathTraversalRejected(format!(
                "Path contains parent traversal: {}",
                path_str
            )));
        }

        Ok(path_str.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_guard_allowed() {
        let guard = WriteGuard::new(false);
        assert!(guard.authorize().is_ok());
    }

    #[test]
    fn test_write_guard_read_only() {
        let guard = WriteGuard::new(true);
        assert!(guard.authorize().is_err());
    }

    #[test]
    fn test_path_guard_valid() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, "content").unwrap();

        let result = PathGuard::validate(
            file_path.to_string_lossy().as_ref(),
            dir.path().to_string_lossy().as_ref(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_path_guard_outside_backing() {
        let dir1 = tempfile::tempdir().unwrap();
        let dir2 = tempfile::tempdir().unwrap();
        let file_path = dir2.path().join("outside.txt");
        std::fs::write(&file_path, "content").unwrap();

        let result = PathGuard::validate(
            file_path.to_string_lossy().as_ref(),
            dir1.path().to_string_lossy().as_ref(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_path_guard_invalid_path() {
        let dir = tempfile::tempdir().unwrap();
        let result = PathGuard::validate(
            "/tmp/__vum_test_nonexistent_path_guard_xyz",
            dir.path().to_string_lossy().as_ref(),
        );
        assert!(result.is_err());
    }
}
