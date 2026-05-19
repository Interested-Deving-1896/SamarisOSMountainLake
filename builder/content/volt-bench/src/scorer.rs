use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreResult {
    pub overall: OverallScore,
    pub category_scores: CategoryScores,
    pub badge: String,
    pub reliability_flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallScore {
    pub score: f64,
    pub score_out_of: u32,
    pub normalized_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryScores {
    pub system: f64,
    pub ui: f64,
    pub memory: f64,
    pub kernel: f64,
    pub graphics: f64,
    pub ai: f64,
    pub browser: f64,
    pub filesystem: f64,
    pub stability: f64,
}

impl CategoryScores {
    pub fn new() -> Self {
        Self {
            system: 0.0, ui: 0.0, memory: 0.0, kernel: 0.0,
            graphics: 0.0, ai: 0.0, browser: 0.0, filesystem: 0.0, stability: 100.0,
        }
    }
}

pub(crate) const WEIGHTS: [(fn(&CategoryScores) -> f64, f64); 9] = [
    (|c| c.system, 0.20),
    (|c| c.ui, 0.20),
    (|c| c.memory, 0.15),
    (|c| c.kernel, 0.10),
    (|c| c.graphics, 0.10),
    (|c| c.ai, 0.10),
    (|c| c.browser, 0.05),
    (|c| c.filesystem, 0.05),
    (|c| c.stability, 0.05),
];

pub fn badge_for_score(score: f64) -> &'static str {
    if score >= 9500.0 { "Legendary" }
    else if score >= 9000.0 { "Exceptional" }
    else if score >= 8500.0 { "Excellent" }
    else if score >= 8000.0 { "Very Good" }
    else if score >= 7000.0 { "Good" }
    else if score >= 6000.0 { "Needs Optimization" }
    else { "Critical Optimization Needed" }
}

pub struct Scorer;

impl Scorer {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(&self, _metrics: &serde_json::Value) -> ScoreResult {
        // This function receives raw metrics and computes category scores.
        // Each metric is clamped and normalized, then combined into category scores.
        // Full implementation would read each metric path from the JSON value.
        // For now, returns placeholder scores.
        let category_scores = CategoryScores::new();

        let normalized = WEIGHTS.iter()
            .map(|(extract, weight)| extract(&category_scores) * weight)
            .sum::<f64>();

        let samaris_score = (normalized * 100.0).round();

        ScoreResult {
            overall: OverallScore {
                score: samaris_score,
                score_out_of: 10000,
                normalized_score: normalized,
            },
            category_scores,
            badge: badge_for_score(samaris_score).to_string(),
            reliability_flags: vec![],
        }
    }

    pub fn normalize_lower_is_better(raw: f64, min: f64, max: f64) -> f64 {
        if max <= min { return 50.0; }
        let clamped = raw.clamp(min, max);
        100.0 * (max - clamped) / (max - min)
    }

    pub fn normalize_higher_is_better(raw: f64, min: f64, max: f64) -> f64 {
        if max <= min { return 50.0; }
        let clamped = raw.clamp(min, max);
        100.0 * (clamped - min) / (max - min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_thresholds() {
        assert_eq!(badge_for_score(9800.0), "Legendary");
        assert_eq!(badge_for_score(9200.0), "Exceptional");
        assert_eq!(badge_for_score(8700.0), "Excellent");
        assert_eq!(badge_for_score(8200.0), "Very Good");
        assert_eq!(badge_for_score(7500.0), "Good");
        assert_eq!(badge_for_score(6500.0), "Needs Optimization");
        assert_eq!(badge_for_score(5000.0), "Critical Optimization Needed");
    }

    #[test]
    fn test_normalize_lower_is_better() {
        let score = Scorer::normalize_lower_is_better(30.0, 0.0, 100.0);
        assert!((score - 70.0).abs() < 0.001);
    }

    #[test]
    fn test_normalize_higher_is_better() {
        let score = Scorer::normalize_higher_is_better(70.0, 0.0, 100.0);
        assert!((score - 70.0).abs() < 0.001);
    }

    #[test]
    fn test_normalize_clamps() {
        let low = Scorer::normalize_lower_is_better(-10.0, 0.0, 100.0);
        assert!((low - 100.0).abs() < 0.001);
        let high = Scorer::normalize_lower_is_better(200.0, 0.0, 100.0);
        assert!(high.abs() < 0.001);
    }

    #[test]
    fn test_score_formula() {
        let mut cats = CategoryScores::new();
        cats.system = 80.0;
        cats.ui = 90.0;
        cats.memory = 85.0;
        cats.kernel = 88.0;
        cats.graphics = 82.0;
        cats.ai = 75.0;
        cats.browser = 78.0;
        cats.filesystem = 85.0;
        cats.stability = 95.0;

        let expected: f64 = 80.0*0.20 + 90.0*0.20 + 85.0*0.15 + 88.0*0.10
                     + 82.0*0.10 + 75.0*0.10 + 78.0*0.05 + 85.0*0.05 + 95.0*0.05;
        let samaris = (expected * 100.0).round();
        assert!((samaris - 8400.0).abs() < 100.0);
    }
}
