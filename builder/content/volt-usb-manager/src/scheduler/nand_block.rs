pub struct NandBlock {
    pub size: u64,
}

impl NandBlock {
    pub const DEFAULT: Self = Self { size: 131072 };

    pub fn new(size_kb: u64) -> Self {
        NandBlock {
            size: size_kb * 1024,
        }
    }

    pub fn align_offset(&self, offset: u64) -> u64 {
        offset - (offset % self.size)
    }

    pub fn is_aligned(&self, offset: u64, len: u64) -> bool {
        offset % self.size == 0 && (len % self.size == 0 || len == 0)
    }

    pub fn split_blocks(&self, offset: u64, len: u64) -> Vec<(u64, u64)> {
        if len == 0 {
            return Vec::new();
        }
        let start_block = offset / self.size;
        let end_block = (offset + len - 1) / self.size;
        let mut blocks = Vec::new();
        for block_idx in start_block..=end_block {
            let block_start = block_idx * self.size;
            let chunk_start = block_start.max(offset);
            let chunk_end = (block_start + self.size).min(offset + len);
            blocks.push((chunk_start, chunk_end - chunk_start));
        }
        blocks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_size() {
        assert_eq!(NandBlock::DEFAULT.size, 131072);
    }

    #[test]
    fn test_new() {
        let block = NandBlock::new(64);
        assert_eq!(block.size, 65536);
    }

    #[test]
    fn test_align_offset() {
        let block = NandBlock::new(4);
        assert_eq!(block.align_offset(0), 0);
        assert_eq!(block.align_offset(4096), 4096);
        assert_eq!(block.align_offset(5000), 4096);
        assert_eq!(block.align_offset(8191), 4096);
        assert_eq!(block.align_offset(8192), 8192);
    }

    #[test]
    fn test_is_aligned() {
        let block = NandBlock::new(4);
        assert!(block.is_aligned(0, 0));
        assert!(block.is_aligned(4096, 0));
        assert!(block.is_aligned(4096, 4096));
        assert!(!block.is_aligned(100, 100));
        assert!(!block.is_aligned(4096, 100));
    }

    #[test]
    fn test_split_blocks_single() {
        let block = NandBlock::new(4);
        let splits = block.split_blocks(0, 4096);
        assert_eq!(splits.len(), 1);
        assert_eq!(splits[0], (0, 4096));
    }

    #[test]
    fn test_split_blocks_cross_boundary() {
        let block = NandBlock::new(4);
        let splits = block.split_blocks(3000, 5000);
        assert_eq!(splits.len(), 2);
        assert_eq!(splits[0], (3000, 1096)); // 4096 - 3000
        assert_eq!(splits[1], (4096, 3904)); // 5000 - 1096
    }

    #[test]
    fn test_split_blocks_empty() {
        let block = NandBlock::new(4);
        let splits = block.split_blocks(0, 0);
        assert!(splits.is_empty());
    }

    #[test]
    fn test_split_blocks_multi() {
        let block = NandBlock::new(4);
        let splits = block.split_blocks(0, 16384);
        assert_eq!(splits.len(), 4);
        for (i, (start, size)) in splits.iter().enumerate() {
            assert_eq!(*start, (i as u64) * 4096);
            assert_eq!(*size, 4096);
        }
    }
}
