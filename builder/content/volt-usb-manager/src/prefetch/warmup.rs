pub struct WarmupPlan {
    pub files: Vec<String>,
    pub total_bytes: u64,
    pub prefetched_bytes: u64,
    pub pinned: Vec<String>,
    pub duration_ms: u64,
}

impl WarmupPlan {
    pub fn new() -> Self {
        WarmupPlan {
            files: Vec::new(),
            total_bytes: 0,
            prefetched_bytes: 0,
            pinned: Vec::new(),
            duration_ms: 0,
        }
    }

    pub fn add_file(&mut self, path: &str, size: u64, pin: bool) {
        self.files.push(path.to_string());
        self.total_bytes += size;
        if pin {
            self.pinned.push(path.to_string());
        }
    }

    pub fn is_complete(&self) -> bool {
        self.prefetched_bytes >= self.total_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_plan() {
        let plan = WarmupPlan::new();
        assert!(plan.files.is_empty());
        assert_eq!(plan.total_bytes, 0);
        assert_eq!(plan.prefetched_bytes, 0);
        assert!(plan.pinned.is_empty());
        assert_eq!(plan.duration_ms, 0);
    }

    #[test]
    fn test_add_file() {
        let mut plan = WarmupPlan::new();
        plan.add_file("/boot/a.js", 1024, false);
        assert_eq!(plan.files.len(), 1);
        assert_eq!(plan.total_bytes, 1024);
        assert!(plan.pinned.is_empty());
    }

    #[test]
    fn test_add_pinned_file() {
        let mut plan = WarmupPlan::new();
        plan.add_file("/brand/logo.png", 2048, true);
        assert_eq!(plan.pinned.len(), 1);
        assert_eq!(plan.pinned[0], "/brand/logo.png");
    }

    #[test]
    fn test_is_complete() {
        let mut plan = WarmupPlan::new();
        plan.add_file("/a", 100, false);
        assert!(!plan.is_complete());
        plan.prefetched_bytes = 100;
        assert!(plan.is_complete());
    }

    #[test]
    fn test_multiple_files() {
        let mut plan = WarmupPlan::new();
        plan.add_file("/a.js", 100, false);
        plan.add_file("/b.css", 200, true);
        plan.add_file("/c.png", 300, true);
        assert_eq!(plan.files.len(), 3);
        assert_eq!(plan.total_bytes, 600);
        assert_eq!(plan.pinned.len(), 2);
    }
}
