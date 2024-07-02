mod color;
mod common;

use crate::ConfigInjection;
pub use color::ColorTheme;
pub use common::CommonTheme;
use leptos::*;

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub common: CommonTheme,
    pub color: ColorTheme,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            name: "light".into(),
            common: CommonTheme::new(),
            color: ColorTheme::light(),
        }
    }
    pub fn dark() -> Self {
        Self {
            name: "dark".into(),
            common: CommonTheme::new(),
            color: ColorTheme::dark(),
        }
    }

    pub fn use_theme(default: impl Fn() -> Theme) -> ReadSignal<Theme> {
        use_context::<ConfigInjection>()
            .map_or_else(|| RwSignal::new(default()), |c| c.theme)
            .split()
            .0
    }

    pub fn use_rw_theme() -> RwSignal<Theme> {
        expect_context::<ConfigInjection>().theme
    }
}

impl From<String> for Theme {
    fn from(value: String) -> Self {
        if value == "dark" {
            Theme::dark()
        } else {
            Theme::light()
        }
    }
}
