use std::time::Instant;

pub struct Profiler {
    enabled: bool,
}

impl Profiler {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn scope(&self, label: &'static str) -> ProfilerScope {
        ProfilerScope {
            label,
            start: if self.enabled { Some(Instant::now()) } else { None },
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

pub struct ProfilerScope {
    label: &'static str,
    start: Option<Instant>,
}

impl ProfilerScope {
    pub fn elapsed_us(&self) -> Option<u64> {
        self.start.map(|s| s.elapsed().as_micros() as u64)
    }
}

impl Drop for ProfilerScope {
    fn drop(&mut self) {
        if let Some(start) = self.start {
            let elapsed = start.elapsed();
            tracing::debug!("[profile] {} took {:?}", self.label, elapsed);
        }
    }
}
