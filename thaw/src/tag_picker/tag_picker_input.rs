use super::{TagPickerControlInjection, TagPickerInjection};
use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn TagPickerInput(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let tag_picker = TagPickerInjection::expect_context();
    let TagPickerControlInjection(active_descendant_controller) =
        TagPickerControlInjection::expect_context();
    let value_trigger = ArcTrigger::new();
    let on_blur = {
        let value_trigger = value_trigger.clone();
        move |_| {
            value_trigger.track();
        }
    };
    let on_input = move |ev| {
        let value = event_target_value(&ev);
        let value = value.trim().to_ascii_lowercase();
        if value.is_empty() {
            active_descendant_controller.blur();
            return;
        }
        if active_descendant_controller
            .find(|id| {
                tag_picker.options.with_value(|options| {
                    let Some((_, text, _)) = options.get(&id) else {
                        return false;
                    };
                    text.to_ascii_lowercase().contains(&value)
                })
            })
            .is_none()
        {
            active_descendant_controller.blur();
        }
    };

    view! {
        <input
            node_ref=tag_picker.input_ref
            type="text"
            role="combobox"
            class=class_list!["thaw-tag-picker-input", class]
            on:blur=on_blur
            on:input=on_input
            prop:value=move || {
                value_trigger.trigger();
                ""
            }
        />
    }
}
