use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct SkeletionTheme {
    pub background_color_start: String,
    pub background_color_end: String,
}

impl ThemeMethod for SkeletionTheme {
    fn light() -> Self {
        Self {
            background_color_start: "#f2f2f2".into(),
            background_color_end: "#e6e6e6".into(),
        }
    }

    fn dark() -> Self {
        Self {
            background_color_start: "rgba(255, 255, 255, 0.12)".into(),
            background_color_end: "rgba(255, 255, 255, 0.18)".into(),
        }
    }
}
