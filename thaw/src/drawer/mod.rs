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

use leptos::prelude::{Get, Signal};

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
    fn as_size_str(&self, position: Signal<DrawerPosition>) -> &'static str {
        match self {
            Self::Small => "320px",
            Self::Medium => "592px",
            Self::Large => "940px",
            Self::Full => match position.get() {
                DrawerPosition::Top | DrawerPosition::Bottom => "100vh",
                DrawerPosition::Left | DrawerPosition::Right => "100vw",
            }
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum DrawerModalType {
    /// When this type of dialog is open,
    /// the rest of the page is dimmed out and cannot be interacted with.
    #[default]
    Modal,
    /// When a non-modal dialog is open,
    /// the rest of the page is not dimmed out and users can interact with the rest of the page.
    NonModal,
}
