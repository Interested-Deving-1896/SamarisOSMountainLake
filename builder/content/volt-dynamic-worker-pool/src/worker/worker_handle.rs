use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;

use crossbeam::channel::Receiver;

use crate::job::job::Job;
use crate::worker::worker_id::WorkerId;
use crate::worker::worker_loop::WorkerLoop;
use crate::worker::worker_state::WorkerState;

pub struct WorkerHandle {
    pub id: WorkerId,
    pub state: Arc<AtomicU8>,
    pub cancel: Arc<AtomicBool>,
    pub join_handle: Option<thread::JoinHandle<()>>,
}

impl WorkerHandle {
    pub fn new(id: WorkerId) -> Self {
        Self {
            id,
            state: Arc::new(AtomicU8::new(WorkerState::Idle.to_u8())),
            cancel: Arc::new(AtomicBool::new(false)),
            join_handle: None,
        }
    }

    pub fn spawn(id: WorkerId, job_rx: Receiver<Job>) -> Self {
        let handle = Self::new(id);
        let cancel = handle.cancel.clone();
        let state = handle.state.clone();
        let wid = id;

        let join = thread::Builder::new()
            .name(format!("worker-{}", id))
            .spawn(move || {
                WorkerLoop::run(wid, job_rx, cancel, state);
            })
            .expect("failed to spawn worker thread");

        let mut result = Self::new(id);
        result.state = handle.state.clone();
        result.cancel = handle.cancel.clone();
        result.join_handle = Some(join);
        result
    }

    pub fn cancel(&self) {
        self.cancel.store(true, Ordering::SeqCst);
    }

    pub fn join(mut self) -> thread::Result<()> {
        if let Some(handle) = self.join_handle.take() {
            handle.join()
        } else {
            Ok(())
        }
    }

    pub fn is_finished(&self) -> bool {
        self.state.load(Ordering::SeqCst) == WorkerState::Stopped.to_u8()
    }

    pub fn current_state(&self) -> WorkerState {
        WorkerState::from_u8(self.state.load(Ordering::SeqCst)).unwrap_or(WorkerState::Error)
    }

    pub fn set_state(&self, new_state: WorkerState) {
        self.state.store(new_state.to_u8(), Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel.load(Ordering::SeqCst)
    }

    pub fn is_busy(&self) -> bool {
        self.current_state() == WorkerState::Busy
    }

    pub fn is_idle(&self) -> bool {
        self.current_state() == WorkerState::Idle
    }

    pub fn try_cancel_and_join(self) -> thread::Result<()> {
        self.cancel.store(true, Ordering::SeqCst);
        self.join()
    }
}

impl std::fmt::Debug for WorkerHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkerHandle")
            .field("id", &self.id)
            .field("state", &self.current_state().name())
            .field("cancelled", &self.is_cancelled())
            .field("finished", &self.is_finished())
            .finish()
    }
}

impl Drop for WorkerHandle {
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::SeqCst);
        if let Some(handle) = self.join_handle.take() {
            let _ = handle.join();
        }
    }
}
