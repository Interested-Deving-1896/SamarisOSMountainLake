use crate::pressure::level::PressureLevel;
use crate::pressure::thresholds::PressureThresholds;

pub struct HysteresisController {
    current: PressureLevel,
    thresholds: PressureThresholds,
}

impl HysteresisController {
    pub fn new(thresholds: PressureThresholds) -> Self {
        HysteresisController {
            current: PressureLevel::Green,
            thresholds,
        }
    }

    pub fn evaluate(&mut self, usage_pct: f64) -> Option<PressureLevel> {
        let old = self.current;
        let new = match self.current {
            PressureLevel::Green => {
                if usage_pct >= self.thresholds.yellow_enter {
                    PressureLevel::Yellow
                } else {
                    PressureLevel::Green
                }
            }
            PressureLevel::Yellow => {
                if usage_pct >= self.thresholds.orange_enter {
                    PressureLevel::Orange
                } else if usage_pct < self.thresholds.yellow_exit {
                    PressureLevel::Green
                } else {
                    PressureLevel::Yellow
                }
            }
            PressureLevel::Orange => {
                if usage_pct >= self.thresholds.red_enter {
                    PressureLevel::Red
                } else if usage_pct < self.thresholds.orange_exit {
                    PressureLevel::Yellow
                } else {
                    PressureLevel::Orange
                }
            }
            PressureLevel::Red => {
                if usage_pct < self.thresholds.red_exit {
                    PressureLevel::Orange
                } else {
                    PressureLevel::Red
                }
            }
        };

        if new != old {
            self.current = new;
            Some(new)
        } else {
            None
        }
    }

    pub fn current(&self) -> PressureLevel {
        self.current
    }

    pub fn reset(&mut self) {
        self.current = PressureLevel::Green;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_thresholds() -> PressureThresholds {
        PressureThresholds::default()
    }

    #[test]
    fn test_starts_green() {
        let c = HysteresisController::new(default_thresholds());
        assert_eq!(c.current(), PressureLevel::Green);
    }

    #[test]
    fn test_green_to_yellow() {
        let mut c = HysteresisController::new(default_thresholds());
        let result = c.evaluate(75.0);
        assert_eq!(result, Some(PressureLevel::Yellow));
        assert_eq!(c.current(), PressureLevel::Yellow);
    }

    #[test]
    fn test_stays_green_below_threshold() {
        let mut c = HysteresisController::new(default_thresholds());
        let result = c.evaluate(50.0);
        assert!(result.is_none());
        assert_eq!(c.current(), PressureLevel::Green);
    }

    #[test]
    fn test_yellow_to_green() {
        let mut c = HysteresisController::new(default_thresholds());
        c.evaluate(75.0);
        let result = c.evaluate(60.0);
        assert_eq!(result, Some(PressureLevel::Green));
    }

    #[test]
    fn test_yellow_to_orange() {
        let mut c = HysteresisController::new(default_thresholds());
        c.evaluate(75.0);
        let result = c.evaluate(88.0);
        assert_eq!(result, Some(PressureLevel::Orange));
    }

    #[test]
    fn test_orange_to_red() {
        let mut c = HysteresisController::new(default_thresholds());
        c.evaluate(75.0);
        c.evaluate(88.0);
        let result = c.evaluate(97.0);
        assert_eq!(result, Some(PressureLevel::Red));
    }

    #[test]
    fn test_red_to_orange() {
        let mut c = HysteresisController::new(default_thresholds());
        c.evaluate(75.0);
        c.evaluate(88.0);
        c.evaluate(97.0);
        let result = c.evaluate(85.0);
        assert_eq!(result, Some(PressureLevel::Orange));
    }

    #[test]
    fn test_stays_in_level() {
        let mut c = HysteresisController::new(default_thresholds());
        c.evaluate(75.0);
        let result = c.evaluate(72.0);
        assert!(result.is_none());
        assert_eq!(c.current(), PressureLevel::Yellow);
    }

    #[test]
    fn test_reset() {
        let mut c = HysteresisController::new(default_thresholds());
        c.evaluate(75.0);
        assert_eq!(c.current(), PressureLevel::Yellow);
        c.reset();
        assert_eq!(c.current(), PressureLevel::Green);
    }
}
