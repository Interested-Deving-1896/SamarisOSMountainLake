use std::time::{Duration, Instant};

pub struct ScalingCooldown {
    last_scale_time: Instant,
    cooldown_duration: Duration,
}

impl ScalingCooldown {
    pub fn new(cooldown_duration: Duration) -> Self {
        ScalingCooldown {
            last_scale_time: Instant::now() - cooldown_duration - Duration::from_secs(1),
            cooldown_duration,
        }
    }

    pub fn with_duration_ms(ms: u64) -> Self {
        ScalingCooldown {
            last_scale_time: Instant::now() - Duration::from_millis(ms) - Duration::from_secs(1),
            cooldown_duration: Duration::from_millis(ms),
        }
    }

    pub fn can_scale(&self) -> bool {
        Instant::now().duration_since(self.last_scale_time) >= self.cooldown_duration
    }

    pub fn record_scale(&mut self) {
        self.last_scale_time = Instant::now();
    }

    pub fn remaining(&self) -> Duration {
        let elapsed = Instant::now().duration_since(self.last_scale_time);
        if elapsed >= self.cooldown_duration {
            Duration::ZERO
        } else {
            self.cooldown_duration - elapsed
        }
    }

    pub fn is_cooling(&self) -> bool {
        !self.can_scale()
    }

    pub fn cooldown_duration(&self) -> Duration {
        self.cooldown_duration
    }

    pub fn reset(&mut self) {
        self.last_scale_time = Instant::now() - self.cooldown_duration - Duration::from_secs(1);
    }

    pub fn set_cooldown_duration(&mut self, duration: Duration) {
        self.cooldown_duration = duration;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cooldown_initial_state() {
        let cd = ScalingCooldown::new(Duration::from_millis(100));
        assert!(cd.can_scale());
        assert!(!cd.is_cooling());
        assert_eq!(cd.remaining(), Duration::ZERO);
    }

    #[test]
    fn test_cooldown_after_record() {
        let mut cd = ScalingCooldown::new(Duration::from_millis(200));
        cd.record_scale();
        assert!(!cd.can_scale());
        assert!(cd.is_cooling());
        assert!(cd.remaining() > Duration::ZERO);
    }

    #[test]
    fn test_cooldown_expires() {
        let mut cd = ScalingCooldown::new(Duration::from_millis(10));
        cd.record_scale();
        thread::sleep(Duration::from_millis(20));
        assert!(cd.can_scale());
        assert_eq!(cd.remaining(), Duration::ZERO);
    }

    #[test]
    fn test_with_duration_ms() {
        let cd = ScalingCooldown::with_duration_ms(5000);
        assert_eq!(cd.cooldown_duration(), Duration::from_millis(5000));
    }

    #[test]
    fn test_reset() {
        let mut cd = ScalingCooldown::new(Duration::from_millis(500));
        cd.record_scale();
        assert!(cd.is_cooling());
        cd.reset();
        assert!(cd.can_scale());
    }
}
