use crate::generator::generated_config::GeneratedKernelBConfig;
use crate::hardware::profile::HardwareProfile;
use crate::policies::kernel_b::kernel_b_workers;

pub fn generate_kernel_b_config(hw: &HardwareProfile) -> GeneratedKernelBConfig {
    GeneratedKernelBConfig {
        workers: kernel_b_workers(hw),
    }
}
