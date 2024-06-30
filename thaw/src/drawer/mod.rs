mod drawer_body;
mod drawer_header;
mod drawer_header_title;
mod inline_drawer;
mod overlay_drawer;

pub use drawer_body::*;
pub use drawer_header::*;
pub use drawer_header_title::*;
pub use inline_drawer::*;
pub use overlay_drawer::*;

use leptos::{MaybeSignal, SignalWith};

#[derive(Clone, Default, Copy)]
pub enum DrawerPosition {
    Top,
    Bottom,
    #[default]
    Left,
    Right,
}

impl DrawerPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

#[derive(Clone, Default, Copy)]
pub enum DrawerSize {
    #[default]
    Small,
    Medium,
    Large,
    Full,
}

impl DrawerSize {
    fn as_size_str(&self, position: MaybeSignal<DrawerPosition>) -> &'static str {
        match self {
            Self::Small => "320px",
            Self::Medium => "592px",
            Self::Large => "940px",
            Self::Full => position.with(|p| match p {
                DrawerPosition::Top | DrawerPosition::Bottom => "100vh",
                DrawerPosition::Left | DrawerPosition::Right => "100vw",
            }),
        }
    }
}