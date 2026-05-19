use volt_dynamic_worker_pool::prelude::*;

fn main() {
    let pool = DynamicWorkerPool::new(WorkerPoolConfig::default());
    pool.start().expect("start failed");

    pool.submit_job(Job::new(JobId::new(), "snap-a".into(), JobPriority::Normal, 4096))
        .expect("submit failed");
    pool.submit_job(Job::new(JobId::new(), "snap-b".into(), JobPriority::High, 8192))
        .expect("submit failed");

    pool.set_desktop_pressure(0.3);

    let s = pool.metrics();
    println!("=== MetricsSnapshot ===");
    println!("  submitted        = {}", s.total_jobs_submitted);
    println!("  active_workers   = {}", s.active_workers);
    println!("  idle_workers     = {}", s.idle_workers);
    println!("  queue_depth      = {}", s.queue_depth);
    println!("  desktop_pressure = {:.2}", s.desktop_pressure);
    println!("  uptime_ms        = {}", s.uptime_ms);
    println!("  state            = {}", s.worker_pool_state);

    pool.shutdown().expect("shutdown failed");
}
