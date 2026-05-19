use std::fs;

use volt_usb_manager::journal::journal::{Journal, JournalConfig};
use volt_usb_manager::journal::recovery::RecoveryEngine;

fn main() {
    let base = std::env::temp_dir().join("volt_recovery_example");
    fs::create_dir_all(&base).ok();

    let journal_path = base.join("journal");
    fs::create_dir_all(&journal_path).ok();
    let backing_path = base.join("backing");
    fs::create_dir_all(&backing_path).ok();

    let jconfig = JournalConfig {
        path: journal_path.to_str().unwrap().to_string(),
        fsync_on_record: true,
        checkpoint_interval_ms: 5000,
    };

    let journal = Journal::open(jconfig.clone()).expect("Failed to open journal");
    println!("Journal opened. Dirty: {}", journal.is_dirty());

    let writes: Vec<(&str, Vec<u8>)> = vec![
        ("important.txt", b"critical data: hello world".to_vec()),
        ("config.ini", b"[settings]\nkey=value\n".to_vec()),
        ("notes.md", b"# Recovery Simulation\n\nThis file survived journal recovery.".to_vec()),
    ];

    for (name, content) in &writes {
        let id = journal
            .begin_write(name, content.clone())
            .expect("Failed to begin write");
        journal
            .commit_write(id)
            .expect("Failed to commit write");
        println!("  Journaled: {} (record_id={})", name, id);
    }
    journal.clean_shutdown().expect("Clean shutdown failed");
    println!("Journal cleanly shut down. Dirty: {}", journal.is_dirty());

    let jconfig2 = JournalConfig {
        path: journal_path.to_str().unwrap().to_string(),
        fsync_on_record: false,
        checkpoint_interval_ms: 5000,
    };
    let journal2 = Journal::open(jconfig2).expect("Failed to reopen journal");
    println!("Reopened journal. Dirty: {}", journal2.is_dirty());

    if journal2.is_dirty() {
        println!("Journal is dirty — recovery required!");
        let result = RecoveryEngine::run(
            journal_path.to_str().unwrap(),
            backing_path.to_str().unwrap(),
        );
        match result {
            Ok(replay) => {
                println!(
                    "Recovery complete: {} records replayed, {} writes applied, {} rolled back",
                    replay.records_replayed, replay.writes_applied, replay.writes_rolled_back
                );
                for (name, content) in &writes {
                    let path = backing_path.join(name);
                    if path.exists() {
                        let recovered = fs::read(&path).expect("Failed to read recovered file");
                        assert_eq!(&recovered, content);
                        println!("  Verified: {} ({} bytes)", name, recovered.len());
                    }
                }
            }
            Err(e) => {
                eprintln!("Recovery failed: {}", e);
                eprintln!("Falling back to read-only mode.");
            }
        }
    } else {
        println!("Journal is clean — no recovery needed.");
    }

    println!("Recovery simulation complete.");
}
