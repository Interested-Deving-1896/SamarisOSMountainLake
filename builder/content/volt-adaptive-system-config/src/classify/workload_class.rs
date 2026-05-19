use crate::hardware::profile::HardwareProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadClass {
    General,
    Development,
    Server,
    Media,
}

impl WorkloadClass {
    pub fn name(&self) -> &'static str {
        match self {
            WorkloadClass::General => "general",
            WorkloadClass::Development => "development",
            WorkloadClass::Server => "server",
            WorkloadClass::Media => "media",
        }
    }
}

pub fn classify_workload(_hw: &HardwareProfile) -> WorkloadClass {
    WorkloadClass::General
}
