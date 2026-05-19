use volt_dynamic_worker_pool::orbit::burst_controller::{OrbitBurstDecision, OrbitBurstRequest};
use volt_dynamic_worker_pool::prelude::*;

fn main() {
    let pool = DynamicWorkerPool::new(WorkerPoolConfig::default());
    pool.start().expect("start failed");

    let req = OrbitBurstRequest::new("burst-1".into(), 3, 500, "demo".into());
    match pool.request_orbit_burst(req) {
        OrbitBurstDecision::Accepted => println!("burst accepted"),
        other => println!("burst rejected: {}", other.reason()),
    }

    let req2 = OrbitBurstRequest::new("burst-2".into(), 7, 1000, "second".into());
    match pool.request_orbit_burst(req2) {
        OrbitBurstDecision::Accepted => println!("second burst accepted"),
        other => println!("second burst rejected: {}", other.reason()),
    }

    pool.shutdown().expect("shutdown failed");
}
