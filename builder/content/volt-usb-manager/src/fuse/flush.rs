use crate::fuse::filesystem::VumFilesystem;
use crate::fuse::inode::Inode;
use crate::core::result::VumResult;

pub fn flush(fs: &VumFilesystem, inode: Inode) -> VumResult<()> {
    fs.flush(inode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flush_root() {
        let fs = VumFilesystem::new("/b", "/m");
        assert!(flush(&fs, Inode::ROOT).is_ok());
    }

    #[test]
    fn test_flush_created_file() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        assert!(flush(&fs, f).is_ok());
    }

    #[test]
    fn test_flush_twice() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        flush(&fs, f).unwrap();
        assert!(flush(&fs, f).is_ok());
    }

    #[test]
    fn test_flush_nonexistent_inode() {
        let fs = VumFilesystem::new("/b", "/m");
        let bogus = crate::fuse::inode::Inode::from_u64(999);
        assert!(flush(&fs, bogus).is_ok());
    }
}
