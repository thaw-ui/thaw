use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct ButtonTheme {
    pub border_color_solid: String,
    pub color_text_hover: String,
    pub color_text_active: String,
    pub color_text_disabled: String,
    pub color_background_disabled: String,
    pub color_border_disabled: String,
}

impl ThemeMethod for ButtonTheme {
    fn light() -> Self {
        Self {
            border_color_solid: "#e0e0e6".into(),
            color_text_hover: "#f1f3f5".into(),
            color_text_active: "#eceef0".into(),
            color_text_disabled: "#00000040".into(),
            color_background_disabled: "#0000000a".into(),
            color_border_disabled: "#d9d9d9".into(),
        }
    }

    fn dark() -> Self {
        Self {
            border_color_solid: "#ffffff3d".into(),
            color_text_hover: "#ffffff1a".into(),
            color_text_active: "#ffffff26".into(),
            color_text_disabled: "#4c5155".into(),
            color_background_disabled: "#2b2f31".into(),
            color_border_disabled: "#2b2f31".into(),
        }
    }
}
