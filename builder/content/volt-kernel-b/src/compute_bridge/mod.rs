pub mod buffer;
pub mod context;
pub mod task;

use std::collections::HashMap;

use crate::compute_bridge::buffer::BufferManager;
use crate::compute_bridge::context::ComputeContext;
use crate::core::error::Result;

pub struct ComputeBridge {
    contexts: HashMap<u32, ComputeContext>,
    buffer_mgr: BufferManager,
}

impl ComputeBridge {
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
            buffer_mgr: BufferManager::new(1024 * 1024 * 1024),
        }
    }

    pub fn execute_task(&mut self, task: &task::ComputeTask) -> Result<task::ComputeResult> {
        let ctx = self.contexts.entry(task.app_id).or_insert_with(|| {
            ComputeContext::new(task.app_id)
        });

        ctx.tasks_started += 1;

        let start = std::time::Instant::now();
        let output = task::execute_compute(task)?;
        let elapsed = start.elapsed();

        ctx.cpu_time_us += elapsed.as_micros() as u64;
        ctx.tasks_completed += 1;

        Ok(task::ComputeResult {
            task_id: task.task_id,
            output,
            elapsed_us: elapsed.as_micros() as u64,
        })
    }

    pub fn get_context(&self, app_id: u32) -> Option<&ComputeContext> {
        self.contexts.get(&app_id)
    }

    pub fn buffer_manager(&self) -> &BufferManager {
        &self.buffer_mgr
    }

    pub fn buffer_manager_mut(&mut self) -> &mut BufferManager {
        &mut self.buffer_mgr
    }
}
