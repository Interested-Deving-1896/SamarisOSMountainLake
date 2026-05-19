use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::core::error::VumError;
use crate::core::result::VumResult;

pub struct CorruptionDetector;

impl CorruptionDetector {
    pub fn check_journal_integrity(path: &str) -> VumResult<bool> {
        let p = Path::new(path);
        if !p.exists() {
            return Err(VumError::FileNotFound(format!("Journal not found: {}", path)));
        }
        let metadata = std::fs::metadata(p)
            .map_err(|e| VumError::JournalReadFailed(format!("Cannot stat journal: {}", e)))?;

        if metadata.len() == 0 {
            return Ok(false);
        }

        let mut file = std::fs::File::open(p)
            .map_err(|e| VumError::JournalReadFailed(format!("Cannot open journal: {}", e)))?;

        let mut buffer = Vec::with_capacity(metadata.len() as usize);
        file.read_to_end(&mut buffer)
            .map_err(|e| VumError::JournalReadFailed(format!("Cannot read journal: {}", e)))?;

        if buffer.len() < 8 {
            return Ok(false);
        }

        let stored_crc = u32::from_le_bytes([
            buffer[buffer.len() - 4],
            buffer[buffer.len() - 3],
            buffer[buffer.len() - 2],
            buffer[buffer.len() - 1],
        ]);
        let data_to_check = &buffer[..buffer.len() - 4];
        let computed_crc = crc32fast::hash(data_to_check);

        Ok(computed_crc == stored_crc)
    }

    pub fn check_file_integrity(path: &str, expected_crc: u32) -> VumResult<bool> {
        let p = Path::new(path);
        if !p.exists() {
            return Err(VumError::FileNotFound(format!("File not found: {}", path)));
        }

        let mut file = std::fs::File::open(p)
            .map_err(|e| VumError::ReadFailed(format!("Cannot open file: {}", e)))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| VumError::ReadFailed(format!("Cannot read file: {}", e)))?;

        let computed_crc = crc32fast::hash(&buffer);
        Ok(computed_crc == expected_crc)
    }

    pub fn compute_sha256(path: &str) -> VumResult<String> {
        let p = Path::new(path);
        if !p.exists() {
            return Err(VumError::FileNotFound(format!("File not found: {}", path)));
        }

        let mut file = std::fs::File::open(p)
            .map_err(|e| VumError::ReadFailed(format!("Cannot open file: {}", e)))?;

        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 8192];
        loop {
            let n = file
                .read(&mut buffer)
                .map_err(|e| VumError::ReadFailed(format!("Read error: {}", e)))?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_check_journal_nonexistent() {
        let result = CorruptionDetector::check_journal_integrity(
            "/tmp/__vum_test_nonexistent_journal_corruption",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_check_journal_empty() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty.journal");
        std::fs::write(&path, b"").unwrap();

        let result =
            CorruptionDetector::check_journal_integrity(path.to_string_lossy().as_ref());
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_check_file_integrity_match() {
        let data = b"hello world, this is test data for integrity check";
        let crc = crc32fast::hash(data);
        let dir = tempdir().unwrap();
        let path = dir.path().join("integrity.dat");
        std::fs::write(&path, data).unwrap();

        let result =
            CorruptionDetector::check_file_integrity(path.to_string_lossy().as_ref(), crc);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_check_file_integrity_mismatch() {
        let data = b"original data";
        let wrong_crc = 0xDEADBEEF;
        let dir = tempdir().unwrap();
        let path = dir.path().join("mismatch.dat");
        std::fs::write(&path, data).unwrap();

        let result =
            CorruptionDetector::check_file_integrity(path.to_string_lossy().as_ref(), wrong_crc);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_compute_sha256() {
        let data = b"test data for sha256";
        let dir = tempdir().unwrap();
        let path = dir.path().join("hash_me.dat");
        std::fs::write(&path, data).unwrap();

        let hash = CorruptionDetector::compute_sha256(path.to_string_lossy().as_ref());
        assert!(hash.is_ok());
        let hash = hash.unwrap();
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
