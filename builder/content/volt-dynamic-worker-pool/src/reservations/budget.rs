#[derive(Clone, Debug)]
pub struct BudgetSnapshot {
    pub total_workers: u32,
    pub desktop_reserved: u32,
    pub system_reserved: u32,
    pub orbit_fraction: f64,
    pub available: u32,
    pub orbit_max: u32,
    pub background_max: u32,
}

pub struct ReservationBudget {
    pub total_workers: u32,
    pub desktop_reserved: u32,
    pub system_reserved: u32,
    pub orbit_fraction: f64,
    pub available: u32,
}

impl ReservationBudget {
    pub fn new(
        total_workers: u32,
        desktop_min: u32,
        system_min: u32,
        orbit_fraction: f64,
    ) -> Self {
        Self {
            total_workers,
            desktop_reserved: desktop_min,
            system_reserved: system_min,
            orbit_fraction,
            available: 0,
        }
    }

    pub fn compute(&mut self) {
        let reserved = self.desktop_reserved.saturating_add(self.system_reserved);
        self.available = self.total_workers.saturating_sub(reserved);
    }

    pub fn available_for_orbit(&self) -> u32 {
        (self.available as f64 * self.orbit_fraction) as u32
    }

    pub fn available_for_background(&self) -> u32 {
        self.available.saturating_sub(self.available_for_orbit())
    }

    pub fn snapshot(&self) -> BudgetSnapshot {
        let orbit_max = self.available_for_orbit();
        let background_max = self.available_for_background();
        BudgetSnapshot {
            total_workers: self.total_workers,
            desktop_reserved: self.desktop_reserved,
            system_reserved: self.system_reserved,
            orbit_fraction: self.orbit_fraction,
            available: self.available,
            orbit_max,
            background_max,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let b = ReservationBudget::new(100, 10, 5, 0.5);
        assert_eq!(b.total_workers, 100);
        assert_eq!(b.desktop_reserved, 10);
        assert_eq!(b.system_reserved, 5);
        assert_eq!(b.orbit_fraction, 0.5);
        assert_eq!(b.available, 0);
    }

    #[test]
    fn test_compute() {
        let mut b = ReservationBudget::new(100, 10, 5, 0.5);
        b.compute();
        assert_eq!(b.available, 85);
    }

    #[test]
    fn test_compute_saturating() {
        let mut b = ReservationBudget::new(10, 10, 5, 0.5);
        b.compute();
        assert_eq!(b.available, 0);
    }

    #[test]
    fn test_available_for_orbit() {
        let mut b = ReservationBudget::new(100, 10, 10, 0.4);
        b.compute();
        assert_eq!(b.available_for_orbit(), 32);
    }

    #[test]
    fn test_available_for_background() {
        let mut b = ReservationBudget::new(100, 10, 10, 0.4);
        b.compute();
        assert_eq!(b.available_for_background(), 48);
    }

    #[test]
    fn test_snapshot() {
        let mut b = ReservationBudget::new(100, 10, 5, 0.5);
        b.compute();
        let s = b.snapshot();
        assert_eq!(s.total_workers, 100);
        assert_eq!(s.available, 85);
        assert_eq!(s.orbit_max, 42);
        assert_eq!(s.background_max, 43);
    }
}
