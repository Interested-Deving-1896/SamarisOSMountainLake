use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::error::VumError;
use crate::core::result::VumResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub timestamp: u64,
    pub record_count: u64,
    pub last_record_id: u64,
}

impl Checkpoint {
    fn checkpoint_path(path: &str) -> std::path::PathBuf {
        let p = Path::new(path);
        let ext = p.extension().map(|e| e.to_str().unwrap_or("")).unwrap_or("").to_string();
        if ext.is_empty() {
            Path::new(&format!("{}.checkpoint", path)).to_path_buf()
        } else {
            p.with_extension(format!("{}.checkpoint", ext))
        }
    }

    pub fn write(path: &str, last_id: u64) -> VumResult<()> {
        let cp = Checkpoint {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_micros() as u64,
            record_count: 0,
            last_record_id: last_id,
        };
        let json = serde_json::to_string_pretty(&cp)
            .map_err(|e| VumError::WriteFailed(format!("Checkpoint serialize: {}", e)))?;
        std::fs::write(Self::checkpoint_path(path), &json)
            .map_err(|e| VumError::WriteFailed(format!("Checkpoint write: {}", e)))?;
        Ok(())
    }

    pub fn read(path: &str) -> VumResult<Self> {
        let cp_path = Self::checkpoint_path(path);
        let json = std::fs::read_to_string(&cp_path)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    VumError::FileNotFound(format!("Checkpoint not found: {}", cp_path.display()))
                } else {
                    VumError::ReadFailed(format!("Checkpoint read: {}", e))
                }
            })?;
        serde_json::from_str(&json)
            .map_err(|e| VumError::JournalCorrupt(format!("Checkpoint parse: {}", e)))
    }

    pub fn clear(path: &str) -> VumResult<()> {
        let cp_path = Self::checkpoint_path(path);
        if cp_path.exists() {
            std::fs::remove_file(&cp_path)
                .map_err(|e| VumError::WriteFailed(format!("Checkpoint clear: {}", e)))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_checkpoint_write_and_read() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("journal.wal").to_str().unwrap().to_string();
        Checkpoint::write(&path, 42).unwrap();
        let cp = Checkpoint::read(&path).unwrap();
        assert_eq!(cp.last_record_id, 42);
        assert!(cp.timestamp > 0);
    }

    #[test]
    fn test_checkpoint_clear() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("journal.wal").to_str().unwrap().to_string();
        Checkpoint::write(&path, 1).unwrap();
        assert!(Checkpoint::read(&path).is_ok());
        Checkpoint::clear(&path).unwrap();
        assert!(Checkpoint::read(&path).is_err());
    }

    #[test]
    fn test_checkpoint_clear_nonexistent() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent").to_str().unwrap().to_string();
        Checkpoint::clear(&path).unwrap();
    }

    #[test]
    fn test_checkpoint_read_not_found() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("no_cp_here").to_str().unwrap().to_string();
        let result = Checkpoint::read(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_checkpoint_write_read_empty_last_id() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty_id").to_str().unwrap().to_string();
        Checkpoint::write(&path, 0).unwrap();
        let cp = Checkpoint::read(&path).unwrap();
        assert_eq!(cp.last_record_id, 0);
        assert!(cp.timestamp > 0);
    }

    #[test]
    fn test_checkpoint_multiple_writes() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("multi").to_str().unwrap().to_string();
        Checkpoint::write(&path, 10).unwrap();
        Checkpoint::write(&path, 20).unwrap();
        let cp = Checkpoint::read(&path).unwrap();
        assert_eq!(cp.last_record_id, 20);
    }

    #[test]
    fn test_checkpoint_file_extension_handling() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("plain").to_str().unwrap().to_string();
        Checkpoint::write(&path, 99).unwrap();
        let cp = Checkpoint::read(&path).unwrap();
        assert_eq!(cp.last_record_id, 99);
    }
}
