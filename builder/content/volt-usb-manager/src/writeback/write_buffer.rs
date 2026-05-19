use std::sync::atomic::{AtomicU64, Ordering};

use crossbeam::channel::{unbounded, Receiver, Sender};

use crate::core::result::VumResult;
use crate::writeback::ack::{WriteAck, WriteAckKind};
use crate::writeback::pending_write::PendingWrite;
use crate::writeback::write_status::WriteStatus;

pub struct WriteBuffer {
    writes: Vec<PendingWrite>,
    max_bytes: u64,
    used_bytes: AtomicU64,
    next_id: AtomicU64,
    ack_tx: Sender<WriteAck>,
    ack_rx: Receiver<WriteAck>,
}

impl WriteBuffer {
    pub fn new(max_mb: u64) -> Self {
        let (tx, rx) = unbounded();
        Self {
            writes: Vec::new(),
            max_bytes: max_mb * 1024 * 1024,
            used_bytes: AtomicU64::new(0),
            next_id: AtomicU64::new(1),
            ack_tx: tx,
            ack_rx: rx,
        }
    }

    pub fn enqueue(
        &mut self,
        path: &str,
        offset: u64,
        data: Vec<u8>,
        priority: u8,
        journal_id: u64,
    ) -> VumResult<PendingWrite> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let mut pw = PendingWrite::new(id, path, offset, data, priority);
        pw.journal_record_id = journal_id;
        pw.status = WriteStatus::Pending;
        pw.requires_fsync = true;
        let size = pw.size() as u64;
        self.used_bytes.fetch_add(size, Ordering::Relaxed);
        self.writes.push(pw.clone());
        self.ack_tx
            .send(WriteAck::buffered(id))
            .expect("WriteBuffer ack channel closed");
        Ok(pw)
    }

    pub fn flush_batch(&mut self, max_batch_kb: u64) -> Vec<PendingWrite> {
        let max_bytes = (max_batch_kb * 1024) as u64;
        let mut batch = Vec::new();
        let mut accumulated: u64 = 0;
        let mut remaining = Vec::new();
        for mut write in self.writes.drain(..) {
            if write.is_flushable() && accumulated + write.size() as u64 <= max_bytes {
                write.status = WriteStatus::Flushing;
                accumulated += write.size() as u64;
                batch.push(write);
            } else {
                remaining.push(write);
            }
        }
        self.writes = remaining;
        for pw in &batch {
            let s = pw.size() as u64;
            self.used_bytes.fetch_sub(s, Ordering::Relaxed);
        }
        batch
    }

    pub fn acknowledge(&mut self, write_id: u64, kind: WriteAckKind) {
        for write in &mut self.writes {
            if write.write_id == write_id {
                write.status = match kind {
                    WriteAckKind::Durable => WriteStatus::Durable,
                    WriteAckKind::Error => WriteStatus::Failed,
                    WriteAckKind::Buffered => WriteStatus::Pending,
                };
                break;
            }
        }
    }

    pub fn pending_count(&self) -> usize {
        self.writes
            .iter()
            .filter(|w| w.status == WriteStatus::Pending)
            .count()
    }

    pub fn dirty_bytes(&self) -> u64 {
        self.used_bytes.load(Ordering::Relaxed)
    }

    pub fn usage_pct(&self) -> f64 {
        if self.max_bytes == 0 {
            return 0.0;
        }
        self.dirty_bytes() as f64 / self.max_bytes as f64 * 100.0
    }

    pub fn acks(&self) -> &Receiver<WriteAck> {
        &self.ack_rx
    }

    pub fn ack_sender(&self) -> &Sender<WriteAck> {
        &self.ack_tx
    }

    pub fn needs_flush(&self, threshold_pct: u8) -> bool {
        self.usage_pct() >= threshold_pct as f64
    }

    pub fn max_bytes(&self) -> u64 {
        self.max_bytes
    }

    pub fn next_id(&self) -> u64 {
        self.next_id.load(Ordering::Relaxed)
    }
}

impl std::fmt::Debug for WriteBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WriteBuffer")
            .field("pending_count", &self.pending_count())
            .field("dirty_bytes", &self.dirty_bytes())
            .field("max_bytes", &self.max_bytes)
            .field("usage_pct", &self.usage_pct())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_buffer_new() {
        let buf = WriteBuffer::new(1);
        assert_eq!(buf.pending_count(), 0);
        assert_eq!(buf.dirty_bytes(), 0);
        assert!(buf.usage_pct() < 0.01);
        assert!(!buf.needs_flush(1));
    }

    #[test]
    fn test_enqueue_increases_count() {
        let mut buf = WriteBuffer::new(1);
        let pw = buf.enqueue("/test", 0, vec![1, 2, 3], 0, 0).unwrap();
        assert_eq!(pw.write_id, 1);
        assert_eq!(buf.pending_count(), 1);
        assert_eq!(buf.dirty_bytes(), 3);
    }

    #[test]
    fn test_flush_batch_respects_max() {
        let mut buf = WriteBuffer::new(10);
        buf.enqueue("/a", 0, vec![0u8; 600], 0, 0).unwrap();
        buf.enqueue("/b", 0, vec![0u8; 600], 0, 0).unwrap();
        buf.enqueue("/c", 0, vec![0u8; 600], 0, 0).unwrap();
        let batch = buf.flush_batch(1); // 1KB = 1024 bytes, only fits one 600-byte write
        assert_eq!(batch.len(), 1);
        assert_eq!(buf.pending_count(), 2);
    }

    #[test]
    fn test_flush_batch_all() {
        let mut buf = WriteBuffer::new(10);
        buf.enqueue("/a", 0, vec![0u8; 100], 0, 0).unwrap();
        buf.enqueue("/b", 0, vec![0u8; 200], 0, 0).unwrap();
        let batch = buf.flush_batch(1024);
        assert_eq!(batch.len(), 2);
        assert_eq!(buf.pending_count(), 0);
        assert_eq!(buf.dirty_bytes(), 0);
    }

    #[test]
    fn test_acknowledge_durable() {
        let mut buf = WriteBuffer::new(1);
        let pw = buf.enqueue("/f", 0, vec![1], 0, 0).unwrap();
        buf.acknowledge(pw.write_id, WriteAckKind::Durable);
        assert_eq!(buf.pending_count(), 0);
    }

    #[test]
    fn test_acknowledge_error() {
        let mut buf = WriteBuffer::new(1);
        let pw = buf.enqueue("/f", 0, vec![1], 0, 0).unwrap();
        buf.acknowledge(pw.write_id, WriteAckKind::Error);
    }

    #[test]
    fn test_acks_channel() {
        let mut buf = WriteBuffer::new(1);
        buf.enqueue("/f", 0, vec![1], 0, 0).unwrap();
        let ack = buf.acks().recv().unwrap();
        assert_eq!(ack.write_id, 1);
        assert_eq!(ack.kind, WriteAckKind::Buffered);
    }

    #[test]
    fn test_needs_flush() {
        let mut buf = WriteBuffer::new(1);
        assert!(!buf.needs_flush(50));
        buf.enqueue("/big", 0, vec![0u8; 1024 * 512], 0, 0).unwrap();
        assert!(buf.needs_flush(50));
    }

    #[test]
    fn test_multiple_enqueue_ids_increment() {
        let mut buf = WriteBuffer::new(10);
        buf.enqueue("/a", 0, vec![], 0, 0).unwrap();
        buf.enqueue("/b", 0, vec![], 0, 0).unwrap();
        buf.enqueue("/c", 0, vec![], 0, 0).unwrap();
        assert_eq!(buf.next_id(), 4);
    }

    #[test]
    fn test_flush_batch_empty() {
        let mut buf = WriteBuffer::new(1);
        let batch = buf.flush_batch(64);
        assert!(batch.is_empty());
    }
}
