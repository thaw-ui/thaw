mod types;

pub use types::*;

use crate::{InfoRegularIcon, Popover, PopoverTrigger, PopoverTriggerType};
use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn InfoButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Size of the InfoButton.
    #[prop(optional, into)] size: Signal<InfoButtonSize>,
    children: Children,
) -> impl IntoView {
    view! {
        <Popover trigger_type=PopoverTriggerType::Click>
            <PopoverTrigger slot>
                <button class=class_list![
                    "thaw-info-button",
                    move || format!("thaw-info-button--{}", size.get().as_str()),
                    class
                ]>
                    <InfoRegularIcon />
                </button>
            </PopoverTrigger>
            {children()}
        </Popover>
    }
}
