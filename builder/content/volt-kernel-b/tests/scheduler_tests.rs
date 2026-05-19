use std::time::Instant;

use crossbeam::channel::unbounded;

use tesseract_engine::protocol::header::SbpHeader;
use tesseract_engine::protocol::opcodes::Opcode;
use tesseract_engine::protocol::TesseractCommand;
use tesseract_engine::scheduler::priority::Priority;
use tesseract_engine::scheduler::queue::PriorityQueues;
use tesseract_engine::scheduler::ScheduledTask;

fn make_task(priority: u8, opcode: Opcode) -> ScheduledTask {
    let header = SbpHeader::new(opcode, priority, 0x01, 0);
    let cmd = TesseractCommand::new(header, vec![]);
    let (tx, _) = unbounded();
    ScheduledTask {
        command: cmd,
        received_at: Instant::now(),
        response_tx: tx,
    }
}

#[test]
fn test_priority_from_byte() {
    assert_eq!(Priority::from_byte(0), Priority::Critical);
    assert_eq!(Priority::from_byte(1), Priority::High);
    assert_eq!(Priority::from_byte(2), Priority::Normal);
    assert_eq!(Priority::from_byte(3), Priority::Low);
    assert_eq!(Priority::from_byte(4), Priority::Idle);
    assert_eq!(Priority::from_byte(255), Priority::Idle);
}

#[test]
fn test_priority_ordering_correct() {
    assert!(Priority::Critical < Priority::High);
    assert!(Priority::High < Priority::Normal);
    assert!(Priority::Normal < Priority::Low);
    assert!(Priority::Low < Priority::Idle);
}

#[test]
fn test_max_per_cycle() {
    assert_eq!(Priority::Critical.max_per_cycle(), None);
    assert_eq!(Priority::High.max_per_cycle(), Some(8));
    assert_eq!(Priority::Normal.max_per_cycle(), Some(4));
    assert_eq!(Priority::Low.max_per_cycle(), Some(2));
    assert_eq!(Priority::Idle.max_per_cycle(), Some(1));
}

#[test]
fn test_priority_queue_enqueue_dequeue_order() {
    let mut pq = PriorityQueues::new();

    pq.enqueue(make_task(2, Opcode::Heartbeat));
    pq.enqueue(make_task(0, Opcode::Heartbeat));
    pq.enqueue(make_task(4, Opcode::Heartbeat));

    let tasks = pq.dequeue_cycle();
    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].command.priority(), 0);
    assert_eq!(tasks[1].command.priority(), 2);
    assert_eq!(tasks[2].command.priority(), 4);
}

#[test]
fn test_priority_caps_enforced() {
    let mut pq = PriorityQueues::new();

    for _ in 0..100 {
        pq.enqueue(make_task(1, Opcode::Heartbeat));
    }
    for _ in 0..100 {
        pq.enqueue(make_task(2, Opcode::Heartbeat));
    }

    let tasks = pq.dequeue_cycle();
    assert_eq!(tasks.len(), 12);
    assert_eq!(tasks.iter().filter(|t| t.command.priority() == 1).count(), 8);
    assert_eq!(tasks.iter().filter(|t| t.command.priority() == 2).count(), 4);
}

#[test]
fn test_critical_unbounded() {
    let mut pq = PriorityQueues::new();
    for _ in 0..50 {
        pq.enqueue(make_task(0, Opcode::Heartbeat));
    }
    let tasks = pq.dequeue_cycle();
    assert_eq!(tasks.len(), 50);
}

#[test]
fn test_empty_queue() {
    let mut pq = PriorityQueues::new();
    assert!(pq.is_empty());
    let tasks = pq.dequeue_cycle();
    assert!(tasks.is_empty());
}

#[test]
fn test_queue_lengths() {
    let mut pq = PriorityQueues::new();
    pq.enqueue(make_task(0, Opcode::Heartbeat));
    pq.enqueue(make_task(1, Opcode::Heartbeat));
    pq.enqueue(make_task(1, Opcode::Heartbeat));

    assert_eq!(pq.queue_len(Priority::Critical), 1);
    assert_eq!(pq.queue_len(Priority::High), 2);
    assert_eq!(pq.queue_len(Priority::Normal), 0);
}

#[test]
fn test_deterministic_order_same_input() {
    let mut pq1 = PriorityQueues::new();
    let mut pq2 = PriorityQueues::new();

    let tasks = vec![
        make_task(2, Opcode::Heartbeat),
        make_task(0, Opcode::Heartbeat),
        make_task(1, Opcode::Heartbeat),
        make_task(3, Opcode::Heartbeat),
    ];

    for t in &tasks {
        pq1.enqueue(ScheduledTask {
            command: TesseractCommand::new(
                SbpHeader::new(t.command.opcode().unwrap(), t.command.priority(), t.command.app_id(), 0),
                t.command.payload.clone(),
            ),
            received_at: t.received_at,
            response_tx: t.response_tx.clone(),
        });
        pq2.enqueue(ScheduledTask {
            command: TesseractCommand::new(
                SbpHeader::new(t.command.opcode().unwrap(), t.command.priority(), t.command.app_id(), 0),
                t.command.payload.clone(),
            ),
            received_at: t.received_at,
            response_tx: t.response_tx.clone(),
        });
    }

    let r1 = pq1.dequeue_cycle();
    let r2 = pq2.dequeue_cycle();

    assert_eq!(r1.len(), r2.len());
    for (a, b) in r1.iter().zip(r2.iter()) {
        assert_eq!(a.command.priority(), b.command.priority());
    }
}

#[test]
fn test_round_robin_within_level() {
    let mut pq = PriorityQueues::new();

    pq.enqueue(make_task(1, Opcode::Heartbeat));
    pq.enqueue(make_task(1, Opcode::Heartbeat));
    pq.enqueue(make_task(1, Opcode::Heartbeat));

    let tasks = pq.dequeue_cycle();
    assert_eq!(tasks.len(), 3);
}

#[test]
fn test_priority_to_byte() {
    assert_eq!(Priority::Critical.to_byte(), 0);
    assert_eq!(Priority::High.to_byte(), 1);
    assert_eq!(Priority::Normal.to_byte(), 2);
    assert_eq!(Priority::Low.to_byte(), 3);
    assert_eq!(Priority::Idle.to_byte(), 4);
}

#[test]
fn test_priority_name() {
    assert_eq!(Priority::Critical.name(), "CRITICAL");
    assert_eq!(Priority::High.name(), "HIGH");
    assert_eq!(Priority::Normal.name(), "NORMAL");
    assert_eq!(Priority::Low.name(), "LOW");
    assert_eq!(Priority::Idle.name(), "IDLE");
}

#[test]
fn test_priority_display() {
    assert_eq!(format!("{}", Priority::Critical), "CRITICAL");
    assert_eq!(format!("{}", Priority::Idle), "IDLE");
}

#[test]
fn test_queue_peek() {
    let mut pq = PriorityQueues::new();
    pq.enqueue(make_task(2, Opcode::Heartbeat));
    pq.enqueue(make_task(0, Opcode::Heartbeat));
    let peeked = pq.peek();
    assert!(peeked.is_some());
    assert_eq!(peeked.unwrap().command.priority(), 0);
    assert_eq!(pq.len(), 2);
}

#[test]
fn test_queue_clear() {
    let mut pq = PriorityQueues::new();
    pq.enqueue(make_task(1, Opcode::Heartbeat));
    pq.enqueue(make_task(2, Opcode::Heartbeat));
    assert!(!pq.is_empty());
    pq.clear();
    assert!(pq.is_empty());
    assert_eq!(pq.len(), 0);
}

#[test]
fn test_scheduler_submit_async() {
    use std::sync::Arc;
    use tesseract_engine::scheduler::Scheduler;
    use tesseract_engine::telemetry::Telemetry;

    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(2, 1, telemetry));

    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 0, 0x01, 0),
        vec![],
    );

    let rx = scheduler.submit_async(cmd);
    let response = rx.recv().unwrap_or(Err("channel closed".into()));
    assert!(response.is_ok());
}

#[test]
fn test_worker_pool_register_handler() {
    use std::sync::Arc;
    use parking_lot::RwLock;
    use tesseract_engine::scheduler::worker_pool::WorkerPool;

    let pool = WorkerPool::new(2);
    pool.register_handler(0xCC, |cmd| {
        Ok(cmd.payload.clone())
    });

    let mut header = SbpHeader::new(Opcode::Heartbeat, 0, 1, 4);
    header.opcode = 0xCC;
    let test_cmd = TesseractCommand::new(header, b"test".to_vec());

    let result = pool.execute(&test_cmd).unwrap();
    assert_eq!(result, b"test");
}

#[test]
fn test_worker_pool_active_count() {
    use tesseract_engine::scheduler::worker_pool::WorkerPool;

    let pool = WorkerPool::new(4);
    assert_eq!(pool.count(), 4);
    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 0, 1, 0),
        vec![],
    );
    pool.execute(&cmd).unwrap();
    // active_count should be 0 after execution completes
    assert_eq!(pool.active_count(), 0);
}
