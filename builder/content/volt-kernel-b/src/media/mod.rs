pub mod audio;
pub mod sync;
pub mod video;

use crate::core::error::Result;
use crate::media::audio::AudioProcessor;
use crate::media::sync::AvSync;
use crate::media::video::VideoProcessor;

pub struct MediaEngine {
    video: VideoProcessor,
    audio: AudioProcessor,
    sync: AvSync,
}

impl MediaEngine {
    pub fn new() -> Self {
        Self {
            video: VideoProcessor::new(),
            audio: AudioProcessor::new(),
            sync: AvSync::new(),
        }
    }

    pub fn process_video_frame(&mut self, data: &[u8], pts_us: u64) -> Result<Vec<u8>> {
        self.sync.add_video_frame(pts_us, 33_333);
        self.video.process_frame(data)
    }

    pub fn process_audio_frame(&mut self, data: &[u8], pts_us: u64, sample_rate: u32, channels: u8) -> Result<Vec<u8>> {
        self.sync.add_audio_frame(pts_us, (1_000_000 / sample_rate) as u64);
        self.audio.process_frame(data, sample_rate, channels)
    }

    pub fn sync_status(&self) -> AvSync {
        self.sync.clone()
    }

    pub fn is_synced(&self) -> bool {
        self.sync.is_synced()
    }
}
