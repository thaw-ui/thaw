use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct ButtonTheme {
    pub color_text_hover: String,
    pub color_text_active: String,
}

impl ThemeMethod for ButtonTheme {
    fn light() -> Self {
        Self {
            color_text_hover: "#f1f3f5".into(),
            color_text_active: "#eceef0".into(),
        }
    }

    fn dark() -> Self {
        Self {
            color_text_hover: "#ffffff1a".into(),
            color_text_active: "#ffffff26".into(),
        }
    }
}
