use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct NavBarTheme {
    pub background_color: String,
}

impl ThemeMethod for NavBarTheme {
    fn light() -> Self {
        Self {
            background_color: "#fff".into(),
        }
    }

    fn dark() -> Self {
        Self {
            background_color: "#323233".into(),
        }
    }
}
