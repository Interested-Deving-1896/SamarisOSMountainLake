#[derive(Debug, Clone)]
pub struct AvSync {
    video_pts: Vec<u64>,
    audio_pts: Vec<u64>,
    tolerance_us: u64,
}

impl AvSync {
    pub fn new() -> Self {
        Self {
            video_pts: Vec::with_capacity(64),
            audio_pts: Vec::with_capacity(64),
            tolerance_us: 40_000,
        }
    }

    pub fn add_video_frame(&mut self, pts_us: u64, _duration_us: u64) {
        self.video_pts.push(pts_us);
        if self.video_pts.len() > 128 {
            self.video_pts.remove(0);
        }
    }

    pub fn add_audio_frame(&mut self, pts_us: u64, _duration_us: u64) {
        self.audio_pts.push(pts_us);
        if self.audio_pts.len() > 128 {
            self.audio_pts.remove(0);
        }
    }

    pub fn is_synced(&self) -> bool {
        let video = self.video_pts.last().copied().unwrap_or(0);
        let audio = self.audio_pts.last().copied().unwrap_or(0);

        if video == 0 || audio == 0 {
            return true;
        }

        let diff = if video > audio {
            video - audio
        } else {
            audio - video
        };

        diff < self.tolerance_us
    }

    pub fn current_diff_us(&self) -> i64 {
        let video = self.video_pts.last().copied().unwrap_or(0);
        let audio = self.audio_pts.last().copied().unwrap_or(0);
        video as i64 - audio as i64
    }

    pub fn set_tolerance(&mut self, tolerance_us: u64) {
        self.tolerance_us = tolerance_us;
    }

    pub fn reset(&mut self) {
        self.video_pts.clear();
        self.audio_pts.clear();
    }
}
