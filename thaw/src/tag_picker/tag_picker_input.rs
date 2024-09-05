use leptos::prelude::*;
use super::TagPickerInjection;
use thaw_utils::class_list;

#[component]
pub fn TagPickerInput(#[prop(optional, into)] class: MaybeProp<String>,) -> impl IntoView {
    let tag_picker = TagPickerInjection::expect_context();
    let on_blur = move |_| {
        // tag_picker.is_show_listbox.set(true);
    };

    view! {
        <input
            node_ref=tag_picker.input_ref
            type="text"
            role="combobox"
            class=class_list!["thaw-tag-picker-input", class]
            on:blur=on_blur
        />
    }
}
