use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct CalendarTheme {
    pub border_color: String,
    pub other_month_font_color: String,
    pub background_color_hover: String,
}

impl ThemeMethod for CalendarTheme {
    fn light() -> Self {
        Self {
            border_color: "#efeff5".into(),
            other_month_font_color: "#c2c2c2".into(),
            background_color_hover: "#f3f3f5".into(),
        }
    }

    fn dark() -> Self {
        Self {
            border_color: "#2d2d30".into(),
            other_month_font_color: "#ffffff61".into(),
            background_color_hover: "#2d2d30".into(),
        }
    }
}
