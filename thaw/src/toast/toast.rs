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
            Self::TopStart => "top-start",
            Self::TopEnd => "top-dnc",
            Self::Bottom => "bottom",
            Self::BottomStart => "bottom-start",
            Self::BottomEnd => "bottom-end",
        }
    }
}

#[derive(Debug, Clone)]
pub enum ToastIntent {
    Success,
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
    pub fn with_position(mut self, position: ToastPosition) -> Self {
        self.position = Some(position);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_intent(mut self, intent: ToastIntent) -> Self {
        self.intent = Some(intent);
        self
    }
}
