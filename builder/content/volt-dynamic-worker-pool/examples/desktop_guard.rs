use volt_dynamic_worker_pool::prelude::*;

fn main() {
    let pool = DynamicWorkerPool::new(WorkerPoolConfig::default());
    pool.start().expect("start failed");

    pool.submit_job(Job::new(JobId::new(), "desktop-task".into(), JobPriority::High, 4096))
        .expect("submit failed");

    println!("before: desktop_pressure={:.2}", pool.metrics().desktop_pressure);

    pool.set_desktop_pressure(0.85);
    println!("after pressure=0.85: desktop_pressure={:.2}", pool.metrics().desktop_pressure);

    pool.set_desktop_pressure(0.0);
    println!("after reset: desktop_pressure={:.2}", pool.metrics().desktop_pressure);

    pool.shutdown().expect("shutdown failed");
}
