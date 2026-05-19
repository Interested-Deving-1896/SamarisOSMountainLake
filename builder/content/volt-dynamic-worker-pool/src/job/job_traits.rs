use crate::job::job_context::JobContext;
use crate::job::job_result::JobResult;
use crate::priority::level::PriorityLevel;

pub type JobPriority = PriorityLevel;

pub trait JobFn: Send + 'static {
    fn execute(&self, ctx: &mut JobContext) -> JobResult<()>;
    fn name(&self) -> &str;
}

impl<F> JobFn for F
where
    F: Fn(&mut JobContext) -> JobResult<()> + Send + 'static,
{
    fn execute(&self, ctx: &mut JobContext) -> JobResult<()> {
        self(ctx)
    }

    fn name(&self) -> &str {
        "closure"
    }
}
