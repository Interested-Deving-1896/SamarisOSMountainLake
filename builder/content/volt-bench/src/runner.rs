use std::time::Instant;
use tracing::{info, warn};
use crate::collectors::{CollectorRegistry, CollectorResult};
use crate::scorer::{ScoreResult, Scorer};
use crate::errors::BenchError;
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;
use crate::export::OptimizerInput;

pub struct BenchRun {
    pub run_id: String,
    pub mode: String,
    pub collectors: CollectorRegistry,
    pub scorer: Scorer,
    pub hardware: HardwareInfo,
    pub environment: EnvironmentInfo,
    pub iterations: u32,
    pub warmup_iterations: u32,
}

pub struct RunResult {
    pub run_id: String,
    pub timestamp: String,
    pub mode: String,
    pub duration_seconds: f64,
    pub iterations: u32,
    pub warmup_iterations: u32,
    pub cold_run: bool,
    pub median_score: f64,
    pub mean_score: f64,
    pub min_score: f64,
    pub max_score: f64,
    pub stddev_score: f64,
    pub confidence: String,
    pub reliability_flags: Vec<String>,
    pub hardware: HardwareInfo,
    pub environment: EnvironmentInfo,
    pub scores: ScoreResult,
    pub optimizer: OptimizerInput,
}

impl BenchRun {
    pub fn new(mode: &str) -> Self {
        let iterations = match mode {
            "full" => 5,
            "quick" => 3,
            "stress" => 3,
            "ci" => 3,
            _ => 3,
        };
        let warmup = if mode == "full" || mode == "stress" { 2 } else { 1 };

        Self {
            run_id: format!("bench-{}", chrono::Utc::now().format("%Y-%m-%d-%H%M%S")),
            mode: mode.to_string(),
            collectors: CollectorRegistry::new(),
            scorer: Scorer::new(),
            hardware: HardwareInfo::detect(),
            environment: EnvironmentInfo::detect(),
            iterations,
            warmup_iterations: warmup,
        }
    }

    pub fn run(&mut self) -> Result<RunResult, BenchError> {
        let start = Instant::now();
        let mut all_scores: Vec<f64> = Vec::new();
        let mut all_reliability = Vec::new();
        let mut all_metrics = serde_json::Value::Null;

        // Warmup
        for w in 0..self.warmup_iterations {
            info!("Warmup iteration {}/{}", w + 1, self.warmup_iterations);
            self.collectors.collect_all(&self.hardware, &self.environment)?;
        }

        // Measurement iterations
        for i in 0..self.iterations {
            info!("Measurement iteration {}/{}", i + 1, self.iterations);
            let iter_metrics = self.collectors.collect_all(&self.hardware, &self.environment)?;
            let iter_score = self.scorer.compute(&iter_metrics);
            all_scores.push(iter_score.overall.score);
            all_reliability.extend(self.collectors.reliability_flags());
            all_metrics = serde_json::to_value(&iter_metrics).unwrap_or(serde_json::Value::Null);
        }

        let duration = start.elapsed().as_secs_f64();
        let n = all_scores.len() as f64;
        let mean = all_scores.iter().sum::<f64>() / n;
        let variance = all_scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / n;
        let stddev = variance.sqrt();
        let mut sorted = all_scores.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = if sorted.len() % 2 == 0 {
            (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
        } else {
            sorted[sorted.len() / 2]
        };

        let confidence = if self.iterations >= 3 && stddev / mean.max(1.0) < 0.03 && all_reliability.is_empty() {
            "high".to_string()
        } else if self.iterations >= 2 && stddev / mean.max(1.0) < 0.08 {
            "medium".to_string()
        } else {
            "low".to_string()
        };

        let scores = self.scorer.compute(&serde_json::Value::Null);

        Ok(RunResult {
            run_id: self.run_id.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            mode: self.mode.clone(),
            duration_seconds: duration,
            iterations: self.iterations,
            warmup_iterations: self.warmup_iterations,
            cold_run: self.warmup_iterations == 0,
            median_score: median,
            mean_score: mean,
            min_score: all_scores.iter().cloned().fold(f64::MAX, f64::min),
            max_score: all_scores.iter().cloned().fold(f64::MIN, f64::max),
            stddev_score: stddev,
            confidence,
            reliability_flags: all_reliability,
            hardware: self.hardware.clone(),
            environment: self.environment.clone(),
            scores,
            optimizer: OptimizerInput::default(),
        })
    }
}
