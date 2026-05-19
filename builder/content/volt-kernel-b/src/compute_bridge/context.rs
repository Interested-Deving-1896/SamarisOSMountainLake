#[derive(Debug, Clone)]
pub struct ComputeContext {
    pub app_id: u32,
    pub cpu_time_us: u64,
    pub memory_allocated: u64,
    pub tasks_started: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
}

impl ComputeContext {
    pub fn new(app_id: u32) -> Self {
        Self {
            app_id,
            cpu_time_us: 0,
            memory_allocated: 0,
            tasks_started: 0,
            tasks_completed: 0,
            tasks_failed: 0,
        }
    }

    pub fn record_failure(&mut self) {
        self.tasks_failed += 1;
    }

    pub fn allocate_memory(&mut self, bytes: u64) {
        self.memory_allocated += bytes;
    }

    pub fn deallocate_memory(&mut self, bytes: u64) {
        self.memory_allocated = self.memory_allocated.saturating_sub(bytes);
    }
}
