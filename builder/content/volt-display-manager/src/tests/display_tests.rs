#[cfg(test)]
mod tests {
    use volt_display_manager::display::backends::xrandr;

    #[test]
    fn test_xrandr_parse_single_monitor() {
        let output = r#"
Screen 0: minimum 8 x 8, current 1920 x 1080, maximum 32767 x 32767
HDMI-1 connected primary 1920x1080+0+0 (normal left inverted right x axis y axis) 527mm x 296mm
   1920x1080     60.00*+  59.94    50.00    29.97
   1680x1050     59.88
   1280x1024     75.02    60.02
   1280x720      60.00    59.94
   1024x768      75.03    70.07    60.00
DP-1 disconnected (normal left inverted right x axis y axis)
"#;
        let screens = xrandr::query_xrandr_from_str(output).unwrap_or_else(|_| {
            // Test the internal parser
            super::super::backends::xrandr::parse_for_test(output).unwrap()
        });
        assert!(screens.len() >= 1);
    }

    #[test]
    fn test_xrandr_parse_4k_monitor() {
        let output = r#"
Screen 0: minimum 8 x 8, current 3840 x 2160, maximum 32767 x 32767
DP-1 connected primary 3840x2160+0+0 (normal left inverted right x axis y axis) 600mm x 340mm
   3840x2160     60.00*+  30.00    29.97
   2560x1440     60.00
   1920x1080     60.00    59.94
HDMI-1 disconnected (normal left inverted right x axis y axis)
"#;
        let screens = xrandr::query_xrandr_from_str(output).unwrap_or_else(|_| {
            super::super::backends::xrandr::parse_for_test(output).unwrap()
        });
        assert!(screens.len() >= 1);
    }

    #[test]
    fn test_empty_output_is_error() {
        let output = "Screen 0: minimum 8 x 8, current 8 x 8, maximum 32767 x 32767\n";
        // This should fail because no connected screens
        assert!(xrandr::query_xrandr_from_str(output).is_err());
    }
}

// Re-export for tests
pub mod backends {
    pub mod xrandr {
        pub use volt_display_manager::display::backends::xrandr::*;
    }
}
