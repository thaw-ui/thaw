use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct InputTheme {
    pub font_color: String,
    pub placeholder_color: String,
    pub border_color: String,
    pub background_color: String,
}

impl ThemeMethod for InputTheme {
    fn light() -> Self {
        Self {
            font_color: "#333639".into(),
            placeholder_color: "#c2c2c2".into(),
            border_color: "#e0e0e6".into(),
            background_color: "#fff".into(),
        }
    }

    fn dark() -> Self {
        Self {
            font_color: "#ffffffd1".into(),
            placeholder_color: "#c2c2c2".into(),
            border_color: "#0000".into(),
            background_color: "#ffffff1a".into(),
        }
    }
}
