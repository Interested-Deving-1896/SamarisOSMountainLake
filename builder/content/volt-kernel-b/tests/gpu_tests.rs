use tesseract_engine::gpu_canvas::commands::{RgbaColor, GpuCommand, Layer};
use tesseract_engine::gpu_canvas::compositor::Compositor;
use tesseract_engine::gpu_canvas::renderer::{GpuRenderer, Renderer};
use tesseract_engine::gpu_canvas::fallback::{CpuFallbackRenderer, compose_layers_software};
use tesseract_engine::gpu_canvas::GpuCanvas;

#[test]
fn test_rgba_color_roundtrip() {
    let c = RgbaColor::new(128, 64, 32, 255);
    let u = c.to_u32();
    let c2 = RgbaColor::from_u32(u);
    assert_eq!(c.r, c2.r);
    assert_eq!(c.g, c2.g);
    assert_eq!(c.b, c2.b);
    assert_eq!(c.a, c2.a);
}

#[test]
fn test_rgba_color_constants() {
    assert_eq!(RgbaColor::BLACK.r, 0);
    assert_eq!(RgbaColor::BLACK.g, 0);
    assert_eq!(RgbaColor::BLACK.b, 0);
    assert_eq!(RgbaColor::BLACK.a, 255);
    assert_eq!(RgbaColor::WHITE.r, 255);
    assert_eq!(RgbaColor::WHITE.a, 255);
    assert_eq!(RgbaColor::TRANSPARENT.a, 0);
    assert_eq!(RgbaColor::TRANSPARENT.to_u32(), 0x00000000);
}

#[test]
fn test_gpu_renderer_rect_size() {
    let r = GpuRenderer::new();
    let pixels = r.render_rect(0, 0, 100, 200, 0.0, 0.0, 0.0, 0.0,
                               RgbaColor::WHITE, RgbaColor::BLACK, 0.0).unwrap();
    assert_eq!(pixels.len(), 100 * 200 * 4);
}

#[test]
fn test_gpu_renderer_clear_returns_data() {
    let r = GpuRenderer::new();
    let pixels = r.clear(255, 0, 0, 255).unwrap();
    assert!(!pixels.is_empty());
}

#[test]
fn test_gpu_renderer_rect_too_large_error() {
    let r = GpuRenderer::new();
    let result = r.render_rect(0, 0, 4097, 4097, 0.0, 0.0, 0.0, 0.0,
                               RgbaColor::WHITE, RgbaColor::BLACK, 0.0);
    assert!(result.is_err());
}

#[test]
fn test_gpu_renderer_is_gpu() {
    assert!(GpuRenderer::new().is_gpu());
}

#[test]
fn test_cpu_fallback_rect_size() {
    let r = CpuFallbackRenderer::new();
    let pixels = r.render_rect(0, 0, 50, 30, 0.0, 0.0, 0.0, 0.0,
                               RgbaColor::WHITE, RgbaColor::BLACK, 0.0).unwrap();
    assert_eq!(pixels.len(), 50 * 30 * 4);
}

#[test]
fn test_cpu_fallback_zero_size_rect() {
    let r = CpuFallbackRenderer::new();
    let pixels = r.render_rect(0, 0, 0, 0, 0.0, 0.0, 0.0, 0.0,
                               RgbaColor::WHITE, RgbaColor::BLACK, 0.0).unwrap();
    assert!(pixels.is_empty());
}

#[test]
fn test_cpu_fallback_is_not_gpu() {
    assert!(!CpuFallbackRenderer::new().is_gpu());
}

#[test]
fn test_compositor_layer_management() {
    let mut comp = Compositor::new();
    assert_eq!(comp.layer_count(), 0);
    comp.add_layer(Layer {
        z_index: 0, opacity: 1.0,
        command: GpuCommand::Clear { r: 0, g: 0, b: 0, a: 255 },
    });
    assert_eq!(comp.layer_count(), 1);
    comp.remove_layer(0);
    assert_eq!(comp.layer_count(), 0);
}

#[test]
fn test_compositor_clear_layers() {
    let mut comp = Compositor::new();
    comp.add_layer(Layer {
        z_index: 1, opacity: 0.5,
        command: GpuCommand::Clear { r: 255, g: 0, b: 0, a: 255 },
    });
    comp.clear_layers();
    assert_eq!(comp.layer_count(), 0);
}

#[test]
fn test_compositor_compose_output_size() {
    let comp = Compositor::new();
    let renderer = GpuRenderer::new();
    let result = comp.compose(&renderer, 1024, 768).unwrap();
    assert_eq!(result.len(), 1024 * 768 * 4);
}

#[test]
fn test_compose_layers_software() {
    let layers = vec![
        Layer {
            z_index: 0, opacity: 1.0,
            command: GpuCommand::render_rect(0, 0, 100, 100, 255, 0, 0, 255),
        },
    ];
    let (pixels, w, h) = compose_layers_software(&layers).unwrap();
    assert!(!pixels.is_empty());
    assert_eq!(w, 100);
    assert_eq!(h, 100);
    assert_eq!(pixels.len(), 100 * 100 * 4);
}

#[test]
fn test_gpu_canvas_new() {
    let canvas = GpuCanvas::new();
    // Should not panic; GPU may or may not be available
    let _ = canvas.is_gpu_available();
}

#[test]
fn test_gpu_canvas_execute_render_rect() {
    let canvas = GpuCanvas::new();
    let cmd = GpuCommand::render_rect(0, 0, 320, 240, 64, 128, 192, 255);
    let output = canvas.execute_command(&cmd).unwrap();
    assert_eq!(output.width, 320);
    assert_eq!(output.height, 240);
    assert_eq!(output.pixels.len(), 320 * 240 * 4);
}

#[test]
fn test_gpu_canvas_execute_clear() {
    let canvas = GpuCanvas::new();
    let cmd = GpuCommand::Clear { r: 0, g: 0, b: 0, a: 255 };
    let output = canvas.execute_command(&cmd).unwrap();
    assert!(!output.pixels.is_empty());
}
