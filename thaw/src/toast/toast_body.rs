use leptos::*;
use thaw_components::OptionComp;

#[component]
pub fn ToastBody(
    #[prop(optional)] toast_body_subtitle: Option<ToastBodySubtitle>,
    children: Children,
) -> impl IntoView {
    view! {
        {children()}
        <OptionComp value=toast_body_subtitle let:subtitle>
            {(subtitle.children)()}
        </OptionComp>
    }
}

#[slot]
pub struct ToastBodySubtitle {
    children: Children,
}
