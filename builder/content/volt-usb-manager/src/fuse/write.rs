use crate::fuse::filesystem::VumFilesystem;
use crate::fuse::inode::Inode;
use crate::core::result::VumResult;

pub fn write(fs: &VumFilesystem, inode: Inode, offset: u64, data: Vec<u8>) -> VumResult<u64> {
    fs.write(inode, offset, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_returns_length() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        let n = write(&fs, f, 0, vec![1, 2, 3]).unwrap();
        assert_eq!(n, 3);
    }

    #[test]
    fn test_write_updates_size() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        write(&fs, f, 0, vec![0u8; 100]).unwrap();
        let attr = fs.getattr(f).unwrap();
        assert_eq!(attr.size, 100);
    }

    #[test]
    fn test_write_at_offset() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        write(&fs, f, 50, vec![1, 2]).unwrap();
        let attr = fs.getattr(f).unwrap();
        assert_eq!(attr.size, 52);
    }

    #[test]
    fn test_write_empty() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        let n = write(&fs, f, 0, vec![]).unwrap();
        assert_eq!(n, 0);
    }
}
