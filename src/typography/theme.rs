use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct TypographyTheme {
    pub code_background_color: String,
}

impl ThemeMethod for TypographyTheme {
    fn light() -> Self {
        Self {
            code_background_color: "#f4f4f8".into(),
        }
    }

    fn dark() -> Self {
        Self {
            code_background_color: "#ffffff1f".into(),
        }
    }
}
