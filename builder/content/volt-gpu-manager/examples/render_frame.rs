use volt_gpu_manager::scheduler::{GpuScheduler, GpuPriority, GpuCommand, GpuCommandKind};

fn main() {
    let sched = GpuScheduler::new(16);

    println!("=== Render Frame Simulation ===");

    sched.begin_frame();
    println!("Frame started.");

    sched.submit(GpuCommand::new(GpuCommandKind::Render, GpuPriority::Critical, "desktop_frame"));
    sched.submit(GpuCommand::new(GpuCommandKind::Compute, GpuPriority::High, "ui_composite"));
    sched.submit(GpuCommand::new(GpuCommandKind::Transfer, GpuPriority::Normal, "texture_upload"));
    sched.submit(GpuCommand::new(GpuCommandKind::Compress, GpuPriority::Idle, "bg_compress"));

    println!("Submitted 4 commands.");
    println!("Queued: {}", sched.queued_count());

    if let Some(batch) = sched.dequeue() {
        println!("Dequeued {} commands:", batch.len());
        for cmd in &batch {
            println!("  [{}] {:?} priority={:?}", cmd.id, cmd.kind, cmd.priority);
        }
    }

    sched.end_frame(12);
    println!("Frame completed in 12ms (within budget).");
    println!("Frame pressure: {}", sched.is_frame_pressure());
}
