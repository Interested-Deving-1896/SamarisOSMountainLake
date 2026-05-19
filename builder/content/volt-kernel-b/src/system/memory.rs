use std::io::{BufRead, BufReader};

use crate::core::error::{Result, TesseractError};

#[derive(Debug, Clone, Default)]
pub struct MemoryMetrics {
    pub total_kb: u64,
    pub available_kb: u64,
    pub used_kb: u64,
    pub swap_total_kb: u64,
    pub swap_used_kb: u64,
    pub usage_percent: f64,
}

impl MemoryMetrics {
    pub fn collect() -> Result<Self> {
        let file = std::fs::File::open("/proc/meminfo")
            .map_err(|e| TesseractError::System(format!("cannot read /proc/meminfo: {e}")))?;
        let reader = BufReader::new(file);

        let mut total_kb = 0u64;
        let mut available_kb = 0u64;
        let mut swap_total_kb = 0u64;
        let mut swap_free_kb = 0u64;

        for line in reader.lines() {
            let line = line.map_err(|e| TesseractError::System(e.to_string()))?;
            if line.starts_with("MemTotal:") {
                total_kb = parse_meminfo_value(&line);
            } else if line.starts_with("MemAvailable:") {
                available_kb = parse_meminfo_value(&line);
            } else if line.starts_with("SwapTotal:") {
                swap_total_kb = parse_meminfo_value(&line);
            } else if line.starts_with("SwapFree:") {
                swap_free_kb = parse_meminfo_value(&line);
            }
        }

        let used_kb = total_kb.saturating_sub(available_kb);
        let usage_percent = if total_kb > 0 {
            (used_kb as f64 / total_kb as f64) * 100.0
        } else {
            0.0
        };

        Ok(Self {
            total_kb,
            available_kb,
            used_kb,
            swap_total_kb,
            swap_used_kb: swap_total_kb.saturating_sub(swap_free_kb),
            usage_percent,
        })
    }
}

fn parse_meminfo_value(line: &str) -> u64 {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        parts[1].parse().unwrap_or(0)
    } else {
        0
    }
}
