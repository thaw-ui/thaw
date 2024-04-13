use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct PopoverTheme {
    pub background_color: String,
    pub tooltip_background_color: String,
}

impl ThemeMethod for PopoverTheme {
    fn light() -> Self {
        Self {
            background_color: "#fff".into(),
            tooltip_background_color: "#262626".into(),
        }
    }

    fn dark() -> Self {
        Self {
            background_color: "#48484e".into(),
            tooltip_background_color: "#48484e".into(),
        }
    }
}
