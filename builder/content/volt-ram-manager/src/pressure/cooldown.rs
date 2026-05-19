use std::time::{Duration, Instant};

pub struct Cooldown {
    last_action: Option<Instant>,
    cooldown_ms: u64,
}

impl Cooldown {
    pub fn new(cooldown_ms: u64) -> Self {
        Cooldown {
            last_action: None,
            cooldown_ms,
        }
    }

    pub fn is_ready(&self) -> bool {
        match self.last_action {
            Some(last) => last.elapsed().as_millis() >= self.cooldown_ms as u128,
            None => true,
        }
    }

    pub fn reset(&mut self) {
        self.last_action = Some(Instant::now());
    }

    pub fn time_since_last_ms(&self) -> u64 {
        match self.last_action {
            Some(last) => last.elapsed().as_millis() as u64,
            None => u64::MAX,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_ready() {
        let c = Cooldown::new(1000);
        assert!(c.is_ready());
    }

    #[test]
    fn test_after_reset_not_ready() {
        let mut c = Cooldown::new(5000);
        c.reset();
        assert!(!c.is_ready());
    }

    #[test]
    fn test_time_since_last() {
        let c = Cooldown::new(1000);
        assert_eq!(c.time_since_last_ms(), u64::MAX);
    }
}
