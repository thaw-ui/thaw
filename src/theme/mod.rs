mod common;
use leptos::*;

use self::common::CommonTheme;
use crate::{ButtonTheme, InputTheme};

pub trait ThemeMethod {
    fn light() -> Self;
    fn dark() -> Self;
}

#[derive(Clone)]
pub struct Theme {
    pub common: CommonTheme,
    pub button: ButtonTheme,
    pub input: InputTheme,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            common: CommonTheme::light(),
            button: ButtonTheme::light(),
            input: InputTheme::light(),
        }
    }
    pub fn dark() -> Self {
        Self {
            common: CommonTheme::dark(),
            button: ButtonTheme::dark(),
            input: InputTheme::dark(),
        }
    }
}

impl ThemeMethod for Theme {
    fn light() -> Self {
        Self {
            common: CommonTheme::light(),
            button: ButtonTheme::light(),
            input: InputTheme::dark(),
        }
    }
    fn dark() -> Self {
        Self {
            common: CommonTheme::dark(),
            button: ButtonTheme::dark(),
            input: InputTheme::dark(),
        }
    }
}

pub fn use_theme(cx: Scope, default: impl Fn() -> Theme) -> ReadSignal<Theme> {
    use_context::<ReadSignal<Theme>>(cx).unwrap_or_else(|| create_signal(cx, default()).0)
}

#[cfg(test)]
mod tests {
    use super::{use_theme, Theme};
    use leptos::*;

    fn _t_use_theme(cx: Scope) {
        use_theme(cx, Theme::dark);
    }
}
