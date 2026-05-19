use std::path::Path;

use nix::sys::statvfs::statvfs;

use crate::core::error::VumError;
use crate::core::result::VumResult;
use crate::device::detector::UsbDeviceDetector;
use crate::device::device_id::UsbDeviceId;

#[derive(Debug, Clone)]
pub struct BackingStore {
    pub path: String,
    pub device_id: Option<String>,
    pub is_removable: bool,
    pub is_read_only: bool,
    pub total_bytes: u64,
    pub available_bytes: u64,
}

impl BackingStore {
    pub fn open(path: &str) -> VumResult<Self> {
        let canonical = std::fs::canonicalize(path)
            .map_err(|e| VumError::BackingPathMissing(format!("{}: {}", path, e)))?;
        if !canonical.exists() {
            return Err(VumError::BackingPathMissing(format!(
                "Path does not exist: {}",
                canonical.display()
            )));
        }
        let metadata = std::fs::metadata(&canonical)
            .map_err(|e| VumError::BackingPathInvalid(format!("{}: {}", path, e)))?;
        let is_read_only = metadata.permissions().readonly();
        let is_removable = UsbDeviceDetector::is_removable(canonical.to_string_lossy().as_ref())
            .unwrap_or(false);
        let s = statvfs(&canonical)
            .map_err(|e| VumError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        let fragment_size = s.fragment_size() as u64;
        let total_bytes = s.blocks() as u64 * fragment_size;
        let available_bytes = s.blocks_available() as u64 * fragment_size;
        let device_id = UsbDeviceId::from_path(canonical.to_string_lossy().as_ref())
            .ok()
            .map(|id| id.0);
        Ok(BackingStore {
            path: canonical.to_string_lossy().to_string(),
            device_id,
            is_removable,
            is_read_only,
            total_bytes,
            available_bytes,
        })
    }

    pub fn validate(path: &str) -> VumResult<()> {
        let p = Path::new(path);
        if !p.exists() {
            return Err(VumError::BackingPathMissing(format!(
                "Backing path does not exist: {}",
                path
            )));
        }
        let metadata = std::fs::metadata(p)
            .map_err(|e| VumError::BackingPathInvalid(format!("{}: {}", path, e)))?;
        if metadata.len() == 0 && metadata.is_file() {
            return Err(VumError::BackingPathInvalid(format!(
                "Backing file is empty: {}",
                path
            )));
        }
        Ok(())
    }

    pub fn check_space(&self, needed: u64) -> VumResult<()> {
        if self.available_bytes < needed {
            return Err(VumError::BackingPathInvalid(format!(
                "Insufficient space: need {} bytes, available {} bytes",
                needed, self.available_bytes
            )));
        }
        Ok(())
    }

    pub fn refresh(&mut self) -> VumResult<()> {
        let s = statvfs(Path::new(&self.path))
            .map_err(|e| VumError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        let fragment_size = s.fragment_size() as u64;
        self.total_bytes = s.blocks() as u64 * fragment_size;
        self.available_bytes = s.blocks_available() as u64 * fragment_size;
        let metadata = std::fs::metadata(&self.path).ok();
        if let Some(m) = metadata {
            self.is_read_only = m.permissions().readonly();
        }
        Ok(())
    }

    pub fn is_same_device(&self, other: &str) -> bool {
        let canonical = match std::fs::canonicalize(other) {
            Ok(p) => p,
            Err(_) => return false,
        };
        let metadata_self = match std::fs::metadata(&self.path) {
            Ok(m) => m,
            Err(_) => return false,
        };
        let metadata_other = match std::fs::metadata(&canonical) {
            Ok(m) => m,
            Err(_) => return false,
        };
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            metadata_self.dev() == metadata_other.dev()
                && metadata_self.ino() == metadata_other.ino()
        }
        #[cfg(not(unix))]
        {
            let _ = metadata_other;
            self.path == canonical.to_string_lossy().as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_existing_directory() {
        let dir = std::env::temp_dir();
        assert!(BackingStore::validate(dir.to_string_lossy().as_ref()).is_ok());
    }

    #[test]
    fn test_validate_nonexistent_path() {
        let path = "/tmp/__vum_test_nonexistent_backing_volt";
        assert!(BackingStore::validate(path).is_err());
    }

    #[test]
    fn test_check_space_sufficient() {
        let store = BackingStore {
            path: "/tmp".into(),
            device_id: None,
            is_removable: false,
            is_read_only: false,
            total_bytes: 1_000_000_000,
            available_bytes: 500_000_000,
        };
        assert!(store.check_space(100_000).is_ok());
    }

    #[test]
    fn test_check_space_insufficient() {
        let store = BackingStore {
            path: "/tmp".into(),
            device_id: None,
            is_removable: false,
            is_read_only: false,
            total_bytes: 1_000_000_000,
            available_bytes: 100,
        };
        assert!(store.check_space(1_000_000).is_err());
    }

    #[test]
    fn test_is_same_device_self() {
        let dir = std::env::temp_dir();
        let path = dir.to_string_lossy().to_string();
        let store = BackingStore {
            path: path.clone(),
            device_id: None,
            is_removable: false,
            is_read_only: false,
            total_bytes: 0,
            available_bytes: 0,
        };
        assert!(store.is_same_device(&path));
    }

    #[test]
    fn test_refresh_updates_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_string_lossy().to_string();
        let mut store = BackingStore {
            path: path.clone(),
            device_id: None,
            is_removable: false,
            is_read_only: false,
            total_bytes: 0,
            available_bytes: 0,
        };
        assert!(store.refresh().is_ok());
        assert!(store.total_bytes > 0);
        assert!(store.available_bytes > 0);
    }
}
