use volt_gpu_manager::compute::compute_job::GpuComputeJob;
use volt_gpu_manager::compute::compute_job::GpuComputeJobKind;
use volt_gpu_manager::scheduler::GpuPriority;

fn main() {
    println!("=== Compute Job Submission ===");

    let jobs = vec![
        GpuComputeJob::new(GpuComputeJobKind::MatMul, GpuPriority::High, 1),
        GpuComputeJob::new(GpuComputeJobKind::Blur, GpuPriority::Normal, 1),
        GpuComputeJob::new(GpuComputeJobKind::VramCompress, GpuPriority::Idle, 1),
    ];

    println!("Created {} compute jobs:", jobs.len());
    for job in &jobs {
        println!(
            "  Job {}: kind={:?} priority={:?} app={}",
            job.job_id, job.kind, job.priority, job.app_id
        );
    }

    let first = &jobs[0];
    println!();
    println!("First job kind name: {}", first.kind.name());
    println!("First job priority:  {:?}", first.priority);
    println!("First job app_id:    {}", first.app_id);
    println!("Compute jobs ready for submission.");
}
