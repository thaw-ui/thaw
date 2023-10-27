use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct MessageTheme {
    pub background_color: String,
}

impl ThemeMethod for MessageTheme {
    fn light() -> Self {
        Self {
            background_color: "#fff".into(),
        }
    }

    fn dark() -> Self {
        Self {
            background_color: "#48484e".into(),
        }
    }
}
