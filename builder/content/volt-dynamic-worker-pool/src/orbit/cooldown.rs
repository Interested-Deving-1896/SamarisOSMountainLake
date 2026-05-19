use std::time::{Duration, Instant};

pub struct OrbitCooldown {
    pub cooldown_ms: u64,
    pub window_ms: u64,
    pub max_consecutive: u32,
    burst_timestamps: Vec<Instant>,
}

impl OrbitCooldown {
    pub fn new(cooldown_ms: u64, window_ms: u64, max_consecutive: u32) -> Self {
        Self {
            cooldown_ms,
            window_ms,
            max_consecutive,
            burst_timestamps: Vec::new(),
        }
    }

    pub fn is_in_cooldown(&self) -> bool {
        if let Some(last) = self.burst_timestamps.last() {
            last.elapsed().as_millis() < self.cooldown_ms as u128
        } else {
            false
        }
    }

    pub fn can_burst(&self) -> bool {
        if self.is_in_cooldown() {
            return false;
        }
        self.consecutive_bursts() < self.max_consecutive
    }

    pub fn record_burst(&mut self) {
        self.prune();
        self.burst_timestamps.push(Instant::now());
    }

    pub fn consecutive_bursts(&self) -> u32 {
        let cutoff = Instant::now() - Duration::from_millis(self.window_ms as u64);
        self.burst_timestamps
            .iter()
            .filter(|&&t| t > cutoff)
            .count() as u32
    }

    pub fn cooldown_remaining_ms(&self) -> u64 {
        if let Some(last) = self.burst_timestamps.last() {
            let elapsed = last.elapsed().as_millis() as u64;
            if elapsed < self.cooldown_ms {
                return self.cooldown_ms - elapsed;
            }
        }
        0
    }

    fn prune(&mut self) {
        let cutoff = Instant::now() - Duration::from_millis(self.window_ms as u64);
        self.burst_timestamps.retain(|&t| t > cutoff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_no_bursts_not_in_cooldown() {
        let cd = OrbitCooldown::new(1000, 5000, 3);
        assert!(!cd.is_in_cooldown());
        assert!(cd.can_burst());
        assert_eq!(cd.cooldown_remaining_ms(), 0);
    }

    #[test]
    fn test_record_and_cooldown() {
        let mut cd = OrbitCooldown::new(50, 5000, 3);
        cd.record_burst();
        assert!(cd.is_in_cooldown());
        assert!(!cd.can_burst());
        assert!(cd.cooldown_remaining_ms() > 0);
        assert!(cd.cooldown_remaining_ms() <= 50);
    }

    #[test]
    fn test_cooldown_expires() {
        let mut cd = OrbitCooldown::new(1, 5000, 3);
        cd.record_burst();
        sleep(Duration::from_millis(5));
        assert!(!cd.is_in_cooldown());
    }

    #[test]
    fn test_consecutive_bursts() {
        let mut cd = OrbitCooldown::new(0, 5000, 3);
        assert_eq!(cd.consecutive_bursts(), 0);
        cd.record_burst();
        assert_eq!(cd.consecutive_bursts(), 1);
        cd.record_burst();
        assert_eq!(cd.consecutive_bursts(), 2);
    }

    #[test]
    fn test_max_consecutive_blocks_burst() {
        let mut cd = OrbitCooldown::new(0, 5000, 2);
        assert!(cd.can_burst());
        cd.record_burst();
        assert!(cd.can_burst());
        cd.record_burst();
        assert!(!cd.can_burst());
    }

    #[test]
    fn test_window_expires_bursts() {
        let mut cd = OrbitCooldown::new(0, 10, 5);
        cd.record_burst();
        cd.record_burst();
        cd.record_burst();
        assert_eq!(cd.consecutive_bursts(), 3);
        sleep(Duration::from_millis(15));
        assert_eq!(cd.consecutive_bursts(), 0);
    }

    #[test]
    fn test_record_after_cooldown() {
        let mut cd = OrbitCooldown::new(0, 5000, 3);
        cd.record_burst();
        cd.record_burst();
        assert!(!cd.is_in_cooldown());
        assert!(cd.can_burst());
        cd.record_burst();
        assert_eq!(cd.consecutive_bursts(), 3);
    }
}
