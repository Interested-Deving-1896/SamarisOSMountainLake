use crate::hardware::profile::HardwareProfile;
use crate::policies::worker_pool::worker_pool_desktop_min;

pub struct GeneratedDesktopConfig {
    pub desktop_min_workers: usize,
}

pub fn generate_desktop_config(hw: &HardwareProfile) -> GeneratedDesktopConfig {
    GeneratedDesktopConfig {
        desktop_min_workers: worker_pool_desktop_min(hw),
    }
}
