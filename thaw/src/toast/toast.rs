use leptos::prelude::*;
use std::time::Duration;
use thaw_utils::class_list;

#[component]
pub fn Toast(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! { <div class=class_list!["thaw-toast", class]>{children()}</div> }
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
            Self::TopStart => "top-start",
            Self::TopEnd => "top-dnc",
            Self::Bottom => "bottom",
            Self::BottomStart => "bottom-start",
            Self::BottomEnd => "bottom-end",
        }
    }
}

#[derive(Default, Debug, Clone)]
pub enum ToastIntent {
    Success,
    #[default]
    Info,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct ToastOptions {
    pub(crate) id: uuid::Uuid,
    pub(crate) position: Option<ToastPosition>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) intent: Option<ToastIntent>,
}

impl Default for ToastOptions {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            position: None,
            timeout: None,
            intent: None,
        }
    }
}

impl ToastOptions {
    /// The position the toast should render.
    pub fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = Some(position);
        self
    }

    /// Auto dismiss timeout in milliseconds.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Intent.
    pub fn with_intent(mut self, intent: ToastIntent) -> Self {
        self.intent = Some(intent);
        self
    }
}
