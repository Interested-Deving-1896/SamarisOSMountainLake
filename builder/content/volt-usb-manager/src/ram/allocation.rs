#[derive(Debug, Clone)]
pub struct RamAllocation {
    pub bytes: u64,
    pub purpose: String,
}

impl RamAllocation {
    pub fn new(bytes: u64, purpose: &str) -> Self {
        RamAllocation {
            bytes,
            purpose: purpose.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_allocation() {
        let alloc = RamAllocation::new(4096, "write buffer");
        assert_eq!(alloc.bytes, 4096);
        assert_eq!(alloc.purpose, "write buffer");
    }

    #[test]
    fn test_zero_bytes_allocation() {
        let alloc = RamAllocation::new(0, "empty");
        assert_eq!(alloc.bytes, 0);
    }

    #[test]
    fn test_allocation_clone() {
        let a = RamAllocation::new(8192, "cache");
        let b = a.clone();
        assert_eq!(a.bytes, b.bytes);
        assert_eq!(a.purpose, b.purpose);
    }

    #[test]
    fn test_allocation_debug() {
        let alloc = RamAllocation::new(1024, "test");
        let debug = format!("{:?}", alloc);
        assert!(debug.contains("1024"));
        assert!(debug.contains("test"));
    }
}
