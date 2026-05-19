use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;

use crossbeam::channel::Receiver;

use crate::job::job::Job;
use crate::job::job_context::JobContext;
use crate::job::job_handle::JobHandle;
use crate::worker::worker_id::WorkerId;
use crate::worker::worker_state::WorkerState;

pub struct WorkerLoop;

impl WorkerLoop {
    pub fn run(
        worker_id: WorkerId,
        job_rx: Receiver<Job>,
        cancel: Arc<AtomicBool>,
        state: Arc<AtomicU8>,
    ) {
        state.store(WorkerState::Idle.to_u8(), Ordering::Relaxed);

        loop {
            if cancel.load(Ordering::Relaxed) {
                break;
            }

            match job_rx.recv() {
                Ok(job) => {
                    state.store(WorkerState::Busy.to_u8(), Ordering::Relaxed);

                    let handle = JobHandle::new(job.id().clone(), job.name().to_string());
                    let ctx = JobContext::new(job, handle, 0, worker_id.as_u32());

                    if !ctx.is_cancelled() {
                        ctx.mark_completed();
                    }

                    state.store(WorkerState::Idle.to_u8(), Ordering::Relaxed);
                }
                Err(_) => break,
            }
        }

        state.store(WorkerState::Stopped.to_u8(), Ordering::Relaxed);
    }
}
