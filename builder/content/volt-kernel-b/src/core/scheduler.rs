use std::sync::Arc;

use crate::protocol::TesseractCommand;
use crate::scheduler::Scheduler as SchedulerImpl;

pub use crate::scheduler::priority::Priority;
pub use crate::scheduler::{CommandResponse, ScheduledTask};

pub struct CoreScheduler {
    inner: Arc<SchedulerImpl>,
}

impl CoreScheduler {
    pub fn new(scheduler: Arc<SchedulerImpl>) -> Self {
        Self { inner: scheduler }
    }

    pub fn submit(&self, cmd: TesseractCommand) -> CommandResponse {
        self.inner.submit(cmd)
    }

    pub fn inner(&self) -> &Arc<SchedulerImpl> {
        &self.inner
    }
}
