use volt_gpu_manager::scheduler::{GpuScheduler, GpuPriority, GpuCommand, GpuCommandKind, GpuBatch};
use volt_gpu_manager::scheduler::desktop_guard::DesktopFrameGuard;

fn make_cmd(kind: GpuCommandKind, priority: GpuPriority) -> GpuCommand {
    GpuCommand::new(kind, priority, "test")
}

#[test]
fn critical_before_high_normal_idle() {
    let sched = GpuScheduler::new(16);
    sched.submit(make_cmd(GpuCommandKind::Compute, GpuPriority::Idle));
    sched.submit(make_cmd(GpuCommandKind::Compute, GpuPriority::Critical));
    sched.submit(make_cmd(GpuCommandKind::Compute, GpuPriority::Normal));
    sched.submit(make_cmd(GpuCommandKind::Compute, GpuPriority::High));
    let batch = sched.dequeue().unwrap();
    assert_eq!(batch[0].priority, GpuPriority::Critical);
    assert_eq!(batch[1].priority, GpuPriority::High);
    assert_eq!(batch[2].priority, GpuPriority::Normal);
    assert_eq!(batch[3].priority, GpuPriority::Idle);
}

#[test]
fn idle_paused_under_frame_pressure() {
    let mut guard = DesktopFrameGuard::new(16);
    guard.end_frame(30);
    assert!(guard.should_pause_priority(GpuPriority::Idle));
    assert!(!guard.should_pause_priority(GpuPriority::Critical));
    assert!(!guard.should_pause_priority(GpuPriority::High));
}

#[test]
fn compression_jobs_not_run_during_critical_frame() {
    let sched = GpuScheduler::new(16);
    sched.submit(make_cmd(GpuCommandKind::Compress, GpuPriority::Normal));
    sched.submit(make_cmd(GpuCommandKind::Render, GpuPriority::Critical));
    let batch = sched.dequeue().unwrap();
    assert_eq!(batch[0].priority, GpuPriority::Critical);
    assert_eq!(batch[0].kind, GpuCommandKind::Render);
}

#[test]
fn batch_size_by_priority() {
    assert_eq!(GpuPriority::Critical.batch_size(), 1);
    assert_eq!(GpuPriority::High.batch_size(), 4);
    assert_eq!(GpuPriority::Normal.batch_size(), 8);
    assert_eq!(GpuPriority::Idle.batch_size(), 16);
}

#[test]
fn scheduler_submit_batch() {
    let sched = GpuScheduler::new(16);
    let mut batch = GpuBatch::new(GpuPriority::Normal);
    batch.add(make_cmd(GpuCommandKind::Transfer, GpuPriority::Normal));
    batch.add(make_cmd(GpuCommandKind::Compute, GpuPriority::Normal));
    sched.submit_batch(batch).unwrap();
    assert_eq!(sched.queued_count(), 2);
}
