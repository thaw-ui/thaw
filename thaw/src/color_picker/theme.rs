use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct ColorPickerTheme {
    pub popover_background_color: String,
}

impl ThemeMethod for ColorPickerTheme {
    fn light() -> Self {
        Self {
            popover_background_color: "#fff".into(),
        }
    }

    fn dark() -> Self {
        Self {
            popover_background_color: "#48484e".into(),
        }
    }
}
