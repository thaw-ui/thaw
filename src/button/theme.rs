use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct ButtonTheme {
    pub color_text_hover: String,
    pub color_text_active: String,
    pub color_text_disabled: String,
    pub color_background_disabled: String,
    pub color_border_disabled: String
}

impl ThemeMethod for ButtonTheme {
    fn light() -> Self {
        Self {
            color_text_hover: "#f1f3f5".into(),
            color_text_active: "#eceef0".into(),
            color_text_disabled: "#737373".into(),
            color_background_disabled: "#ceced1".into(),
            color_border_disabled: "#909090".into(),
        }
    }

    fn dark() -> Self {
        Self {
            color_text_hover: "#ffffff1a".into(),
            color_text_active: "#ffffff26".into(),
            color_text_disabled: "#ceced1".into(),
            color_background_disabled: "#737373".into(),
            color_border_disabled: "#ceced1".into(),
        }
    }
}
