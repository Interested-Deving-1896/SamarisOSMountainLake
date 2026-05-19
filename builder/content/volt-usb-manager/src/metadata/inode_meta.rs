use crate::fuse::inode::Inode;

#[derive(Debug, Clone)]
pub struct InodeMeta {
    pub inode: Inode,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub mtime: u64,
    pub mode: u32,
}

impl InodeMeta {
    pub fn new(inode: Inode, path: &str) -> Self {
        InodeMeta {
            inode,
            path: path.to_string(),
            size: 0,
            is_dir: false,
            mtime: 0,
            mode: 0o644,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let inode = Inode::new();
        let meta = InodeMeta::new(inode, "/test/path");
        assert_eq!(meta.inode, inode);
        assert_eq!(meta.path, "/test/path");
        assert_eq!(meta.size, 0);
        assert!(!meta.is_dir);
        assert_eq!(meta.mode, 0o644);
    }

    #[test]
    fn test_new_directory_path() {
        let inode = Inode::from_u64(42);
        let meta = InodeMeta::new(inode, "/var/log");
        assert_eq!(meta.inode.0, 42);
        assert_eq!(meta.path, "/var/log");
    }

    #[test]
    fn test_fields_independent() {
        let a = InodeMeta::new(Inode::from_u64(1), "/a");
        let b = InodeMeta::new(Inode::from_u64(2), "/b");
        assert_ne!(a.inode, b.inode);
        assert_ne!(a.path, b.path);
    }

    #[test]
    fn test_clone() {
        let meta = InodeMeta::new(Inode::from_u64(7), "/clone");
        let cloned = meta.clone();
        assert_eq!(meta.inode, cloned.inode);
        assert_eq!(meta.path, cloned.path);
    }
}
