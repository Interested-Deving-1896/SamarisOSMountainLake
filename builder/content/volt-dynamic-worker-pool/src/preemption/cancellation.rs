use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    pub fn token(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.cancelled)
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_not_cancelled() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());
    }

    #[test]
    fn test_cancel() {
        let token = CancellationToken::new();
        token.cancel();
        assert!(token.is_cancelled());
    }

    #[test]
    fn test_clone_shares_state() {
        let a = CancellationToken::new();
        let b = a.clone();
        a.cancel();
        assert!(b.is_cancelled());
    }

    #[test]
    fn test_token_arc() {
        let token = CancellationToken::new();
        let arc = token.token();
        assert!(!arc.load(Ordering::SeqCst));
        token.cancel();
        assert!(arc.load(Ordering::SeqCst));
    }
}
