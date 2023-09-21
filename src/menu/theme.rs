use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct MenuTheme {
    pub color: String,
    pub item_color_hover: String,
}

impl ThemeMethod for MenuTheme {
    fn light() -> Self {
        Self {
            color: "#333639".into(),
            item_color_hover: "#f3f5f6".into(),
        }
    }

    fn dark() -> Self {
        Self {
            color: "#333639".into(),
            item_color_hover: "#f3f5f6".into(),
        }
    }
}
