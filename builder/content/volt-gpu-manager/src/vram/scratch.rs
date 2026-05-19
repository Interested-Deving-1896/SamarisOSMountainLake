pub struct VramScratchBudget {
    pub reserved_bytes: u64,
    pub min_free_bytes: u64,
}

impl VramScratchBudget {
    pub fn new(decompression_mb: u64, min_free_mb: u64) -> Self {
        Self {
            reserved_bytes: decompression_mb * 1024 * 1024,
            min_free_bytes: min_free_mb * 1024 * 1024,
        }
    }

    pub fn can_restore(&self, current_free_vram: u64, resource_size: u64) -> bool {
        let needed = self.reserved_bytes + resource_size + self.min_free_bytes;
        current_free_vram >= needed
    }

    pub fn reserve(&mut self, size: u64) -> bool {
        if self.reserved_bytes >= size {
            self.reserved_bytes -= size;
            true
        } else {
            false
        }
    }

    pub fn release(&mut self, size: u64) {
        self.reserved_bytes = self.reserved_bytes.saturating_add(size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_budget() {
        let budget = VramScratchBudget::new(64, 16);
        assert_eq!(budget.reserved_bytes, 64 * 1024 * 1024);
        assert_eq!(budget.min_free_bytes, 16 * 1024 * 1024);
    }

    #[test]
    fn test_can_restore_sufficient() {
        let budget = VramScratchBudget::new(64, 16);
        let free_vram = 200u64 * 1024 * 1024;
        let resource_size = 50u64 * 1024 * 1024;
        assert!(budget.can_restore(free_vram, resource_size));
    }

    #[test]
    fn test_can_restore_insufficient() {
        let budget = VramScratchBudget::new(64, 16);
        let free_vram = 100u64 * 1024 * 1024;
        let resource_size = 50u64 * 1024 * 1024;
        assert!(!budget.can_restore(free_vram, resource_size));
    }

    #[test]
    fn test_reserve_success() {
        let mut budget = VramScratchBudget::new(64, 16);
        assert!(budget.reserve(32 * 1024 * 1024));
        assert_eq!(budget.reserved_bytes, 32 * 1024 * 1024);
    }

    #[test]
    fn test_reserve_failure() {
        let mut budget = VramScratchBudget::new(10, 5);
        assert!(!budget.reserve(20 * 1024 * 1024));
        assert_eq!(budget.reserved_bytes, 10 * 1024 * 1024);
    }

    #[test]
    fn test_release() {
        let mut budget = VramScratchBudget::new(64, 16);
        budget.reserve(32 * 1024 * 1024);
        budget.release(16 * 1024 * 1024);
        assert_eq!(budget.reserved_bytes, 48 * 1024 * 1024);
    }

    #[test]
    fn test_release_overflow() {
        let mut budget = VramScratchBudget::new(10, 5);
        budget.release(100 * 1024 * 1024);
        assert_eq!(budget.reserved_bytes, 110 * 1024 * 1024);
    }

    #[test]
    fn test_can_restore_exact_minimum() {
        let budget = VramScratchBudget::new(10, 10);
        let free_vram = 30u64 * 1024 * 1024;
        let resource_size = 10u64 * 1024 * 1024;
        assert!(budget.can_restore(free_vram, resource_size));
    }
}
