use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::result::VumResult;
use crate::core::error::VumError;
use crate::metadata::file_meta::FileMeta;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub entries: HashMap<String, FileMeta>,
    pub created_at: u64,
    pub version: u64,
}

impl Manifest {
    pub fn new() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Manifest {
            entries: HashMap::new(),
            created_at: now,
            version: 1,
        }
    }

    pub fn add_file(&mut self, meta: FileMeta) {
        self.entries.insert(meta.path.clone(), meta);
    }

    pub fn remove_file(&mut self, path: &str) {
        self.entries.remove(path);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    pub fn from_json(json: &str) -> VumResult<Self> {
        serde_json::from_str(json)
            .map_err(|e| VumError::InvalidConfig(format!("Manifest parse error: {}", e)))
    }
}

impl Default for Manifest {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_manifest() {
        let m = Manifest::new();
        assert!(m.entries.is_empty());
        assert_eq!(m.version, 1);
        assert!(m.created_at > 0);
    }

    #[test]
    fn test_add_file() {
        let mut m = Manifest::new();
        let meta = FileMeta {
            path: "/f.txt".into(),
            size: 100,
            modified_at: 0,
            checksum: None,
            is_directory: false,
            permissions: 0o644,
        };
        m.add_file(meta);
        assert_eq!(m.entries.len(), 1);
    }

    #[test]
    fn test_remove_file() {
        let mut m = Manifest::new();
        let meta = FileMeta {
            path: "/f.txt".into(),
            size: 100,
            modified_at: 0,
            checksum: None,
            is_directory: false,
            permissions: 0o644,
        };
        m.add_file(meta);
        m.remove_file("/f.txt");
        assert!(m.entries.is_empty());
    }

    #[test]
    fn test_to_json() {
        let mut m = Manifest::new();
        m.add_file(FileMeta {
            path: "/a".into(),
            size: 1,
            modified_at: 0,
            checksum: None,
            is_directory: false,
            permissions: 0,
        });
        let json = m.to_json();
        assert!(json.contains("/a"));
    }

    #[test]
    fn test_from_json() {
        let json = r#"{"entries":{"/x":{"path":"/x","size":10,"modified_at":0,"checksum":null,"is_directory":false,"permissions":0}},"created_at":100,"version":1}"#;
        let m = Manifest::from_json(json).unwrap();
        assert_eq!(m.entries.len(), 1);
        assert_eq!(m.entries.get("/x").unwrap().size, 10);
        assert_eq!(m.version, 1);
    }

    #[test]
    fn test_from_invalid_json() {
        let result = Manifest::from_json("not valid json");
        assert!(result.is_err());
    }

    #[test]
    fn test_json_roundtrip() {
        let mut m = Manifest::new();
        m.add_file(FileMeta {
            path: "/round".into(),
            size: 42,
            modified_at: 123,
            checksum: Some(999),
            is_directory: false,
            permissions: 0o644,
        });
        let json = m.to_json();
        let restored = Manifest::from_json(&json).unwrap();
        assert_eq!(restored.entries.len(), 1);
        assert_eq!(
            restored.entries.get("/round").unwrap().size,
            42
        );
        assert_eq!(restored.version, m.version);
    }
}
