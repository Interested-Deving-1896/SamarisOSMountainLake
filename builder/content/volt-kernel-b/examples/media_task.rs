use tesseract_engine::media::audio::AudioProcessor;
use tesseract_engine::media::video::VideoProcessor;
use tesseract_engine::media::sync::AvSync;
use tesseract_engine::media::MediaEngine;

fn main() {
    // Video processing
    let video = VideoProcessor::new();
    let frame_data = vec![0u8; 640 * 480 * 3];
    match video.process_frame(&frame_data) {
        Ok(processed) => {
            println!("Video frame: {} bytes processed", processed.len());
        }
        Err(e) => {
            eprintln!("Video processing failed: {e}");
        }
    }

    // Audio processing
    let audio = AudioProcessor::new();
    let pcm_data = vec![0u8; 44100 * 2]; // 1 second of 16-bit mono
    match audio.process_frame(&pcm_data, 44100, 1) {
        Ok(processed) => {
            println!("Audio frame: {} bytes processed", processed.len());
        }
        Err(e) => {
            eprintln!("Audio processing failed: {e}");
        }
    }

    // Audio mixing
    let buf1 = vec![100u8; 1000];
    let buf2 = vec![50u8; 1000];
    match audio.mix(&[&buf1, &buf2]) {
        Ok(mixed) => {
            println!("Mixed audio: {} bytes", mixed.len());
        }
        Err(e) => {
            eprintln!("Audio mixing failed: {e}");
        }
    }

    // Audio resampling
    match audio.resample(&pcm_data, 44100, 22050) {
        Ok(resampled) => {
            println!("Resampled: {} -> {} bytes", pcm_data.len(), resampled.len());
        }
        Err(e) => {
            eprintln!("Resampling failed: {e}");
        }
    }

    // A/V Sync
    let mut sync = AvSync::new();
    sync.add_video_frame(0, 33_333);
    sync.add_audio_frame(0, 22_676);
    println!("A/V synced: {} (diff: {}µs)", sync.is_synced(), sync.current_diff_us());

    sync.add_video_frame(100_000, 33_333);
    sync.add_audio_frame(50_000, 22_676);
    println!("A/V synced after drift: {} (diff: {}µs)", sync.is_synced(), sync.current_diff_us());

    // Media Engine integration
    let mut engine = MediaEngine::new();
    match engine.process_video_frame(&frame_data, 0) {
        Ok(result) => {
            println!("Media engine video: {} bytes", result.len());
        }
        Err(e) => {
            eprintln!("Media engine video failed: {e}");
        }
    }

    println!("Sync status: {}", engine.sync_status().current_diff_us());
}
