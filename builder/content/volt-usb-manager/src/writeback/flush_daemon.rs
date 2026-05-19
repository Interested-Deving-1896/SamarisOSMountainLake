use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use parking_lot::Mutex;

use crate::core::result::VumResult;
use crate::journal::journal::Journal;
use crate::writeback::ack::WriteAck;
use crate::writeback::write_buffer::WriteBuffer;

pub struct FlushDaemon {
    running: Arc<AtomicBool>,
    handle: Mutex<Option<std::thread::JoinHandle<()>>>,
    buffer: Arc<Mutex<WriteBuffer>>,
    journal: Arc<Journal>,
    backing_path: String,
    flush_interval_ms: u64,
    flush_at_percent: u8,
    batch_size_kb: u64,
}

impl FlushDaemon {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        buffer: Arc<Mutex<WriteBuffer>>,
        journal: Arc<Journal>,
        backing_path: &str,
        interval_ms: u64,
        at_pct: u8,
        batch_kb: u64,
    ) -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            handle: Mutex::new(None),
            buffer,
            journal,
            backing_path: backing_path.to_string(),
            flush_interval_ms: interval_ms,
            flush_at_percent: at_pct,
            batch_size_kb: batch_kb,
        }
    }

    pub fn start(&self) {
        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let buffer = self.buffer.clone();
        let journal = self.journal.clone();
        let backing = self.backing_path.clone();
        let flush_interval = Duration::from_millis(self.flush_interval_ms.min(100));
        let at_pct = self.flush_at_percent;
        let batch_kb = self.batch_size_kb;

        let handle = std::thread::spawn(move || {
            let check_interval = Duration::from_millis(100);
            loop {
                std::thread::sleep(check_interval);
                if !running.load(Ordering::SeqCst) {
                    let _ = flush_all_inner(&buffer, &journal, &backing, batch_kb);
                    break;
                }
                let should_flush = {
                    let buf = buffer.lock();
                    buf.needs_flush(at_pct)
                };
                if should_flush || last_flush_elapsed(&buffer) >= flush_interval {
                    if let Err(e) = flush_once_inner(&buffer, &journal, &backing, batch_kb) {
                        tracing::warn!("FlushDaemon flush_once error: {:?}", e);
                    }
                }
            }
        });
        *self.handle.lock() = Some(handle);
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.lock().take() {
            let _ = handle.join();
        }
    }

    pub fn flush_once(&self) -> VumResult<u64> {
        flush_once_inner(&self.buffer, &self.journal, &self.backing_path, self.batch_size_kb)
    }

    pub fn flush_all(&self) -> VumResult<u64> {
        flush_all_inner(&self.buffer, &self.journal, &self.backing_path, self.batch_size_kb)
    }
}

fn last_flush_elapsed(buffer: &Arc<Mutex<WriteBuffer>>) -> Duration {
    let _buf = buffer.lock();
    Duration::from_secs(0)
}

fn flush_once_inner(
    buffer: &Arc<Mutex<WriteBuffer>>,
    journal: &Journal,
    backing_path: &str,
    batch_size_kb: u64,
) -> VumResult<u64> {
    let batch = {
        let mut buf = buffer.lock();
        buf.flush_batch(batch_size_kb)
    };
    if batch.is_empty() {
        return Ok(0);
    }
    let count = batch.len() as u64;
    for write in &batch {
        let rel_path = write.path.trim_start_matches('/');
        let dest = Path::new(backing_path).join(rel_path);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&dest)
            .map_err(|e| crate::core::error::VumError::WriteFailed(format!("Flush write: {}", e)))?;
        file.write_all(&write.data)
            .map_err(|e| crate::core::error::VumError::WriteFailed(format!("Flush write: {}", e)))?;
        if write.requires_fsync {
            file.sync_all()
                .map_err(|e| crate::core::error::VumError::FsyncFailed(format!("Flush fsync: {}", e)))?;
        }
        if write.journal_record_id != 0 {
            journal
                .commit_write(write.journal_record_id)
                .map_err(|e| {
                    crate::core::error::VumError::JournalWriteFailed(format!(
                        "Flush journal commit: {}",
                        e
                    ))
                })?;
        }
        {
            let buf = buffer.lock();
            buf.ack_sender()
                .send(WriteAck::durable(write.write_id))
                .ok();
        }
    }
    Ok(count)
}

fn flush_all_inner(
    buffer: &Arc<Mutex<WriteBuffer>>,
    journal: &Journal,
    backing_path: &str,
    batch_size_kb: u64,
) -> VumResult<u64> {
    let mut total = 0u64;
    loop {
        let batch = {
            let mut buf = buffer.lock();
            buf.flush_batch(batch_size_kb)
        };
        if batch.is_empty() {
            break;
        }
        for write in &batch {
            let rel_path = write.path.trim_start_matches('/');
            let dest = Path::new(backing_path).join(rel_path);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&dest)
                .map_err(|e| {
                    crate::core::error::VumError::WriteFailed(format!("Flush write: {}", e))
                })?;
            file.write_all(&write.data)
                .map_err(|e| {
                    crate::core::error::VumError::WriteFailed(format!("Flush write: {}", e))
                })?;
            if write.requires_fsync {
                file.sync_all().map_err(|e| {
                    crate::core::error::VumError::FsyncFailed(format!("Flush fsync: {}", e))
                })?;
            }
            if write.journal_record_id != 0 {
                journal.commit_write(write.journal_record_id).map_err(|e| {
                    crate::core::error::VumError::JournalWriteFailed(format!(
                        "Flush journal commit: {}",
                        e
                    ))
                })?;
            }
            {
                let buf = buffer.lock();
                buf.ack_sender().send(WriteAck::durable(write.write_id)).ok();
            }
        }
        total += batch.len() as u64;
    }
    Ok(total)
}

impl std::fmt::Debug for FlushDaemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FlushDaemon")
            .field("running", &self.running.load(Ordering::Relaxed))
            .field("backing_path", &self.backing_path)
            .field("flush_interval_ms", &self.flush_interval_ms)
            .field("flush_at_percent", &self.flush_at_percent)
            .field("batch_size_kb", &self.batch_size_kb)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::journal::journal::JournalConfig;
    use crate::writeback::ack::WriteAckKind;
    use tempfile::tempdir;

    fn setup() -> (Arc<Mutex<WriteBuffer>>, Arc<Journal>, tempfile::TempDir, tempfile::TempDir) {
        let buf_dir = tempdir().unwrap();
        let backing_dir = tempdir().unwrap();
        let buffer = Arc::new(Mutex::new(WriteBuffer::new(10)));

        let config = JournalConfig {
            path: buf_dir.path().to_str().unwrap().to_string(),
            fsync_on_record: false,
            checkpoint_interval_ms: 99999,
        };
        let journal = Arc::new(Journal::open(config).unwrap());
        (buffer, journal, buf_dir, backing_dir)
    }

    #[test]
    fn test_flush_once_empty() {
        let (buffer, journal, _bd, backing) = setup();
        let daemon = FlushDaemon::new(buffer, journal, backing.path().to_str().unwrap(), 1000, 90, 64);
        let count = daemon.flush_once().unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_flush_once_with_writes() {
        let (buffer, journal, _bd, backing) = setup();
        {
            let mut buf = buffer.lock();
            buf.enqueue("/test.txt", 0, b"hello world".to_vec(), 0, 0).unwrap();
        }
        let daemon = FlushDaemon::new(
            buffer.clone(),
            journal.clone(),
            backing.path().to_str().unwrap(),
            1000,
            90,
            64,
        );
        let count = daemon.flush_once().unwrap();
        assert_eq!(count, 1);
        let dest = backing.path().join("test.txt");
        assert!(dest.exists());
        assert_eq!(
            std::fs::read_to_string(&dest).unwrap(),
            "hello world"
        );
    }

    #[test]
    fn test_flush_all_multiple_writes() {
        let (buffer, journal, _bd, backing) = setup();
        {
            let mut buf = buffer.lock();
            buf.enqueue("/a.txt", 0, b"aaa".to_vec(), 0, 0).unwrap();
            buf.enqueue("/b.txt", 0, b"bbb".to_vec(), 0, 0).unwrap();
            buf.enqueue("/c.txt", 0, b"ccc".to_vec(), 0, 0).unwrap();
        }
        let daemon = FlushDaemon::new(
            buffer,
            journal,
            backing.path().to_str().unwrap(),
            1000,
            90,
            1,
        );
        let count = daemon.flush_all().unwrap();
        assert_eq!(count, 3);
        assert!(backing.path().join("a.txt").exists());
        assert!(backing.path().join("b.txt").exists());
        assert!(backing.path().join("c.txt").exists());
    }

    #[test]
    fn test_flush_once_writes_to_correct_path() {
        let (buffer, journal, _bd, backing) = setup();
        {
            let mut buf = buffer.lock();
            buf.enqueue("/subdir/nested.txt", 0, b"nested".to_vec(), 0, 0).unwrap();
        }
        let daemon = FlushDaemon::new(
            buffer,
            journal,
            backing.path().to_str().unwrap(),
            1000,
            90,
            64,
        );
        daemon.flush_once().unwrap();
        let dest = backing.path().join("subdir").join("nested.txt");
        assert!(dest.exists());
        assert_eq!(std::fs::read_to_string(dest).unwrap(), "nested");
    }

    #[test]
    fn test_flush_once_durable_ack_sent() {
        let (buffer, journal, _bd, backing) = setup();
        {
            let mut buf = buffer.lock();
            buf.enqueue("/ack_test", 0, b"data".to_vec(), 0, 0).unwrap();
        }
        // drain initial buffered ack
        let buffered = buffer.lock().acks().recv().unwrap();
        assert_eq!(buffered.kind, WriteAckKind::Buffered);

        let daemon = FlushDaemon::new(
            buffer.clone(),
            journal,
            backing.path().to_str().unwrap(),
            1000,
            90,
            64,
        );
        daemon.flush_once().unwrap();
        let ack = buffer.lock().acks().recv().unwrap();
        assert_eq!(ack.kind, WriteAckKind::Durable);
    }

    #[test]
    fn test_flush_daemon_start_stop() {
        let (buffer, journal, _bd, backing) = setup();
        let daemon = FlushDaemon::new(
            buffer,
            journal,
            backing.path().to_str().unwrap(),
            5000,
            99,
            64,
        );
        daemon.start();
        std::thread::sleep(Duration::from_millis(50));
        daemon.stop();
    }
}
