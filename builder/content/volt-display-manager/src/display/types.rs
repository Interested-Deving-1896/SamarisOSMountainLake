use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Complete display configuration — the single source of truth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub primary: String,
    pub timestamp: u64,
    pub backend: DisplayBackend,
    pub session: DisplaySession,
    pub layout_mode: LayoutMode,
    pub safe_mode: bool,
    pub screens: Vec<Screen>,
    pub profile: DisplayProfile,
}

/// A single connected display output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screen {
    pub name: String,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub width: u32,
    pub height: u32,
    pub native_width: u32,
    pub native_height: u32,
    pub refresh_rate: f32,
    #[serde(default)]
    pub available_refresh_rates: Vec<f32>,
    #[serde(default)]
    pub available_modes: Vec<(u32, u32)>,
    pub scale_factor: f32,
    pub rotation: Rotation,
    pub connected: bool,
    pub primary: bool,
    pub x: i32,
    pub y: i32,
    pub dpi_class: DpiClass,
}

/// Frontend-facing profile — consumed by scaleEngine, CSS tokens, UI layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayProfile {
    pub scale_factor: f32,
    pub dpi_class: DpiClass,
    pub recommended_ui_density: f32,
    pub recommended_font_scale: f32,
    pub recommended_spacing_scale: f32,
    pub primary_logical_width: u32,
    pub primary_logical_height: u32,
    pub safe_mode: bool,
}

impl Default for DisplayProfile {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            dpi_class: DpiClass::Standard,
            recommended_ui_density: 1.0,
            recommended_font_scale: 1.0,
            recommended_spacing_scale: 1.0,
            primary_logical_width: 1920,
            primary_logical_height: 1080,
            safe_mode: false,
        }
    }
}

/// Persistent user overrides.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserDisplayConfig {
    #[serde(default)]
    pub preferences: UserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPreferences {
    pub primary: Option<String>,
    pub layout_mode: Option<LayoutMode>,
    #[serde(default)]
    pub screens: std::collections::HashMap<String, UserScreenPref>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserScreenPref {
    pub scale_factor: Option<f32>,
    pub rotation: Option<Rotation>,
    pub resolution_override: Option<(u32, u32)>,
    pub refresh_rate_override: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rotation {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "inverted")]
    UpsideDown,
}

impl Rotation {
    pub fn xrandr_arg(&self) -> &str {
        match self {
            Rotation::Normal => "normal",
            Rotation::Left => "left",
            Rotation::Right => "right",
            Rotation::UpsideDown => "inverted",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplayBackend {
    #[serde(rename = "xrandr")]
    Xrandr,
    #[serde(rename = "drm")]
    Drm,
    #[serde(rename = "wayland")]
    Wayland,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplaySession {
    #[serde(rename = "xorg")]
    Xorg,
    #[serde(rename = "wayland")]
    Wayland,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "mirror")]
    Mirror,
    #[serde(rename = "extend")]
    Extend,
    #[serde(rename = "single")]
    Single,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DpiClass {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "hidpi")]
    Hidpi,
    #[serde(rename = "retina")]
    Retina,
}

#[derive(Debug, Error)]
pub enum DisplayError {
    #[error("X11 session unavailable: {0}")]
    SessionUnavailable(String),

    #[error("xrandr binary not found in PATH")]
    XrandrNotFound,

    #[error("Permission denied accessing display: {0}")]
    PermissionDenied(String),

    #[error("Failed to parse xrandr output: {0}")]
    ParseError(String),

    #[error("Failed to apply display config: {0}")]
    ApplyFailed(String),

    #[error("Validation failed — {0}")]
    ValidationFailed(String),

    #[error("Rollback failed: {0}")]
    RollbackFailed(String),

    #[error("Config read failed: {0}")]
    ConfigReadFailed(String),

    #[error("Config write failed: {0}")]
    ConfigWriteFailed(String),

    #[error("Hotplug watch failed: {0}")]
    HotplugWatchFailed(String),

    #[error("No screens detected")]
    NoScreensDetected,

    #[error("Safe mode active — running with fallback configuration")]
    SafeModeActive,
}
