use crate::core::error::VgmError;
use crate::core::result::VgmResult;
use crate::resources::resource_id::GpuResourceId;

#[derive(Debug, Clone)]
pub struct SafetyGuard {
    pub enabled: bool,
}

impl SafetyGuard {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub fn with_enabled(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn check_permission(&self, action: &str) -> VgmResult<()> {
        if self.enabled {
            Ok(())
        } else {
            Err(VgmError::PermissionDenied(format!(
                "Action '{}' denied by safety guard",
                action
            )))
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for SafetyGuard {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ResourceGuard {
    resource_id: GpuResourceId,
    released: bool,
}

impl ResourceGuard {
    pub fn new(resource_id: GpuResourceId) -> Self {
        Self {
            resource_id,
            released: false,
        }
    }

    pub fn resource_id(&self) -> &GpuResourceId {
        &self.resource_id
    }

    pub fn release(&mut self) {
        self.released = true;
    }

    pub fn is_active(&self) -> bool {
        !self.released
    }
}

impl Drop for ResourceGuard {
    fn drop(&mut self) {
        if !self.released {
            tracing::warn!("ResourceGuard for {} dropped without explicit release", self.resource_id);
        }
    }
}

pub struct FrameGuard {
    frame_index: u64,
    released: bool,
}

impl FrameGuard {
    pub fn new(frame_index: u64) -> Self {
        Self {
            frame_index,
            released: false,
        }
    }

    pub fn frame_index(&self) -> u64 {
        self.frame_index
    }

    pub fn release(&mut self) {
        self.released = true;
    }
}

impl Drop for FrameGuard {
    fn drop(&mut self) {
        if !self.released {
            tracing::warn!("FrameGuard for frame {} dropped without release", self.frame_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_guard_enabled() {
        let guard = SafetyGuard::new();
        assert!(guard.check_permission("allocate").is_ok());
    }

    #[test]
    fn test_safety_guard_disabled() {
        let guard = SafetyGuard::with_enabled(false);
        assert!(guard.check_permission("dangerous").is_err());
    }

    #[test]
    fn test_resource_guard() {
        let id = GpuResourceId::new();
        let mut guard = ResourceGuard::new(id);
        assert_eq!(guard.resource_id(), &id);
        assert!(guard.is_active());
        guard.release();
        assert!(!guard.is_active());
    }

    #[test]
    fn test_frame_guard() {
        let mut guard = FrameGuard::new(42);
        assert_eq!(guard.frame_index(), 42);
        guard.release();
    }

    #[test]
    fn test_safety_guard_toggle() {
        let mut guard = SafetyGuard::with_enabled(false);
        assert!(!guard.is_enabled());
        guard.set_enabled(true);
        assert!(guard.is_enabled());
    }
}
