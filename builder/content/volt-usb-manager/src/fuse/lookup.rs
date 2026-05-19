use crate::fuse::filesystem::VumFilesystem;
use crate::fuse::inode::Inode;
use crate::core::result::VumResult;

pub fn lookup(fs: &VumFilesystem, parent: Inode, name: &str) -> VumResult<Inode> {
    fs.lookup(parent, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_found() {
        let fs = VumFilesystem::new("/b", "/m");
        let c = fs.create(Inode::ROOT, "f").unwrap();
        let r = lookup(&fs, Inode::ROOT, "f").unwrap();
        assert_eq!(r, c);
    }

    #[test]
    fn test_lookup_not_found() {
        let fs = VumFilesystem::new("/b", "/m");
        let r = lookup(&fs, Inode::ROOT, "nope");
        assert!(r.is_err());
    }

    #[test]
    fn test_lookup_root() {
        let fs = VumFilesystem::new("/b", "/m");
        let r = lookup(&fs, Inode::ROOT, "/").unwrap();
        assert_eq!(r, Inode::ROOT);
    }

    #[test]
    fn test_lookup_after_create() {
        let fs = VumFilesystem::new("/b", "/m");
        fs.create(Inode::ROOT, "a").unwrap();
        fs.create(Inode::ROOT, "b").unwrap();
        let a = lookup(&fs, Inode::ROOT, "a").unwrap();
        let b = lookup(&fs, Inode::ROOT, "b").unwrap();
        assert_ne!(a, b);
    }
}
