use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct SliderTheme {
    pub background_color: String,
}

impl ThemeMethod for SliderTheme {
    fn light() -> Self {
        Self {
            background_color: "#dbdbdf".into(),
        }
    }

    fn dark() -> Self {
        Self {
            background_color: "#ffffff33".into(),
        }
    }
}
