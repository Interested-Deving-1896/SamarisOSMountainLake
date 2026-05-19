pub use super::WorkerId;

impl WorkerId {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn next(&self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}
