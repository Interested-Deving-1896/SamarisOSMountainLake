use crate::hardware::profile::StorageType;

pub fn probe() -> StorageType {
    #[cfg(target_os = "linux")]
    {
        if let Some(st) = probe_storage_linux() {
            return st;
        }
    }

    StorageType::Unknown
}

#[cfg(target_os = "linux")]
fn probe_storage_linux() -> Option<StorageType> {
    let sys_block = std::path::Path::new("/sys/block");
    if !sys_block.exists() {
        return None;
    }

    let entries = std::fs::read_dir(sys_block).ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if name_str.starts_with("nvme") {
            return Some(StorageType::Nvme);
        }

        if name_str.starts_with("sd") || name_str.starts_with("vd") || name_str.starts_with("xvd") {
            let rotational = entry.path().join("queue/rotational");
            if let Ok(content) = std::fs::read_to_string(&rotational) {
                if content.trim() == "0" {
                    return Some(StorageType::Ssd);
                }
                return Some(StorageType::Hdd);
            }
        }
    }

    None
}
