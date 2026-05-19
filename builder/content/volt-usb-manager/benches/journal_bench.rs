use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tempfile::tempdir;
use volt_usb_manager::journal::journal::{Journal, JournalConfig};
use volt_usb_manager::journal::record::JournalRecord;
use volt_usb_manager::journal::record_type::RecordType;
use volt_usb_manager::journal::wal::WalWriter;

fn bench_append_records(c: &mut Criterion) {
    c.bench_function("journal_append_records", |b| {
        b.iter(|| {
            let dir = tempdir().unwrap();
            let config = JournalConfig {
                path: dir.path().to_str().unwrap().to_string(),
                fsync_on_record: false,
                checkpoint_interval_ms: 50000,
            };
            let journal = Journal::open(config).unwrap();
            for i in 0..100 {
                let id = journal
                    .begin_write(black_box(&format!("/bench/{}", i)), vec![i as u8; 128])
                    .unwrap();
                journal.commit_write(id).unwrap();
            }
        });
    });
}

fn bench_replay_records(c: &mut Criterion) {
    c.bench_function("journal_replay_records", |b| {
        b.iter(|| {
            let dir = tempdir().unwrap();
            let wal = dir.path().join("replay.wal").to_str().unwrap().to_string();
            let backing = dir.path().join("backing").to_str().unwrap().to_string();
            std::fs::create_dir_all(&backing).unwrap();
            let writer = WalWriter::open(&wal).unwrap();
            for i in 0..200 {
                let rec = JournalRecord::new(
                    RecordType::BeginWrite,
                    i,
                    &format!("/replay/{}", i),
                    vec![i as u8; 64],
                );
                writer.append(&rec).unwrap();
                let commit =
                    JournalRecord::new(RecordType::CommitWrite, i, "", vec![]);
                writer.append(&commit).unwrap();
            }
            writer.fsync().unwrap();
            drop(writer);

            let result =
                volt_usb_manager::journal::recovery::RecoveryEngine::run(&wal, &backing)
                    .unwrap();
            black_box(result);
        });
    });
}

fn bench_wal_write_throughput(c: &mut Criterion) {
    c.bench_function("wal_write_throughput", |b| {
        b.iter(|| {
            let dir = tempdir().unwrap();
            let wal = dir.path().join("throughput.wal").to_str().unwrap().to_string();
            let writer = WalWriter::open(&wal).unwrap();
            for i in 0..500 {
                let rec = JournalRecord::new(
                    RecordType::BeginWrite,
                    i,
                    "/throughput",
                    vec![i as u8; 32],
                );
                writer.append(&rec).unwrap();
            }
            writer.fsync().unwrap();
            black_box(writer.record_count());
        });
    });
}

criterion_group!(benches, bench_append_records, bench_replay_records, bench_wal_write_throughput);
criterion_main!(benches);
