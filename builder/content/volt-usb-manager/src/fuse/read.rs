use crate::fuse::filesystem::VumFilesystem;
use crate::fuse::inode::Inode;
use crate::core::result::VumResult;

pub fn read(fs: &VumFilesystem, inode: Inode, offset: u64, size: u64) -> VumResult<Vec<u8>> {
    fs.read(inode, offset, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_empty() {
        let fs = VumFilesystem::new("/b", "/m");
        let data = read(&fs, Inode::ROOT, 0, 10).unwrap();
        assert!(data.is_empty());
    }

    #[test]
    fn test_read_with_offset() {
        let fs = VumFilesystem::new("/b", "/m");
        let data = read(&fs, Inode::ROOT, 5, 10).unwrap();
        assert!(data.is_empty());
    }

    #[test]
    fn test_read_zero_size() {
        let fs = VumFilesystem::new("/b", "/m");
        let data = read(&fs, Inode::ROOT, 0, 0).unwrap();
        assert!(data.is_empty());
    }

    #[test]
    fn test_read_large_offset() {
        let fs = VumFilesystem::new("/b", "/m");
        let data = read(&fs, Inode::ROOT, 999999, 100).unwrap();
        assert!(data.is_empty());
    }
}
