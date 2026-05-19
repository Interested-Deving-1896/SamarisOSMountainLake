use std::sync::Arc;
use std::time::Instant;

use dashmap::DashMap;
use parking_lot::RwLock;

use crate::backend::{GpuBackendKind, GpuHardware};
use crate::config::schema::VgmConfig;
use crate::core::error::VgmError;
use crate::core::lifecycle::{Lifecycle, LifecyclePhase};
use crate::core::result::VgmResult;
use crate::core::state::{ErrorState, VgmState};
use crate::device::GpuCapabilities;
use crate::metrics::GpuMetricsSnapshot;
use crate::resources::GpuResourceId;
use crate::sbp_gpu::{SbpGpuMessage, SbpGpuOpcode, SbpGpuResponse};
use crate::thermal::ThermalState;
use crate::vram::VramResidencyTier;

#[allow(dead_code)]
struct ResourceEntry {
    size: u64,
    tier: VramResidencyTier,
    pinned: bool,
    compressed: bool,
}

pub struct VoltGpuManager {
    state: RwLock<VgmState>,
    config: VgmConfig,
    lifecycle: RwLock<Lifecycle>,
    backend: RwLock<Option<GpuBackendKind>>,
    hardware: RwLock<Vec<GpuHardware>>,
    capabilities: RwLock<GpuCapabilities>,
    resource_table: Arc<DashMap<GpuResourceId, ResourceEntry>>,
    thermal_state: RwLock<ThermalState>,
    #[allow(dead_code)]
    metrics: RwLock<GpuMetricsSnapshot>,
    init_time: Instant,
}

impl VoltGpuManager {
    pub fn new(config: VgmConfig) -> Self {
        Self {
            state: RwLock::new(VgmState::Uninitialized),
            config,
            lifecycle: RwLock::new(Lifecycle::new()),
            backend: RwLock::new(None),
            hardware: RwLock::new(Vec::new()),
            capabilities: RwLock::new(GpuCapabilities::default()),
            resource_table: Arc::new(DashMap::new()),
            thermal_state: RwLock::new(ThermalState::default()),
            metrics: RwLock::new(GpuMetricsSnapshot::default()),
            init_time: Instant::now(),
        }
    }

    pub fn init(&self) -> VgmResult<()> {
        {
            let mut state = self.state.write();
            if !state.can_transition_to(&VgmState::ConfigLoaded) {
                return Err(VgmError::InvalidState(
                    "Manager already initialized or in invalid state".into(),
                ));
            }
            *state = VgmState::ConfigLoaded;
        }

        self.lifecycle
            .write()
            .transition(LifecyclePhase::Config)
            .map_err(|_| {
                VgmError::InternalInvariantViolation("Lifecycle transition to Config failed".into())
            })?;

        let backend_kind = match self.config.gpu.backend.as_str() {
            "wgpu" => GpuBackendKind::Wgpu,
            "vulkan" => GpuBackendKind::Vulkan,
            "metal" => GpuBackendKind::Metal,
            "null" => GpuBackendKind::Null,
            other => {
                return Err(VgmError::UnsupportedBackend(format!(
                    "Unknown backend '{}'",
                    other
                )))
            }
        };

        {
            let mut state = self.state.write();
            *state = VgmState::BackendSelected;
        }
        *self.backend.write() = Some(backend_kind);
        self.lifecycle
            .write()
            .transition(LifecyclePhase::Backend)
            .map_err(|_| {
                VgmError::InternalInvariantViolation(
                    "Lifecycle transition to Backend failed".into(),
                )
            })?;

        {
            let mut state = self.state.write();
            *state = VgmState::DeviceProbed;
        }
        let hw = GpuHardware::new(
            "Simulated GPU",
            "Samaris",
            backend_kind,
            8 * 1024 * 1024 * 1024,
        );
        *self.hardware.write() = vec![hw];
        *self.capabilities.write() = GpuCapabilities::default();
        self.lifecycle
            .write()
            .transition(LifecyclePhase::Devices)
            .map_err(|_| {
                VgmError::InternalInvariantViolation(
                    "Lifecycle transition to Devices failed".into(),
                )
            })?;

        {
            let mut state = self.state.write();
            *state = VgmState::ResourcesReady;
        }
        self.lifecycle
            .write()
            .transition(LifecyclePhase::Resources)
            .map_err(|_| {
                VgmError::InternalInvariantViolation(
                    "Lifecycle transition to Resources failed".into(),
                )
            })?;
        self.lifecycle
            .write()
            .transition(LifecyclePhase::VramReady)
            .map_err(|_| {
                VgmError::InternalInvariantViolation(
                    "Lifecycle transition to VramReady failed".into(),
                )
            })?;

        {
            let mut state = self.state.write();
            *state = VgmState::Running;
        }
        self.lifecycle
            .write()
            .transition(LifecyclePhase::Running)
            .map_err(|_| {
                VgmError::InternalInvariantViolation(
                    "Lifecycle transition to Running failed".into(),
                )
            })?;

        tracing::info!("VoltGpuManager initialized successfully");
        Ok(())
    }

    pub fn shutdown(&self) {
        let mut state = self.state.write();
        if state.can_transition_to(&VgmState::Shutdown) {
            *state = VgmState::Shutdown;
            let mut lifecycle = self.lifecycle.write();
            let _ = lifecycle.transition(LifecyclePhase::Shutdown);
            tracing::info!("VoltGpuManager shut down");
        }
    }

    pub fn state(&self) -> VgmState {
        *self.state.read()
    }

    pub fn set_error(&self, error: ErrorState) {
        let mut state = self.state.write();
        if state.can_transition_to(&VgmState::Error(error)) {
            *state = VgmState::Error(error);
            tracing::error!("VoltGpuManager entered error state: {:?}", error);
        }
    }

    pub fn handle_sbp(&self, msg: SbpGpuMessage) -> SbpGpuResponse {
        match msg.opcode {
            SbpGpuOpcode::Init => match self.init() {
                Ok(()) => SbpGpuResponse::ok(b"initialized".to_vec()),
                Err(e) => SbpGpuResponse::err(format!("{:?}", e).into_bytes()),
            },
            SbpGpuOpcode::Shutdown => {
                self.shutdown();
                SbpGpuResponse::ok(b"shutdown".to_vec())
            }
            SbpGpuOpcode::Query => {
                let snap = self.snapshot();
                SbpGpuResponse::ok(format!("{:?}", snap).into_bytes())
            }
            SbpGpuOpcode::Submit => SbpGpuResponse::ok(b"submitted".to_vec()),
            SbpGpuOpcode::Reset => {
                self.shutdown();
                SbpGpuResponse::ok(b"reset".to_vec())
            }
            _ => SbpGpuResponse::err(b"unsupported opcode".to_vec()),
        }
    }

    pub fn snapshot(&self) -> GpuMetricsSnapshot {
        let _state = *self.state.read();
        let thermal = self.thermal_state.read();
        let caps = self.capabilities.read();
        let backend = self.backend.read().map(|b| b.name().to_string()).unwrap_or_default();
        GpuMetricsSnapshot {
            gpu_enabled: true,
            backend,
            device_count: self.hardware.read().len() as u32,
            active_device: 0,
            vram_total_bytes: caps.max_buffer_size,
            vram_used_bytes: 0,
            thermal_state: thermal.level.name().into(),
            resource_count: self.resource_table.len() as u64,
            ..Default::default()
        }
    }

    pub fn config(&self) -> &VgmConfig {
        &self.config
    }

    pub fn hardware(&self) -> Vec<GpuHardware> {
        self.hardware.read().clone()
    }

    pub fn lifecycle(&self) -> Lifecycle {
        self.lifecycle.read().clone()
    }

    pub fn backend(&self) -> Option<GpuBackendKind> {
        *self.backend.read()
    }

    pub fn capabilities(&self) -> GpuCapabilities {
        self.capabilities.read().clone()
    }

    pub fn thermal_state(&self) -> ThermalState {
        self.thermal_state.read().clone()
    }

    pub fn resource_count(&self) -> usize {
        self.resource_table.len()
    }

    pub fn uptime_ms(&self) -> u64 {
        self.init_time.elapsed().as_millis() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::VgmConfig;

    #[test]
    fn test_manager_new() {
        let config = VgmConfig::default();
        let manager = VoltGpuManager::new(config);
        assert_eq!(manager.state(), VgmState::Uninitialized);
    }

    #[test]
    fn test_manager_init_and_shutdown() {
        let config = VgmConfig::default();
        let manager = VoltGpuManager::new(config);
        assert!(manager.init().is_ok());
        assert_eq!(manager.state(), VgmState::Running);
        assert!(manager.backend().is_some());
        manager.shutdown();
        assert_eq!(manager.state(), VgmState::Shutdown);
    }

    #[test]
    fn test_manager_double_init_fails() {
        let config = VgmConfig::default();
        let manager = VoltGpuManager::new(config);
        assert!(manager.init().is_ok());
        assert!(manager.init().is_err());
    }

    #[test]
    fn test_manager_snapshot() {
        let config = VgmConfig::default();
        let manager = VoltGpuManager::new(config);
        manager.init().unwrap();
        let snap = manager.snapshot();
        assert!(snap.gpu_enabled);
    }

    #[test]
    fn test_manager_sbp_handling() {
        let config = VgmConfig::default();
        let manager = VoltGpuManager::new(config);

        let query_msg = SbpGpuMessage::new(SbpGpuOpcode::Query, vec![]);
        let resp = manager.handle_sbp(query_msg);
        assert!(resp.is_success());

        let init_msg = SbpGpuMessage::new(SbpGpuOpcode::Init, vec![]);
        let resp = manager.handle_sbp(init_msg);
        assert!(resp.is_success());

        let shutdown_msg = SbpGpuMessage::new(SbpGpuOpcode::Shutdown, vec![]);
        let resp = manager.handle_sbp(shutdown_msg);
        assert!(resp.is_success());
    }
}
