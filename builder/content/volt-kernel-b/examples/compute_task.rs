use tesseract_engine::compute_bridge::task::{ComputeKind, ComputeTask, execute_compute};
use tesseract_engine::compute_bridge::ComputeBridge;

fn main() {
    // Standalone compute execution
    let task = ComputeTask::new(0x01, ComputeKind::HashSha256, b"Hello, Tesseract Engine!".to_vec());
    match execute_compute(&task) {
        Ok(output) => {
            println!("SHA-256 hash (simplified): {} bytes", output.len());
            for byte in output.iter().take(8) {
                print!("{byte:02X}");
            }
            println!("...");
        }
        Err(e) => {
            eprintln!("Compute failed: {e}");
        }
    }

    // Compression demo
    let data = b"AAAABBBBCCCCDDDDAAAAEEEEFFFF".to_vec();
    let compress_task = ComputeTask::new(0x01, ComputeKind::Compress, data.clone());
    match execute_compute(&compress_task) {
        Ok(compressed) => {
            println!("RLE compressed {} -> {} bytes", data.len(), compressed.len());
            // Decompress
            let decompress_task = ComputeTask::new(0x01, ComputeKind::Decompress, compressed);
            if let Ok(decompressed) = execute_compute(&decompress_task) {
                println!("Decompressed: {} bytes", decompressed.len());
                assert_eq!(decompressed, data);
                println!("Round-trip verified!");
            }
        }
        Err(e) => {
            eprintln!("Compression failed: {e}");
        }
    }

    // Using ComputeBridge
    let mut bridge = ComputeBridge::new();
    let task = ComputeTask::new(0x01, ComputeKind::ImageBlur, vec![0u8; 1024]);
    match bridge.execute_task(&task) {
        Ok(result) => {
            println!("Compute task {} completed in {}µs", result.task_id, result.elapsed_us);
        }
        Err(e) => {
            eprintln!("Bridge compute failed: {e}");
        }
    }
}
