use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct MenuTheme {
    pub color: String,
    pub item_color_hover: String,
    pub group_color: String,
}

impl ThemeMethod for MenuTheme {
    fn light() -> Self {
        Self {
            color: "#4b5263".into(),
            item_color_hover: "#f3f5f6".into(),
            group_color: "#111727".into(),
        }
    }

    fn dark() -> Self {
        Self {
            color: "#4b5263".into(),
            item_color_hover: "#f3f5f6".into(),
            group_color: "#111727".into(),
        }
    }
}
