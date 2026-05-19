use crate::scheduler::queue::IoJob;

pub struct IoBatcher {
    max_batch_bytes: u64,
    max_batch_items: usize,
}

impl IoBatcher {
    pub fn new(max_batch_kb: u64) -> Self {
        IoBatcher {
            max_batch_bytes: max_batch_kb * 1024,
            max_batch_items: 64,
        }
    }

    pub fn batch(&self, jobs: Vec<IoJob>) -> Vec<Vec<IoJob>> {
        let mut batches: Vec<Vec<IoJob>> = Vec::new();
        for job in jobs {
            let placed = batches.last_mut().map_or(false, |batch| {
                if batch.len() >= self.max_batch_items {
                    return false;
                }
                if let Some(last) = batch.last() {
                    if !Self::can_batch(last, &job) {
                        return false;
                    }
                    let current_bytes: u64 =
                        batch.iter().map(|j| j.data_len).sum();
                    if current_bytes + job.data_len > self.max_batch_bytes {
                        return false;
                    }
                    true
                } else {
                    false
                }
            });

            if placed {
                batches.last_mut().unwrap().push(job);
            } else {
                batches.push(vec![job]);
            }
        }
        batches
    }

    pub fn can_batch(a: &IoJob, b: &IoJob) -> bool {
        a.path == b.path && a.priority == b.priority
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::priority::IoPriority;

    fn job(path: &str, offset: u64, len: u64, priority: IoPriority) -> IoJob {
        IoJob::new(1, path, offset, len, priority)
    }

    #[test]
    fn test_batch_single() {
        let batcher = IoBatcher::new(128);
        let jobs = vec![job("/a", 0, 4096, IoPriority::Desktop)];
        let batches = batcher.batch(jobs);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].len(), 1);
    }

    #[test]
    fn test_batch_multiple_same_path() {
        let batcher = IoBatcher::new(128);
        let jobs = vec![
            job("/a", 0, 4096, IoPriority::Desktop),
            job("/a", 4096, 4096, IoPriority::Desktop),
        ];
        let batches = batcher.batch(jobs);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0].len(), 2);
    }

    #[test]
    fn test_batch_different_paths_separate() {
        let batcher = IoBatcher::new(128);
        let jobs = vec![
            job("/a", 0, 4096, IoPriority::Desktop),
            job("/b", 0, 4096, IoPriority::Desktop),
        ];
        let batches = batcher.batch(jobs);
        assert_eq!(batches.len(), 2);
    }

    #[test]
    fn test_can_batch_same_path() {
        let a = job("/x", 0, 100, IoPriority::Desktop);
        let b = job("/x", 100, 100, IoPriority::Desktop);
        assert!(IoBatcher::can_batch(&a, &b));
    }

    #[test]
    fn test_can_batch_different_path() {
        let a = job("/x", 0, 100, IoPriority::Desktop);
        let b = job("/y", 0, 100, IoPriority::Desktop);
        assert!(!IoBatcher::can_batch(&a, &b));
    }

    #[test]
    fn test_batch_respects_max_items() {
        let batcher = IoBatcher { max_batch_bytes: 1024 * 1024, max_batch_items: 2 };
        let jobs = vec![
            job("/a", 0, 100, IoPriority::Desktop),
            job("/a", 100, 100, IoPriority::Desktop),
            job("/a", 200, 100, IoPriority::Desktop),
        ];
        let batches = batcher.batch(jobs);
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0].len(), 2);
        assert_eq!(batches[1].len(), 1);
    }

    #[test]
    fn test_empty_jobs() {
        let batcher = IoBatcher::new(128);
        let batches = batcher.batch(vec![]);
        assert!(batches.is_empty());
    }
}
