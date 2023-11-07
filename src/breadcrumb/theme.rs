use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct BreadcrumbTheme {
    pub item_font_color: String,
    pub item_font_color_hover: String,
    pub item_background_color_hover: String,
}

impl ThemeMethod for BreadcrumbTheme {
    fn light() -> Self {
        Self {
            item_font_color: "#767c82".into(),
            item_font_color_hover: "#333639".into(),
            item_background_color_hover: "#2e333817".into(),
        }
    }

    fn dark() -> Self {
        Self {
            item_font_color: "#ffffff85".into(),
            item_font_color_hover: "#ffffffd1".into(),
            item_background_color_hover: "#ffffff1f".into(),
        }
    }
}
