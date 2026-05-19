#[derive(Debug, Clone, Copy)]
pub struct PressureThresholds {
    pub green_max: f64,
    pub yellow_enter: f64,
    pub yellow_exit: f64,
    pub orange_enter: f64,
    pub orange_exit: f64,
    pub red_enter: f64,
    pub red_exit: f64,
}

impl Default for PressureThresholds {
    fn default() -> Self {
        PressureThresholds {
            green_max: 65.0,
            yellow_enter: 70.0,
            yellow_exit: 65.0,
            orange_enter: 85.0,
            orange_exit: 80.0,
            red_enter: 95.0,
            red_exit: 90.0,
        }
    }
}

impl PressureThresholds {
    pub fn validate(&self) -> Result<(), String> {
        if self.green_max < 0.0 || self.green_max > 100.0 {
            return Err("green_max must be between 0 and 100".into());
        }
        if self.yellow_enter < 0.0 || self.yellow_enter > 100.0 {
            return Err("yellow_enter must be between 0 and 100".into());
        }
        if self.yellow_exit < 0.0 || self.yellow_exit > 100.0 {
            return Err("yellow_exit must be between 0 and 100".into());
        }
        if self.orange_enter < 0.0 || self.orange_enter > 100.0 {
            return Err("orange_enter must be between 0 and 100".into());
        }
        if self.orange_exit < 0.0 || self.orange_exit > 100.0 {
            return Err("orange_exit must be between 0 and 100".into());
        }
        if self.red_enter < 0.0 || self.red_enter > 100.0 {
            return Err("red_enter must be between 0 and 100".into());
        }
        if self.red_exit < 0.0 || self.red_exit > 100.0 {
            return Err("red_exit must be between 0 and 100".into());
        }
        if self.yellow_enter <= self.green_max {
            return Err("yellow_enter must be greater than green_max".into());
        }
        if self.yellow_exit >= self.yellow_enter {
            return Err("yellow_exit must be less than yellow_enter".into());
        }
        if self.orange_enter <= self.yellow_enter {
            return Err("orange_enter must be greater than yellow_enter".into());
        }
        if self.orange_exit >= self.orange_enter {
            return Err("orange_exit must be less than orange_enter".into());
        }
        if self.red_enter <= self.orange_enter {
            return Err("red_enter must be greater than orange_enter".into());
        }
        if self.red_exit >= self.red_enter {
            return Err("red_exit must be less than red_enter".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_valid() {
        let t = PressureThresholds::default();
        assert!(t.validate().is_ok());
    }

    #[test]
    fn test_invalid_green_max() {
        let t = PressureThresholds {
            green_max: -1.0,
            ..Default::default()
        };
        assert!(t.validate().is_err());
    }

    #[test]
    fn test_invalid_ordering() {
        let t = PressureThresholds {
            yellow_enter: 50.0,
            green_max: 60.0,
            ..Default::default()
        };
        assert!(t.validate().is_err());
    }

    #[test]
    fn test_bad_hysteresis() {
        let t = PressureThresholds {
            yellow_exit: 75.0,
            ..Default::default()
        };
        assert!(t.validate().is_err());
    }
}
