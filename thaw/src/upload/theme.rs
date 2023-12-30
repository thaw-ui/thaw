use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct UploadTheme {
    pub dragger_background_color: String,
    pub dragger_border_color: String,
}

impl ThemeMethod for UploadTheme {
    fn light() -> Self {
        Self {
            dragger_background_color: "#fafafc".into(),
            dragger_border_color: "#e0e0e6".into(),
        }
    }

    fn dark() -> Self {
        Self {
            dragger_background_color: "#ffffff0f".into(),
            dragger_border_color: "#ffffff3d".into(),
        }
    }
}
