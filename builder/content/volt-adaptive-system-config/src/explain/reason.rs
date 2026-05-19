pub fn cpu_worker_reason(cores: usize, workers: usize) -> String {
    format!(
        "{} CPU cores → {}% → {} workers",
        cores,
        if cores <= 2 { 50 } else { 75 },
        workers
    )
}

pub fn ram_quota_reason(ram_mb: u64, quota_mb: u64, purpose: &str) -> String {
    let pct = if ram_mb > 0 {
        (quota_mb as f64 / ram_mb as f64 * 100.0) as u64
    } else {
        0
    };
    format!(
        "{} RAM → {}% allocated to {} → {} MB",
        ram_mb, pct, purpose, quota_mb
    )
}

pub fn storage_cache_reason(storage_type: &str, cache_mb: u64) -> String {
    format!("{} storage → cache set to {} MB", storage_type, cache_mb)
}

pub fn orbit_burst_reason(is_laptop: bool, window_ms: u64) -> String {
    if is_laptop {
        format!("Laptop detected → extended burst window of {} ms", window_ms)
    } else {
        format!("Desktop detected → standard burst window of {} ms", window_ms)
    }
}

pub fn clamp_reason(param: &str, original: &str, clamped: &str) -> String {
    format!(
        "{} clamped from {} to {} (safety cap applied)",
        param, original, clamped
    )
}

pub fn fallback_reason(component: &str) -> String {
    format!(
        "{} policy used safe defaults (probe result unavailable)",
        component
    )
}

pub fn budget_reason(allocated: u64, cap: u64) -> String {
    let pct = if cap > 0 {
        (allocated as f64 / cap as f64 * 100.0) as u64
    } else {
        0
    };
    format!(
        "Allocated {} MB / {} MB cap ({}%)",
        allocated, cap, pct
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_worker_reason() {
        let r = cpu_worker_reason(8, 6);
        assert!(r.contains("8"));
        assert!(r.contains("6"));
    }

    #[test]
    fn test_ram_quota_reason() {
        let r = ram_quota_reason(8192, 2048, "desktop");
        assert!(r.contains("desktop"));
    }
}
