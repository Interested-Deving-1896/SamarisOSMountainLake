use std::fs;
use std::path::Path;

use volt_usb_manager::writeback::ack::WriteAckKind;
use volt_usb_manager::writeback::write_buffer::WriteBuffer;

fn main() {
    let base = std::env::temp_dir().join("volt_write_flush_example");
    fs::create_dir_all(&base).ok();
    let backing = base.join("backing");
    fs::create_dir_all(&backing).ok();

    let mut buf = WriteBuffer::new(64);

    let files = vec!["readme.txt", "config.toml", "data.bin"];
    let contents: Vec<Vec<u8>> = files
        .iter()
        .enumerate()
        .map(|(i, _)| vec![i as u8; 8192])
        .collect();

    for (i, file) in files.iter().enumerate() {
        let path = backing.join(file);
        let pending = buf
            .enqueue(path.to_str().unwrap(), 0, contents[i].clone(), 0, 0)
            .expect("Failed to enqueue write");
        println!("Enqueued: {} (id={}, {} bytes)", file, pending.write_id, pending.size());
    }

    println!("Pending writes: {}", buf.pending_count());
    println!("Dirty bytes: {}", buf.dirty_bytes());
    println!("Usage: {:.1}%", buf.usage_pct());

    let batch = buf.flush_batch(512);
    println!("Flushed {} entries", batch.len());

    for pw in &batch {
        let path = Path::new(&pw.path);
        fs::write(path, &pw.data).expect("Failed to write to backing store");
        println!("  Wrote {} ({} bytes) to backing store", pw.path, pw.size());
        buf.acknowledge(pw.write_id, WriteAckKind::Durable);
    }

    for file in &files {
        let path = backing.join(file);
        if path.exists() {
            let meta = fs::metadata(&path).unwrap();
            println!("Verified: {} ({} bytes)", file, meta.len());
        }
    }

    println!("Pending after flush: {}", buf.pending_count());
    println!("Dirty bytes after flush: {}", buf.dirty_bytes());
    println!("Write-and-flush example complete.");
}
