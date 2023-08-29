use super::ThemeMethod;

#[derive(Clone)]
pub struct CommonTheme {
    pub font_family: String,

    pub color_primary: String,
    pub color_success: String,
    pub color_warning: String,
    pub color_error: String,

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
            font_family: "-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,'Helvetica Neue',Arial,'Noto Sans',sans-serif,'Apple Color Emoji','Segoe UI Emoji','Segoe UI Symbol','Noto Color Emoji'".into(),
            
            color_primary: "".into(),
            color_success: "".into(),
            color_warning: "".into(),
            color_error: "".into(),

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
            color_primary: "#f5222d".into(),
            color_success: "#18a058".into(),
            color_warning: "#f0a020".into(),
            color_error: "#d03050".into(),
            ..CommonTheme::common()
        }
    }
    fn dark() -> Self {
        Self {
            color_primary: "#d32029".into(),
            color_success: "#18a058".into(),
            color_warning: "#f0a020".into(),
            color_error: "#d03050".into(),
            ..CommonTheme::common()
        }
    }
}
