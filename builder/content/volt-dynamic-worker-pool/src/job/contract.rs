use crate::core::result::WorkerPoolResult;
use crate::job::job_context::JobContext;

pub trait JobContract: Send + 'static {
    fn execute(&self, ctx: &mut JobContext) -> WorkerPoolResult<()>;

    fn on_cancel(&self, _ctx: &JobContext) {}

    fn on_complete(&self, _ctx: &JobContext) {}

    fn name(&self) -> &str;
}
