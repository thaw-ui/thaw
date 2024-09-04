use leptos::prelude::*;
use super::TagPickerInjection;

#[component]
pub fn TagPickerInput() -> impl IntoView {
    let tag_picker = TagPickerInjection::expect_context();
    let on_blur = move |_| {
        tag_picker.is_show_listbox.set(true);
    };

    view! {
        <input
            type="text"
            role="combobox"
            class="thaw-tag-picker-input"
            on:blur=on_blur
        />
    }
}
