use std::sync::Arc;
use std::sync::atomic::Ordering;
use parking_lot::RwLock;

use crate::fuse::inode::Inode;
use crate::fuse::inode_table::{InodeTable, InodeData};
use crate::cache::read_cache::ReadCache;
use crate::cache::cache_key::CacheKey;
use crate::writeback::write_buffer::WriteBuffer;
use crate::journal::journal::Journal;
use crate::core::result::VumResult;
use crate::core::error::VumError;
use crate::metrics::counters::MetricsCounters;

pub struct VumFilesystem {
    pub inode_table: Arc<InodeTable>,
    pub read_cache: Option<Arc<RwLock<ReadCache>>>,
    pub write_buffer: Option<Arc<RwLock<WriteBuffer>>>,
    pub journal: Option<Arc<Journal>>,
    pub counters: Arc<MetricsCounters>,
    pub backing_path: String,
    pub mount_point: String,
}

impl VumFilesystem {
    pub fn new(backing: &str, mount: &str) -> Self {
        let inode_table = Arc::new(InodeTable::new());
        inode_table.create_root();
        VumFilesystem {
            inode_table,
            read_cache: None,
            write_buffer: None,
            journal: None,
            counters: Arc::new(MetricsCounters::new()),
            backing_path: backing.to_string(),
            mount_point: mount.to_string(),
        }
    }

    pub fn lookup(&self, parent: Inode, name: &str) -> VumResult<Inode> {
        self.inode_table
            .lookup(parent, name)
            .ok_or_else(|| VumError::FileNotFound(name.to_string()))
    }

    pub fn getattr(&self, inode: Inode) -> VumResult<InodeData> {
        self.inode_table
            .get(inode)
            .ok_or_else(|| VumError::FileNotFound(inode.0.to_string()))
    }

    pub fn readdir(&self, parent: Inode) -> VumResult<Vec<String>> {
        let children = self.inode_table.children(parent);
        Ok(children.iter().map(|c| c.name.clone()).collect())
    }

    pub fn read(&self, inode: Inode, offset: u64, size: u64) -> VumResult<Vec<u8>> {
        self.counters
            .bytes_read_logical
            .fetch_add(size, Ordering::Relaxed);

        if let Some(ref cache) = self.read_cache {
            let key =
                CacheKey::new(&format!("{}:{}", self.backing_path, inode.0), 0, 0);
            let mut guard = cache.write();
            if let Some(data) = guard.get(&key) {
                self.counters.cache_hits.fetch_add(1, Ordering::Relaxed);
                let sliced: Vec<u8> = data
                    .into_iter()
                    .skip(offset as usize)
                    .take(size as usize)
                    .collect();
                return Ok(sliced);
            }
            self.counters.cache_misses.fetch_add(1, Ordering::Relaxed);
        }

        Ok(Vec::new())
    }

    pub fn write(&self, inode: Inode, offset: u64, data: Vec<u8>) -> VumResult<u64> {
        let len = data.len() as u64;
        self.inode_table.update_size(inode, offset + len);

        if let Some(ref wb) = self.write_buffer {
            wb.write().enqueue(
                &format!("{}:{}", self.backing_path, inode.0),
                offset,
                data,
                0,
                0,
            )?;
        }

        self.counters
            .bytes_written_logical
            .fetch_add(len, Ordering::Relaxed);
        Ok(len)
    }

    pub fn create(&self, parent: Inode, name: &str) -> VumResult<Inode> {
        if self.inode_table.lookup(parent, name).is_some() {
            return Err(VumError::FileAlreadyExists(name.to_string()));
        }
        Ok(self.inode_table.insert(parent, name, false, 0))
    }

    pub fn mkdir(&self, parent: Inode, name: &str) -> VumResult<Inode> {
        if self.inode_table.lookup(parent, name).is_some() {
            return Err(VumError::FileAlreadyExists(name.to_string()));
        }
        Ok(self.inode_table.insert(parent, name, true, 0))
    }

    pub fn unlink(&self, parent: Inode, name: &str) -> VumResult<()> {
        if name == "." || name == ".." {
            return Err(VumError::PermissionDenied("Cannot unlink . or ..".into()));
        }
        let child = self.lookup(parent, name)?;
        self.inode_table.remove(child)
    }

    pub fn rename(
        &self,
        parent: Inode,
        name: &str,
        new_parent: Inode,
        new_name: &str,
    ) -> VumResult<()> {
        let child = self.lookup(parent, name)?;
        self.inode_table
            .update_parent(child, new_parent, new_name);
        Ok(())
    }

    pub fn flush(&self, _inode: Inode) -> VumResult<()> {
        if let Some(ref wb) = self.write_buffer {
            let batch = wb.write().flush_batch(64);
            if !batch.is_empty() {
                self.counters
                    .flush_count
                    .fetch_add(batch.len() as u64, Ordering::Relaxed);
            }
        }
        Ok(())
    }

    pub fn fsync(&self, inode: Inode) -> VumResult<()> {
        self.flush(inode)?;
        if let Some(ref journal) = self.journal {
            journal.checkpoint()?;
        }
        Ok(())
    }

    pub(crate) fn setattr_internal(
        &self,
        inode: Inode,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        mtime: Option<u64>,
    ) -> VumResult<()> {
        self.getattr(inode)?;
        self.inode_table
            .update_attrs(inode, mode, uid, gid, mtime);
        Ok(())
    }
}

impl std::fmt::Debug for VumFilesystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VumFilesystem")
            .field("backing_path", &self.backing_path)
            .field("mount_point", &self.mount_point)
            .field("inode_table_size", &self.inode_table.children(Inode::ROOT).len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_root() {
        let fs = VumFilesystem::new("/backing", "/mnt");
        let root = fs.getattr(Inode::ROOT).unwrap();
        assert_eq!(root.name, "/");
        assert!(root.is_dir);
    }

    #[test]
    fn test_create_and_lookup() {
        let fs = VumFilesystem::new("/backing", "/mnt");
        let child = fs.create(Inode::ROOT, "file.txt").unwrap();
        let found = fs.lookup(Inode::ROOT, "file.txt").unwrap();
        assert_eq!(child, found);
    }

    #[test]
    fn test_create_duplicate_fails() {
        let fs = VumFilesystem::new("/backing", "/mnt");
        fs.create(Inode::ROOT, "dup").unwrap();
        let result = fs.create(Inode::ROOT, "dup");
        assert!(result.is_err());
    }

    #[test]
    fn test_mkdir() {
        let fs = VumFilesystem::new("/backing", "/mnt");
        let dir = fs.mkdir(Inode::ROOT, "subdir").unwrap();
        let attr = fs.getattr(dir).unwrap();
        assert!(attr.is_dir);
    }

    #[test]
    fn test_read_write() {
        let fs = VumFilesystem::new("/backing", "/mnt");
        let f = fs.create(Inode::ROOT, "data").unwrap();
        let written = fs.write(f, 0, vec![1, 2, 3]).unwrap();
        assert_eq!(written, 3);
        let attr = fs.getattr(f).unwrap();
        assert_eq!(attr.size, 3);
    }

    #[test]
    fn test_readdir() {
        let fs = VumFilesystem::new("/backing", "/mnt");
        fs.create(Inode::ROOT, "a").unwrap();
        fs.create(Inode::ROOT, "b").unwrap();
        fs.mkdir(Inode::ROOT, "c").unwrap();
        let entries = fs.readdir(Inode::ROOT).unwrap();
        assert_eq!(entries.len(), 3);
        assert!(entries.contains(&"a".to_string()));
        assert!(entries.contains(&"b".to_string()));
        assert!(entries.contains(&"c".to_string()));
    }

    #[test]
    fn test_unlink() {
        let fs = VumFilesystem::new("/backing", "/mnt");
        fs.create(Inode::ROOT, "tmp").unwrap();
        fs.unlink(Inode::ROOT, "tmp").unwrap();
        let result = fs.lookup(Inode::ROOT, "tmp");
        assert!(result.is_err());
    }

    #[test]
    fn test_rename() {
        let fs = VumFilesystem::new("/backing", "/mnt");
        let dir = fs.mkdir(Inode::ROOT, "dir").unwrap();
        fs.create(Inode::ROOT, "old").unwrap();
        fs.rename(Inode::ROOT, "old", dir, "new").unwrap();
        let result = fs.lookup(Inode::ROOT, "old");
        assert!(result.is_err());
        let found = fs.lookup(dir, "new").unwrap();
        assert_eq!(found.0, 3);
    }

    #[test]
    fn test_flush_fsync_noop_without_backing() {
        let fs = VumFilesystem::new("/backing", "/mnt");
        assert!(fs.flush(Inode::ROOT).is_ok());
        assert!(fs.fsync(Inode::ROOT).is_ok());
    }

    #[test]
    fn test_lookup_not_found() {
        let fs = VumFilesystem::new("/backing", "/mnt");
        let result = fs.lookup(Inode::ROOT, "nope");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_returns_empty_without_cache() {
        let fs = VumFilesystem::new("/backing", "/mnt");
        let f = fs.create(Inode::ROOT, "f").unwrap();
        let data = fs.read(f, 0, 100).unwrap();
        assert!(data.is_empty());
    }
}
