use leptos::prelude::*;
use thaw_components::OptionComp;

#[component]
pub fn MessageBarActions(
    #[prop(optional)] message_bar_container_action: Option<MessageBarContainerAction>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="thaw-message-bar-actions">{children()}</div>
        <OptionComp value=message_bar_container_action.map(|a| a.children) let:children>
            <div class="tha-message-bar-actions__container-action">
                {children()}
            </div>
        </OptionComp>
    }
}

#[slot]
pub struct MessageBarContainerAction {
    children: Children,
}
