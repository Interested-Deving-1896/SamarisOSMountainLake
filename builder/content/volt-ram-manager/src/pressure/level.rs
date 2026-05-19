#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PressureLevel {
    Green,
    Yellow,
    Orange,
    Red,
}

impl PressureLevel {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "green" => PressureLevel::Green,
            "yellow" => PressureLevel::Yellow,
            "orange" => PressureLevel::Orange,
            "red" => PressureLevel::Red,
            _ => PressureLevel::Green,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            PressureLevel::Green => "green",
            PressureLevel::Yellow => "yellow",
            PressureLevel::Orange => "orange",
            PressureLevel::Red => "red",
        }
    }

    pub fn name(&self) -> &'static str {
        self.as_str()
    }

    pub fn is_critical(&self) -> bool {
        matches!(self, PressureLevel::Orange | PressureLevel::Red)
    }

    pub fn display(&self) -> &'static str {
        match self {
            PressureLevel::Green => "GREEN",
            PressureLevel::Yellow => "YELLOW",
            PressureLevel::Orange => "ORANGE",
            PressureLevel::Red => "RED",
        }
    }
}
