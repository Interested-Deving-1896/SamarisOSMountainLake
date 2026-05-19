use crate::core::error::Result;
use crate::gpu_canvas::commands::RgbaColor;

pub trait Renderer {
    fn render_rect(
        &self,
        x: i32, y: i32, w: u32, h: u32,
        border_radius: f32,
        shadow_blur: f32,
        shadow_offset_x: f32,
        shadow_offset_y: f32,
        fill_color: RgbaColor,
        border_color: RgbaColor,
        border_width: f32,
    ) -> Result<Vec<u8>>;

    fn clear(&self, r: u8, g: u8, b: u8, a: u8) -> Result<Vec<u8>>;

    fn is_gpu(&self) -> bool;
}

pub struct GpuRenderer;

impl GpuRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for GpuRenderer {
    fn render_rect(
        &self,
        _x: i32, _y: i32, w: u32, h: u32,
        _border_radius: f32,
        _shadow_blur: f32,
        _shadow_offset_x: f32,
        _shadow_offset_y: f32,
        fill_color: RgbaColor,
        _border_color: RgbaColor,
        _border_width: f32,
    ) -> Result<Vec<u8>> {
        software_rasterize_rect(w, h, fill_color)
    }

    fn clear(&self, r: u8, g: u8, b: u8, a: u8) -> Result<Vec<u8>> {
        let size = 1024 * 768 * 4;
        let color = RgbaColor::new(r, g, b, a);
        Ok(vec![color.r, color.g, color.b, color.a].repeat(size / 4))
    }

    fn is_gpu(&self) -> bool {
        true
    }
}

fn software_rasterize_rect(w: u32, h: u32, fill: RgbaColor) -> Result<Vec<u8>> {
    let total_pixels = (w as usize).saturating_mul(h as usize);
    if total_pixels > 4_194_304 {
        return Err(crate::core::error::TesseractError::Gpu(
            "render rect too large".into(),
        ));
    }

    let mut pixels = Vec::with_capacity(total_pixels * 4);
    for _ in 0..total_pixels {
        pixels.push(fill.r);
        pixels.push(fill.g);
        pixels.push(fill.b);
        pixels.push(fill.a);
    }
    Ok(pixels)
}
