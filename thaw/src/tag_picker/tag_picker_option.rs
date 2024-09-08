use super::TagPickerInjection;
use crate::listbox::ListboxInjection;
use leptos::{either::Either, ev, prelude::*};
use thaw_utils::class_list;

#[component]
pub fn TagPickerOption(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Sets an option to the disabled state.
    #[prop(optional, into)]
    disabled: MaybeSignal<bool>,
    /// Defines a unique identifier for the option.
    #[prop(into)]
    value: String,
    /// An optional override the string value of the Option's display text, defaulting to the Option's child content.
    #[prop(into)]
    text: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let tag_picker = TagPickerInjection::expect_context();
    let listbox = ListboxInjection::expect_context();
    let value = StoredValue::new(value);
    let text = StoredValue::new(text);
    let is_selected = Memo::new(move |_| value.with_value(|value| tag_picker.is_selected(&value)));
    let id = uuid::Uuid::new_v4().to_string();

    let on_click = move |e: ev::MouseEvent| {
        if disabled.get_untracked() {
            e.stop_propagation();
            return;
        }
        value.with_value(|value| {
            tag_picker.select_option(value);
        });
    };
    {
        tag_picker.insert_option(id.clone(), (value.get_value(), text.get_value(), disabled));
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
            aria-disabled=move || if disabled.get() { "true" } else { "" }
            aria-selected=move || is_selected.get().to_string()
            id=id
            class=class_list![
                "thaw-tag-picker-option",
                ("thaw-tag-picker-option--selected", move || is_selected.get()),
                ("thaw-tag-picker-option--disabled", move || disabled.get()),
                class
            ]
            on:click=on_click
        >
            {if let Some(children) = children {
                Either::Left(children())
            } else {
                Either::Right(text.get_value())
            }}
        </div>
    }
}
