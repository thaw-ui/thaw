mod color;
mod common;

use self::common::CommonTheme;
use crate::ProgressTheme;
pub use color::ColorTheme;
use leptos::*;

pub trait ThemeMethod {
    fn light() -> Self;
    fn dark() -> Self;
}

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub common: CommonTheme,
    pub color: ColorTheme,
    pub progress: ProgressTheme,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            name: "light".into(),
            common: CommonTheme::light(),
            color: ColorTheme::light(),
            progress: ProgressTheme::light(),
        }
    }
    pub fn dark() -> Self {
        Self {
            name: "dark".into(),
            common: CommonTheme::dark(),
            color: ColorTheme::dark(),
            progress: ProgressTheme::dark(),
        }
    }
}

impl ThemeMethod for Theme {
    fn light() -> Self {
        Theme::light()
    }
    fn dark() -> Self {
        Theme::dark()
    }
}

#[component]
pub fn ThemeProvider(
    #[prop(optional, into)] theme: Option<RwSignal<Theme>>,
    children: Children,
) -> impl IntoView {
    let theme = if let Some(theme) = theme {
        theme
    } else {
        create_rw_signal(Theme::light())
    };

    view! { <Provider value=theme children/> }
}

pub fn use_theme(default: impl Fn() -> Theme) -> ReadSignal<Theme> {
    use_context::<RwSignal<Theme>>()
        .unwrap_or_else(|| create_rw_signal(default()))
        .split()
        .0
}

pub fn use_rw_theme() -> RwSignal<Theme> {
    expect_context::<RwSignal<Theme>>()
}

#[cfg(test)]
mod tests {
    use super::{use_theme, Theme};

    fn _t_use_theme() {
        use_theme(Theme::dark);
    }
}
