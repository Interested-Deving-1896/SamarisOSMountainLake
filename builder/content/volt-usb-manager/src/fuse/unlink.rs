use crate::fuse::filesystem::VumFilesystem;
use crate::fuse::inode::Inode;
use crate::core::result::VumResult;

pub fn unlink(fs: &VumFilesystem, parent: Inode, name: &str) -> VumResult<()> {
    fs.unlink(parent, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unlink_file() {
        let fs = VumFilesystem::new("/b", "/m");
        fs.create(Inode::ROOT, "f").unwrap();
        unlink(&fs, Inode::ROOT, "f").unwrap();
        assert!(fs.lookup(Inode::ROOT, "f").is_err());
    }

    #[test]
    fn test_unlink_nonexistent() {
        let fs = VumFilesystem::new("/b", "/m");
        let r = unlink(&fs, Inode::ROOT, "nope");
        assert!(r.is_err());
    }

    #[test]
    fn test_unlink_dot_fails() {
        let fs = VumFilesystem::new("/b", "/m");
        let r = unlink(&fs, Inode::ROOT, ".");
        assert!(r.is_err());
    }

    #[test]
    fn test_unlink_removes_from_readdir() {
        let fs = VumFilesystem::new("/b", "/m");
        fs.create(Inode::ROOT, "a").unwrap();
        fs.create(Inode::ROOT, "b").unwrap();
        unlink(&fs, Inode::ROOT, "a").unwrap();
        let entries = fs.readdir(Inode::ROOT).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], "b");
    }
}
