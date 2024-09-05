use crate::{
    _aria::use_active_descendant,
    icon::ChevronDownRegularIcon,
    listbox::{listbox_keyboard_event, Listbox},
};
use leptos::{context::Provider, html, prelude::*};
use std::collections::HashMap;
use thaw_components::{Binder, Follower, FollowerPlacement, FollowerWidth};
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn TagPicker(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] selected_options: Model<Vec<String>>,
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
    let options = StoredValue::new(HashMap::<String, (String, MaybeSignal<bool>)>::new());
    let (set_listbox, active_descendant_controller) =
        use_active_descendant(move |el| el.class_list().contains("thaw-tag-picker-option"));

    let tag_picker_injection = TagPickerInjection {
        selected_options,
        input_ref,
        options,
    };
    let on_click = move |_| {
        is_show_listbox.update(|show| *show = !*show);
        if let Some(el) = input_ref.get_untracked() {
            let _ = el.focus();
        }
    };
    let on_focusout = move |_| {
        is_show_listbox.set(false);
    };
    let on_keydown = move |e| {
        listbox_keyboard_event(
            e,
            is_show_listbox,
            true,
            &active_descendant_controller,
            move |option| {
                tag_picker_injection.options.with_value(|options| {
                    if let Some((value, disabled)) = options.get(&option.id()) {
                        if disabled.get_untracked() {
                            return;
                        }
                        tag_picker_injection.select_option(value);
                    }
                });
            },
        );
    };
    view! {
        <Binder target_ref=trigger_ref>
            <div
                class=class_list!["thaw-tag-picker-control", class]
                node_ref=trigger_ref
                on:keydown=on_keydown
                on:click=on_click
                on:focusout=on_focusout
            >
                <Provider value=tag_picker_injection>{control_children()}</Provider>
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
            >
                <Provider value=tag_picker_injection>
                    <Listbox
                        open=is_show_listbox.read_only()
                        set_listbox
                        listbox_ref
                        class="thaw-tag-picker__listbox"
                    >
                        {children()}
                    </Listbox>
                </Provider>
            </Follower>
        </Binder>
    }
}

#[slot]
pub struct TagPickerControl {
    children: Children,
}

#[derive(Clone, Copy)]
pub(crate) struct TagPickerInjection {
    pub input_ref: NodeRef<html::Input>,
    selected_options: Model<Vec<String>>,
    options: StoredValue<HashMap<String, (String, MaybeSignal<bool>)>>,
}

impl TagPickerInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    /// value: (value, disabled)
    pub fn insert_option(&self, id: String, value: (String, MaybeSignal<bool>)) {
        self.options
            .update_value(|options| options.insert(id, value));
    }

    pub fn remove_option(&self, id: &String) {
        self.options.update_value(|options| options.remove(id));
    }

    pub fn select_option(&self, value: &String) {
        self.selected_options.update(|options| {
            if let Some(index) = options.iter().position(|v| v == value) {
                options.remove(index);
            } else {
                options.push(value.clone());
            }
        });
    }
}
