use crate::render::frame::RenderFrame;
use crate::core::result::VgmResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Over,
    Add,
    Multiply,
    Screen,
    Alpha,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub order: u32,
    pub blend: BlendMode,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub struct GpuCompositor {
    layers: Vec<Layer>,
    frame_count: std::sync::atomic::AtomicU64,
}

impl GpuCompositor {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            frame_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn push_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn clear_layers(&mut self) {
        self.layers.clear();
    }

    pub fn compose(&mut self, output: &mut RenderFrame) -> VgmResult<()> {
        self.layers.sort_by_key(|l| l.order);
        self.frame_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        for layer in &self.layers {
            let dst_size = output.framebuffer.len();
            let src_size = layer.data.len();
            let copy_len = dst_size.min(src_size);
            if copy_len > 0 {
                output.framebuffer[..copy_len].copy_from_slice(&layer.data[..copy_len]);
            }
        }
        Ok(())
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for GpuCompositor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_compositor() {
        let comp = GpuCompositor::new();
        assert_eq!(comp.layer_count(), 0);
    }

    #[test]
    fn test_push_and_clear_layers() {
        let mut comp = GpuCompositor::new();
        let layer = Layer {
            order: 0,
            blend: BlendMode::Over,
            x: 0, y: 0,
            width: 10, height: 10,
            data: vec![0; 400],
        };
        comp.push_layer(layer);
        assert_eq!(comp.layer_count(), 1);
        comp.clear_layers();
        assert_eq!(comp.layer_count(), 0);
    }

    #[test]
    fn test_compose() {
        let mut comp = GpuCompositor::new();
        let mut frame = RenderFrame::new(10, 10, 0);
        let layer = Layer {
            order: 0,
            blend: BlendMode::Over,
            x: 0, y: 0,
            width: 10, height: 10,
            data: vec![0xFF; 400],
        };
        comp.push_layer(layer);
        assert!(comp.compose(&mut frame).is_ok());
        assert_eq!(frame.framebuffer[0], 0xFF);
    }

    #[test]
    fn test_frame_count_increments() {
        let mut comp = GpuCompositor::new();
        let mut frame = RenderFrame::new(1, 1, 0);
        comp.compose(&mut frame).unwrap();
        comp.compose(&mut frame).unwrap();
        assert_eq!(comp.frame_count(), 2);
    }
}
