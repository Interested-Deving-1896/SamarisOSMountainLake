use volt_dynamic_worker_pool::prelude::*;

fn main() {
    let pool = DynamicWorkerPool::new(WorkerPoolConfig::default());
    pool.start().expect("start failed");

    let before = pool.metrics();
    println!("before: active={} idle={}", before.active_workers, before.idle_workers);

    for i in 0..25 {
        pool.submit_job(Job::new(JobId::new(), format!("load-{}", i), JobPriority::Normal, 1024))
            .expect("submit failed");
    }

    let mid = pool.metrics();
    println!("after 25 jobs: active={} idle={} queue={}",
        mid.active_workers, mid.idle_workers, mid.queue_depth);

    pool.shutdown().expect("shutdown failed");
}
