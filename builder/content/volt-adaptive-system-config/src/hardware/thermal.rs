pub fn probe() -> bool {
    #[cfg(target_os = "linux")]
    {
        let thermal_path = std::path::Path::new("/sys/class/thermal/thermal_zone0/temp");
        if thermal_path.exists() {
            if let Ok(content) = std::fs::read_to_string(thermal_path) {
                if let Ok(_temp) = content.trim().parse::<i64>() {
                    return true;
                }
            }
        }
    }

    false
}
