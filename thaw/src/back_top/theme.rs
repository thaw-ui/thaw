use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct BackTopTheme {
    pub background_color: String,
    
}

impl ThemeMethod for BackTopTheme {
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
