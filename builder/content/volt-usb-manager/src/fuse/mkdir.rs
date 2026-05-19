use crate::fuse::filesystem::VumFilesystem;
use crate::fuse::inode::Inode;
use crate::core::result::VumResult;

pub fn mkdir(fs: &VumFilesystem, parent: Inode, name: &str) -> VumResult<Inode> {
    fs.mkdir(parent, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mkdir_basic() {
        let fs = VumFilesystem::new("/b", "/m");
        let dir = mkdir(&fs, Inode::ROOT, "sub").unwrap();
        let attr = fs.getattr(dir).unwrap();
        assert!(attr.is_dir);
    }

    #[test]
    fn test_mkdir_duplicate_fails() {
        let fs = VumFilesystem::new("/b", "/m");
        mkdir(&fs, Inode::ROOT, "sub").unwrap();
        let r = mkdir(&fs, Inode::ROOT, "sub");
        assert!(r.is_err());
    }

    #[test]
    fn test_mkdir_nested() {
        let fs = VumFilesystem::new("/b", "/m");
        let a = mkdir(&fs, Inode::ROOT, "a").unwrap();
        let b = mkdir(&fs, a, "b").unwrap();
        let attr = fs.getattr(b).unwrap();
        assert!(attr.is_dir);
        assert_eq!(attr.parent, a);
    }

    #[test]
    fn test_mkdir_readdir() {
        let fs = VumFilesystem::new("/b", "/m");
        mkdir(&fs, Inode::ROOT, "d1").unwrap();
        mkdir(&fs, Inode::ROOT, "d2").unwrap();
        let entries = fs.readdir(Inode::ROOT).unwrap();
        assert_eq!(entries.len(), 2);
    }
}
