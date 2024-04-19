use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct AnchorTheme {
    pub rail_background_color: String,
}

impl ThemeMethod for AnchorTheme {
    fn light() -> Self {
        Self {
            rail_background_color: "#dbdbdf".into(),
        }
    }

    fn dark() -> Self {
        Self {
            rail_background_color: "#ffffff33".into(),
        }
    }
}
