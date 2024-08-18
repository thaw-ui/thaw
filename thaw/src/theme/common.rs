use thaw_macro::WriteCSSVars;

#[derive(Clone, WriteCSSVars)]
pub struct CommonTheme {
    pub font_family_base: String,
    pub font_family_monospace: String,
    pub font_family_numeric: String,

    pub font_size_base_100: String,
    pub font_size_base_200: String,
    pub font_size_base_300: String,
    pub font_size_base_400: String,
    pub font_size_base_500: String,
    pub font_size_base_600: String,
    pub font_size_base_700: String,
    pub font_size_base_800: String,
    pub font_size_base_900: String,
    pub font_size_base_1000: String,

    pub line_height_base_200: String,
    pub line_height_base_300: String,
    pub line_height_base_400: String,
    pub line_height_base_500: String,

    pub font_weight_regular: String,
    pub font_weight_semibold: String,
    pub font_weight_bold: String,

    pub stroke_width_thin: String,
    pub stroke_width_thick: String,
    pub stroke_width_thicker: String,
    pub stroke_width_thickest: String,

    pub border_radius_none: String,
    pub border_radius_small: String,
    pub border_radius_medium: String,
    pub border_radius_large: String,
    pub border_radius_x_large: String,
    pub border_radius_circular: String,

    pub spacing_horizontal_x_x_s: String,
    pub spacing_horizontal_x_s: String,
    pub spacing_horizontal_s_nudge: String,
    pub spacing_horizontal_s: String,
    pub spacing_horizontal_m_nudge: String,
    pub spacing_horizontal_m: String,
    pub spacing_horizontal_l: String,
    pub spacing_horizontal_x_x_l: String,
    pub spacing_vertical_none: String,
    pub spacing_vertical_x_s: String,
    pub spacing_vertical_s_nudge: String,
    pub spacing_vertical_s: String,
    pub spacing_vertical_m_nudge: String,
    pub spacing_vertical_m: String,
    pub spacing_vertical_l: String,
    pub spacing_vertical_x_x_l: String,

    pub duration_ultra_fast: String,
    pub duration_faster: String,
    pub duration_normal: String,
    pub duration_slow: String,
    pub curve_accelerate_mid: String,
    pub curve_decelerate_max: String,
    pub curve_decelerate_mid: String,
    pub curve_easy_ease: String,
}

impl CommonTheme {
    pub fn new() -> Self {
        Self {
            font_family_base: "'Segoe UI', 'Segoe UI Web (West European)', ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Roboto, 'Helvetica Neue', sans-serif".into(),
            font_family_monospace: "Consolas, ui-monospace, 'Courier New', Courier, monospace".into(),
            font_family_numeric: "Bahnschrift, 'Segoe UI', 'Segoe UI Web (West European)', ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Roboto, 'Helvetica Neue', sans-serif".into(),

            font_size_base_100: "10px".into(),
            font_size_base_200: "12px".into(),
            font_size_base_300: "14px".into(),
            font_size_base_400: "16px".into(),
            font_size_base_500: "20px".into(),
            font_size_base_600: "24px".into(),
            font_size_base_700: "28px".into(),
            font_size_base_800: "32px".into(),
            font_size_base_900: "40px".into(),
            font_size_base_1000: "60px".into(),

            line_height_base_200: "16px".into(),
            line_height_base_300: "20px".into(),
            line_height_base_400: "22px".into(),
            line_height_base_500: "28px".into(),

            font_weight_regular: "400".into(),
            font_weight_semibold: "600".into(),
            font_weight_bold: "700".into(),

            stroke_width_thin: "1px".into(),
            stroke_width_thick: "2px".into(),
            stroke_width_thicker: "3px".into(),
            stroke_width_thickest: "4px".into(),

            border_radius_none: "0".into(),
            border_radius_small: "2px".into(),
            border_radius_medium: "4px".into(),
            border_radius_large: "6px".into(),
            border_radius_x_large: "8px".into(),
            border_radius_circular: "10000px".into(),

            spacing_horizontal_x_x_s: "2px".into(),
            spacing_horizontal_x_s: "4px".into(),
            spacing_horizontal_s_nudge: "6px".into(),
            spacing_horizontal_s: "8px".into(),
            spacing_horizontal_m_nudge: "10px".into(),
            spacing_horizontal_m: "12px".into(),
            spacing_horizontal_l: "16px".into(),
            spacing_horizontal_x_x_l: "24px".into(),
            spacing_vertical_none: "0".into(),
            spacing_vertical_x_s: "4px".into(),
            spacing_vertical_s_nudge: "6px".into(),
            spacing_vertical_s: "8px".into(),
            spacing_vertical_m_nudge: "10px".into(),
            spacing_vertical_m: "12px".into(),
            spacing_vertical_l: "16px".into(),
            spacing_vertical_x_x_l: "24px".into(),

            duration_ultra_fast: "50ms".into(),
            duration_faster: "100ms".into(),
            duration_normal: "200ms".into(),
            duration_slow: "300ms".into(),
            curve_accelerate_mid: "cubic-bezier(1,0,1,1)".into(),
            curve_decelerate_max: "cubic-bezier(0.1,0.9,0.2,1)".into(),
            curve_decelerate_mid: "cubic-bezier(0,0,0,1)".into(),
            curve_easy_ease: "cubic-bezier(0.33,0,0.67,1)".into(),
        }
    }
}
