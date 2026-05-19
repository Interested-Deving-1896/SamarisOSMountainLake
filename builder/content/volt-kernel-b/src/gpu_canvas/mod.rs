pub mod commands;
pub mod compositor;
pub mod fallback;
pub mod renderer;

use crate::core::error::Result;
use crate::gpu_canvas::commands::GpuCommand;
use crate::gpu_canvas::renderer::{GpuRenderer, Renderer};

pub struct GpuCanvas {
    renderer: Box<dyn Renderer + Send + Sync>,
    gpu_available: bool,
}

impl GpuCanvas {
    pub fn new() -> Self {
        let (renderer, gpu_available) = if Self::probe_gpu() {
            tracing::info!("GPU acceleration available");
            (Box::new(GpuRenderer::new()) as Box<dyn Renderer + Send + Sync>, true)
        } else {
            tracing::info!("GPU unavailable — using CPU fallback");
            (Box::new(fallback::CpuFallbackRenderer::new()) as Box<dyn Renderer + Send + Sync>, false)
        };

        Self {
            renderer,
            gpu_available,
        }
    }

    pub fn execute_command(&self, cmd: &GpuCommand) -> Result<RenderOutput> {
        match cmd {
            GpuCommand::RenderRect { x, y, w, h, border_radius, shadow_blur, shadow_offset_x, shadow_offset_y, fill_color, border_color, border_width } => {
                let pixels = self.renderer.render_rect(*x, *y, *w, *h, *border_radius, *shadow_blur, *shadow_offset_x, *shadow_offset_y, *fill_color, *border_color, *border_width)?;
                Ok(RenderOutput {
                    pixels,
                    width: *w,
                    height: *h,
                    gpu: self.gpu_available,
                })
            }
            GpuCommand::Clear { r, g, b, a } => {
                let pixels = self.renderer.clear(*r, *g, *b, *a)?;
                Ok(RenderOutput {
                    pixels,
                    width: 0,
                    height: 0,
                    gpu: self.gpu_available,
                })
            }
            GpuCommand::Compose { layers } => {
                let (pixels, w, h) = fallback::compose_layers_software(layers)?;
                Ok(RenderOutput {
                    pixels,
                    width: w,
                    height: h,
                    gpu: self.gpu_available,
                })
            }
        }
    }

    pub fn is_gpu_available(&self) -> bool {
        self.gpu_available
    }

    fn probe_gpu() -> bool {
        std::fs::metadata("/dev/dri").is_ok()
            || std::fs::metadata("/dev/nvidia0").is_ok()
            || std::option_env!("TESSERACT_FORCE_GPU").is_some()
    }
}

#[derive(Debug, Clone)]
pub struct RenderOutput {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub gpu: bool,
}
