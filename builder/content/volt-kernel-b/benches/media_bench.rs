use criterion::{criterion_group, criterion_main, Criterion};

use tesseract_engine::media::audio::AudioProcessor;
use tesseract_engine::media::video::VideoProcessor;
use tesseract_engine::media::MediaEngine;

fn bench_video_process_frame(c: &mut Criterion) {
    let vp = VideoProcessor::new();
    let data = vec![0u8; 1920 * 1080 * 3];
    c.bench_function("media_video_process_1080p", |b| {
        b.iter(|| {
            let _ = vp.process_frame(&data);
        });
    });
}

fn bench_audio_process_frame(c: &mut Criterion) {
    let ap = AudioProcessor::new();
    let data = vec![0u8; 44100 * 2];
    c.bench_function("media_audio_process_1s_pcm", |b| {
        b.iter(|| {
            let _ = ap.process_frame(&data, 44100, 2);
        });
    });
}

fn bench_audio_resample(c: &mut Criterion) {
    let ap = AudioProcessor::new();
    let data = vec![0u8; 44100 * 2];
    c.bench_function("media_audio_resample_44100_to_22050", |b| {
        b.iter(|| {
            let _ = ap.resample(&data, 44100, 22050);
        });
    });
}

fn bench_audio_mix_2ch(c: &mut Criterion) {
    let ap = AudioProcessor::new();
    let buf1 = vec![100u8; 4096];
    let buf2 = vec![50u8; 4096];
    c.bench_function("media_audio_mix_2ch_4k", |b| {
        b.iter(|| {
            let _ = ap.mix(&[&buf1, &buf2]);
        });
    });
}

fn bench_media_engine_video(c: &mut Criterion) {
    let mut engine = MediaEngine::new();
    let data = vec![0u8; 640 * 480 * 3];
    c.bench_function("media_engine_video_frame", |b| {
        b.iter(|| {
            let _ = engine.process_video_frame(&data, 0);
        });
    });
}

criterion_group!(
    benches,
    bench_video_process_frame,
    bench_audio_process_frame,
    bench_audio_resample,
    bench_audio_mix_2ch,
    bench_media_engine_video
);
criterion_main!(benches);
