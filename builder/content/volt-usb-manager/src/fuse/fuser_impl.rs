use std::sync::Arc;
use parking_lot::RwLock;

use crate::cache::ReadCache;
use crate::core::error::VumError;
use crate::core::result::VumResult;
use crate::writeback::WriteBuffer;

pub struct VoltFuseFilesystem {
    read_cache: Arc<RwLock<ReadCache>>,
    write_buffer: Arc<RwLock<WriteBuffer>>,
    read_only: bool,
}

impl VoltFuseFilesystem {
    pub fn new(
        read_cache: Arc<RwLock<ReadCache>>,
        write_buffer: Arc<RwLock<WriteBuffer>>,
        read_only: bool,
    ) -> Self {
        VoltFuseFilesystem {
            read_cache,
            write_buffer,
            read_only,
        }
    }
}

impl fuser::Filesystem for VoltFuseFilesystem {
    fn lookup(
        &mut self,
        _req: &fuser::Request<'_>,
        _parent: u64,
        _name: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        reply.error(libc::ENOENT);
    }

    fn getattr(
        &mut self,
        _req: &fuser::Request<'_>,
        _ino: u64,
        reply: fuser::ReplyAttr,
    ) {
        let tt = std::time::Duration::new(0, 0);
        let attr = fuser::FileAttr {
            ino: 1,
            size: 0,
            blocks: 0,
            atime: tt,
            mtime: tt,
            ctime: tt,
            crtime: tt,
            kind: fuser::FileType::Directory,
            perm: 0o755,
            nlink: 2,
            uid: unsafe { libc::getuid() },
            gid: unsafe { libc::getgid() },
            rdev: 0,
            blksize: 4096,
            flags: 0,
        };
        reply.attr(&tt, attr);
    }

    fn read(
        &mut self,
        _req: &fuser::Request<'_>,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        _size: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: fuser::ReplyData,
    ) {
        reply.data(&[]);
    }

    fn write(
        &mut self,
        _req: &fuser::Request<'_>,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        data: &[u8],
        _write_flags: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: fuser::ReplyWrite,
    ) {
        if self.read_only {
            reply.error(libc::EROFS);
            return;
        }
        reply.written(data.len() as u32);
    }

    fn readdir(
        &mut self,
        _req: &fuser::Request<'_>,
        _ino: u64,
        _fh: u64,
        offset: i64,
        reply: fuser::ReplyDirectory,
    ) {
        if offset == 0 {
            reply.add(1, 0, fuser::FileType::Directory, ".");
            reply.add(1, 1, fuser::FileType::Directory, "..");
        }
        reply.ok();
    }
}
