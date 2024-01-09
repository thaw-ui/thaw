use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct CollapseTheme {
    pub border_color: String,
}

impl ThemeMethod for CollapseTheme {
    fn light() -> Self {
        Self {
            border_color: "#efeff5".into(),
        }
    }

    fn dark() -> Self {
        Self {
            border_color: "#ffffff17".into(),
        }
    }
}
