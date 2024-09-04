use super::TagPickerInjection;
use leptos::prelude::*;
use thaw_components::{Fallback, If, OptionComp, Then};
use thaw_utils::class_list;

#[component]
pub fn TagPickerOption(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {

    view! {
        <div
            role="option"
            class=class_list![
                "thaw-tag-picker-option",
                class
            ]
        >
            
        </div>
    }
}
