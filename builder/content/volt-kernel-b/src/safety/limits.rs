use crate::core::error::{Result, TesseractError};

#[derive(Debug, Clone)]
pub struct ResourceLimiter {
    pub max_total_memory: u64,
    pub max_open_sockets: u32,
    pub max_concurrent_tasks: u32,
    pub current_sockets: u32,
    pub current_tasks: u32,
}

impl ResourceLimiter {
    pub fn new() -> Self {
        Self {
            max_total_memory: 1024 * 1024 * 1024,
            max_open_sockets: 64,
            max_concurrent_tasks: 16,
            current_sockets: 0,
            current_tasks: 0,
        }
    }

    pub fn check_memory(&self, requested: u64) -> Result<()> {
        if requested > self.max_total_memory {
            return Err(TesseractError::QuotaExceeded(format!(
                "memory request {} exceeds limit {}",
                requested, self.max_total_memory
            )));
        }
        Ok(())
    }

    pub fn reserve_socket(&mut self) -> Result<()> {
        if self.current_sockets >= self.max_open_sockets {
            return Err(TesseractError::QuotaExceeded(format!(
                "socket limit reached: {}/{}",
                self.current_sockets, self.max_open_sockets
            )));
        }
        self.current_sockets += 1;
        Ok(())
    }

    pub fn release_socket(&mut self) {
        self.current_sockets = self.current_sockets.saturating_sub(1);
    }

    pub fn reserve_task(&mut self) -> Result<()> {
        if self.current_tasks >= self.max_concurrent_tasks {
            return Err(TesseractError::QuotaExceeded(format!(
                "task limit reached: {}/{}",
                self.current_tasks, self.max_concurrent_tasks
            )));
        }
        self.current_tasks += 1;
        Ok(())
    }

    pub fn release_task(&mut self) {
        self.current_tasks = self.current_tasks.saturating_sub(1);
    }

    pub fn set_limits(&mut self, memory: u64, sockets: u32, tasks: u32) {
        self.max_total_memory = memory;
        self.max_open_sockets = sockets;
        self.max_concurrent_tasks = tasks;
    }
}
