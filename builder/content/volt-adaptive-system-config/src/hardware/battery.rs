pub fn probe() -> (bool, bool) {
    #[cfg(target_os = "linux")]
    {
        if let Some(result) = probe_battery_linux() {
            return result;
        }
    }

    (false, false)
}

#[cfg(target_os = "linux")]
fn probe_battery_linux() -> Option<(bool, bool)> {
    let power_supply = std::path::Path::new("/sys/class/power_supply");
    if !power_supply.exists() {
        return None;
    }

    let mut is_laptop = false;
    let mut battery_present = false;

    if let Ok(entries) = std::fs::read_dir(power_supply) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if name_str.starts_with("BAT") || name_str.starts_with("bat") {
                battery_present = true;
                is_laptop = true;
            }

            if name_str.starts_with("AC") || name_str.starts_with("ac") {
                is_laptop = true;
            }
        }
    }

    Some((is_laptop, battery_present))
}
