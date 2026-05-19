use serde::{Deserialize, Serialize};
use crate::runner::RunResult;
use crate::storage::Storage;
use crate::errors::BenchError;

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchOutput {
    pub version: String,
    pub timestamp: String,
    pub run: serde_json::Value,
    pub hardware: serde_json::Value,
    pub environment: serde_json::Value,
    pub os: serde_json::Value,
    pub overall: serde_json::Value,
    pub category_scores: serde_json::Value,
    pub metrics: serde_json::Value,
    pub comparison: serde_json::Value,
    pub optimizer: serde_json::Value,
}

pub struct Reporter;

impl Reporter {
    pub fn write_result(result: &RunResult, storage: &Storage) -> Result<(), BenchError> {
        let output = serde_json::json!({
            "version": "1.0",
            "timestamp": result.timestamp,
            "run": {
                "run_id": result.run_id,
                "mode": result.mode,
                "duration_seconds": result.duration_seconds,
                "iterations": result.iterations,
                "warmup_iterations": result.warmup_iterations,
                "cold_run": result.cold_run,
                "median_score": result.median_score,
                "mean_score": result.mean_score,
                "min_score": result.min_score,
                "max_score": result.max_score,
                "stddev_score": result.stddev_score,
                "confidence": result.confidence,
                "reliability_flags": result.reliability_flags,
            },
            "hardware": serde_json::to_value(&result.hardware).unwrap_or_default(),
            "environment": serde_json::to_value(&result.environment).unwrap_or_default(),
            "os": {
                "name": "Samaris OS",
                "version": env!("CARGO_PKG_VERSION"),
                "build": "",
                "commit_hash": null,
                "release_channel": "alpha",
                "boot_mode": "uefi",
            },
            "overall": {
                "score": result.scores.overall.score,
                "score_out_of": result.scores.overall.score_out_of,
                "normalized_score": result.scores.overall.normalized_score,
                "badge": result.scores.badge,
                "validity": "same_hardware_only",
            },
            "category_scores": serde_json::to_value(&result.scores.category_scores).unwrap_or_default(),
            "metrics": {},
            "comparison": {
                "baselines": [],
                "comparison_validity": "none",
            },
            "optimizer": serde_json::to_value(&result.optimizer).unwrap_or_default(),
        });

        let json_str = serde_json::to_string_pretty(&output)?;
        storage.write_latest(&json_str)?;
        storage.append_history(&serde_json::json!({
            "timestamp": result.timestamp,
            "run_id": result.run_id,
            "mode": result.mode,
            "score": result.scores.overall.score,
            "badge": result.scores.badge,
            "duration_seconds": result.duration_seconds,
            "hardware_class": result.hardware.class,
            "category_scores": serde_json::to_value(&result.scores.category_scores).unwrap_or_default(),
            "reliability_flags": result.reliability_flags,
        }).to_string())?;

        Ok(())
    }

    pub fn format_console(result: &RunResult) -> String {
        format!(
            "Samaris OS Bench\n\
             ─────────────────────────────\n\
             Run ID:      {}\n\
             Mode:        {}\n\
             Score:       {}/10000 ({})\n\
             Normalized:  {:.1}/100\n\
             Badge:       {}\n\
             Confidence:  {}\n\
             Iterations:  {}\n\
             StdDev:      {:.1}\n\
             Duration:    {:.1}s\n\
             Hardware:    {} — {}",
            result.run_id,
            result.mode,
            result.scores.overall.score,
            result.scores.badge,
            result.scores.overall.normalized_score,
            result.scores.badge,
            result.confidence,
            result.iterations,
            result.stddev_score,
            result.duration_seconds,
            result.hardware.class,
            result.hardware.cpu,
        )
    }
}
