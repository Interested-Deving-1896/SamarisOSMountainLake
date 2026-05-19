use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub struct JobCheckpoint {
    pub data: Vec<u8>,
    pub progress: f64,
    pub saved_at: Instant,
}

impl JobCheckpoint {
    pub fn new(data: Vec<u8>, progress: f64) -> Self {
        Self {
            data,
            progress,
            saved_at: Instant::now(),
        }
    }

    pub fn is_stale(&self, max_age_ms: u64) -> bool {
        let elapsed = self.saved_at.elapsed().as_millis() as u64;
        elapsed > max_age_ms
    }

    pub fn merge(&mut self, other: Self) {
        self.data = other.data;
        self.progress = other.progress;
        self.saved_at = other.saved_at;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_new() {
        let cp = JobCheckpoint::new(vec![0xAB, 0xCD], 0.75);
        assert_eq!(cp.data, vec![0xAB, 0xCD]);
        assert!((cp.progress - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_is_stale() {
        let cp = JobCheckpoint::new(vec![], 0.0);
        assert!(!cp.is_stale(10_000));
        thread::sleep(Duration::from_millis(5));
        assert!(cp.is_stale(1));
    }

    #[test]
    fn test_merge() {
        let mut cp1 = JobCheckpoint::new(vec![1, 2], 0.3);
        let cp2 = JobCheckpoint::new(vec![3, 4], 0.9);
        cp1.merge(cp2);
        assert_eq!(cp1.data, vec![3, 4]);
        assert!((cp1.progress - 0.9).abs() < f64::EPSILON);
    }
}
