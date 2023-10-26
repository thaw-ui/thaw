use super::ThemeMethod;

#[derive(Clone)]
pub struct CommonTheme {
    pub font_family: String,
    pub font_color: String,
    pub background_color: String,
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

    pub font_size: String,
    pub font_size_small: String,
    pub font_size_medium: String,
    pub font_size_large: String,
    pub font_size_huge: String,

    pub line_height: String,
    pub line_height_small: String,
    pub line_height_medium: String,
    pub line_height_large: String,
    pub line_height_huge: String,

    pub border_radius: String,
    pub border_radius_small: String,
    pub border_radius_medium: String,
    pub border_radius_large: String,
}

impl CommonTheme {
    fn common() -> Self {
        Self {
            font_family: r#"Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Helvetica Neue, Arial, Noto Sans, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", Segoe UI Symbol, "Noto Color Emoji""#.into(),
            font_color: "".into(),
            background_color: "".into(),
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
            font_size: "14px".into(),
            font_size_small: "12px".into(),
            font_size_medium: "16px".into(),
            font_size_large: "20px".into(),
            font_size_huge: "24px".into(),
            line_height: "22px".into(),
            line_height_small: "20px".into(),
            line_height_medium: "24px".into(),
            line_height_large: "28px".into(),
            line_height_huge: "32px".into(),
            border_radius: "3px".into(),
            border_radius_small: "2px".into(),
            border_radius_medium: "4px".into(),
            border_radius_large: "8px".into(),
        }
    }
}

impl ThemeMethod for CommonTheme {
    fn light() -> Self {
        Self {
            font_color: "#11181c".into(),
            background_color: "#fff".into(),
            color_scheme: "light".into(),
            color_primary: "#f5222d".into(),
            color_primary_hover: "#ff4d4f".into(),
            color_primary_active: "#cf1322".into(),
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
            font_color: "#ecedee".into(),
            background_color: "#1a1d1e".into(),
            color_scheme: "dark".into(),
            color_primary: "#d32029".into(),
            color_primary_hover: "#e04648".into(),
            color_primary_active: "#ad111e".into(),
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
