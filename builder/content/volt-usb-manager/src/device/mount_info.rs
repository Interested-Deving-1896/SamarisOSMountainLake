use std::path::Path;

#[derive(Debug, Clone)]
pub struct MountInfo {
    pub source: String,
    pub target: String,
    pub fstype: String,
    pub options: Vec<String>,
}

impl MountInfo {
    pub fn current(path: &str) -> Option<Self> {
        let p = Path::new(path);
        let canonical_target = std::fs::canonicalize(p).ok()?;
        let target_str = canonical_target.to_string_lossy();

        let mount_sources: [&str; 2] = ["/proc/self/mountinfo", "/proc/mounts"];

        for source_path in &mount_sources {
            if let Ok(content) = std::fs::read_to_string(source_path) {
                if source_path.ends_with("mountinfo") {
                    for line in content.lines() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 10 && parts[6] == "-" {
                            let mount_point = parts[4];
                            if mount_point == target_str.as_ref()
                                || mount_point == path
                            {
                                let source = parts[9].to_string();
                                let target = parts[4].to_string();
                                let fstype = parts[8].to_string();
                                let options = parts[5].split(',').map(String::from).collect();
                                return Some(MountInfo {
                                    source,
                                    target,
                                    fstype,
                                    options,
                                });
                            }
                        }
                    }
                } else {
                    for line in content.lines() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 4 && parts[1] == path {
                            let source = parts[0].to_string();
                            let target = parts[1].to_string();
                            let fstype = parts[2].to_string();
                            let options = parts[3].split(',').map(String::from).collect();
                            return Some(MountInfo {
                                source,
                                target,
                                fstype,
                                options,
                            });
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("mount").output() {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    for line in stdout.lines() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 && parts[1] == "on" {
                            let source = parts[0].to_string();
                            let maybe_target = parts[2];
                            if maybe_target == path || maybe_target == target_str.as_ref() {
                                let fstype = if let Some(paren) = line.find('(') {
                                    line[paren + 1..]
                                        .split(',')
                                        .next()
                                        .unwrap_or("unknown")
                                        .trim()
                                        .to_string()
                                } else {
                                    "unknown".to_string()
                                };
                                return Some(MountInfo {
                                    source,
                                    target: maybe_target.to_string(),
                                    fstype,
                                    options: Vec::new(),
                                });
                            }
                        }
                    }
                }
            }
        }

        None
    }

    pub fn is_mounted(path: &str) -> bool {
        Self::current(path).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_mounted_root() {
        let result = MountInfo::is_mounted("/");
        assert!(result);
    }

    #[test]
    fn test_is_mounted_nonexistent() {
        let result = MountInfo::is_mounted("/__vum_test_nonexistent_mount_point_xyz");
        assert!(!result);
    }

    #[test]
    fn test_current_root() {
        let info = MountInfo::current("/");
        assert!(info.is_some());
        let info = info.unwrap();
        assert!(info.fstype.contains("ext") || !info.fstype.is_empty());
        assert!(!info.source.is_empty());
        assert_eq!(info.target, "/");
    }

    #[test]
    fn test_current_nonexistent_path() {
        let result = MountInfo::current("/__vum_test_nonexistent_path_xyz_123");
        assert!(result.is_none());
    }
}
