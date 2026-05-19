use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DetectionConfidence {
    pub cpu: f32,
    pub ram: f32,
    pub gpu: f32,
    pub storage: f32,
    pub usb: f32,
    pub vm: f32,
    pub laptop: f32,
}

impl DetectionConfidence {
    #[allow(non_upper_case_globals)]
    pub const High: DetectionConfidence = DetectionConfidence {
        cpu: 0.95,
        ram: 0.95,
        gpu: 0.95,
        storage: 0.95,
        usb: 0.95,
        vm: 0.95,
        laptop: 0.95,
    };

    #[allow(non_upper_case_globals)]
    pub const Medium: DetectionConfidence = DetectionConfidence {
        cpu: 0.6,
        ram: 0.6,
        gpu: 0.6,
        storage: 0.6,
        usb: 0.6,
        vm: 0.6,
        laptop: 0.6,
    };

    pub fn new(high: bool) -> Self {
        let val: f32 = if high { 0.95 } else { 0.3 };
        Self {
            cpu: val.clamp(0.0, 1.0),
            ram: val.clamp(0.0, 1.0),
            gpu: val.clamp(0.0, 1.0),
            storage: val.clamp(0.0, 1.0),
            usb: val.clamp(0.0, 1.0),
            vm: val.clamp(0.0, 1.0),
            laptop: val.clamp(0.0, 1.0),
        }
    }
}

impl Default for DetectionConfidence {
    fn default() -> Self {
        Self {
            cpu: 0.0,
            ram: 0.0,
            gpu: 0.0,
            storage: 0.0,
            usb: 0.0,
            vm: 0.0,
            laptop: 0.0,
        }
    }
}
