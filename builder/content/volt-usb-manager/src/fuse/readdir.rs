use crate::fuse::filesystem::VumFilesystem;
use crate::fuse::inode::Inode;
use crate::core::result::VumResult;

pub fn readdir(fs: &VumFilesystem, parent: Inode) -> VumResult<Vec<String>> {
    fs.readdir(parent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readdir_empty_root() {
        let fs = VumFilesystem::new("/b", "/m");
        let entries = readdir(&fs, Inode::ROOT).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn test_readdir_with_children() {
        let fs = VumFilesystem::new("/b", "/m");
        fs.create(Inode::ROOT, "a").unwrap();
        fs.create(Inode::ROOT, "b").unwrap();
        let entries = readdir(&fs, Inode::ROOT).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_readdir_isolated() {
        let fs = VumFilesystem::new("/b", "/m");
        let dir = fs.mkdir(Inode::ROOT, "dir").unwrap();
        fs.create(Inode::ROOT, "root_file").unwrap();
        fs.create(dir, "child_file").unwrap();
        let root_entries = readdir(&fs, Inode::ROOT).unwrap();
        let dir_entries = readdir(&fs, dir).unwrap();
        assert_eq!(root_entries.len(), 2);
        assert_eq!(dir_entries.len(), 1);
    }

    #[test]
    fn test_readdir_nonexistent() {
        let fs = VumFilesystem::new("/b", "/m");
        let bogus = Inode::from_u64(999);
        let entries = readdir(&fs, bogus).unwrap();
        assert!(entries.is_empty());
    }
}
