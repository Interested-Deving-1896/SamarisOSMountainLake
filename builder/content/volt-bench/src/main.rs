use clap::Parser;
use tracing_subscriber::EnvFilter;
use volt_bench::{
    BenchCli, Storage, reporter::Reporter, runner::BenchRun,
    export::OptimizerInput,
    errors::BenchError,
};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("volt_bench=info".parse().unwrap()))
        .init();

    let cli = BenchCli::parse();

    let storage = Storage::new(None);
    let mode = cli.mode();

    let result: Result<(), BenchError> = match mode {
        "quick" | "full" | "stress" | "watch" | "ci" => {
            let mut run = BenchRun::new(mode);
            match run.run() {
                Ok(run_result) => {
                    println!("{}", Reporter::format_console(&run_result));
                    Reporter::write_result(&run_result, &storage)
                }
                Err(e) => Err(e),
            }
        }
        "none" => {
            if cli.latest {
                latest(&storage)
            } else if cli.history {
                history(&storage)
            } else if let Some(path) = &cli.import_baseline {
                import_baseline(path, &storage)
            } else if cli.compare {
                compare(&storage)
            } else if cli.optimizer_export {
                optimizer_export(&storage)
            } else if let Some(format) = &cli.export {
                export_result(format, &storage)
            } else if cli.score {
                rescore(&storage)
            } else if cli.dump {
                dump(&storage)
            } else {
                eprintln!("Usage: bench --run (or --quick, --full, --stress, --watch, --ci)");
                eprintln!("       bench --latest | --history | --compare | --score | --dump");
                eprintln!("       bench --export json | csv");
                eprintln!("       bench --import-baseline <file>");
                eprintln!("       bench --optimizer-export");
                Ok(())
            }
        }
        _ => {
            eprintln!("Unknown mode: {}", mode);
            std::process::exit(1);
        }
    };

    match result {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn latest(storage: &Storage) -> Result<(), BenchError> {
    let data = storage.read_latest()?;
    println!("{}", data);
    Ok(())
}

fn history(storage: &Storage) -> Result<(), BenchError> {
    let data = storage.read_history()?;
    let parsed: serde_json::Value = serde_json::from_str(&data)?;
    if let Some(entries) = parsed["entries"].as_array() {
        for entry in entries {
            println!("{} | {:<12} | {:>5}/10000 | {}",
                entry["timestamp"].as_str().unwrap_or("?"),
                entry["mode"].as_str().unwrap_or("?"),
                entry["score"].as_i64().unwrap_or(0),
                entry["badge"].as_str().unwrap_or("?"),
            );
        }
    }
    Ok(())
}

fn import_baseline(path: &str, storage: &Storage) -> Result<(), BenchError> {
    let data = std::fs::read_to_string(path)
        .map_err(|e| BenchError::BaselineImportError(format!("Cannot read {}: {}", path, e)))?;
    let parsed: serde_json::Value = serde_json::from_str(&data)
        .map_err(|e| BenchError::BaselineImportError(format!("Invalid JSON in {}: {}", path, e)))?;
    let name = std::path::Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("imported");
    storage.import_baseline(name, &data)?;
    println!("Imported baseline '{}' from {}", name, path);
    let label = parsed["import_label"].as_str().unwrap_or("(no label)");
    let score = parsed["overall"]["score"].as_f64().unwrap_or(0.0);
    println!("  Label: {} | Score: {}/10000", label, score);
    let hw_class = parsed["hardware"]["class"].as_str().unwrap_or("?");
    println!("  Hardware: {} | Validity: reference_only (imported, may differ from current hardware)", hw_class);
    Ok(())
}

fn compare(_storage: &Storage) -> Result<(), BenchError> {
    println!("Comparison mode: see --import-baseline first, then --compare shows deltas.");
    println!("No comparison loaded. Run: bench --import-baseline <file>");
    Ok(())
}

fn optimizer_export(storage: &Storage) -> Result<(), BenchError> {
    let latest_data = storage.read_latest()?;
    let parsed: serde_json::Value = serde_json::from_str(&latest_data)?;
    let score = parsed["overall"]["score"].as_f64().unwrap_or(0.0);
    let run_id = parsed["run"]["run_id"].as_str().unwrap_or("unknown");
    let hw_class = parsed["hardware"]["class"].as_str().unwrap_or("unknown");
    let opt = OptimizerInput::from_scores(score, vec![], run_id.to_string(), hw_class.to_string());
    let json = serde_json::to_string_pretty(&opt)?;
    println!("{}", json);
    storage.write_optimizer_input(&json)?;
    Ok(())
}

fn export_result(format: &str, storage: &Storage) -> Result<(), BenchError> {
    let latest_data = storage.read_latest()?;
    let parsed: serde_json::Value = serde_json::from_str(&latest_data)?;
    match format {
        "json" => println!("{}", latest_data),
        "csv" => {
            // Build a minimal csv from the JSON
            println!("category,metric,value");
            if let Some(cats) = parsed["category_scores"].as_object() {
                for (name, score) in cats {
                    if let Some(s) = score.as_f64() {
                        println!("category,{},{}", name, s);
                    }
                }
            }
            if let Some(score) = parsed["overall"]["score"].as_f64() {
                println!("overall,score,{}", score);
            }
        }
        _ => eprintln!("Unsupported export format: {}. Use json or csv.", format),
    }
    Ok(())
}

fn rescore(_storage: &Storage) -> Result<(), BenchError> {
    println!("Re-scoring: reading latest.json and recomputing...");
    // For now, just re-print latest
    latest(_storage)
}

fn dump(_storage: &Storage) -> Result<(), BenchError> {
    println!("Bench dump");
    println!("Storage path: /var/lib/samaris/bench");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
