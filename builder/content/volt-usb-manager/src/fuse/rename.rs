use crate::fuse::filesystem::VumFilesystem;
use crate::fuse::inode::Inode;
use crate::core::result::VumResult;

pub fn rename(
    fs: &VumFilesystem,
    parent: Inode,
    name: &str,
    new_parent: Inode,
    new_name: &str,
) -> VumResult<()> {
    fs.rename(parent, name, new_parent, new_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename_same_dir() {
        let fs = VumFilesystem::new("/b", "/m");
        fs.create(Inode::ROOT, "old").unwrap();
        rename(&fs, Inode::ROOT, "old", Inode::ROOT, "new").unwrap();
        assert!(fs.lookup(Inode::ROOT, "old").is_err());
        assert!(fs.lookup(Inode::ROOT, "new").is_ok());
    }

    #[test]
    fn test_rename_other_dir() {
        let fs = VumFilesystem::new("/b", "/m");
        let dir = fs.mkdir(Inode::ROOT, "dir").unwrap();
        fs.create(Inode::ROOT, "f").unwrap();
        rename(&fs, Inode::ROOT, "f", dir, "moved").unwrap();
        assert!(fs.lookup(Inode::ROOT, "f").is_err());
        assert!(fs.lookup(dir, "moved").is_ok());
    }

    #[test]
    fn test_rename_nonexistent() {
        let fs = VumFilesystem::new("/b", "/m");
        let r = rename(&fs, Inode::ROOT, "nope", Inode::ROOT, "new");
        assert!(r.is_err());
    }

    #[test]
    fn test_rename_preserves_inode() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        rename(&fs, Inode::ROOT, "f", Inode::ROOT, "g").unwrap();
        let found = fs.lookup(Inode::ROOT, "g").unwrap();
        assert_eq!(found, f);
    }
}
