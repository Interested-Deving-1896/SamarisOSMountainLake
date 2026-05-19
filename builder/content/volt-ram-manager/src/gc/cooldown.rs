use std::time::{Duration, Instant};

pub struct GcCooldown {
    last_run: Instant,
    cooldown_duration: Duration,
}

impl GcCooldown {
    pub fn new(cooldown_ms: u64) -> Self {
        Self {
            last_run: Instant::now(),
            cooldown_duration: Duration::from_millis(cooldown_ms),
        }
    }

    pub fn with_duration(duration: Duration) -> Self {
        Self {
            last_run: Instant::now(),
            cooldown_duration: duration,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.elapsed() >= self.cooldown_duration
    }

    pub fn reset(&mut self) {
        self.last_run = Instant::now();
    }

    pub fn elapsed(&self) -> Duration {
        self.last_run.elapsed()
    }

    pub fn remaining(&self) -> Duration {
        self.cooldown_duration.saturating_sub(self.elapsed())
    }

    pub fn cooldown_ms(&self) -> u64 {
        self.cooldown_duration.as_millis() as u64
    }

    pub fn set_cooldown_ms(&mut self, ms: u64) {
        self.cooldown_duration = Duration::from_millis(ms);
    }

    pub fn set_cooldown(&mut self, duration: Duration) {
        self.cooldown_duration = duration;
    }

    pub fn since_last_run_ms(&self) -> u64 {
        self.elapsed().as_millis() as u64
    }

    pub fn force_ready(&mut self) {
        self.last_run = Instant::now()
            .checked_sub(self.cooldown_duration)
            .map(|t| t.checked_sub(Duration::from_millis(100)).unwrap_or(t))
            .unwrap_or_else(|| {
                let past = self.cooldown_duration.as_millis() as u64;
                Instant::now() - Duration::from_millis(past + 200)
            });
    }
}

impl Default for GcCooldown {
    fn default() -> Self {
        Self::new(5000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooldown_not_ready_immediately() {
        let cd = GcCooldown::new(1000);
        assert!(!cd.is_ready());
    }

    #[test]
    fn test_cooldown_ready_after_duration() {
        let mut cd = GcCooldown::new(1);
        std::thread::sleep(Duration::from_millis(2));
        assert!(cd.is_ready());
    }

    #[test]
    fn test_reset() {
        let mut cd = GcCooldown::new(1);
        std::thread::sleep(Duration::from_millis(2));
        assert!(cd.is_ready());
        cd.reset();
        assert!(!cd.is_ready());
    }

    #[test]
    fn test_remaining() {
        let cd = GcCooldown::new(1000);
        let remaining = cd.remaining();
        assert!(remaining > Duration::from_millis(990));
        assert!(remaining <= Duration::from_millis(1000));
    }

    #[test]
    fn test_force_ready() {
        let mut cd = GcCooldown::new(5000);
        assert!(!cd.is_ready());
        cd.force_ready();
        assert!(cd.is_ready());
    }

    #[test]
    fn test_default() {
        let cd = GcCooldown::default();
        assert_eq!(cd.cooldown_ms(), 5000);
    }

    #[test]
    fn test_since_last_run_ms() {
        let cd = GcCooldown::new(1000);
        let elapsed = cd.since_last_run_ms();
        assert!(elapsed < 10);
    }

    #[test]
    fn test_set_cooldown() {
        let mut cd = GcCooldown::new(100);
        assert_eq!(cd.cooldown_ms(), 100);
        cd.set_cooldown_ms(500);
        assert_eq!(cd.cooldown_ms(), 500);
        cd.set_cooldown(Duration::from_millis(1000));
        assert_eq!(cd.cooldown_ms(), 1000);
    }
}
