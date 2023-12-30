use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct AvatarTheme {
    pub background_color: String,
}

impl ThemeMethod for AvatarTheme {
    fn light() -> Self {
        Self {
            background_color: "#f7f7f7".into(),
        }
    }

    fn dark() -> Self {
        Self {
            background_color: "#424245".into(),
        }
    }
}
