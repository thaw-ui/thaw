use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct SwitchTheme {
    pub background_color: String,
}

impl ThemeMethod for SwitchTheme {
    fn light() -> Self {
        Self {
            background_color: "#00000024".into(),
        }
    }

    fn dark() -> Self {
        Self {
            background_color: "#ffffff33".into(),
        }
    }
}
