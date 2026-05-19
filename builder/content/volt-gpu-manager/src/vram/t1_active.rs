use dashmap::DashMap;
use crate::resources::resource_id::GpuResourceId;

fn id_to_u64(id: GpuResourceId) -> u64 {
    let v = id.0.as_u128();
    (v ^ (v >> 64)) as u64
}

pub struct ActiveVramPool {
    pub used_bytes: u64,
    pub max_bytes: u64,
    resources: DashMap<u64, u64>,
}

impl ActiveVramPool {
    pub fn new(max_bytes: u64) -> Self {
        Self {
            used_bytes: 0,
            max_bytes,
            resources: DashMap::new(),
        }
    }

    pub fn allocate(&mut self, id: GpuResourceId, size: u64) -> bool {
        if self.used_bytes + size > self.max_bytes {
            return false;
        }
        self.resources.insert(id_to_u64(id), size);
        self.used_bytes += size;
        true
    }

    pub fn release(&mut self, id: GpuResourceId) {
        if let Some((_, size)) = self.resources.remove(&id_to_u64(id)) {
            self.used_bytes = self.used_bytes.saturating_sub(size);
        }
    }

    pub fn resource_size(&self, id: GpuResourceId) -> Option<u64> {
        self.resources.get(&id_to_u64(id)).map(|r| *r)
    }

    pub fn usage_pct(&self) -> f64 {
        if self.max_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes as f64 / self.max_bytes as f64) * 100.0
    }

    pub fn available(&self) -> u64 {
        self.max_bytes.saturating_sub(self.used_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rid(v: u128) -> GpuResourceId {
        GpuResourceId(uuid::Uuid::from_u128(v))
    }

    #[test]
    fn test_new_pool() {
        let pool = ActiveVramPool::new(1024);
        assert_eq!(pool.used_bytes, 0);
        assert_eq!(pool.max_bytes, 1024);
        assert_eq!(pool.available(), 1024);
    }

    #[test]
    fn test_allocate_success() {
        let mut pool = ActiveVramPool::new(1024);
        assert!(pool.allocate(GpuResourceId::new(), 512));
        assert_eq!(pool.used_bytes, 512);
    }

    #[test]
    fn test_allocate_exhausts_pool() {
        let mut pool = ActiveVramPool::new(100);
        assert!(pool.allocate(GpuResourceId::new(), 60));
        assert!(pool.allocate(GpuResourceId::new(), 40));
        assert!(!pool.allocate(GpuResourceId::new(), 1));
    }

    #[test]
    fn test_release_frees_bytes() {
        let mut pool = ActiveVramPool::new(1024);
        let id = GpuResourceId::new();
        pool.allocate(id, 256);
        assert_eq!(pool.used_bytes, 256);
        pool.release(id);
        assert_eq!(pool.used_bytes, 0);
    }

    #[test]
    fn test_release_unknown_id() {
        let mut pool = ActiveVramPool::new(1024);
        pool.release(GpuResourceId::new());
        assert_eq!(pool.used_bytes, 0);
    }

    #[test]
    fn test_usage_pct() {
        let mut pool = ActiveVramPool::new(1000);
        assert_eq!(pool.usage_pct(), 0.0);
        pool.allocate(GpuResourceId::new(), 250);
        assert!((pool.usage_pct() - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_zero_max() {
        let pool = ActiveVramPool::new(0);
        assert_eq!(pool.usage_pct(), 0.0);
        assert_eq!(pool.available(), 0);
    }

    #[test]
    fn test_available_decreases() {
        let mut pool = ActiveVramPool::new(500);
        assert_eq!(pool.available(), 500);
        pool.allocate(rid(1), 200);
        assert_eq!(pool.available(), 300);
    }

    #[test]
    fn test_multiple_allocations_and_releases() {
        let mut pool = ActiveVramPool::new(1000);
        let ids: Vec<_> = (0..5).map(|_| GpuResourceId::new()).collect();
        for id in &ids {
            assert!(pool.allocate(*id, 200));
        }
        assert_eq!(pool.used_bytes, 1000);
        pool.release(ids[0]);
        assert_eq!(pool.used_bytes, 800);
        pool.release(ids[1]);
        assert_eq!(pool.used_bytes, 600);
    }
}
