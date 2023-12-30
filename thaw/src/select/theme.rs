use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct SelectTheme {
    pub font_color: String,
    pub border_color: String,
    pub background_color: String,
    pub menu_background_color: String,
    pub menu_background_color_hover: String,
}

impl ThemeMethod for SelectTheme {
    fn light() -> Self {
        Self {
            font_color: "#333639".into(),
            border_color: "#e0e0e6".into(),
            background_color: "#fff".into(),
            menu_background_color: "#fff".into(),
            menu_background_color_hover: "#f3f5f6".into(),
        }
    }

    fn dark() -> Self {
        Self {
            font_color: "#ffffffd1".into(),
            border_color: "#0000".into(),
            background_color: "#ffffff1a".into(),
            menu_background_color: "#48484e".into(),
            menu_background_color_hover: "#ffffff17".into(),
        }
    }
}
