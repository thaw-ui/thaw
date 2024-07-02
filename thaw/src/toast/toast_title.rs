use leptos::*;
use thaw_components::OptionComp;

#[component]
pub fn ToastTitle(
    #[prop(optional)] toast_title_media: Option<ToastTitleMedia>,
    children: Children,
    #[prop(optional)] toast_title_action: Option<ToastTitleAction>,
) -> impl IntoView {
    view! {
        <div class="thaw-toast-title__media">
            <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                <path d="M18 10a8 8 0 1 0-16 0 8 8 0 0 0 16 0ZM9.5 8.91a.5.5 0 0 1 1 0V13.6a.5.5 0 0 1-1 0V8.9Zm-.25-2.16a.75.75 0 1 1 1.5 0 .75.75 0 0 1-1.5 0Z" fill="currentColor"></path>
            </svg>
        </div>
        {children()}
        <OptionComp value=toast_title_action let:action>
            {(action.children)()}
        </OptionComp>
    }
}

#[slot]
pub struct ToastTitleMedia {
    children: Children,
}

#[slot]
pub struct ToastTitleAction {
    children: Children,
}
