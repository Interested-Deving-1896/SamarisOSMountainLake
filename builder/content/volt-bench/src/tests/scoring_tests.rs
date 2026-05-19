use crate::scorer::{badge_for_score, Scorer, CategoryScores, WEIGHTS};

#[test]
fn test_badge_legendary() {
    assert_eq!(badge_for_score(9500.0), "Legendary");
}

#[test]
fn test_badge_exceptional() {
    assert_eq!(badge_for_score(9200.0), "Exceptional");
}

#[test]
fn test_badge_critical() {
    assert_eq!(badge_for_score(5000.0), "Critical Optimization Needed");
}

#[test]
fn test_normalize_lower_better() {
    let s = Scorer::normalize_lower_is_better(10.0, 0.0, 100.0);
    assert!((s - 90.0).abs() < 0.001);
}

#[test]
fn test_normalize_higher_better() {
    let s = Scorer::normalize_higher_is_better(80.0, 0.0, 100.0);
    assert!((s - 80.0).abs() < 0.001);
}

#[test]
fn test_normalize_clamp_low() {
    let s = Scorer::normalize_lower_is_better(-50.0, 0.0, 100.0);
    assert!((s - 100.0).abs() < 0.001);
}

#[test]
fn test_normalize_clamp_high() {
    let s = Scorer::normalize_lower_is_better(200.0, 0.0, 100.0);
    assert!(s.abs() < 0.001);
}

#[test]
fn test_score_formula() {
    let mut cats = CategoryScores::new();
    cats.system = 100.0;
    cats.ui = 100.0;
    cats.memory = 100.0;
    cats.kernel = 100.0;
    cats.graphics = 100.0;
    cats.ai = 100.0;
    cats.browser = 100.0;
    cats.filesystem = 100.0;
    cats.stability = 100.0;

    let normalized = WEIGHTS.iter()
        .map(|(extract, weight)| extract(&cats) * weight)
        .sum::<f64>();
    let samaris = (normalized * 100.0).round();
    assert_eq!(samaris, 10000.0);
}

#[test]
fn test_score_zero_handling() {
    let normalized = WEIGHTS.iter()
        .map(|(_extract, weight)| 0.0 * weight)
        .sum::<f64>();
    let samaris = (normalized * 100.0).round();
    assert_eq!(samaris, 0.0);
}
