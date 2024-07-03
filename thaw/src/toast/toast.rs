use leptos::*;

#[component]
pub fn Toast(children: Children) -> impl IntoView {
    view! {
        <div class="thaw-toast">
            {children()}
        </div>
    }
}

#[derive(Default, Clone)]
pub enum ToastPosition {
    Top,
    TopStart,
    #[default]
    TopEnd,
    Bottom,
    BottomStart,
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
    pub id: uuid::Uuid,
    pub postition: Option<ToastPosition>,
}

impl Default for ToastOptions {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            postition: None,
        }
    }
}
