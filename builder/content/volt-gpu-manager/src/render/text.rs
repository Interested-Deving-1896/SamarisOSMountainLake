use crate::core::result::VgmResult;

#[derive(Debug, Clone)]
pub struct GlyphInfo {
    pub char_code: char,
    pub width: u32,
    pub height: u32,
    pub bearing_x: i32,
    pub bearing_y: i32,
    pub advance: f32,
    pub bitmap: Vec<u8>,
}

pub struct GpuTextPipeline {
    glyph_cache: std::collections::HashMap<char, GlyphInfo>,
    font_size: f32,
}

impl GpuTextPipeline {
    pub fn new() -> Self {
        Self {
            glyph_cache: std::collections::HashMap::new(),
            font_size: 14.0,
        }
    }

    pub fn with_font_size(size: f32) -> Self {
        Self {
            glyph_cache: std::collections::HashMap::new(),
            font_size: size,
        }
    }

    pub fn cache_glyph(&mut self, glyph: GlyphInfo) {
        self.glyph_cache.insert(glyph.char_code, glyph);
    }

    pub fn get_glyph(&self, c: char) -> Option<&GlyphInfo> {
        self.glyph_cache.get(&c)
    }

    pub fn render_text(&self, text: &str) -> VgmResult<Vec<u8>> {
        if text.is_empty() {
            return Ok(Vec::new());
        }
        let line_height = self.font_size.ceil() as u32;
        let width = (text.len() as f32 * self.font_size * 0.6).ceil() as u32;
        let height = line_height;
        let mut output = vec![0u8; (width * height * 4) as usize];
        for (_i, _ch) in text.chars().enumerate() {
            if let Some(_glyph) = self.glyph_cache.get(&_ch) {
            } else {
                let x = (_i as u32 * (self.font_size * 0.6).ceil() as u32) * 4;
                let base = y_offset(width, line_height, line_height - 1);
                if base + x as usize + 3 < output.len() {
                    output[base + x as usize] = 255;
                }
            }
        }
        Ok(output)
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn cached_glyph_count(&self) -> usize {
        self.glyph_cache.len()
    }
}

fn y_offset(width: u32, _height: u32, y: u32) -> usize {
    (y * width * 4) as usize
}

impl Default for GpuTextPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_glyph(c: char) -> GlyphInfo {
        GlyphInfo {
            char_code: c,
            width: 8,
            height: 12,
            bearing_x: 0,
            bearing_y: 0,
            advance: 8.0,
            bitmap: vec![0u8; 8 * 12],
        }
    }

    #[test]
    fn test_new_pipeline() {
        let pipe = GpuTextPipeline::new();
        assert_eq!(pipe.font_size(), 14.0);
        assert_eq!(pipe.cached_glyph_count(), 0);
    }

    #[test]
    fn test_cache_and_get_glyph() {
        let mut pipe = GpuTextPipeline::new();
        let glyph = sample_glyph('A');
        pipe.cache_glyph(glyph);
        assert!(pipe.get_glyph('A').is_some());
        assert!(pipe.get_glyph('B').is_none());
    }

    #[test]
    fn test_render_empty_text() {
        let pipe = GpuTextPipeline::new();
        let result = pipe.render_text("").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_render_non_empty_text() {
        let pipe = GpuTextPipeline::with_font_size(16.0);
        let result = pipe.render_text("Hello").unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_custom_font_size() {
        let pipe = GpuTextPipeline::with_font_size(24.0);
        assert_eq!(pipe.font_size(), 24.0);
    }
}
