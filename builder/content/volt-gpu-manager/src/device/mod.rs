pub mod adapter;
pub mod device_info;
pub mod hardware;
pub mod limits;
pub mod probe;
pub mod selection;

pub use hardware::GpuHardware;
pub use hardware::GpuVendor;
pub use adapter::GpuAdapter;
pub use device_info::GpuDeviceInfo;
pub use limits::GpuLimits;
pub use probe::GpuProbe;
pub use selection::GpuSelection;

// Backward-compatibility re-export
pub use crate::backend::backend::GpuCapabilities;
