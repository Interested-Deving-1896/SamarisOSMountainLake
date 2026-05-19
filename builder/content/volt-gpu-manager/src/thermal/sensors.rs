use crate::thermal::state::ThermalLevel;

pub struct ThermalSensor;

impl ThermalSensor {
    pub fn read_temp_celsius() -> Option<f64> {
        #[cfg(target_os = "linux")]
        {
            Self::read_linux_temp()
        }
        #[cfg(not(target_os = "linux"))]
        {
            None
        }
    }

    pub fn read_gpu_temp() -> Option<f64> {
        Self::read_temp_celsius()
    }

    pub fn estimate_state(temp_c: Option<f64>) -> ThermalLevel {
        match temp_c {
            None => ThermalLevel::Unknown,
            Some(t) if t.is_nan() || t.is_infinite() => ThermalLevel::Unknown,
            Some(t) if t >= 100.0 => ThermalLevel::Fatal,
            Some(t) if t >= 95.0 => ThermalLevel::Emergency,
            Some(t) if t >= 85.0 => ThermalLevel::Critical,
            Some(t) if t >= 80.0 => ThermalLevel::Throttle,
            Some(t) if t >= 70.0 => ThermalLevel::Hot,
            Some(t) if t >= 55.0 => ThermalLevel::Warm,
            _ => ThermalLevel::Normal,
        }
    }

    #[cfg(target_os = "linux")]
    fn read_linux_temp() -> Option<f64> {
        let paths = glob::glob("/sys/class/thermal/thermal_zone*/temp").ok()?;
        for entry in paths {
            if let Ok(path) = entry {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(millidegrees) = content.trim().parse::<f64>() {
                        let celsius = millidegrees / 1000.0;
                        if celsius > 0.0 && celsius < 200.0 {
                            return Some(celsius);
                        }
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_state_normal() {
        assert_eq!(ThermalSensor::estimate_state(Some(35.0)), ThermalLevel::Normal);
        assert_eq!(ThermalSensor::estimate_state(Some(50.0)), ThermalLevel::Normal);
    }

    #[test]
    fn test_estimate_state_warm() {
        assert_eq!(ThermalSensor::estimate_state(Some(60.0)), ThermalLevel::Warm);
    }

    #[test]
    fn test_estimate_state_hot() {
        assert_eq!(ThermalSensor::estimate_state(Some(75.0)), ThermalLevel::Hot);
    }

    #[test]
    fn test_estimate_state_throttle() {
        assert_eq!(ThermalSensor::estimate_state(Some(82.0)), ThermalLevel::Throttle);
    }

    #[test]
    fn test_estimate_state_critical() {
        assert_eq!(ThermalSensor::estimate_state(Some(90.0)), ThermalLevel::Critical);
    }

    #[test]
    fn test_estimate_state_emergency() {
        assert_eq!(ThermalSensor::estimate_state(Some(97.0)), ThermalLevel::Emergency);
    }

    #[test]
    fn test_estimate_state_fatal() {
        assert_eq!(ThermalSensor::estimate_state(Some(105.0)), ThermalLevel::Fatal);
    }

    #[test]
    fn test_estimate_state_unknown() {
        assert_eq!(ThermalSensor::estimate_state(None), ThermalLevel::Unknown);
        assert_eq!(ThermalSensor::estimate_state(Some(f64::NAN)), ThermalLevel::Unknown);
        assert_eq!(ThermalSensor::estimate_state(Some(f64::INFINITY)), ThermalLevel::Unknown);
    }
}
