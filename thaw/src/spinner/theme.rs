use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct SpinnerTheme {
    pub background_color: String,
}

impl ThemeMethod for SpinnerTheme {
    fn light() -> Self {
        Self {
            background_color: "#0000000a".into(),
        }
    }

    fn dark() -> Self {
        Self {
            background_color: "#2b2f31".into(),
        }
    }
}
