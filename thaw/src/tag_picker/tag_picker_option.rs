use crate::listbox::ListboxInjection;
use super::TagPickerInjection;
use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn TagPickerOption(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)]
    disabled: MaybeSignal<bool>,
    #[prop(into)]
    value: String,
    children: Children,
) -> impl IntoView {
    let tag_picker = TagPickerInjection::expect_context();
    let listbox = ListboxInjection::expect_context();
    let value = StoredValue::new(value);
    let id = uuid::Uuid::new_v4().to_string();

    let on_click = move |_| {
        if disabled.get_untracked() {
            return;
        }
        value.with_value(|value| {
            tag_picker.select_option(value);
        });
    };
    {
        tag_picker.insert_option(id.clone(), (value.get_value(), disabled));
        let id = id.clone();
        listbox.trigger();
        on_cleanup(move || {
            tag_picker.remove_option(&id);
            listbox.trigger();
        });
    }
    view! {
        <div
            role="option"
            id=id
            class=class_list!["thaw-tag-picker-option",
                class]
            on:click=on_click
        >
            {children()}
        </div>
    }
}
