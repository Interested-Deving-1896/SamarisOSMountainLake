use crate::backend::backend::{GpuBackend, GpuBackendKind, GpuCapabilities};

pub struct NullBackend {
    capabilities: GpuCapabilities,
}

impl NullBackend {
    pub fn new() -> Self {
        Self {
            capabilities: GpuCapabilities::null(),
        }
    }

    pub fn new_with_capabilities(caps: GpuCapabilities) -> Self {
        Self { capabilities: caps }
    }
}

impl Default for NullBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuBackend for NullBackend {
    fn kind(&self) -> GpuBackendKind {
        GpuBackendKind::Null
    }

    fn capabilities(&self) -> GpuCapabilities {
        self.capabilities.clone()
    }

    fn name(&self) -> &'static str {
        "NullBackend"
    }

    fn is_available(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_backend_new() {
        let backend = NullBackend::new();
        assert_eq!(backend.kind(), GpuBackendKind::Null);
        assert!(backend.is_available());
        assert_eq!(backend.name(), "NullBackend");
    }

    #[test]
    fn test_null_backend_capabilities() {
        let backend = NullBackend::new();
        let caps = backend.capabilities();
        assert_eq!(caps.backend, GpuBackendKind::Null);
        assert!(!caps.compute);
    }

    #[test]
    fn test_null_backend_with_custom_caps() {
        let mut caps = GpuCapabilities::for_backend(GpuBackendKind::Null);
        caps.compute = true;
        let backend = NullBackend::new_with_capabilities(caps.clone());
        assert_eq!(backend.capabilities().compute, true);
    }

    #[test]
    fn test_null_backend_default() {
        let backend: NullBackend = Default::default();
        assert_eq!(backend.kind(), GpuBackendKind::Null);
    }

    #[test]
    fn test_null_backend_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<NullBackend>();
    }
}
