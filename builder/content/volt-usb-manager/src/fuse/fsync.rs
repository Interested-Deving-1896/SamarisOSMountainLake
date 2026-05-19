use crate::fuse::filesystem::VumFilesystem;
use crate::fuse::inode::Inode;
use crate::core::result::VumResult;

pub fn fsync(fs: &VumFilesystem, inode: Inode) -> VumResult<()> {
    fs.fsync(inode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fsync_root() {
        let fs = VumFilesystem::new("/b", "/m");
        assert!(fsync(&fs, Inode::ROOT).is_ok());
    }

    #[test]
    fn test_fsync_created_file() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        assert!(fsync(&fs, f).is_ok());
    }

    #[test]
    fn test_fsync_twice() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        fsync(&fs, f).unwrap();
        assert!(fsync(&fs, f).is_ok());
    }

    #[test]
    fn test_fsync_after_write() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        let _ = fs.write(f, 0, vec![1, 2, 3]);
        assert!(fsync(&fs, f).is_ok());
    }
}
