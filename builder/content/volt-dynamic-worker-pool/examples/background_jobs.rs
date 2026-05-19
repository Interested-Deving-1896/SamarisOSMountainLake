use volt_dynamic_worker_pool::prelude::*;

fn main() {
    let pool = DynamicWorkerPool::new(WorkerPoolConfig::default());
    pool.start().expect("start failed");

    for i in 0..4 {
        pool.submit_job(Job::new(JobId::new(), format!("bg-{}", i), JobPriority::Low, 512))
            .expect("bg submit failed");
    }
    for i in 0..4 {
        pool.submit_job(Job::new(JobId::new(), format!("normal-{}", i), JobPriority::Normal, 4096))
            .expect("normal submit failed");
    }

    let m = pool.metrics();
    println!("submitted={} queue={} active={} idle={}",
        m.total_jobs_submitted, m.queue_depth, m.active_workers, m.idle_workers);

    pool.shutdown().expect("shutdown failed");
}
