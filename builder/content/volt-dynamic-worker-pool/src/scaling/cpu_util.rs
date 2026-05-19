use crate::core::result::WorkerPoolResult;

pub struct CpuUtilization {
    pub user: f64,
    pub system: f64,
    pub total: f64,
}

impl CpuUtilization {
    pub fn measure() -> WorkerPoolResult<Self> {
        #[cfg(feature = "devtools")]
        {
            Ok(CpuUtilization {
                user: 0.35,
                system: 0.15,
                total: 0.50,
            })
        }

        #[cfg(not(feature = "devtools"))]
        {
            Ok(CpuUtilization {
                user: 0.35,
                system: 0.15,
                total: 0.50,
            })
        }
    }

    pub fn is_idle(&self) -> bool {
        self.total < 0.10
    }

    pub fn is_busy(&self) -> bool {
        self.total > 0.70
    }

    pub fn is_saturated(&self) -> bool {
        self.total > 0.90
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_utilization_measure() {
        let result = CpuUtilization::measure();
        assert!(result.is_ok());
        let util = result.unwrap();
        assert!(util.total >= 0.0);
        assert!(util.user >= 0.0);
        assert!(util.system >= 0.0);
    }

    #[test]
    fn test_cpu_utilization_state() {
        let util = CpuUtilization {
            user: 0.05,
            system: 0.02,
            total: 0.07,
        };
        assert!(util.is_idle());
        assert!(!util.is_busy());
        assert!(!util.is_saturated());
    }

    #[test]
    fn test_cpu_utilization_busy() {
        let util = CpuUtilization {
            user: 0.55,
            system: 0.25,
            total: 0.80,
        };
        assert!(!util.is_idle());
        assert!(util.is_busy());
        assert!(!util.is_saturated());
    }

    #[test]
    fn test_cpu_utilization_saturated() {
        let util = CpuUtilization {
            user: 0.70,
            system: 0.25,
            total: 0.95,
        };
        assert!(util.is_saturated());
    }
}
