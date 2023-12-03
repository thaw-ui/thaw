use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct CalendarTheme {
    pub border_color: String,
}

impl ThemeMethod for CalendarTheme {
    fn light() -> Self {
        Self {
            border_color: "#efeff5".into(),
        }
    }

    fn dark() -> Self {
        Self {
            border_color: "#2d2d30".into(),
        }
    }
}
