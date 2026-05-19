pub struct Scheduler {
    pub min_workers: u32,
    pub max_workers: u32,
    pub current_workers: u32,
    pub active_workers: u32,
    pub idle_workers: u32,
    pub desktop_frame_pressure: f64,
}

impl Scheduler {
    pub fn new(min_workers: u32, max_workers: u32) -> Self {
        Self {
            min_workers,
            max_workers,
            current_workers: min_workers,
            active_workers: 0,
            idle_workers: min_workers,
            desktop_frame_pressure: 0.0,
        }
    }

    pub fn should_scale_up(&self, queue_depth: usize, cpu_util: f64) -> bool {
        let scale_factor = self.max_workers as f64 * 0.75;
        if queue_depth as f64 > self.current_workers as f64 * 2.0
            && cpu_util < 0.80
            && self.current_workers < self.max_workers
        {
            return true;
        }
        if queue_depth as f64 > scale_factor && self.current_workers < self.max_workers {
            return true;
        }
        false
    }

    pub fn should_scale_down(&self, queue_depth: usize, cpu_util: f64) -> bool {
        if queue_depth as f64 > self.current_workers as f64 * 1.0 {
            return false;
        }
        if cpu_util > 0.30 {
            return false;
        }
        if self.current_workers <= self.min_workers {
            return false;
        }
        true
    }
}
