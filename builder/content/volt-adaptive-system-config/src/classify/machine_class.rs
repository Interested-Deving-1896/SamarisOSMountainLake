use crate::hardware::profile::{BootMedium, HardwareProfile, StorageType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MachineClass {
    LowRam,
    StandardLaptop,
    PerformanceLaptop,
    Desktop,
    Workstation,
    Server,
    VirtualMachine,
    UsbBoot,
    BatteryPowered,
    GpuAvailable,
    CpuOnly,
    ThermalSensitive,
    HighMemory,
    Constrained,
}

impl MachineClass {
    pub fn name(&self) -> &str {
        match self {
            MachineClass::LowRam => "low-ram",
            MachineClass::StandardLaptop => "standard-laptop",
            MachineClass::PerformanceLaptop => "performance-laptop",
            MachineClass::Desktop => "desktop",
            MachineClass::Workstation => "workstation",
            MachineClass::Server => "server",
            MachineClass::VirtualMachine => "virtual-machine",
            MachineClass::UsbBoot => "usb-boot",
            MachineClass::BatteryPowered => "battery-powered",
            MachineClass::GpuAvailable => "gpu-available",
            MachineClass::CpuOnly => "cpu-only",
            MachineClass::ThermalSensitive => "thermal-sensitive",
            MachineClass::HighMemory => "high-memory",
            MachineClass::Constrained => "constrained",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            MachineClass::LowRam => "System with less than 4 GB of RAM",
            MachineClass::StandardLaptop => "Standard laptop with less than 16 GB of RAM",
            MachineClass::PerformanceLaptop => "High-performance laptop with 16 GB or more RAM",
            MachineClass::Desktop => "Standard desktop computer",
            MachineClass::Workstation => "High-end workstation with 16+ cores or 16+ GB RAM",
            MachineClass::Server => "Server-class system with 32+ cores or 32+ GB RAM",
            MachineClass::VirtualMachine => "Virtual machine environment detected",
            MachineClass::UsbBoot => "System booted from USB media",
            MachineClass::BatteryPowered => "System with battery power available",
            MachineClass::GpuAvailable => "Discrete or integrated GPU available",
            MachineClass::CpuOnly => "No GPU available, CPU-only system",
            MachineClass::ThermalSensitive => "System with thermal constraints",
            MachineClass::HighMemory => "System with 32 GB or more RAM",
            MachineClass::Constrained => "Resource-constrained system with 4 or fewer cores or less than 4 GB RAM",
        }
    }
}

pub fn classify(hw: &HardwareProfile) -> Vec<MachineClass> {
    let mut classes = Vec::with_capacity(6);

    if hw.ram_total_mb < 4096 {
        classes.push(MachineClass::LowRam);
    }

    if hw.ram_total_mb >= 32768 {
        classes.push(MachineClass::HighMemory);
    }

    if hw.cpu_cores >= 32 || hw.ram_total_mb >= 32768 {
        classes.push(MachineClass::Server);
    }

    if hw.cpu_cores >= 16 || hw.ram_total_mb >= 16384 {
        classes.push(MachineClass::Workstation);
    }

    if hw.is_laptop && hw.ram_total_mb < 16384 {
        classes.push(MachineClass::StandardLaptop);
    }

    if hw.is_laptop && hw.ram_total_mb >= 16384 {
        classes.push(MachineClass::PerformanceLaptop);
    }

    if hw.is_vm {
        classes.push(MachineClass::VirtualMachine);
    }

    if hw.boot_medium == BootMedium::Usb || hw.storage_type == StorageType::Usb {
        classes.push(MachineClass::UsbBoot);
    }

    if !hw.gpu_available {
        classes.push(MachineClass::CpuOnly);
    } else {
        classes.push(MachineClass::GpuAvailable);
    }

    if hw.battery_present {
        classes.push(MachineClass::BatteryPowered);
    }

    if hw.is_laptop || hw.battery_present || hw.thermal_available {
        classes.push(MachineClass::ThermalSensitive);
    }

    if !hw.is_laptop && !hw.is_vm && hw.cpu_cores < 16 && hw.ram_total_mb < 16384 {
        classes.push(MachineClass::Desktop);
    }

    if hw.cpu_cores <= 4 || hw.ram_total_mb < 4096 {
        classes.push(MachineClass::Constrained);
    }

    classes
}
