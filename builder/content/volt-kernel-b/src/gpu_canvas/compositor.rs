use crate::core::error::{Result, TesseractError};
use crate::gpu_canvas::commands::{GpuCommand, Layer};
use crate::gpu_canvas::fallback::compose_layers_software;
use crate::gpu_canvas::renderer::Renderer;

pub struct Compositor {
    layers: Vec<Layer>,
}

impl Compositor {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
        self.layers.sort_by_key(|l| l.z_index);
    }

    pub fn remove_layer(&mut self, z_index: i32) {
        self.layers.retain(|l| l.z_index != z_index);
    }

    pub fn clear_layers(&mut self) {
        self.layers.clear();
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn compose(&self, renderer: &dyn Renderer, width: u32, height: u32) -> Result<Vec<u8>> {
        let total = (width as usize).saturating_mul(height as usize) * 4;
        if total > 16_777_216 {
            return Err(TesseractError::Gpu("composite target too large".into()));
        }

        let mut framebuffer = vec![0u8; total];

        for layer in &self.layers {
            let layer_pixels = match &layer.command {
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
                GpuCommand::Compose { layers } => {
                    compose_layers_software(layers)?.0
                }
            };

            if layer.opacity < 1.0 {
                let alpha = (layer.opacity * 255.0) as u8;
                composite_over(&mut framebuffer, &layer_pixels, alpha);
            } else {
                composite_over(&mut framebuffer, &layer_pixels, 255);
            }
        }

        Ok(framebuffer)
    }
}

fn composite_over(dst: &mut [u8], src: &[u8], src_alpha: u8) {
    let len = dst.len().min(src.len());
    for i in (0..len).step_by(4) {
        if i + 3 >= len {
            break;
        }
        let sa = (src[i + 3] as u32 * src_alpha as u32 / 255) as u8;
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
