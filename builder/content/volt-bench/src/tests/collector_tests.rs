use crate::collectors::{
    system::SystemCollector, boot::BootCollector, memory::MemoryCollector,
    cpu::CpuCollector, disk::DiskCollector, network::NetworkCollector,
    Collector, CollectorRegistry,
};
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;

#[test]
fn test_system_collector_runs() {
    let col = SystemCollector;
    let hw = HardwareInfo::detect();
    let env = EnvironmentInfo::detect();
    let result = col.collect(&hw, &env);
    assert!(result.is_ok());
    let v = result.unwrap();
    assert!(v.get("ram_idle_percent").is_some());
}

#[test]
fn test_boot_collector_runs() {
    let col = BootCollector;
    let hw = HardwareInfo::detect();
    let env = EnvironmentInfo::detect();
    let result = col.collect(&hw, &env);
    assert!(result.is_ok());
}

#[test]
fn test_memory_collector_runs() {
    let col = MemoryCollector;
    let hw = HardwareInfo::detect();
    let env = EnvironmentInfo::detect();
    let result = col.collect(&hw, &env);
    assert!(result.is_ok());
}

#[test]
fn test_cpu_collector_runs() {
    let col = CpuCollector;
    let hw = HardwareInfo::detect();
    let env = EnvironmentInfo::detect();
    let result = col.collect(&hw, &env);
    assert!(result.is_ok());
}

#[test]
fn test_disk_collector_runs() {
    let col = DiskCollector;
    let hw = HardwareInfo::detect();
    let env = EnvironmentInfo::detect();
    let result = col.collect(&hw, &env);
    assert!(result.is_ok());
}

#[test]
fn test_network_collector_runs() {
    let col = NetworkCollector;
    let hw = HardwareInfo::detect();
    let env = EnvironmentInfo::detect();
    let result = col.collect(&hw, &env);
    assert!(result.is_ok());
}

#[test]
fn test_collector_registry_creates_all() {
    let mut reg = CollectorRegistry::new();
    let hw = HardwareInfo::detect();
    let env = EnvironmentInfo::detect();
    let result = reg.collect_all(&hw, &env);
    assert!(result.is_ok());
}
