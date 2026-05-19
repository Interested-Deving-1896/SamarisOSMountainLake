#[cfg(feature = "multi_gpu")]
use crate::core::result::VgmResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FusionMode {
    None,
    SplitWorkgroups,
    AlternateFrames,
    MemoryMirror,
}

impl FusionMode {
    pub fn name(&self) -> &'static str {
        match self {
            FusionMode::None => "none",
            FusionMode::SplitWorkgroups => "split_workgroups",
            FusionMode::AlternateFrames => "alternate_frames",
            FusionMode::MemoryMirror => "memory_mirror",
        }
    }
}

pub struct GpuFusion {
    pub enabled: bool,
    pub mode: FusionMode,
}

impl GpuFusion {
    pub fn new() -> Self {
        Self {
            enabled: cfg!(feature = "multi_gpu"),
            mode: FusionMode::None,
        }
    }

    pub fn set_mode(&mut self, mode: FusionMode) {
        self.mode = mode;
    }

    pub fn is_available(&self) -> bool {
        cfg!(feature = "multi_gpu") && self.enabled
    }

    #[cfg(feature = "multi_gpu")]
    pub fn compute_split(&self, total_work: u32, device_count: u32) -> VgmResult<Vec<u32>> {
        if device_count == 0 {
            return Err(crate::core::error::VgmError::MultiGpuUnsupported(
                "No devices for fusion compute split".into(),
            ));
        }
        let base = total_work / device_count;
        let remainder = total_work % device_count;
        let mut splits = Vec::with_capacity(device_count as usize);
        for i in 0..device_count {
            let extra = if i < remainder { 1 } else { 0 };
            splits.push(base + extra);
        }
        Ok(splits)
    }

    #[cfg(not(feature = "multi_gpu"))]
    pub fn compute_split(&self, _total_work: u32, _device_count: u32) -> Vec<u32> {
        vec![_total_work]
    }
}

impl Default for GpuFusion {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fusion() {
        let fusion = GpuFusion::new();
        assert_eq!(fusion.mode, FusionMode::None);
    }

    #[test]
    fn test_set_mode() {
        let mut fusion = GpuFusion::new();
        fusion.set_mode(FusionMode::AlternateFrames);
        assert_eq!(fusion.mode, FusionMode::AlternateFrames);
    }

    #[test]
    fn test_fusion_mode_names() {
        assert_eq!(FusionMode::None.name(), "none");
        assert_eq!(FusionMode::SplitWorkgroups.name(), "split_workgroups");
        assert_eq!(FusionMode::AlternateFrames.name(), "alternate_frames");
        assert_eq!(FusionMode::MemoryMirror.name(), "memory_mirror");
    }

    #[cfg(feature = "multi_gpu")]
    #[test]
    fn test_compute_split() {
        let fusion = GpuFusion::new();
        let splits = fusion.compute_split(100, 3).unwrap();
        assert_eq!(splits.len(), 3);
        assert_eq!(splits.iter().sum::<u32>(), 100);
    }

    #[test]
    fn test_is_available() {
        let fusion = GpuFusion::new();
        let _ = fusion.is_available();
    }
}
