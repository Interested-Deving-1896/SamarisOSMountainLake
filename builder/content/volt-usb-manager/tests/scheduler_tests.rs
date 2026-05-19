use volt_usb_manager::scheduler::batcher::IoBatcher;
use volt_usb_manager::scheduler::fairness::FairnessPolicy;
use volt_usb_manager::scheduler::io_scheduler::IoScheduler;
use volt_usb_manager::scheduler::priority::IoPriority;
use volt_usb_manager::scheduler::queue::IoJob;

#[test]
fn test_small_writes_batched() {
    let batcher = IoBatcher::new(128);
    let jobs = vec![
        IoJob::new(1, "/file", 0, 4096, IoPriority::Desktop),
        IoJob::new(2, "/file", 4096, 4096, IoPriority::Desktop),
        IoJob::new(3, "/file", 8192, 4096, IoPriority::Desktop),
    ];
    let batches = batcher.batch(jobs);
    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].len(), 3);
}

#[test]
fn test_metadata_priority() {
    let sched = IoScheduler::new(4, 64);
    sched.submit("/meta", 0, 64, IoPriority::CriticalMetadata, true);
    sched.submit("/data", 0, 4096, IoPriority::Desktop, false);
    let batch = sched.dequeue_batch(10);
    assert_eq!(batch.len(), 2);
    assert!(batch[0].is_metadata);
}

#[test]
fn test_fairness_window() {
    let policy = FairnessPolicy::default();
    assert!(policy.can_schedule(IoPriority::CriticalMetadata, 0));
    assert!(policy.can_schedule(IoPriority::CriticalMetadata, 31));
    assert!(!policy.can_schedule(IoPriority::CriticalMetadata, 32));
    assert!(policy.can_schedule(IoPriority::Desktop, 0));
    assert!(policy.can_schedule(IoPriority::Desktop, 15));
    assert!(!policy.can_schedule(IoPriority::Desktop, 16));
    assert!(policy.can_schedule(IoPriority::Background, 0));
    assert!(!policy.can_schedule(IoPriority::Background, 8));
    assert!(policy.can_schedule(IoPriority::Cache, 0));
    assert!(!policy.can_schedule(IoPriority::Cache, 6));
}

#[test]
fn test_flush_order_safe_by_priority() {
    let mut q = volt_usb_manager::scheduler::queue::IoQueue::new();
    q.enqueue(IoJob::new(1, "/bg", 0, 4096, IoPriority::Background));
    q.enqueue(IoJob::new(2, "/meta", 0, 64, IoPriority::CriticalMetadata));
    q.enqueue(IoJob::new(3, "/uv", 0, 512, IoPriority::UserVisible));
    let first = q.dequeue().unwrap();
    assert_eq!(first.priority, IoPriority::CriticalMetadata);
    let second = q.dequeue().unwrap();
    assert_eq!(second.priority, IoPriority::UserVisible);
    let third = q.dequeue().unwrap();
    assert_eq!(third.priority, IoPriority::Background);
}

#[test]
fn test_scheduler_submit_and_dequeue_batch() {
    let sched = IoScheduler::new(4, 64);
    sched.submit("/a", 0, 100, IoPriority::Desktop, false);
    sched.submit("/b", 0, 200, IoPriority::Desktop, false);
    let batch = sched.dequeue_batch(2);
    assert_eq!(batch.len(), 2);
    assert_eq!(sched.pending_count(), 0);
}

#[test]
fn test_can_batch_same_path_same_priority() {
    let a = IoJob::new(1, "/x", 0, 4096, IoPriority::Desktop);
    let b = IoJob::new(2, "/x", 4096, 4096, IoPriority::Desktop);
    assert!(IoBatcher::can_batch(&a, &b));
}

#[test]
fn test_cannot_batch_different_paths() {
    let a = IoJob::new(1, "/x", 0, 4096, IoPriority::Desktop);
    let b = IoJob::new(2, "/y", 0, 4096, IoPriority::Desktop);
    assert!(!IoBatcher::can_batch(&a, &b));
}

#[test]
fn test_scheduler_submit_returns_unique_ids() {
    let sched = IoScheduler::new(4, 64);
    let id1 = sched.submit("/a", 0, 1, IoPriority::Cache, false);
    let id2 = sched.submit("/b", 0, 1, IoPriority::Cache, false);
    assert_ne!(id1, id2);
}

#[test]
fn test_complete_decrements_active_count() {
    let sched = IoScheduler::new(1, 64);
    let id = sched.submit("/x", 0, 512, IoPriority::UserVisible, false);
    let _ = sched.dequeue_batch(1);
    sched.complete(id);
    assert!(sched.can_submit());
}
