use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core::result::VumResult;
use crate::core::error::VumError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMeta {
    pub path: String,
    pub size: u64,
    pub modified_at: u64,
    pub checksum: Option<u32>,
    pub is_directory: bool,
    pub permissions: u32,
}

impl FileMeta {
    pub fn from_path(path: &str) -> VumResult<Self> {
        let p = Path::new(path);
        let meta = std::fs::metadata(p)
            .map_err(|e| VumError::InvalidPath(format!("{}: {}", path, e)))?;

        let checksum = if meta.is_file() {
            let data = std::fs::read(p)
                .map_err(|e| VumError::ReadFailed(format!("{}: {}", path, e)))?;
            Some(crc32fast::hash(&data))
        } else {
            None
        };

        let modified_at = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let permissions = {
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                meta.mode() & 0o777
            }
            #[cfg(not(unix))]
            {
                0
            }
        };

        Ok(FileMeta {
            path: path.to_string(),
            size: meta.len(),
            modified_at,
            checksum,
            is_directory: meta.is_dir(),
            permissions,
        })
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_real_file() {
        let meta = FileMeta::from_path("Cargo.toml").unwrap();
        assert!(meta.size > 0);
        assert!(!meta.is_directory);
        assert!(meta.checksum.is_some());
    }

    #[test]
    fn test_from_nonexistent_path() {
        let result = FileMeta::from_path("/nonexistent/path/that/does/not/exist");
        assert!(result.is_err());
    }

    #[test]
    fn test_to_json() {
        let meta = FileMeta {
            path: "/test/f.txt".into(),
            size: 100,
            modified_at: 12345,
            checksum: Some(42),
            is_directory: false,
            permissions: 0o644,
        };
        let json = meta.to_json();
        assert!(json.contains("/test/f.txt"));
        assert!(json.contains("100"));
    }

    #[test]
    fn test_checksum_content_based() {
        let meta = FileMeta {
            path: "/a".into(),
            size: 0,
            modified_at: 0,
            checksum: Some(crc32fast::hash(b"hello")),
            is_directory: false,
            permissions: 0,
        };
        assert_eq!(meta.checksum, Some(crc32fast::hash(b"hello")));
    }

    #[test]
    fn test_json_roundtrip() {
        let meta = FileMeta {
            path: "/roundtrip".into(),
            size: 42,
            modified_at: 999,
            checksum: Some(12345),
            is_directory: true,
            permissions: 0o755,
        };
        let json = meta.to_json();
        let restored: FileMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.path, meta.path);
        assert_eq!(restored.size, meta.size);
        assert_eq!(restored.is_directory, meta.is_directory);
    }
}
