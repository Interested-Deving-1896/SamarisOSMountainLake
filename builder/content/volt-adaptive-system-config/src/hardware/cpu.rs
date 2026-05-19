pub fn probe() -> (usize, usize, String, String) {
    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let arch = std::env::consts::ARCH.to_string();

    #[cfg(target_os = "linux")]
    {
        let model = read_cpu_model().unwrap_or_else(|| "unknown".to_string());
        let cores = read_cpu_cores().unwrap_or(threads);
        return (cores, threads, model, arch);
    }

    (threads, threads, "unknown".into(), arch)
}

#[cfg(target_os = "linux")]
fn read_cpu_model() -> Option<String> {
    let content = std::fs::read_to_string("/proc/cpuinfo").ok()?;
    content
        .lines()
        .find(|line| line.starts_with("model name"))
        .and_then(|line| line.split(':').nth(1))
        .map(|s| s.trim().to_string())
}

#[cfg(target_os = "linux")]
fn read_cpu_cores() -> Option<usize> {
    let content = std::fs::read_to_string("/proc/cpuinfo").ok()?;
    content
        .lines()
        .filter(|line| line.starts_with("cpu cores"))
        .filter_map(|line| line.split(':').nth(1))
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .next()
}
