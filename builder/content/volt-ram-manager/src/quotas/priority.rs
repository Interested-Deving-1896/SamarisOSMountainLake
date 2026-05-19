use crate::apps::app_profile::AppPriority;

pub const PRIORITY_ORDER: [AppPriority; 5] = [
    AppPriority::Critical,
    AppPriority::High,
    AppPriority::Normal,
    AppPriority::Low,
    AppPriority::Idle,
];

pub fn priority_budget(priority: AppPriority) -> f64 {
    match priority {
        AppPriority::Critical => 0.50,
        AppPriority::High => 0.20,
        AppPriority::Normal => 0.15,
        AppPriority::Low => 0.10,
        AppPriority::Idle => 0.05,
    }
}

pub fn priority_weight(priority: AppPriority) -> u32 {
    match priority {
        AppPriority::Critical => 100,
        AppPriority::High => 75,
        AppPriority::Normal => 50,
        AppPriority::Low => 25,
        AppPriority::Idle => 10,
    }
}

pub fn priority_reclaim_order() -> impl Iterator<Item = AppPriority> {
    PRIORITY_ORDER.iter().rev().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_order() {
        assert_eq!(PRIORITY_ORDER[0], AppPriority::Critical);
        assert_eq!(PRIORITY_ORDER[4], AppPriority::Idle);
    }

    #[test]
    fn test_priority_budget_sums() {
        let total: f64 = PRIORITY_ORDER.iter().map(|p| priority_budget(*p)).sum();
        assert!((total - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_reclaim_order() {
        let reclaim: Vec<AppPriority> = priority_reclaim_order().collect();
        assert_eq!(reclaim[0], AppPriority::Idle);
        assert_eq!(reclaim[4], AppPriority::Critical);
    }

    #[test]
    fn test_priority_weight() {
        assert!(priority_weight(AppPriority::Critical) > priority_weight(AppPriority::Idle));
    }
}
