pub mod backend;
pub mod capabilities;
pub mod metal_backend;
pub mod null_backend;
pub mod vulkan_backend;
pub mod wgpu_backend;

// Backward-compatibility re-exports
pub use backend::{GpuBackend, GpuBackendKind, GpuCapabilities};
pub use crate::device::hardware::GpuHardware;
