use volt_dynamic_worker_pool::prelude::*;

fn main() {
    let pool = DynamicWorkerPool::new(WorkerPoolConfig::default());
    pool.start().expect("start failed");

    pool.submit_job(Job::new(JobId::new(), "alpha".into(), JobPriority::Normal, 4096))
        .expect("submit failed");
    pool.submit_job(Job::new(JobId::new(), "beta".into(), JobPriority::High, 8192))
        .expect("submit failed");
    pool.submit_job(Job::new(JobId::new(), "gamma".into(), JobPriority::Low, 2048))
        .expect("submit failed");

    let m = pool.metrics();
    println!("submitted={} queue={} active={} idle={} state={}",
        m.total_jobs_submitted, m.queue_depth, m.active_workers, m.idle_workers, m.worker_pool_state);

    pool.shutdown().expect("shutdown failed");
}
