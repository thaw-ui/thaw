use crate::theme::ThemeMethod;

#[derive(Clone)]
pub struct InputTheme {

}

impl ThemeMethod for InputTheme {
    fn light() -> Self {
        Self {  }
    }

    fn dark() -> Self {
        Self {  }
    }
}