use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct TimePickerTheme {
    pub panel_background_color: String,
    pub panel_time_item_background_color_hover: String,
}

impl ThemeMethod for TimePickerTheme {
    fn light() -> Self {
        Self {
            panel_background_color: "#fff".into(),
            panel_time_item_background_color_hover: "#f1f3f5".into(),
        }
    }

    fn dark() -> Self {
        Self {
            panel_background_color: "#48484e".into(),
            panel_time_item_background_color_hover: "#ffffff1a".into(),
        }
    }
}
