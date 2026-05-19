use std::sync::Arc;

use tesseract_engine::protocol::header::SbpHeader;
use tesseract_engine::protocol::opcodes::Opcode;
use tesseract_engine::protocol::TesseractCommand;
use tesseract_engine::telemetry::Telemetry;
use tesseract_engine::scheduler::Scheduler;

fn main() {
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(4, 1, telemetry.clone()));

    // Send a Heartbeat command
    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 0, 0x01, 0),
        vec![],
    );

    match scheduler.submit(cmd) {
        Ok(payload) => {
            println!("Heartbeat response: {:?}", payload);
        }
        Err(e) => {
            eprintln!("Heartbeat failed: {e}");
        }
    }

    // Send a QUERY_CORES command
    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::QueryCores, 0, 0x01, 0),
        vec![],
    );

    match scheduler.submit(cmd) {
        Ok(payload) => {
            println!("Query cores response: {:?}", payload);
        }
        Err(e) => {
            eprintln!("Query cores failed: {e}");
        }
    }

    let snapshot = telemetry.snapshot();
    println!(
        "Processed {} commands, {} errors, {}/sec",
        snapshot.commands_processed,
        snapshot.errors_count,
        snapshot.commands_per_second,
    );
}
