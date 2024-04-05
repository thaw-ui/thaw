use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct ScrollbarTheme {
    pub background_color: String,
    pub background_color_hover: String,
}

impl ThemeMethod for ScrollbarTheme {
    fn light() -> Self {
        Self {
            background_color: "#00000040".into(),
            background_color_hover: "#00000066".into(),
        }
    }

    fn dark() -> Self {
        Self {
            background_color: "#ffffff33".into(),
            background_color_hover: "#ffffff4d".into(),
        }
    }
}
