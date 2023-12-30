use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct TableTheme {
    pub background_color: String,
    pub background_color_striped: String,
    pub border_color: String,
}

impl ThemeMethod for TableTheme {
    fn light() -> Self {
        Self {
            background_color: "#fff".into(),
            background_color_striped: "#fafafc".into(),
            border_color: "#efeff5".into(),
        }
    }

    fn dark() -> Self {
        Self {
            background_color: "#18181c".into(),
            background_color_striped: "#26262a".into(),
            border_color: "#2d2d30".into(),
        }
    }
}
