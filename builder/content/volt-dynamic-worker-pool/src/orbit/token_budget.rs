use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

pub struct TokenBudget {
    pub tokens_per_burst: u64,
    pub tokens_remaining: AtomicU64,
    pub refill_rate_per_sec: u64,
    pub last_refill: AtomicU64,
}

impl TokenBudget {
    pub fn new(tokens_per_burst: u64, refill_rate_per_sec: u64) -> Self {
        Self {
            tokens_per_burst,
            tokens_remaining: AtomicU64::new(tokens_per_burst),
            refill_rate_per_sec,
            last_refill: AtomicU64::new(now_ms()),
        }
    }

    pub fn refill(&self) {
        let now_ms = now_ms();
        let last = self.last_refill.load(Ordering::Acquire);
        if now_ms <= last {
            return;
        }
        if self
            .last_refill
            .compare_exchange(last, now_ms, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return;
        }
        let elapsed_ms = now_ms - last;
        let tokens_to_add = (elapsed_ms * self.refill_rate_per_sec) / 1000;
        if tokens_to_add > 0 {
            let mut current = self.tokens_remaining.load(Ordering::Relaxed);
            loop {
                let new_tokens = (current + tokens_to_add).min(self.tokens_per_burst);
                match self.tokens_remaining.compare_exchange(
                    current,
                    new_tokens,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ) {
                    Ok(_) => break,
                    Err(actual) => current = actual,
                }
            }
        }
    }

    pub fn consume(&self, tokens: u64) -> bool {
        self.refill();
        loop {
            let current = self.tokens_remaining.load(Ordering::Acquire);
            if current < tokens {
                return false;
            }
            if self
                .tokens_remaining
                .compare_exchange(current, current - tokens, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return true;
            }
        }
    }

    pub fn remaining(&self) -> u64 {
        self.refill();
        self.tokens_remaining.load(Ordering::Acquire)
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_initial_tokens() {
        let budget = TokenBudget::new(100, 10);
        assert_eq!(budget.remaining(), 100);
    }

    #[test]
    fn test_consume_success() {
        let budget = TokenBudget::new(100, 10);
        assert!(budget.consume(30));
        assert_eq!(budget.remaining(), 70);
    }

    #[test]
    fn test_consume_insufficient() {
        let budget = TokenBudget::new(50, 10);
        assert!(!budget.consume(100));
    }

    #[test]
    fn test_consume_exact() {
        let budget = TokenBudget::new(50, 10);
        assert!(budget.consume(50));
        assert_eq!(budget.remaining(), 0);
    }

    #[test]
    fn test_refill_over_time() {
        let budget = TokenBudget::new(100, 1000);
        assert!(budget.consume(100));
        assert_eq!(budget.remaining(), 0);
        std::thread::sleep(Duration::from_millis(50));
        let remaining = budget.remaining();
        assert!(
            remaining > 0,
            "expected some tokens after refill, got {}",
            remaining
        );
    }

    #[test]
    fn test_refill_caps_at_max() {
        let budget = TokenBudget::new(100, 10000);
        std::thread::sleep(Duration::from_millis(50));
        let remaining = budget.remaining();
        assert!(
            remaining <= 100,
            "tokens should not exceed tokens_per_burst, got {}",
            remaining
        );
    }

    #[test]
    fn test_consume_multiple() {
        let budget = TokenBudget::new(100, 0);
        assert!(budget.consume(40));
        assert!(budget.consume(30));
        assert!(budget.consume(30));
        assert!(!budget.consume(1));
    }
}
