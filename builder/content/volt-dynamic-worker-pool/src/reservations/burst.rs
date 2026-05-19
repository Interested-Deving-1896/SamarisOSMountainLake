use std::time::Instant;

#[derive(Clone, Debug)]
pub struct BurstSnapshot {
    pub active: bool,
    pub elapsed_ms: u64,
    pub remaining_ms: u64,
    pub expired: bool,
}

pub struct BurstReservation {
    pub active: bool,
    pub started_at: Option<Instant>,
    pub duration_ms: u64,
}

impl BurstReservation {
    pub fn new(duration_ms: u64) -> Self {
        Self {
            active: false,
            started_at: None,
            duration_ms,
        }
    }

    pub fn start(&mut self) {
        self.active = true;
        self.started_at = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.active = false;
        self.started_at = None;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.started_at
            .map(|start| start.elapsed().as_millis() as u64)
            .unwrap_or(0)
    }

    pub fn remaining_ms(&self) -> u64 {
        self.duration_ms.saturating_sub(self.elapsed_ms())
    }

    pub fn is_expired(&self) -> bool {
        self.active && self.elapsed_ms() >= self.duration_ms
    }

    pub fn snapshot(&self) -> BurstSnapshot {
        BurstSnapshot {
            active: self.active,
            elapsed_ms: self.elapsed_ms(),
            remaining_ms: self.remaining_ms(),
            expired: self.is_expired(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_new() {
        let b = BurstReservation::new(5000);
        assert!(!b.active);
        assert!(b.started_at.is_none());
        assert_eq!(b.duration_ms, 5000);
    }

    #[test]
    fn test_start_and_stop() {
        let mut b = BurstReservation::new(5000);
        b.start();
        assert!(b.is_active());
        assert!(b.started_at.is_some());
        b.stop();
        assert!(!b.is_active());
        assert!(b.started_at.is_none());
    }

    #[test]
    fn test_elapsed_ms_when_not_started() {
        let b = BurstReservation::new(5000);
        assert_eq!(b.elapsed_ms(), 0);
    }

    #[test]
    fn test_elapsed_ms_increases() {
        let mut b = BurstReservation::new(5000);
        b.start();
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = b.elapsed_ms();
        assert!(elapsed >= 10, "expected >= 10, got {}", elapsed);
    }

    #[test]
    fn test_remaining_ms() {
        let mut b = BurstReservation::new(100);
        b.start();
        std::thread::sleep(Duration::from_millis(10));
        let remaining = b.remaining_ms();
        assert!(remaining <= 100);
        assert!(remaining >= 80, "expected >= 80, got {}", remaining);
    }

    #[test]
    fn test_is_expired() {
        let mut b = BurstReservation::new(1);
        b.start();
        std::thread::sleep(Duration::from_millis(2));
        assert!(b.is_expired());
    }

    #[test]
    fn test_not_expired_when_not_active() {
        let b = BurstReservation::new(1);
        assert!(!b.is_expired());
    }

    #[test]
    fn test_snapshot() {
        let mut b = BurstReservation::new(5000);
        b.start();
        let s = b.snapshot();
        assert!(s.active);
        assert!(!s.expired);
    }
}
