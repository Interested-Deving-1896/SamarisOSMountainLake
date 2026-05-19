use crate::core::result::VrmResult;
use crate::core::state::VrmState;

#[derive(Debug)]
pub enum LifecyclePhase {
    Init,
    ConfigLoaded,
    ShmInitialized,
    PoolsReady,
    AppRegistryReady,
    QuotaGovernorReady,
    PageTableReady,
    SbpRouterReady,
    Running,
    Shutdown,
}

pub struct Lifecycle {
    pub phase: LifecyclePhase,
    state: VrmState,
}

impl Lifecycle {
    pub fn new() -> Self {
        Self {
            phase: LifecyclePhase::Init,
            state: VrmState::new(),
        }
    }

    pub fn transition(&mut self, next: LifecyclePhase) -> VrmResult<()> {
        tracing::debug!("Lifecycle transition: {:?} → {:?}", self.phase, next);
        self.phase = next;
        Ok(())
    }

    pub fn state(&self) -> &VrmState {
        &self.state
    }

    pub fn is_running(&self) -> bool {
        matches!(self.phase, LifecyclePhase::Running)
    }
}
