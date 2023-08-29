use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct ButtonTheme {}

impl ThemeMethod for ButtonTheme {
    fn light() -> Self {
        Self {}
    }

    fn dark() -> Self {
        Self {}
    }
}
