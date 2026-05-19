pub mod system;
pub mod boot;
pub mod memory;
pub mod cpu;
pub mod disk;
pub mod network;
pub mod vrm;
pub mod vgm;
pub mod dwp;
pub mod vum;
pub mod kernel_b;

use serde_json::Value;
use crate::errors::BenchError;
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;

pub type CollectorResult = Result<Value, BenchError>;

pub struct CollectorRegistry {
    collectors: Vec<Box<dyn Collector>>,
    flags: Vec<String>,
}

pub trait Collector {
    fn name(&self) -> &'static str;
    fn collect(&self, hardware: &HardwareInfo, env: &EnvironmentInfo) -> CollectorResult;
}

impl CollectorRegistry {
    pub fn new() -> Self {
        let collectors: Vec<Box<dyn Collector>> = vec![
            Box::new(system::SystemCollector),
            Box::new(boot::BootCollector),
            Box::new(memory::MemoryCollector),
            Box::new(cpu::CpuCollector),
            Box::new(disk::DiskCollector),
            Box::new(network::NetworkCollector),
            Box::new(vrm::VrmCollector),
            Box::new(vgm::VgmCollector),
            Box::new(dwp::DwpCollector),
            Box::new(vum::VumCollector),
            Box::new(kernel_b::KernelBCollector),
        ];
        Self {
            collectors,
            flags: Vec::new(),
        }
    }

    pub fn collect_all(&mut self, hardware: &HardwareInfo, env: &EnvironmentInfo) -> Result<Value, BenchError> {
        let mut map = serde_json::Map::new();
        self.flags.clear();
        for collector in &self.collectors {
            match collector.collect(hardware, env) {
                Ok(value) => { map.insert(collector.name().to_string(), value); }
                Err(e) => {
                    tracing::warn!("Collector '{}' failed: {}", collector.name(), e);
                    self.flags.push("MISSING_COLLECTOR".to_string());
                    map.insert(collector.name().to_string(), serde_json::json!({"error": e.to_string()}));
                }
            }
        }
        Ok(Value::Object(map))
    }

    pub fn reliability_flags(&self) -> Vec<String> {
        self.flags.clone()
    }
}
