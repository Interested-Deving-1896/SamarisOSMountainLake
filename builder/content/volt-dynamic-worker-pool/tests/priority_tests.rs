use volt_dynamic_worker_pool::*;
use volt_dynamic_worker_pool::prelude::*;

#[test]
fn test_priority_level_ordering() {
    use priority::level::PriorityLevel;
    assert!(PriorityLevel::Low < PriorityLevel::Normal);
    assert!(PriorityLevel::Normal < PriorityLevel::High);
    assert!(PriorityLevel::High < PriorityLevel::Critical);
    assert!(PriorityLevel::Critical < PriorityLevel::Realtime);
}

#[test]
fn test_priority_level_roundtrip() {
    use priority::level::PriorityLevel;
    for level in &[
        PriorityLevel::Low,
        PriorityLevel::Normal,
        PriorityLevel::High,
        PriorityLevel::Critical,
        PriorityLevel::Realtime,
    ] {
        assert_eq!(PriorityLevel::from_u8(level.as_u8()), Some(*level));
    }
}

#[test]
fn test_multi_queue_priority_ordering() {
    use priority::multi_queue::MultiQueue;
    use priority::level::PriorityLevel;
    use job::job::Job;

    let mq = MultiQueue::new();
    let low = Job::new(JobId::new(), "low".into(), PriorityLevel::Low, 64);
    let high = Job::new(JobId::new(), "high".into(), PriorityLevel::High, 64);
    mq.enqueue(low);
    mq.enqueue(high);

    let first = mq.dequeue().unwrap();
    assert_eq!(first.priority(), PriorityLevel::High);
}

#[test]
fn test_multi_queue_cancel() {
    use priority::multi_queue::MultiQueue;
    use priority::level::PriorityLevel;
    use job::job::Job;

    let mq = MultiQueue::new();
    let job = Job::new(JobId::new(), "cancelme".into(), PriorityLevel::Normal, 64);
    let id = job.id().clone();
    mq.enqueue(job);
    assert_eq!(mq.queue_depth(), 1);
    assert!(mq.cancel(&id));
    assert_eq!(mq.queue_depth(), 0);
}
