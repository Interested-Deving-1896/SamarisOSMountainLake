use crate::core::error::{Result, TesseractError};
use crate::gpu_canvas::commands::{GpuCommand, Layer, RgbaColor};
use crate::gpu_canvas::renderer::Renderer;

pub struct CpuFallbackRenderer;

impl CpuFallbackRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Renderer for CpuFallbackRenderer {
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
        if w == 0 || h == 0 {
            return Ok(Vec::new());
        }
        let total = (w as usize).saturating_mul(h as usize);
        if total > 4_194_304 {
            return Err(TesseractError::Gpu("CPU fallback: rect too large".into()));
        }
        let mut pixels = Vec::with_capacity(total * 4);
        for _ in 0..total {
            pixels.push(fill_color.r);
            pixels.push(fill_color.g);
            pixels.push(fill_color.b);
            pixels.push(fill_color.a);
        }
        Ok(pixels)
    }

    fn clear(&self, r: u8, g: u8, b: u8, a: u8) -> Result<Vec<u8>> {
        let w = 1024usize;
        let h = 768usize;
        let total = w * h;
        let mut pixels = Vec::with_capacity(total * 4);
        for _ in 0..total {
            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
            pixels.push(a);
        }
        Ok(pixels)
    }

    fn is_gpu(&self) -> bool {
        false
    }
}

pub fn compose_layers_software(layers: &[Layer]) -> Result<(Vec<u8>, u32, u32)> {
    let renderer = CpuFallbackRenderer;
    let mut width = 0u32;
    let mut height = 0u32;

    for layer in layers {
        if let GpuCommand::RenderRect { w, h, .. } = &layer.command {
            if *w > width {
                width = *w;
            }
            if *h > height {
                height = *h;
            }
        }
    }

    if width == 0 || height == 0 {
        width = 1024;
        height = 768;
    }

    let total = (width as usize).saturating_mul(height as usize) * 4;
    let mut framebuffer = vec![0u8; total];

    for layer in layers {
        let pixels = match &layer.command {
            GpuCommand::RenderRect {
                x, y, w, h,
                border_radius, shadow_blur,
                shadow_offset_x, shadow_offset_y,
                fill_color, border_color, border_width,
            } => {
                renderer.render_rect(
                    *x, *y, *w, *h,
                    *border_radius, *shadow_blur,
                    *shadow_offset_x, *shadow_offset_y,
                    *fill_color, *border_color, *border_width,
                )?
            }
            GpuCommand::Clear { r, g, b, a } => {
                renderer.clear(*r, *g, *b, *a)?
            }
            GpuCommand::Compose { layers: sub } => {
                compose_layers_software(sub)?.0
            }
        };

        composite_over(&mut framebuffer, &pixels, (layer.opacity * 255.0) as u8);
    }

    Ok((framebuffer, width, height))
}

fn composite_over(dst: &mut [u8], src: &[u8], alpha: u8) {
    let len = dst.len().min(src.len());
    for i in (0..len).step_by(4) {
        if i + 3 >= len {
            break;
        }
        let sa = (src[i + 3] as u32 * alpha as u32 / 255) as u8;
        if sa == 0 {
            continue;
        }
        if sa == 255 {
            dst[i..i + 4].copy_from_slice(&src[i..i + 4]);
        } else {
            let inv_a = 255u16 - sa as u16;
            dst[i] = ((src[i] as u16 * sa as u16 + dst[i] as u16 * inv_a) / 255) as u8;
            dst[i + 1] = ((src[i + 1] as u16 * sa as u16 + dst[i + 1] as u16 * inv_a) / 255) as u8;
            dst[i + 2] = ((src[i + 2] as u16 * sa as u16 + dst[i + 2] as u16 * inv_a) / 255) as u8;
            dst[i + 3] = dst[i + 3].max(sa);
        }
    }
}
