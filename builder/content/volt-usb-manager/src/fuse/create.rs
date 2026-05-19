use crate::fuse::filesystem::VumFilesystem;
use crate::fuse::inode::Inode;
use crate::core::result::VumResult;

pub fn create(fs: &VumFilesystem, parent: Inode, name: &str) -> VumResult<Inode> {
    fs.create(parent, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = create(&fs, Inode::ROOT, "f").unwrap();
        let attr = fs.getattr(f).unwrap();
        assert!(!attr.is_dir);
    }

    #[test]
    fn test_create_duplicate_fails() {
        let fs = VumFilesystem::new("/b", "/m");
        create(&fs, Inode::ROOT, "f").unwrap();
        let r = create(&fs, Inode::ROOT, "f");
        assert!(r.is_err());
    }

    #[test]
    fn test_create_lookupable() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = create(&fs, Inode::ROOT, "f").unwrap();
        let found = fs.lookup(Inode::ROOT, "f").unwrap();
        assert_eq!(f, found);
    }

    #[test]
    fn test_create_in_subdir() {
        let fs = VumFilesystem::new("/b", "/m");
        let dir = fs.mkdir(Inode::ROOT, "dir").unwrap();
        let f = create(&fs, dir, "nested").unwrap();
        let attr = fs.getattr(f).unwrap();
        assert_eq!(attr.parent, dir);
    }
}
