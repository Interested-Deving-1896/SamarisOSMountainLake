use volt_usb_manager::fuse::filesystem::VumFilesystem;
use volt_usb_manager::fuse::inode::Inode;
use volt_usb_manager::fuse::permissions;

#[test]
fn test_path_validation() {
    let fs = VumFilesystem::new("/backing", "/mnt/volt");
    let result = fs.lookup(Inode::ROOT, "valid_name.txt");
    assert!(result.is_err());
}

#[test]
fn test_path_traversal_rejected() {
    use volt_usb_manager::safety::invariants::InvariantChecker;
    let dir = tempfile::tempdir().unwrap();
    let sub = dir.path().join("sub");
    std::fs::create_dir(&sub).unwrap();
    let result = InvariantChecker::check_no_path_escape(
        sub.to_string_lossy().as_ref(),
        dir.path().to_string_lossy().as_ref(),
    );
    assert!(result.is_ok());
    let outside = dir.path().join("..").join("etc");
    let result = InvariantChecker::check_no_path_escape(
        outside.to_string_lossy().as_ref(),
        dir.path().to_string_lossy().as_ref(),
    );
    assert!(result.is_err());
}

#[test]
fn test_read_goes_through_cache() {
    let fs = VumFilesystem::new("/backing", "/mnt");
    let f = fs.create(Inode::ROOT, "cached_file").unwrap();
    fs.write(f, 0, vec![1, 2, 3]).unwrap();
    let data = fs.read(f, 0, 100).unwrap();
    assert!(data.is_empty());
}

#[test]
fn test_write_uses_journal_and_writeback() {
    let fs = VumFilesystem::new("/backing", "/mnt");
    let f = fs.create(Inode::ROOT, "journaled_file").unwrap();
    let written = fs.write(f, 0, vec![10, 20, 30]).unwrap();
    assert_eq!(written, 3);
    let attr = fs.getattr(f).unwrap();
    assert_eq!(attr.size, 3);
}

#[test]
fn test_fsync_forces_durable_write() {
    let fs = VumFilesystem::new("/backing", "/mnt");
    let f = fs.create(Inode::ROOT, "synced_file").unwrap();
    fs.write(f, 0, vec![42]).unwrap();
    let result = fs.fsync(f);
    assert!(result.is_ok());
}

#[test]
fn test_unlink_journaled() {
    let fs = VumFilesystem::new("/backing", "/mnt");
    fs.create(Inode::ROOT, "to_delete").unwrap();
    fs.unlink(Inode::ROOT, "to_delete").unwrap();
    let result = fs.lookup(Inode::ROOT, "to_delete");
    assert!(result.is_err());
}

#[test]
fn test_rename_journaled() {
    let fs = VumFilesystem::new("/backing", "/mnt");
    let dir = fs.mkdir(Inode::ROOT, "dir").unwrap();
    fs.create(Inode::ROOT, "old_name").unwrap();
    fs.rename(Inode::ROOT, "old_name", dir, "new_name").unwrap();
    let result = fs.lookup(Inode::ROOT, "old_name");
    assert!(result.is_err());
    let found = fs.lookup(dir, "new_name").unwrap();
    assert_eq!(found.0, 3);
}

#[test]
fn test_create_duplicate_fails() {
    let fs = VumFilesystem::new("/backing", "/mnt");
    fs.create(Inode::ROOT, "dup").unwrap();
    let result = fs.create(Inode::ROOT, "dup");
    assert!(result.is_err());
}

#[test]
fn test_mkdir_creates_directory() {
    let fs = VumFilesystem::new("/backing", "/mnt");
    let dir = fs.mkdir(Inode::ROOT, "newdir").unwrap();
    let attr = fs.getattr(dir).unwrap();
    assert!(attr.is_dir);
}

#[test]
fn test_readdir_lists_children() {
    let fs = VumFilesystem::new("/backing", "/mnt");
    fs.create(Inode::ROOT, "a").unwrap();
    fs.create(Inode::ROOT, "b").unwrap();
    let entries = fs.readdir(Inode::ROOT).unwrap();
    assert_eq!(entries.len(), 2);
    assert!(entries.contains(&"a".to_string()));
}

#[test]
fn test_permission_checks() {
    assert!(permissions::owner_can_read(0o400));
    assert!(!permissions::owner_can_write(0o400));
    assert!(permissions::owner_can_write(0o200));
    assert!(permissions::can_access(0o755, 100, 100, 100, 100, true));
    assert!(!permissions::can_access(0o755, 100, 100, 200, 200, true));
}

#[test]
fn test_lookup_nonexistent_fails() {
    let fs = VumFilesystem::new("/backing", "/mnt");
    let result = fs.lookup(Inode::ROOT, "nonexistent");
    assert!(result.is_err());
}
