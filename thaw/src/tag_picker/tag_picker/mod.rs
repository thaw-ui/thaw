mod types;

pub use types::*;

use crate::{
    _aria::use_active_descendant,
    icon::ChevronDownRegularIcon,
    listbox::{listbox_keyboard_event, Listbox},
};
use leptos::{context::Provider, ev, html, prelude::*};
use std::collections::HashMap;
use thaw_components::{Follower, FollowerPlacement, FollowerWidth};
use thaw_utils::{call_on_click_outside_with_list, class_list, mount_style, Model};

#[component]
pub fn TagPicker(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Whether the tag picker is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// An array of selected option keys.
    #[prop(optional, into)]
    selected_options: Model<Vec<String>>,
    /// The size of the TagPicker.
    #[prop(optional, into)]
    size: Signal<TagPickerSize>,
    tag_picker_control: TagPickerControl,
    children: Children,
) -> impl IntoView {
    mount_style("tag-picker", include_str!("./tag-picker.css"));
    let TagPickerControl {
        children: control_children,
    } = tag_picker_control;
    let is_show_listbox = RwSignal::new(false);
    let trigger_ref = NodeRef::<html::Div>::new();
    let input_ref = NodeRef::<html::Input>::new();
    let listbox_ref = NodeRef::<html::Div>::new();
    let listbox_hidden_callback = StoredValue::new(vec![]);
    let options = StoredValue::new(HashMap::<String, (String, String, Signal<bool>)>::new());
    let (set_listbox, active_descendant_controller) =
        use_active_descendant(move |el| el.class_list().contains("thaw-tag-picker-option"));

    let tag_picker_control_injection =
        TagPickerControlInjection(active_descendant_controller.clone());
    let tag_picker_injection = TagPickerInjection {
        disabled,
        size,
        selected_options,
        input_ref,
        options,
        is_show_listbox,
        listbox_hidden_callback,
    };
    let on_click = move |e: ev::MouseEvent| {
        if disabled.get() {
            return;
        }
        if e.default_prevented() {
            if is_show_listbox.get() {
                is_show_listbox.set(false);
            }
            return;
        }
        let Some(el) = input_ref.get_untracked() else {
            return;
        };

        if document().active_element().as_ref() != Some(&**el) {
            let _ = el.focus();
        }
        is_show_listbox.update(|show| *show = !*show);
    };
    call_on_click_outside_with_list(
        vec![trigger_ref, listbox_ref],
        {
            move || {
                is_show_listbox.set(false);
            }
        }
        .into(),
    );

    let on_keydown = move |e| {
        listbox_keyboard_event(
            e,
            is_show_listbox,
            true,
            &active_descendant_controller,
            move |option| {
                tag_picker_injection.options.with_value(|options| {
                    if let Some((value, _text, disabled)) = options.get(&option.id()) {
                        if disabled.get_untracked() {
                            return;
                        }
                        tag_picker_injection.select_option(value);
                    }
                });
            },
        );
    };

    let on_after_leave = move || {
        if let Some(list) =
            listbox_hidden_callback.try_update_value(|list| list.drain(..).collect::<Vec<_>>())
        {
            list.into_iter().for_each(|f| f());
        }
    };

    view! {
        <crate::_binder::Binder on_css_transition_after_leave=on_after_leave>
            <div
                class=class_list![
                    "thaw-tag-picker-control",
                    ("thaw-tag-picker-control--disabled", move || disabled.get()),
                    move || format!("thaw-tag-picker-control--{}", size.get().as_str()),
                    class
                ]
                node_ref=trigger_ref
                on:keydown=on_keydown
                on:click=on_click
            >
                <Provider value=tag_picker_injection>
                    <Provider value=tag_picker_control_injection>{control_children()}</Provider>
                </Provider>
                <span class="thaw-tag-picker-control__aside">
                    <span class="thaw-tag-picker-control__expand-icon">
                        <ChevronDownRegularIcon />
                    </span>
                </span>
            </div>
            <Follower
                slot
                show=is_show_listbox
                placement=FollowerPlacement::BottomStart
                width=FollowerWidth::MinTarget
                auto_height=true
            >
                <Listbox set_listbox listbox_ref class="thaw-tag-picker__listbox">
                    <Provider value=tag_picker_injection>{children()}</Provider>
                </Listbox>
            </Follower>
        </crate::_binder::Binder>
    }
}
