use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::result::VumResult;
use crate::core::error::VumError;
use crate::fuse::inode::Inode;

#[derive(Debug, Clone)]
pub struct InodeData {
    pub inode: Inode,
    pub parent: Inode,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub mtime: u64,
    pub ctime: u64,
}

pub struct InodeTable {
    inodes: DashMap<u64, InodeData>,
    next_id: AtomicU64,
}

impl InodeTable {
    pub fn new() -> Self {
        InodeTable {
            inodes: DashMap::new(),
            next_id: AtomicU64::new(2),
        }
    }

    pub fn create_root(&self) {
        let root = InodeData {
            inode: Inode::ROOT,
            parent: Inode::ROOT,
            name: String::from("/"),
            is_dir: true,
            size: 0,
            mode: 0o755,
            uid: 0,
            gid: 0,
            mtime: 0,
            ctime: 0,
        };
        self.inodes.insert(Inode::ROOT.0, root);
    }

    pub fn lookup(&self, parent: Inode, name: &str) -> Option<Inode> {
        for entry in self.inodes.iter() {
            if entry.parent == parent && entry.name == name {
                return Some(entry.inode);
            }
        }
        None
    }

    pub fn get(&self, inode: Inode) -> Option<InodeData> {
        self.inodes.get(&inode.0).map(|r| r.clone())
    }

    pub fn insert(&self, parent: Inode, name: &str, is_dir: bool, size: u64) -> Inode {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let inode = Inode(id);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let data = InodeData {
            inode,
            parent,
            name: name.to_string(),
            is_dir,
            size,
            mode: if is_dir { 0o755 } else { 0o644 },
            uid: 0,
            gid: 0,
            mtime: now,
            ctime: now,
        };
        self.inodes.insert(id, data);
        inode
    }

    pub fn remove(&self, inode: Inode) -> VumResult<()> {
        if inode == Inode::ROOT {
            return Err(VumError::PermissionDenied(
                "Cannot remove root inode".into(),
            ));
        }
        self.inodes.remove(&inode.0);
        Ok(())
    }

    pub fn children(&self, parent: Inode) -> Vec<InodeData> {
        self.inodes
            .iter()
            .filter(|entry| entry.parent == parent && entry.inode.0 != parent.0)
            .map(|r| r.clone())
            .collect()
    }

    pub fn update_size(&self, inode: Inode, size: u64) {
        if let Some(mut entry) = self.inodes.get_mut(&inode.0) {
            entry.size = size;
        }
    }

    pub(crate) fn update_attrs(
        &self,
        inode: Inode,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        mtime: Option<u64>,
    ) {
        if let Some(mut entry) = self.inodes.get_mut(&inode.0) {
            if let Some(m) = mode {
                entry.mode = m;
            }
            if let Some(u) = uid {
                entry.uid = u;
            }
            if let Some(g) = gid {
                entry.gid = g;
            }
            if let Some(t) = mtime {
                entry.mtime = t;
            }
        }
    }

    pub(crate) fn update_parent(&self, inode: Inode, new_parent: Inode, new_name: &str) {
        if let Some(mut entry) = self.inodes.get_mut(&inode.0) {
            entry.parent = new_parent;
            entry.name = new_name.to_string();
        }
    }
}

impl Default for InodeTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_create_root() {
        let table = InodeTable::new();
        table.create_root();
        let root = table.get(Inode::ROOT);
        assert!(root.is_some());
        assert_eq!(root.unwrap().name, "/");
    }

    #[test]
    fn test_insert_and_lookup() {
        let table = InodeTable::new();
        table.create_root();
        let child = table.insert(Inode::ROOT, "file.txt", false, 100);
        let found = table.lookup(Inode::ROOT, "file.txt");
        assert_eq!(found, Some(child));
    }

    #[test]
    fn test_lookup_nonexistent() {
        let table = InodeTable::new();
        table.create_root();
        let found = table.lookup(Inode::ROOT, "nonexistent");
        assert!(found.is_none());
    }

    #[test]
    fn test_remove() {
        let table = InodeTable::new();
        table.create_root();
        let child = table.insert(Inode::ROOT, "del.txt", false, 50);
        table.remove(child).unwrap();
        assert!(table.get(child).is_none());
    }

    #[test]
    fn test_remove_root_fails() {
        let table = InodeTable::new();
        table.create_root();
        let result = table.remove(Inode::ROOT);
        assert!(result.is_err());
    }

    #[test]
    fn test_children() {
        let table = InodeTable::new();
        table.create_root();
        table.insert(Inode::ROOT, "a", false, 10);
        table.insert(Inode::ROOT, "b", false, 20);
        table.insert(Inode::ROOT, "c", true, 0);
        let kids = table.children(Inode::ROOT);
        assert_eq!(kids.len(), 3);
    }

    #[test]
    fn test_update_size() {
        let table = InodeTable::new();
        table.create_root();
        let f = table.insert(Inode::ROOT, "f", false, 100);
        table.update_size(f, 200);
        assert_eq!(table.get(f).unwrap().size, 200);
    }

    #[test]
    fn test_update_attrs() {
        let table = InodeTable::new();
        table.create_root();
        let f = table.insert(Inode::ROOT, "f", false, 0);
        table.update_attrs(f, Some(0o700), Some(1000), Some(1000), None);
        let data = table.get(f).unwrap();
        assert_eq!(data.mode, 0o700);
        assert_eq!(data.uid, 1000);
        assert_eq!(data.gid, 1000);
    }

    #[test]
    fn test_update_parent() {
        let table = InodeTable::new();
        table.create_root();
        let dir = table.insert(Inode::ROOT, "dir", true, 0);
        let f = table.insert(Inode::ROOT, "f", false, 0);
        table.update_parent(f, dir, "moved");
        let data = table.get(f).unwrap();
        assert_eq!(data.parent, dir);
        assert_eq!(data.name, "moved");
    }
}
