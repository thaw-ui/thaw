use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct ProgressTheme {
    pub background_color: String,
}

impl ThemeMethod for ProgressTheme {
    fn light() -> Self {
        Self {
            background_color: "#ebebeb".into(),
        }
    }

    fn dark() -> Self {
        Self {
            background_color: "#ffffff1f".into(),
        }
    }
}
