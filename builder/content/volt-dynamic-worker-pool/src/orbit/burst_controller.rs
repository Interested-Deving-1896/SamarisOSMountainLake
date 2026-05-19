use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitBurstRequest {
    pub job_id: String,
    pub priority_boost: u8,
    pub burst_duration_ms: u64,
    pub reason: String,
}

impl OrbitBurstRequest {
    pub fn new(job_id: String, priority_boost: u8, burst_duration_ms: u64, reason: String) -> Self {
        Self {
            job_id,
            priority_boost: priority_boost.min(10),
            burst_duration_ms,
            reason,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrbitBurstDecision {
    Accepted,
    RejectedCooldown,
    RejectedLimitReached,
    RejectedNoCapacity,
    Rejected(String),
}

impl OrbitBurstDecision {
    pub fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }

    pub fn reason(&self) -> &str {
        match self {
            Self::Accepted => "accepted",
            Self::RejectedCooldown => "cooldown active",
            Self::RejectedLimitReached => "burst limit reached",
            Self::RejectedNoCapacity => "no capacity",
            Self::Rejected(msg) => msg.as_str(),
        }
    }
}

#[derive(Debug)]
pub struct OrbitBurstController {
    burst_count: AtomicU64,
    max_bursts_per_minute: u64,
    cooldown_ms: u64,
    last_burst_at: AtomicU64,
    cooldown_until: AtomicU64,
    max_concurrent_bursts: u32,
    active_bursts: AtomicU64,
}

impl OrbitBurstController {
    pub fn new(max_bursts_per_minute: u64, cooldown_ms: u64, max_concurrent_bursts: u32) -> Self {
        Self {
            burst_count: AtomicU64::new(0),
            max_bursts_per_minute,
            cooldown_ms,
            last_burst_at: AtomicU64::new(0),
            cooldown_until: AtomicU64::new(0),
            max_concurrent_bursts,
            active_bursts: AtomicU64::new(0),
        }
    }

    pub fn request_burst(&self, _req: &OrbitBurstRequest) -> OrbitBurstDecision {
        let now = now_ms();

        let cooldown = self.cooldown_until.load(Ordering::SeqCst);
        if now < cooldown {
            return OrbitBurstDecision::RejectedCooldown;
        }

        let active = self.active_bursts.load(Ordering::SeqCst);
        if active >= self.max_concurrent_bursts as u64 {
            return OrbitBurstDecision::RejectedNoCapacity;
        }

        let count = self.burst_count.fetch_add(1, Ordering::SeqCst) + 1;
        if count > self.max_bursts_per_minute {
            self.burst_count.fetch_sub(1, Ordering::SeqCst);
            return OrbitBurstDecision::RejectedLimitReached;
        }

        self.last_burst_at.store(now, Ordering::SeqCst);
        self.cooldown_until.store(now + self.cooldown_ms, Ordering::SeqCst);
        self.active_bursts.fetch_add(1, Ordering::SeqCst);

        OrbitBurstDecision::Accepted
    }

    pub fn release_burst(&self) {
        self.active_bursts.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn is_in_cooldown(&self) -> bool {
        now_ms() < self.cooldown_until.load(Ordering::SeqCst)
    }

    pub fn cooldown_remaining_ms(&self) -> u64 {
        let cooldown = self.cooldown_until.load(Ordering::SeqCst);
        let now = now_ms();
        if now < cooldown {
            cooldown - now
        } else {
            0
        }
    }

    pub fn burst_count(&self) -> u64 {
        self.burst_count.load(Ordering::SeqCst)
    }

    pub fn active_bursts(&self) -> u32 {
        self.active_bursts.load(Ordering::SeqCst) as u32
    }

    pub fn reset_count(&self) {
        self.burst_count.store(0, Ordering::SeqCst);
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burst_accepted() {
        let ctrl = OrbitBurstController::new(10, 1000, 5);
        let req = OrbitBurstRequest::new("job-1".into(), 2, 500, "test".into());
        let decision = ctrl.request_burst(&req);
        assert!(decision.is_accepted());
        assert_eq!(ctrl.active_bursts(), 1);
    }

    #[test]
    fn test_cooldown() {
        let ctrl = OrbitBurstController::new(10, 100_000, 5);
        let req = OrbitBurstRequest::new("job-1".into(), 1, 100, "test".into());
        assert!(ctrl.request_burst(&req).is_accepted());
        assert!(ctrl.is_in_cooldown());
        assert!(ctrl.cooldown_remaining_ms() > 0);
    }

    #[test]
    fn test_limit_reached() {
        let ctrl = OrbitBurstController::new(2, 0, 5);
        let req = OrbitBurstRequest::new("job".into(), 1, 100, "test".into());
        assert!(ctrl.request_burst(&req).is_accepted());
        assert!(ctrl.request_burst(&req).is_accepted());
        let decision = ctrl.request_burst(&req);
        assert_eq!(decision, OrbitBurstDecision::RejectedLimitReached);
    }

    #[test]
    fn test_release_burst() {
        let ctrl = OrbitBurstController::new(10, 0, 2);
        let req = OrbitBurstRequest::new("job-1".into(), 1, 100, "test".into());
        assert!(ctrl.request_burst(&req).is_accepted());
        assert_eq!(ctrl.active_bursts(), 1);
        ctrl.release_burst();
        assert_eq!(ctrl.active_bursts(), 0);
    }
}
