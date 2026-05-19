use dashmap::DashMap;
use crate::vram::compressed_pool::CompressedVramBlock;
use crate::resources::resource_id::GpuResourceId;
use crate::core::result::VgmResult;

fn id_to_u64(id: GpuResourceId) -> u64 {
    let v = id.0.as_u128();
    (v ^ (v >> 64)) as u64
}

pub struct CompressedVramPool {
    pub used_bytes: u64,
    pub max_bytes: u64,
    pub blocks: DashMap<u64, CompressedVramBlock>,
}

impl CompressedVramPool {
    pub fn new(max_bytes: u64) -> Self {
        Self {
            used_bytes: 0,
            max_bytes,
            blocks: DashMap::new(),
        }
    }

    pub fn insert(&mut self, id: GpuResourceId, block: CompressedVramBlock) -> VgmResult<()> {
        let key = id_to_u64(id);
        if self.blocks.contains_key(&key) {
            return Err(crate::core::error::VgmError::ResourceAlreadyExists(format!(
                "Resource {} already in compressed pool",
                id
            )));
        }
        let total = self.used_bytes + block.compressed_size;
        if total > self.max_bytes {
            return Err(crate::core::error::VgmError::CompressedPoolFull(format!(
                "Compressed pool full: {}+{} > {}",
                self.used_bytes, block.compressed_size, self.max_bytes
            )));
        }
        self.blocks.insert(key, block);
        self.used_bytes = total;
        Ok(())
    }

    pub fn remove(&mut self, id: GpuResourceId) {
        if let Some((_, block)) = self.blocks.remove(&id_to_u64(id)) {
            self.used_bytes = self.used_bytes.saturating_sub(block.compressed_size);
        }
    }

    pub fn get(&self, id: GpuResourceId) -> Option<CompressedVramBlock> {
        self.blocks.get(&id_to_u64(id)).map(|r| r.clone())
    }

    pub fn usage_pct(&self) -> f64 {
        if self.max_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes as f64 / self.max_bytes as f64) * 100.0
    }

    pub fn total_original_bytes(&self) -> u64 {
        self.blocks.iter().map(|r| r.original_size).sum()
    }

    pub fn compression_saved_bytes(&self) -> u64 {
        self.blocks
            .iter()
            .map(|r| r.original_size.saturating_sub(r.compressed_size))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compression::GpuCompressionAlgorithm;
    use crate::resources::resource_id::GpuResourceId;

    fn make_block(id: GpuResourceId, original: u64, compressed: u64) -> CompressedVramBlock {
        CompressedVramBlock::new(id, GpuCompressionAlgorithm::Zstd, original, compressed, 0xDEAD)
    }

    #[test]
    fn test_new_pool() {
        let pool = CompressedVramPool::new(1024);
        assert_eq!(pool.used_bytes, 0);
        assert_eq!(pool.max_bytes, 1024);
    }

    #[test]
    fn test_insert_and_get() {
        let mut pool = CompressedVramPool::new(1024);
        let id = GpuResourceId::new();
        let block = make_block(id, 1000, 300);
        assert!(pool.insert(id, block.clone()).is_ok());
        let retrieved = pool.get(id).unwrap();
        assert_eq!(retrieved.original_size, 1000);
        assert_eq!(retrieved.compressed_size, 300);
    }

    #[test]
    fn test_insert_duplicate_fails() {
        let mut pool = CompressedVramPool::new(1024);
        let id = GpuResourceId::new();
        let block = make_block(id, 100, 50);
        pool.insert(id, block.clone()).unwrap();
        let result = pool.insert(id, block);
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_exhausts_capacity() {
        let mut pool = CompressedVramPool::new(100);
        let b1 = make_block(GpuResourceId::new(), 200, 60);
        let b2 = make_block(GpuResourceId::new(), 200, 60);
        assert!(pool.insert(GpuResourceId::new(), b1).is_ok());
        assert!(pool.insert(GpuResourceId::new(), b2).is_err());
    }

    #[test]
    fn test_remove_frees_bytes() {
        let mut pool = CompressedVramPool::new(1024);
        let id = GpuResourceId::new();
        let block = make_block(id, 1000, 400);
        pool.insert(id, block).unwrap();
        assert_eq!(pool.used_bytes, 400);
        pool.remove(id);
        assert_eq!(pool.used_bytes, 0);
    }

    #[test]
    fn test_get_nonexistent() {
        let pool = CompressedVramPool::new(1024);
        assert!(pool.get(GpuResourceId::new()).is_none());
    }

    #[test]
    fn test_usage_pct() {
        let mut pool = CompressedVramPool::new(1000);
        assert_eq!(pool.usage_pct(), 0.0);
        let b = make_block(GpuResourceId::new(), 500, 250);
        pool.insert(GpuResourceId::new(), b).unwrap();
        assert!((pool.usage_pct() - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_total_original_bytes() {
        let mut pool = CompressedVramPool::new(10_000);
        pool.insert(GpuResourceId::new(), make_block(GpuResourceId::new(), 800, 200)).unwrap();
        pool.insert(GpuResourceId::new(), make_block(GpuResourceId::new(), 400, 100)).unwrap();
        assert_eq!(pool.total_original_bytes(), 1200);
    }

    #[test]
    fn test_compression_saved_bytes() {
        let mut pool = CompressedVramPool::new(10_000);
        pool.insert(GpuResourceId::new(), make_block(GpuResourceId::new(), 1000, 300)).unwrap();
        pool.insert(GpuResourceId::new(), make_block(GpuResourceId::new(), 500, 200)).unwrap();
        assert_eq!(pool.compression_saved_bytes(), (1000 - 300) + (500 - 200));
    }
}
