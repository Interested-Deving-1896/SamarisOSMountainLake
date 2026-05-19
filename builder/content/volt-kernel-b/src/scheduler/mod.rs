pub mod priority;
pub mod queue;
pub mod worker_pool;

use std::sync::Arc;
use std::time::Instant;

use crossbeam::channel::{unbounded, Receiver, Sender};
use parking_lot::RwLock;

use crate::protocol::{CommandPayload, TesseractCommand};
use crate::scheduler::queue::PriorityQueues;
use crate::scheduler::worker_pool::WorkerPool;
use crate::telemetry::Telemetry;

pub type CommandResponse = std::result::Result<CommandPayload, String>;

pub struct ScheduledTask {
    pub command: TesseractCommand,
    pub received_at: Instant,
    pub response_tx: Sender<CommandResponse>,
}

pub struct Scheduler {
    input_tx: Sender<(TesseractCommand, Sender<CommandResponse>)>,
    workers: Arc<RwLock<WorkerPool>>,
    telemetry: Arc<Telemetry>,
}

impl Scheduler {
    pub fn new(max_workers: usize, tick_ms: u64, telemetry: Arc<Telemetry>) -> Self {
        let (input_tx, input_rx) = unbounded::<(TesseractCommand, Sender<CommandResponse>)>();
        let workers = Arc::new(RwLock::new(WorkerPool::new(max_workers)));

        let workers_clone = workers.clone();
        let telemetry_clone = telemetry.clone();

        std::thread::Builder::new()
            .name("tesseract-scheduler".into())
            .spawn(move || {
                let mut queues = PriorityQueues::new();
                let tick = std::time::Duration::from_millis(tick_ms);

                loop {
                    while let Ok((cmd, response_tx)) = input_rx.try_recv() {
                        let task = ScheduledTask {
                            command: cmd,
                            received_at: Instant::now(),
                            response_tx,
                        };
                        queues.enqueue(task);
                    }

                    let tasks = queues.dequeue_cycle();
                    for task in tasks {
                        let start = Instant::now();
                        let worker_pool = workers_clone.read();
                        let result = worker_pool.execute(&task.command);
                        drop(worker_pool);
                        let elapsed = start.elapsed();

                        telemetry_clone.record_command(
                            task.command.header.opcode,
                            0,
                            elapsed.as_micros() as u64,
                        );

                        let response = result.map(|payload| {
                            match task.command.header.opcode {
                                0x01 | 0x02 => CommandPayload::RenderResult {
                                    pixel_data: payload,
                                    width: 0,
                                    height: 0,
                                },
                                _ => CommandPayload::RawBytes(payload),
                            }
                        }).or_else(|e| {
                            Err(e.to_string())
                        });
                        let _ = task.response_tx.send(response);
                    }

                    std::thread::sleep(tick);
                }
            })
            .expect("failed to spawn scheduler thread");

        Self {
            input_tx,
            workers,
            telemetry,
        }
    }

    pub fn submit(&self, cmd: TesseractCommand) -> CommandResponse {
        let (tx, rx) = unbounded();
        if self.input_tx.send((cmd, tx)).is_err() {
            return Err("scheduler input channel closed".into());
        }
        rx.recv().unwrap_or(Err("scheduler response channel closed".into()))
    }

    pub fn submit_async(
        &self,
        cmd: TesseractCommand,
    ) -> Receiver<CommandResponse> {
        let (tx, rx) = unbounded();
        let _ = self.input_tx.send((cmd, tx));
        rx
    }

    pub fn worker_count(&self) -> usize {
        self.workers.read().count()
    }
}
