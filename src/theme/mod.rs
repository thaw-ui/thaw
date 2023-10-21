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
        Self {
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
    fn dark() -> Self {
        Self {
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

pub fn use_theme(default: impl Fn() -> Theme) -> ReadSignal<Theme> {
    use_context::<ReadSignal<Theme>>().unwrap_or_else(|| create_signal(default()).0)
}

#[cfg(test)]
mod tests {
    use super::{use_theme, Theme};

    fn _t_use_theme() {
        use_theme(Theme::dark);
    }
}
