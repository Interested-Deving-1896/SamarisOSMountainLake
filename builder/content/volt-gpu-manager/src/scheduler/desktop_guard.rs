use crate::scheduler::frame_budget::FrameBudget;
use crate::scheduler::priority::GpuPriority;

pub struct DesktopFrameGuard {
    pub frame_budget: FrameBudget,
    pub critical_reserved_pct: u8,
    pub idle_paused: bool,
}

impl DesktopFrameGuard {
    pub fn new(budget_ms: u64) -> Self {
        DesktopFrameGuard {
            frame_budget: FrameBudget::new(budget_ms),
            critical_reserved_pct: 20,
            idle_paused: false,
        }
    }

    pub fn should_pause_priority(&self, priority: GpuPriority) -> bool {
        if !self.is_frame_pressure() {
            return false;
        }
        match priority {
            GpuPriority::Critical => false,
            GpuPriority::High => false,
            GpuPriority::Normal => self.is_frame_pressure(),
            GpuPriority::Idle => true,
        }
    }

    pub fn begin_frame(&mut self) {
        self.frame_budget.start_frame();
        self.idle_paused = self.is_frame_pressure();
    }

    pub fn end_frame(&mut self, elapsed_ms: u64) {
        self.frame_budget.end_frame(elapsed_ms);
        self.idle_paused = self.is_frame_pressure();
    }

    pub fn is_frame_pressure(&self) -> bool {
        !self.frame_budget.is_within_budget()
            || self.frame_budget.miss_rate() > 0.15
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_guard() {
        let g = DesktopFrameGuard::new(16);
        assert_eq!(g.frame_budget.budget_ms, 16);
        assert_eq!(g.critical_reserved_pct, 20);
        assert!(!g.idle_paused);
    }

    #[test]
    fn test_no_pressure_initially() {
        let g = DesktopFrameGuard::new(16);
        assert!(!g.is_frame_pressure());
    }

    #[test]
    fn test_frame_pressure_after_miss() {
        let mut g = DesktopFrameGuard::new(16);
        g.end_frame(30);
        assert!(g.is_frame_pressure());
    }

    #[test]
    fn test_should_not_pause_critical() {
        let mut g = DesktopFrameGuard::new(16);
        g.end_frame(30);
        assert!(!g.should_pause_priority(GpuPriority::Critical));
    }

    #[test]
    fn test_should_pause_idle_under_pressure() {
        let mut g = DesktopFrameGuard::new(16);
        g.end_frame(30);
        assert!(g.should_pause_priority(GpuPriority::Idle));
    }

    #[test]
    fn test_begin_frame_under_pressure() {
        let mut g = DesktopFrameGuard::new(16);
        g.end_frame(30);
        g.begin_frame();
        assert!(g.idle_paused);
    }

    #[test]
    fn test_begin_frame_unpauses_without_pressure() {
        let mut g = DesktopFrameGuard::new(16);
        g.idle_paused = true;
        g.begin_frame();
        assert!(!g.idle_paused);
    }

    #[test]
    fn test_no_pause_without_pressure() {
        let g = DesktopFrameGuard::new(16);
        assert!(!g.should_pause_priority(GpuPriority::Idle));
    }

    #[test]
    fn test_high_priority_not_paused() {
        let mut g = DesktopFrameGuard::new(16);
        g.end_frame(30);
        assert!(!g.should_pause_priority(GpuPriority::High));
    }
}
