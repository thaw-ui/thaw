use leptos::prelude::*;

#[component]
pub fn MessageBarActions(
    message_bar_container_action: MessageBarContainerAction,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="thaw-message-bar-actions">
            {children()}
        </div>
        <div class="tha-message-bar-actions__container-action">
            {(message_bar_container_action.children)()}
        </div>
    }
}

#[slot]
pub struct MessageBarContainerAction {
    children: Children,
}
