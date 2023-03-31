mod common;
use self::common::CommonTheme;
use crate::ButtonTheme;

pub trait ThemeMethod {
    fn light() -> Self;
    fn dark() -> Self;
}

pub struct Theme {
    common: CommonTheme,
    button: ButtonTheme,
}

impl ThemeMethod for Theme {
    fn light() -> Self {
        Self {
            common: CommonTheme::light(),
            button: ButtonTheme::light(),
        }
    }
    fn dark() -> Self {
        Self {
            common: CommonTheme::dark(),
            button: ButtonTheme::dark(),
        }
    }
}
