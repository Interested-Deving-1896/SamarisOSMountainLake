#[derive(Debug, Clone, Copy)]
pub struct RgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RgbaColor {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_u32(self) -> u32 {
        u32::from_le_bytes([self.r, self.g, self.b, self.a])
    }

    pub fn from_u32(v: u32) -> Self {
        let [r, g, b, a] = v.to_le_bytes();
        Self { r, g, b, a }
    }

    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub z_index: i32,
    pub opacity: f32,
    pub command: GpuCommand,
}

#[derive(Debug, Clone)]
pub enum GpuCommand {
    RenderRect {
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        border_radius: f32,
        shadow_blur: f32,
        shadow_offset_x: f32,
        shadow_offset_y: f32,
        fill_color: RgbaColor,
        border_color: RgbaColor,
        border_width: f32,
    },
    Clear {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    },
    Compose {
        layers: Vec<Layer>,
    },
}

impl GpuCommand {
    pub fn render_rect(
        x: i32, y: i32, w: u32, h: u32,
        fill_r: u8, fill_g: u8, fill_b: u8, fill_a: u8,
    ) -> Self {
        Self::RenderRect {
            x, y, w, h,
            border_radius: 0.0,
            shadow_blur: 0.0,
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            fill_color: RgbaColor::new(fill_r, fill_g, fill_b, fill_a),
            border_color: RgbaColor::BLACK,
            border_width: 0.0,
        }
    }
}
