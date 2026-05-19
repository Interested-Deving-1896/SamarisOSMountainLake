pub fn probe() -> (bool, Option<String>, Option<String>, Option<u64>) {
    if let Ok(val) = std::env::var("SAMARIS_GPU_AVAILABLE") {
        if val == "0" || val.eq_ignore_ascii_case("false") || val.eq_ignore_ascii_case("no") {
            return (false, None, None, None);
        }
        if val == "1" || val.eq_ignore_ascii_case("true") || val.eq_ignore_ascii_case("yes") {
            return (true, Some("override".into()), Some("env".into()), None);
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(result) = probe_gpu_linux() {
            return result;
        }
    }

    (false, None, None, None)
}

#[cfg(target_os = "linux")]
fn probe_gpu_linux() -> Option<(bool, Option<String>, Option<String>, Option<u64>)> {
    let drm_path = std::path::Path::new("/sys/class/drm");
    if drm_path.exists() {
        if let Ok(entries) = std::fs::read_dir(drm_path) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("card") && !name_str.contains('-') {
                    let vendor = entry.path().join("device/vendor");
                    let vendor_str = std::fs::read_to_string(&vendor)
                        .ok()
                        .map(|s| s.trim().to_string());
                    return Some((true, vendor_str, Some(name_str.to_string()), None));
                }
            }
        }
    }

    probe_lspci()
}

#[cfg(target_os = "linux")]
fn probe_lspci() -> Option<(bool, Option<String>, Option<String>, Option<u64>)> {
    let output = std::process::Command::new("lspci")
        .arg("-mm")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let lower = line.to_lowercase();
        if lower.contains("vga") || lower.contains("3d") || lower.contains("display") {
            let parts: Vec<&str> = line.split('"').collect();
            if parts.len() >= 3 {
                let vendor = parts.get(1).map(|s| s.trim().to_string());
                let model = parts.get(3).map(|s| s.trim().to_string());
                return Some((true, vendor, model, None));
            }
        }
    }

    None
}
