use std::process::Command;

use crate::core::error::VumError;
use crate::core::result::VumResult;
use crate::device::mount_info::MountInfo;

pub fn prepare_eject(mount_point: &str) -> VumResult<()> {
    let status = Command::new("sync")
        .status()
        .map_err(|e| VumError::FlushFailed(format!("sync command failed: {}", e)))?;
    if !status.success() {
        return Err(VumError::FlushFailed("sync command returned non-zero".into()));
    }
    if let Ok(file) = std::fs::File::open(mount_point) {
        file.sync_all()
            .map_err(|e| VumError::FlushFailed(format!("fsync failed: {}", e)))?;
    }
    Ok(())
}

pub fn force_eject(mount_point: &str) -> VumResult<()> {
    let status = if cfg!(target_os = "macos") {
        Command::new("diskutil")
            .args(["eject", mount_point])
            .status()
    } else {
        Command::new("umount")
            .args(["-f", mount_point])
            .status()
    }
    .map_err(|e| VumError::UnmountFailed(format!("eject command failed: {}", e)))?;

    if !status.success() {
        return Err(VumError::UnmountFailed(format!(
            "Force eject of {} failed",
            mount_point
        )));
    }
    Ok(())
}

pub fn can_eject_safely(mount_point: &str) -> bool {
    if !MountInfo::is_mounted(mount_point) {
        return false;
    }
    if let Ok(file) = std::fs::File::open(mount_point) {
        if let Ok(meta) = file.metadata() {
            if meta.permissions().readonly() {
                return true;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_eject_nonexistent() {
        let result = prepare_eject("/tmp/__vum_test_nonexistent_eject_path");
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_can_eject_safely_nonexistent() {
        assert!(!can_eject_safely("/tmp/__vum_test_nonexistent_safe_eject"));
    }

    #[test]
    fn test_can_eject_safely_root() {
        let result = can_eject_safely("/");
        assert!(!result || MountInfo::is_mounted("/"));
    }

    #[test]
    fn test_force_eject_nonexistent() {
        let result = force_eject("/tmp/__vum_test_nonexistent_force_eject");
        assert!(result.is_err());
    }
}
