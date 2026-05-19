use tesseract_engine::media::audio::AudioProcessor;
use tesseract_engine::media::video::VideoProcessor;
use tesseract_engine::media::sync::AvSync;
use tesseract_engine::media::MediaEngine;

#[test]
fn test_video_process_frame() {
    let vp = VideoProcessor::new();
    let data = vec![0u8; 640 * 480 * 3];
    let result = vp.process_frame(&data).unwrap();
    assert_eq!(result.len(), data.len());
}

#[test]
fn test_video_decode_encode() {
    let vp = VideoProcessor::new();
    let data = vec![0x10; 1024];
    let decoded = vp.decode_frame(&data, 0).unwrap();
    let encoded = vp.encode_frame(&decoded, 0).unwrap();
    assert_eq!(encoded.len(), decoded.len());
}

#[test]
fn test_video_empty_frame_error() {
    let vp = VideoProcessor::new();
    assert!(vp.process_frame(&[]).is_err());
    assert!(vp.decode_frame(&[], 0).is_err());
    assert!(vp.encode_frame(&[], 0).is_err());
}

#[test]
fn test_audio_process_frame() {
    let ap = AudioProcessor::new();
    let data = vec![0u8; 1024];
    let result = ap.process_frame(&data, 44100, 2).unwrap();
    assert_eq!(result.len(), data.len());
}

#[test]
fn test_audio_empty_frame_error() {
    let ap = AudioProcessor::new();
    assert!(ap.process_frame(&[], 44100, 2).is_err());
}

#[test]
fn test_audio_resample_same_rate() {
    let ap = AudioProcessor::new();
    let data = vec![0u8; 1000];
    let result = ap.resample(&data, 44100, 44100).unwrap();
    assert_eq!(result.len(), data.len());
}

#[test]
fn test_audio_resample_half_rate() {
    let ap = AudioProcessor::new();
    let data = vec![0u8; 2000];
    let result = ap.resample(&data, 44100, 22050).unwrap();
    assert_eq!(result.len(), 1000);
}

#[test]
fn test_audio_resample_zero_rate_error() {
    let ap = AudioProcessor::new();
    assert!(ap.resample(&[0u8; 10], 0, 44100).is_err());
    assert!(ap.resample(&[0u8; 10], 44100, 0).is_err());
}

#[test]
fn test_audio_resample_empty_error() {
    let ap = AudioProcessor::new();
    assert!(ap.resample(&[], 44100, 22050).is_err());
}

#[test]
fn test_audio_mix_two_buffers() {
    let ap = AudioProcessor::new();
    let buf1 = vec![100u8; 100];
    let buf2 = vec![50u8; 100];
    let result = ap.mix(&[&buf1, &buf2]).unwrap();
    assert_eq!(result.len(), 100);
}

#[test]
fn test_audio_mix_empty_error() {
    let ap = AudioProcessor::new();
    assert!(ap.mix(&[]).is_err());
}

#[test]
fn test_av_sync_initial() {
    let sync = AvSync::new();
    assert!(sync.is_synced());
    assert_eq!(sync.current_diff_us(), 0);
}

#[test]
fn test_av_sync_within_tolerance() {
    let mut sync = AvSync::new();
    sync.add_video_frame(1_000, 33_333);
    sync.add_audio_frame(10_000, 22_676);
    assert!(sync.is_synced());
}

#[test]
fn test_av_sync_out_of_tolerance() {
    let mut sync = AvSync::new();
    sync.add_video_frame(1_000, 33_333);
    sync.add_audio_frame(100_000, 22_676);
    assert!(!sync.is_synced());
}

#[test]
fn test_av_sync_diff_sign() {
    let mut sync = AvSync::new();
    sync.add_video_frame(50_000, 33_333);
    sync.add_audio_frame(10_000, 22_676);
    assert!(sync.current_diff_us() > 0);
}

#[test]
fn test_av_sync_set_tolerance() {
    let mut sync = AvSync::new();
    sync.set_tolerance(1_000_000);
    sync.add_video_frame(1_000, 33_333);
    sync.add_audio_frame(500_000, 22_676);
    assert!(sync.is_synced());
}

#[test]
fn test_av_sync_reset() {
    let mut sync = AvSync::new();
    sync.add_video_frame(100_000, 33_333);
    sync.add_audio_frame(50_000, 22_676);
    sync.reset();
    assert!(sync.is_synced());
    assert_eq!(sync.current_diff_us(), 0);
}

#[test]
fn test_media_engine_process_video() {
    let mut engine = MediaEngine::new();
    let data = vec![0u8; 100];
    let result = engine.process_video_frame(&data, 1_000).unwrap();
    assert_eq!(result.len(), data.len());
}

#[test]
fn test_media_engine_process_audio() {
    let mut engine = MediaEngine::new();
    let data = vec![0u8; 100];
    let result = engine.process_audio_frame(&data, 2_000, 44100, 2).unwrap();
    assert_eq!(result.len(), data.len());
}

#[test]
fn test_media_engine_sync_status() {
    let mut engine = MediaEngine::new();
    engine.process_video_frame(&[0u8; 10], 100_000).unwrap();
    engine.process_audio_frame(&[0u8; 10], 500, 44100, 2).unwrap();
    let status = engine.sync_status();
    assert!(!status.is_synced());
}
