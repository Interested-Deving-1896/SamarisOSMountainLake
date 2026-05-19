use std::io::{BufRead, BufReader};

use crate::core::error::Result;

#[derive(Debug, Clone, Default)]
pub struct CpuMetrics {
    pub count: usize,
    pub total_load_percent: f64,
    pub per_core_load: Vec<f64>,
}

impl CpuMetrics {
    pub fn collect() -> Result<Self> {
        let count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let (total_load, per_core) = read_cpu_stats(count)?;

        Ok(Self {
            count,
            total_load_percent: total_load,
            per_core_load: per_core,
        })
    }
}

fn read_cpu_stats(core_count: usize) -> Result<(f64, Vec<f64>)> {
    let file = match std::fs::File::open("/proc/stat") {
        Ok(f) => f,
        Err(_) => {
            return Ok((0.0, vec![0.0; core_count]));
        }
    };
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut per_core = Vec::with_capacity(core_count);
    let mut total_idle = 0u64;
    let mut total_non_idle = 0u64;

    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("cpu") && !line.starts_with("cpu ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let user: u64 = parts[1].parse().unwrap_or(0);
                let nice: u64 = parts[2].parse().unwrap_or(0);
                let system: u64 = parts[3].parse().unwrap_or(0);
                let idle: u64 = parts[4].parse().unwrap_or(0);
                let iowait: u64 = parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(0);
                let irq: u64 = parts.get(6).and_then(|s| s.parse().ok()).unwrap_or(0);
                let softirq: u64 = parts.get(7).and_then(|s| s.parse().ok()).unwrap_or(0);

                let non_idle = user + nice + system + iowait + irq + softirq;
                let total = non_idle + idle;

                if total > 0 {
                    let load = (non_idle as f64 / total as f64) * 100.0;
                    per_core.push(load);
                }
                total_idle += idle;
                total_non_idle += non_idle;
            }
        }
        if line.starts_with("ctxt") || line.starts_with("btime") || line.starts_with("processes") {
            break;
        }
    }

    let total_load = if total_non_idle + total_idle > 0 {
        (total_non_idle as f64 / (total_non_idle + total_idle) as f64) * 100.0
    } else {
        0.0
    };

    Ok((total_load, per_core))
}
