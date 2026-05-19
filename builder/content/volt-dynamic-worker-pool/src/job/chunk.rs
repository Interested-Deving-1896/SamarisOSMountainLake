#[derive(Debug, Clone)]
pub struct JobChunk {
    pub chunk_id: u64,
    pub total_chunks: u64,
    pub data: Vec<u8>,
    pub checkpoint: Option<Vec<u8>>,
}

impl JobChunk {
    pub fn new(chunk_id: u64, total_chunks: u64, data: Vec<u8>) -> Self {
        Self {
            chunk_id,
            total_chunks,
            data,
            checkpoint: None,
        }
    }

    pub fn progress(&self) -> f64 {
        if self.total_chunks == 0 {
            return 1.0;
        }
        (self.chunk_id as f64 + 1.0) / self.total_chunks as f64
    }
}
