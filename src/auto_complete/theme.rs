use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct AutoCompleteTheme {
    pub menu_background_color: String,
    pub menu_background_color_hover: String,
}

impl ThemeMethod for AutoCompleteTheme {
    fn light() -> Self {
        Self {
            menu_background_color: "#fff".into(),
            menu_background_color_hover: "#f3f5f6".into(),
        }
    }

    fn dark() -> Self {
        Self {
            menu_background_color: "#48484e".into(),
            menu_background_color_hover: "#ffffff17".into(),
        }
    }
}
