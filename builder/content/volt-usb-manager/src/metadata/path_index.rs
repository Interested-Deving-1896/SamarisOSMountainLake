use dashmap::DashMap;

use crate::fuse::inode::Inode;

pub struct PathIndex {
    entries: DashMap<String, Inode>,
}

impl PathIndex {
    pub fn new() -> Self {
        PathIndex {
            entries: DashMap::new(),
        }
    }

    pub fn insert(&self, path: &str, inode: Inode) {
        self.entries.insert(path.to_string(), inode);
    }

    pub fn lookup(&self, path: &str) -> Option<Inode> {
        self.entries.get(path).map(|r| *r)
    }

    pub fn remove(&self, path: &str) {
        self.entries.remove(path);
    }

    pub fn contains(&self, path: &str) -> bool {
        self.entries.contains_key(path)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Default for PathIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_lookup() {
        let idx = PathIndex::new();
        let inode = Inode::from_u64(42);
        idx.insert("/test/file", inode);
        assert_eq!(idx.lookup("/test/file"), Some(inode));
    }

    #[test]
    fn test_lookup_nonexistent() {
        let idx = PathIndex::new();
        assert_eq!(idx.lookup("/nope"), None);
    }

    #[test]
    fn test_remove() {
        let idx = PathIndex::new();
        idx.insert("/tmp", Inode::from_u64(1));
        idx.remove("/tmp");
        assert!(!idx.contains("/tmp"));
    }

    #[test]
    fn test_contains() {
        let idx = PathIndex::new();
        assert!(!idx.contains("/missing"));
        idx.insert("/present", Inode::from_u64(2));
        assert!(idx.contains("/present"));
    }

    #[test]
    fn test_len() {
        let idx = PathIndex::new();
        assert_eq!(idx.len(), 0);
        idx.insert("/a", Inode::from_u64(1));
        idx.insert("/b", Inode::from_u64(2));
        assert_eq!(idx.len(), 2);
        idx.remove("/a");
        assert_eq!(idx.len(), 1);
    }

    #[test]
    fn test_overwrite() {
        let idx = PathIndex::new();
        idx.insert("/same", Inode::from_u64(1));
        idx.insert("/same", Inode::from_u64(2));
        assert_eq!(idx.lookup("/same"), Some(Inode::from_u64(2)));
        assert_eq!(idx.len(), 1);
    }
}
