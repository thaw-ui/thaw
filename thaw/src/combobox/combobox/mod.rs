mod rule;
mod types;

pub use rule::*;
pub use types::*;

use super::listbox::{listbox_keyboard_event, Listbox};
use crate::{
    ChevronDownRegularIcon, DismissRegularIcon, FieldInjection, Rule, _aria::use_active_descendant,
};
use leptos::{context::Provider, ev, html, prelude::*};
use std::collections::HashMap;
use thaw_components::{Follower, FollowerPlacement, FollowerWidth};
use thaw_utils::{add_event_listener, class_list, mount_style, Model, VecModel, VecModelWithValue};

#[component]
pub fn Combobox(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] rules: Vec<ComboboxRule>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    #[prop(optional, into)] value: Model<String>,
    /// Selected option.
    #[prop(optional, into)]
    selected_options: VecModel<String>,
    /// Whether the input is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Placeholder text for the input.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// If set, the combobox will show an icon to clear the current value.
    #[prop(optional)]
    clearable: bool,
    /// Controls the size of the combobox faceplate.
    #[prop(optional, into)]
    size: Signal<ComboboxSize>,
    children: Children,
) -> impl IntoView {
    mount_style("combobox", include_str!("./combobox.css"));
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let validate = Rule::validate(rules, selected_options, name);
    let input_ref = NodeRef::<html::Input>::new();
    let listbox_ref = NodeRef::<html::Div>::new();
    let is_show_listbox = RwSignal::new(false);
    let options = StoredValue::new(HashMap::<String, (String, String, Signal<bool>)>::new());

    let clear_icon_ref = NodeRef::<html::Span>::new();
    let is_show_clear_icon = Memo::new(move |_| {
        if clearable {
            selected_options.with(|options| match options {
                VecModelWithValue::T(v) => !v.is_empty(),
                VecModelWithValue::Option(v) => v.is_some(),
                VecModelWithValue::Vec(v) => !v.is_empty(),
            })
        } else {
            false
        }
    });
    if clearable {
        Effect::new(move |_| {
            let Some(clear_icon_el) = clear_icon_ref.get() else {
                return;
            };
            let handler = add_event_listener(clear_icon_el, ev::click, move |e| {
                if disabled.get_untracked() {
                    return;
                }
                e.stop_propagation();
                selected_options.set(vec![]);
                value.set(String::new());
                validate.run(Some(ComboboxRuleTrigger::Change));
            });
            on_cleanup(move || handler.remove());
        });
    }

    let multiselect = selected_options.is_vec();
    let combobox_injection = ComboboxInjection {
        value,
        multiselect,
        selected_options,
        options,
        is_show_listbox,
        validate,
    };
    let (set_listbox, active_descendant_controller) =
        use_active_descendant(move |el| el.class_list().contains("thaw-combobox-option"));

    let on_input = {
        let active_descendant_controller = active_descendant_controller.clone();
        move |ev| {
            let input_value = event_target_value(&ev);
            if selected_options.with_untracked(|options| match options {
                VecModelWithValue::T(v) => v != &input_value,
                VecModelWithValue::Option(v) => {
                    if let Some(v) = v.as_ref() {
                        v != &input_value
                    } else {
                        false
                    }
                }
                VecModelWithValue::Vec(_) => false,
            }) {
                selected_options.set(vec![]);
            }
            value.set(input_value);
            let Some(value) = value.with_untracked(|value| {
                let value = value.trim().to_ascii_lowercase();
                if value.is_empty() {
                    None
                } else {
                    Some(value)
                }
            }) else {
                active_descendant_controller.blur();
                return;
            };
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
        }
    };

    let on_blur = {
        let active_descendant_controller = active_descendant_controller.clone();
        move |_| {
            selected_options.with_untracked(|options| match options {
                VecModelWithValue::T(v) => {
                    if v.is_empty() {
                        value.set(String::new())
                    }
                }
                VecModelWithValue::Option(v) => {
                    if v.is_none() {
                        value.set(String::new())
                    }
                }
                VecModelWithValue::Vec(_) => value.set(String::new()),
            });
            active_descendant_controller.blur();
            validate.run(Some(ComboboxRuleTrigger::Blur));
            is_show_listbox.set(false);
        }
    };

    let on_keydown = move |e| {
        listbox_keyboard_event(
            e,
            is_show_listbox,
            multiselect,
            &active_descendant_controller,
            move |option| {
                combobox_injection.options.with_value(|options| {
                    if let Some((value, text, disabled)) = options.get(&option.id()) {
                        if disabled.get_untracked() {
                            return;
                        }
                        combobox_injection.select_option(value, text);
                    }
                });
            },
        );
    };

    view! {
        <crate::_binder::Binder>
            <div class=class_list![
                "thaw-combobox",
                ("thaw-combobox--disabled", move || disabled.get()),
                move || format!("thaw-combobox--{}", size.get().as_str()),
                class
            ]>
                <input
                    type="text"
                    aria-expanded=move || if is_show_listbox.get() { "true" } else { "false" }
                    role="combobox"
                    class="thaw-combobox__input"
                    id=id
                    name=name
                    prop:value=move || { value.get() }
                    placeholder=move || placeholder.get()
                    disabled=move || disabled.get()
                    node_ref=input_ref
                    on:input=on_input
                    on:blur=on_blur
                    on:keydown=on_keydown
                    on:click=move |_| {
                        is_show_listbox.update(|show| *show = !*show);
                    }
                />
                {if clearable {
                    view! {
                        <span
                            aria-hidden="true"
                            class="thaw-combobox__clear-icon"
                            style=move || {
                                (!is_show_clear_icon.get())
                                    .then(|| "display: none")
                                    .unwrap_or_default()
                            }
                            node_ref=clear_icon_ref
                            on:mousedown=|e| e.prevent_default()
                        >
                            <DismissRegularIcon />
                        </span>
                    }
                        .into()
                } else {
                    None
                }}
                <span
                    aria-disabled=move || if disabled.get() { "true" } else { "" }
                    aria-expanded=move || is_show_listbox.get().to_string()
                    role="button"
                    aria-label="Open"
                    class="thaw-combobox__expand-icon"
                    style=move || {
                        is_show_clear_icon.get().then(|| "display: none").unwrap_or_default()
                    }
                    on:mousedown=|e| e.prevent_default()
                    on:click=move |_| {
                        if disabled.get_untracked() {
                            return;
                        }
                        is_show_listbox.update(|show| *show = !*show);
                        if let Some(el) = input_ref.get_untracked() {
                            let _ = el.focus();
                        }
                    }
                >
                    <ChevronDownRegularIcon />
                </span>
            </div>
            <Follower
                slot
                show=is_show_listbox
                placement=FollowerPlacement::BottomStart
                width=FollowerWidth::MinTarget
                auto_height=true
            >
                <Provider value=combobox_injection>
                    <Listbox set_listbox listbox_ref class="thaw-combobox__listbox">
                        {children()}
                    </Listbox>
                </Provider>
            </Follower>
        </crate::_binder::Binder>
    }
}
