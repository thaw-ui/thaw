use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct TimePickerTheme {
    pub panel_background_color: String,
    pub panel_time_item_background_color_hover: String,
    pub panel_border_color: String,
}

impl ThemeMethod for TimePickerTheme {
    fn light() -> Self {
        Self {
            panel_background_color: "#fff".into(),
            panel_time_item_background_color_hover: "#f1f3f5".into(),
            panel_border_color: "#e0e0e6".into(),
        }
    }

    fn dark() -> Self {
        Self {
            panel_background_color: "#48484e".into(),
            panel_time_item_background_color_hover: "#ffffff1a".into(),
            panel_border_color: "#ffffff3d".into(),
        }
    }
}
