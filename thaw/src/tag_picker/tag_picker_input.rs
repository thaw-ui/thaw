use super::{TagPickerControlInjection, TagPickerInjection};
use crate::combobox::utils::KeyboardKey;
use leptos::{ev, prelude::*};
use thaw_tabster::{use_focus_finders, FocusFinders};
use thaw_utils::class_list;

#[component]
pub fn TagPickerInput(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    let TagPickerInjection {
        input_ref, options, ..
    } = TagPickerInjection::expect_context();
    let TagPickerControlInjection {
        active_descendant_controller,
        tag_group_ref,
    } = TagPickerControlInjection::expect_context();
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
                options.with_value(|options| {
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

    let FocusFinders {
        mut find_last_focusable,
        ..
    } = use_focus_finders();

    let on_key_down = move |e: ev::KeyboardEvent| {
        let key = e.key();
        if KeyboardKey::ArrowLeft == key || KeyboardKey::Backspace == key {
            let el = tag_group_ref.get_untracked().unwrap();
            find_last_focusable((*el).clone());
        }
    };

    view! {
        <input
            node_ref=input_ref
            type="text"
            role="combobox"
            class=class_list!["thaw-tag-picker-input", class]
            on:blur=on_blur
            on:input=on_input
            on:keydown=on_key_down
            prop:value=move || {
                value_trigger.notify();
                ""
            }
        />
    }
}
