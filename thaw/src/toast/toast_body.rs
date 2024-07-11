use leptos::prelude::*;
use thaw_components::OptionComp;

#[component]
pub fn ToastBody(
    #[prop(optional)] toast_body_subtitle: Option<ToastBodySubtitle>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="thaw-toast-body">
            {children()}
        </div>
        <OptionComp value=toast_body_subtitle let:subtitle>
            <div class="thaw-toast-body__subtitle">
                {(subtitle.children)()}
            </div>
        </OptionComp>
    }
}

#[slot]
pub struct ToastBodySubtitle {
    children: Children,
}
