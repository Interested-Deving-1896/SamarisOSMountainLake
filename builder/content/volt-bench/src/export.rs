use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizerInput {
    pub fitness_score: f64,
    pub bottlenecks: Vec<String>,
    pub recommendations: Vec<Recommendation>,
    pub unstable_metrics: Vec<String>,
    pub regression_alerts: Vec<RegressionAlert>,
    pub run_id: String,
    pub hardware_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub area: String,
    pub action: String,
    pub priority: String,
    pub expected_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionAlert {
    pub metric: String,
    pub previous_score: f64,
    pub current_score: f64,
    pub delta: f64,
}

impl OptimizerInput {
    pub fn from_scores(
        score: f64,
        bottlenecks: Vec<String>,
        run_id: String,
        hardware_class: String,
    ) -> Self {
        Self {
            fitness_score: score,
            bottlenecks,
            recommendations: vec![],
            unstable_metrics: vec![],
            regression_alerts: vec![],
            run_id,
            hardware_class,
        }
    }
}

pub fn export_to_csv(result: &crate::runner::RunResult) -> String {
    let mut csv = String::from("category,metric,value\n");
    csv.push_str(&format!("overall,score,{}\n", result.scores.overall.score));
    csv.push_str(&format!("overall,normalized,{:.2}\n", result.scores.overall.normalized_score));
    csv.push_str(&format!("overall,badge,{}\n", result.scores.badge));
    csv.push_str(&format!("run,duration_seconds,{:.1}\n", result.duration_seconds));
    csv.push_str(&format!("run,iterations,{}\n", result.iterations));
    csv.push_str(&format!("run,stddev,{:.2}\n", result.stddev_score));
    csv.push_str(&format!("hardware,class,{}\n", result.hardware.class));
    csv.push_str(&format!("hardware,cpu,{}\n", result.hardware.cpu));
    csv.push_str(&format!("hardware,ram_gb,{:.1}\n", result.hardware.ram_gb));

    let categories = [
        ("system", result.scores.category_scores.system),
        ("ui", result.scores.category_scores.ui),
        ("memory", result.scores.category_scores.memory),
        ("kernel", result.scores.category_scores.kernel),
        ("graphics", result.scores.category_scores.graphics),
        ("ai", result.scores.category_scores.ai),
        ("browser", result.scores.category_scores.browser),
        ("filesystem", result.scores.category_scores.filesystem),
        ("stability", result.scores.category_scores.stability),
    ];
    for (name, score) in &categories {
        csv.push_str(&format!("category,{},{:.1}\n", name, score));
    }
    csv
}
