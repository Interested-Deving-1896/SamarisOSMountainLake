pub fn probe() -> (u64, u64, u64) {
    #[cfg(target_os = "linux")]
    {
        if let Some(result) = read_meminfo() {
            return result;
        }
    }

    (2048, 1024, 0)
}

#[cfg(target_os = "linux")]
fn read_meminfo() -> Option<(u64, u64, u64)> {
    let content = std::fs::read_to_string("/proc/meminfo").ok()?;

    let total_kb = parse_meminfo_value(&content, "MemTotal:")?;
    let avail_kb = parse_meminfo_value(&content, "MemAvailable:")
        .or_else(|| parse_meminfo_value(&content, "MemFree:"))
        .unwrap_or(total_kb / 2);
    let swap_kb = parse_meminfo_value(&content, "SwapTotal:").unwrap_or(0);

    Some((total_kb / 1024, avail_kb / 1024, swap_kb / 1024))
}

#[cfg(target_os = "linux")]
fn parse_meminfo_value(content: &str, key: &str) -> Option<u64> {
    content
        .lines()
        .find(|line| line.starts_with(key))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|s| s.parse::<u64>().ok())
}
