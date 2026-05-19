use crate::core::result::VgmResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiElementKind {
    Rect,
    RoundedRect,
    Circle,
    Text,
    Image,
    Gradient,
}

#[derive(Debug, Clone)]
pub struct UiElement {
    pub kind: UiElementKind,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: [u8; 4],
    pub border_radius: u32,
    pub z_order: u32,
}

pub struct GpuUiPipeline {
    elements: Vec<UiElement>,
    pipeline_count: std::sync::atomic::AtomicU64,
}

impl GpuUiPipeline {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            pipeline_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn add_element(&mut self, element: UiElement) {
        self.elements.push(element);
    }

    pub fn remove_element(&mut self, index: usize) {
        if index < self.elements.len() {
            self.elements.remove(index);
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn bake(&self) -> VgmResult<Vec<u8>> {
        let total_pixels: u64 = self.elements.iter().map(|e| e.width as u64 * e.height as u64).sum();
        let buffer_size = (total_pixels * 4).max(4) as usize;
        let mut buffer = vec![0u8; buffer_size];
        for element in &self.elements {
            let pixel_index = (element.y.abs() as u64 * element.width as u64 * 4) as usize;
            if pixel_index + 4 <= buffer.len() {
                buffer[pixel_index..pixel_index + 4].copy_from_slice(&element.color);
            }
        }
        self.pipeline_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(buffer)
    }

    pub fn element_count(&self) -> usize {
        self.elements.len()
    }

    pub fn pipeline_count(&self) -> u64 {
        self.pipeline_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for GpuUiPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn button() -> UiElement {
        UiElement {
            kind: UiElementKind::RoundedRect,
            x: 10, y: 10,
            width: 100, height: 30,
            color: [0x33, 0x66, 0xFF, 0xFF],
            border_radius: 4,
            z_order: 1,
        }
    }

    #[test]
    fn test_new_pipeline() {
        let pipe = GpuUiPipeline::new();
        assert_eq!(pipe.element_count(), 0);
    }

    #[test]
    fn test_add_clear() {
        let mut pipe = GpuUiPipeline::new();
        pipe.add_element(button());
        assert_eq!(pipe.element_count(), 1);
        pipe.clear();
        assert_eq!(pipe.element_count(), 0);
    }

    #[test]
    fn test_bake_returns_buffer() {
        let mut pipe = GpuUiPipeline::new();
        pipe.add_element(button());
        let buffer = pipe.bake().unwrap();
        assert!(!buffer.is_empty());
    }

    #[test]
    fn test_remove_element() {
        let mut pipe = GpuUiPipeline::new();
        pipe.add_element(button());
        pipe.add_element(UiElement {
            kind: UiElementKind::Text,
            x: 0, y: 0,
            width: 10, height: 10,
            color: [255; 4],
            border_radius: 0,
            z_order: 0,
        });
        pipe.remove_element(0);
        assert_eq!(pipe.element_count(), 1);
    }

    #[test]
    fn test_pipeline_count() {
        let mut pipe = GpuUiPipeline::new();
        pipe.add_element(button());
        pipe.bake().unwrap();
        assert_eq!(pipe.pipeline_count(), 1);
    }
}
