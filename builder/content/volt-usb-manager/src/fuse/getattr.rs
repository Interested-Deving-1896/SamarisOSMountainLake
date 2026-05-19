use crate::fuse::filesystem::VumFilesystem;
use crate::fuse::inode::Inode;
use crate::fuse::inode_table::InodeData;
use crate::core::result::VumResult;

pub fn getattr(fs: &VumFilesystem, inode: Inode) -> VumResult<InodeData> {
    fs.getattr(inode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getattr_root() {
        let fs = VumFilesystem::new("/b", "/m");
        let attr = getattr(&fs, Inode::ROOT).unwrap();
        assert!(attr.is_dir);
        assert_eq!(attr.name, "/");
    }

    #[test]
    fn test_getattr_file() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        let attr = getattr(&fs, f).unwrap();
        assert!(!attr.is_dir);
        assert_eq!(attr.name, "f");
    }

    #[test]
    fn test_getattr_nonexistent() {
        let fs = VumFilesystem::new("/b", "/m");
        let bogus = Inode::from_u64(999);
        let result = getattr(&fs, bogus);
        assert!(result.is_err());
    }

    #[test]
    fn test_getattr_returns_size() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        let _ = fs.write(f, 0, vec![0u8; 42]);
        let attr = getattr(&fs, f).unwrap();
        assert_eq!(attr.size, 42);
    }
}
