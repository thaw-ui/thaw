use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct AlertTheme {
    pub success_background_color: String,
    pub success_border_color: String,
    pub warning_background_color: String,
    pub warning_border_color: String,
    pub error_background_color: String,
    pub error_border_color: String,
}

impl ThemeMethod for AlertTheme {
    fn light() -> Self {
        Self {
            success_background_color: "#edf7f2".into(),
            success_border_color: "#c5e7d5".into(),
            warning_background_color: "#fef7ed".into(),
            warning_border_color: "#fae0b5".into(),
            error_background_color: "#fbeef1".into(),
            error_border_color: "#f3cbd3".into(),
        }
    }

    fn dark() -> Self {
        Self {
            success_background_color: "#2a947d40".into(),
            success_border_color: "#2a947d59".into(),
            warning_background_color: "#f08a0040".into(),
            warning_border_color: "#f08a0059".into(),
            error_background_color: "#d03a5240".into(),
            error_border_color: "#d03a5259".into(),
        }
    }
}
