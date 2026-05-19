pub mod cpu;
pub mod memory;
pub mod process;
pub mod thermal;

use crate::core::error::Result;

#[derive(Debug, Clone, Default)]
pub struct SystemSnapshot {
    pub cpu: cpu::CpuMetrics,
    pub memory: memory::MemoryMetrics,
    pub processes: process::ProcessMetrics,
    pub thermal_max: Option<f64>,
}

pub struct SystemMonitor;

impl SystemMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn collect_all(&self) -> Result<SystemSnapshot> {
        let cpu = cpu::CpuMetrics::collect().unwrap_or_default();
        let memory = memory::MemoryMetrics::collect().unwrap_or_default();
        let processes = process::ProcessMetrics::collect().unwrap_or_default();
        let thermal = thermal::ThermalMetrics::collect().ok();
        let thermal_max = thermal.map(|t| t.max_temp);

        Ok(SystemSnapshot {
            cpu,
            memory,
            processes,
            thermal_max,
        })
    }
}
