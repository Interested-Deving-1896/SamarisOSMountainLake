use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::core::result::VumResult;
use crate::core::error::VumError;

static MOUNTED: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct FuseMount;

impl FuseMount {
    pub fn mount(mount_point: &str, backing: &str) -> VumResult<()> {
        #[cfg(feature = "fuse")]
        {
            let _ = (mount_point, backing);
        }

        let mut map =
            MOUNTED.lock().map_err(|e| VumError::InternalInvariantViolation(e.to_string()))?;
        if map.contains_key(mount_point) {
            return Err(VumError::MountFailed("Already mounted".into()));
        }
        map.insert(mount_point.to_string(), backing.to_string());
        Ok(())
    }

    pub fn unmount(mount_point: &str) -> VumResult<()> {
        #[cfg(feature = "fuse")]
        {
            let _ = mount_point;
        }

        let mut map =
            MOUNTED.lock().map_err(|e| VumError::InternalInvariantViolation(e.to_string()))?;
        if map.remove(mount_point).is_none() {
            return Err(VumError::UnmountFailed("Not mounted".into()));
        }
        Ok(())
    }

    pub fn is_mounted(path: &str) -> bool {
        MOUNTED.lock().map(|m| m.contains_key(path)).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mount_unmount() {
        FuseMount::mount("/mnt/test", "/backing/test").unwrap();
        assert!(FuseMount::is_mounted("/mnt/test"));
        FuseMount::unmount("/mnt/test").unwrap();
        assert!(!FuseMount::is_mounted("/mnt/test"));
    }

    #[test]
    fn test_double_mount_fails() {
        FuseMount::mount("/mnt/double", "/backing/double").unwrap();
        let result = FuseMount::mount("/mnt/double", "/backing/other");
        assert!(result.is_err());
        FuseMount::unmount("/mnt/double").unwrap();
    }

    #[test]
    fn test_unmount_not_mounted_fails() {
        let result = FuseMount::unmount("/mnt/nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_is_mounted_false_for_unknown() {
        assert!(!FuseMount::is_mounted("/mnt/unknown"));
    }

    #[test]
    fn test_multiple_mounts() {
        FuseMount::mount("/mnt/a", "/backing/a").unwrap();
        FuseMount::mount("/mnt/b", "/backing/b").unwrap();
        assert!(FuseMount::is_mounted("/mnt/a"));
        assert!(FuseMount::is_mounted("/mnt/b"));
        FuseMount::unmount("/mnt/a").unwrap();
        assert!(!FuseMount::is_mounted("/mnt/a"));
        assert!(FuseMount::is_mounted("/mnt/b"));
        FuseMount::unmount("/mnt/b").unwrap();
    }
}
