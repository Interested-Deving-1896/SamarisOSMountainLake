use crate::fuse::filesystem::VumFilesystem;
use crate::fuse::inode::Inode;
use crate::core::result::VumResult;

pub fn setattr(
    fs: &VumFilesystem,
    inode: Inode,
    mode: Option<u32>,
    uid: Option<u32>,
    gid: Option<u32>,
    mtime: Option<u64>,
) -> VumResult<()> {
    fs.setattr_internal(inode, mode, uid, gid, mtime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setattr_mode() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        setattr(&fs, f, Some(0o700), None, None, None).unwrap();
        let attr = fs.getattr(f).unwrap();
        assert_eq!(attr.mode, 0o700);
    }

    #[test]
    fn test_setattr_uid_gid() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        setattr(&fs, f, None, Some(1000), Some(100), None).unwrap();
        let attr = fs.getattr(f).unwrap();
        assert_eq!(attr.uid, 1000);
        assert_eq!(attr.gid, 100);
    }

    #[test]
    fn test_setattr_mtime() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        setattr(&fs, f, None, None, None, Some(1234567890)).unwrap();
        let attr = fs.getattr(f).unwrap();
        assert_eq!(attr.mtime, 1234567890);
    }

    #[test]
    fn test_setattr_all() {
        let fs = VumFilesystem::new("/b", "/m");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        setattr(
            &fs,
            f,
            Some(0o444),
            Some(1),
            Some(2),
            Some(3),
        )
        .unwrap();
        let attr = fs.getattr(f).unwrap();
        assert_eq!(attr.mode, 0o444);
        assert_eq!(attr.uid, 1);
        assert_eq!(attr.gid, 2);
        assert_eq!(attr.mtime, 3);
    }
}
