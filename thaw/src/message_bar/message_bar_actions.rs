use super::{MessageBarInjection, MessageBarLayout};
use crate::ButtonSizeInjection;
use leptos::{context::Provider, either::Either, prelude::*};
use thaw_components::OptionComp;

#[component]
pub fn MessageBarActions(
    #[prop(optional)] message_bar_container_action: Option<MessageBarContainerAction>,
    children: Children,
) -> impl IntoView {
    let layout = MessageBarInjection::expect_context().layout;

    if layout == MessageBarLayout::Multiline {
        Either::Left(view! {
            <Provider value=ButtonSizeInjection(crate::ButtonSize::Small)>
                <OptionComp value=message_bar_container_action.map(|a| a.children) let:children>
                    <div class="thaw-message-bar-actions__container-action">{children()}</div>
                </OptionComp>
                <div class="thaw-message-bar-actions">{children()}</div>
            </Provider>
        })
    } else {
        Either::Right(view! {
            <Provider value=ButtonSizeInjection(crate::ButtonSize::Small)>
                <div class="thaw-message-bar-actions">{children()}</div>
                <OptionComp value=message_bar_container_action.map(|a| a.children) let:children>
                    <div class="thaw-message-bar-actions__container-action">{children()}</div>
                </OptionComp>
            </Provider>
        })
    }
}

#[slot]
pub struct MessageBarContainerAction {
    children: Children,
}
