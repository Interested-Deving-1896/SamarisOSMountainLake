use volt_dynamic_worker_pool::prelude::*;

struct DemoAdapter;

impl WorkerPoolAdapter for DemoAdapter {
    fn module_id(&self) -> ModuleId {
        ModuleId::new("demo")
    }

    fn profile(&self) -> ModuleProfile {
        ModuleProfile::new(ModuleId::new("demo"), JobPriority::Normal)
            .with_resource_fraction(0.5)
            .with_can_burst()
    }

    fn submit_default_jobs(&self, pool: &DynamicWorkerPool) -> WorkerPoolResult<Vec<Job>> {
        let mut jobs = Vec::new();
        for i in 0..3 {
            let job = Job::new(JobId::new(), format!("demo-job-{}", i), JobPriority::Normal, 2048);
            pool.submit_job(job.clone())?;
            jobs.push(job);
        }
        Ok(jobs)
    }

    fn on_pressure_update(&self, metrics: &MetricsSnapshot) -> WorkerPoolResult<()> {
        println!("pressure update: desktop={:.2} queue={}",
            metrics.desktop_pressure, metrics.queue_depth);
        Ok(())
    }
}

fn main() {
    let pool = DynamicWorkerPool::new(WorkerPoolConfig::default());
    pool.start().expect("start failed");

    let adapter = DemoAdapter;
    pool.register_module(adapter.profile()).expect("register failed");

    let jobs = adapter.submit_default_jobs(&pool).expect("submit failed");
    println!("submitted {} adapter jobs", jobs.len());

    adapter.on_pressure_update(&pool.metrics()).expect("pressure update failed");

    pool.shutdown().expect("shutdown failed");
}
