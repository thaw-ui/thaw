use super::ThemeMethod;
use thaw_macro::WriteCSSVars;

#[derive(Clone, WriteCSSVars)]
pub struct CommonTheme {
    pub font_family_base: String,
    pub border_color: String,
    pub color_scheme: String,

    pub color_primary: String,
    pub color_primary_hover: String,
    pub color_primary_active: String,
    pub color_success: String,
    pub color_success_hover: String,
    pub color_success_active: String,
    pub color_warning: String,
    pub color_warning_hover: String,
    pub color_warning_active: String,
    pub color_error: String,
    pub color_error_hover: String,
    pub color_error_active: String,

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

    pub font_weight_regular: String,
    pub font_weight_semibold: String,
    pub font_weight_bold: String,

    pub stroke_width_thin: String,

    pub border_radius_none: String,
    pub border_radius_medium: String,
    pub border_radius_circular: String,

    pub spacing_horizontal_x_x_s: String,
    pub spacing_horizontal_s_nudge: String,
    pub spacing_horizontal_s: String,
    pub spacing_horizontal_m_nudge: String,
    pub spacing_horizontal_m: String,
    pub spacing_horizontal_l: String,

    pub duration_ultra_fast: String,
    pub duration_faster: String,
    pub duration_normal: String,
    pub curve_accelerate_mid: String,
    pub curve_decelerate_mid: String,
    pub curve_easy_ease: String,

    pub height_tiny: String,
    pub height_small: String,
    pub height_medium: String,
    pub height_large: String,

    pub border_radius: String,
    pub border_radius_small: String,
    pub border_radius_large: String,
}

impl CommonTheme {
    fn common() -> Self {
        Self {
            font_family_base: "'Segoe UI', 'Segoe UI Web (West European)', -apple-system, BlinkMacSystemFont, Roboto, 'Helvetica Neue', sans-serif".into(),
            border_color: "".into(),
            color_scheme: "".into(),
            color_primary: "".into(),
            color_primary_hover: "".into(),
            color_primary_active: "".into(),
            color_success: "".into(),
            color_success_hover: "".into(),
            color_success_active: "".into(),
            color_warning: "".into(),
            color_warning_hover: "".into(),
            color_warning_active: "".into(),
            color_error: "".into(),
            color_error_hover: "".into(),
            color_error_active: "".into(),

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

            font_weight_regular: "400".into(),
            font_weight_semibold: "600".into(),
            font_weight_bold: "700".into(),

            stroke_width_thin: "1px".into(),

            border_radius_none: "0".into(),
            border_radius_medium: "4px".into(),
            border_radius_circular: "10000px".into(),

            spacing_horizontal_x_x_s: "2px".into(),
            spacing_horizontal_s_nudge: "6px".into(),
            spacing_horizontal_s: "8px".into(),
            spacing_horizontal_m_nudge: "10px".into(),
            spacing_horizontal_m: "12px".into(),
            spacing_horizontal_l: "16px".into(),

            duration_ultra_fast: "50ms".into(),
            duration_faster: "100ms".into(),
            duration_normal: "200ms".into(),
            curve_accelerate_mid: "cubic-bezier(1,0,1,1)".into(),
            curve_decelerate_mid: "cubic-bezier(0,0,0,1)".into(),
            curve_easy_ease: "cubic-bezier(0.33,0,0.67,1)".into(),

            height_tiny: "22px".into(),
            height_small: "28px".into(),
            height_medium: "34px".into(),
            height_large: "40px".into(),

            border_radius: "3px".into(),
            border_radius_small: "2px".into(),
            border_radius_large: "8px".into(),
        }
    }
}

impl ThemeMethod for CommonTheme {
    fn light() -> Self {
        Self {
            color_scheme: "light".into(),
            color_primary: "#0078ff".into(),
            color_primary_hover: "#2994ff".into(),
            color_primary_active: "#005ed9".into(),
            color_success: "#18a058".into(),
            color_success_hover: "#36ad6a".into(),
            color_success_active: "#0c7a43".into(),
            color_warning: "#f0a020".into(),
            color_warning_hover: "#fcb040".into(),
            color_warning_active: "#c97c10".into(),
            color_error: "#d03050".into(),
            color_error_hover: "#de576d".into(),
            color_error_active: "#ab1f3f".into(),
            border_color: "#e5e8eb".into(),

            ..CommonTheme::common()
        }
    }
    fn dark() -> Self {
        Self {
            color_scheme: "dark".into(),
            color_primary: "#0078ff".into(),
            color_primary_hover: "#2994ff".into(),
            color_primary_active: "#005ed9".into(),
            // color_success: "#63e2b7".into(),
            // color_success_hover: "#7fe7c4".into(),
            color_success: "#18a058".into(),
            color_success_hover: "#36ad6a".into(),
            color_success_active: "#5acea7".into(),
            color_warning: "#f0a020".into(),
            color_warning_hover: "#fcb040".into(),
            color_warning_active: "#e6c260".into(),
            color_error: "#d03050".into(),
            color_error_hover: "#de576d".into(),
            color_error_active: "#e57272".into(),
            border_color: "#1f2537".into(),
            ..CommonTheme::common()
        }
    }
}
