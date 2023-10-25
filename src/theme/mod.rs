mod common;

use self::common::CommonTheme;
use crate::{AlertTheme, ButtonTheme, InputTheme, MenuTheme, SkeletionTheme, TableTheme, TagTheme};
use leptos::*;

pub trait ThemeMethod {
    fn light() -> Self;
    fn dark() -> Self;
}

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub common: CommonTheme,
    pub button: ButtonTheme,
    pub input: InputTheme,
    pub menu: MenuTheme,
    pub table: TableTheme,
    pub alert: AlertTheme,
    pub skeletion: SkeletionTheme,
    pub tag: TagTheme,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            name: "Light".into(),
            common: CommonTheme::light(),
            button: ButtonTheme::light(),
            input: InputTheme::light(),
            menu: MenuTheme::light(),
            table: TableTheme::light(),
            alert: AlertTheme::light(),
            skeletion: SkeletionTheme::light(),
            tag: TagTheme::light(),
        }
    }
    pub fn dark() -> Self {
        Self {
            name: "Dark".into(),
            common: CommonTheme::dark(),
            button: ButtonTheme::dark(),
            input: InputTheme::dark(),
            menu: MenuTheme::dark(),
            table: TableTheme::dark(),
            alert: AlertTheme::dark(),
            skeletion: SkeletionTheme::dark(),
            tag: TagTheme::dark(),
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
    provide_context(theme);
    children()
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
