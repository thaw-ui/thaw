use leptos::prelude::*;
use std::time::Duration;

#[component]
pub fn Toast(children: Children) -> impl IntoView {
    view! {
        <div class="thaw-toast">
            {children()}
        </div>
    }
}

#[derive(Default, Clone, Copy)]
pub enum ToastPosition {
    Top,
    TopStart,
    TopEnd,
    Bottom,
    BottomStart,
    #[default]
    BottomEnd,
}

impl ToastPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::TopStart => "top-left",
            Self::TopEnd => "top-right",
            Self::Bottom => "bottom",
            Self::BottomStart => "bottom-left",
            Self::BottomEnd => "bottom-right",
        }
    }
}

#[derive(Clone)]
pub struct ToastOptions {
    pub(crate) id: uuid::Uuid,
    pub(crate) position: Option<ToastPosition>,
    pub(crate) timeout: Option<Duration>,
}

impl Default for ToastOptions {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            position: None,
            timeout: None,
        }
    }
}
