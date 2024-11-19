use crate::{
    _aria::{use_active_descendant, ActiveDescendantController},
    icon::ChevronDownRegularIcon,
    listbox::{listbox_keyboard_event, Listbox},
};
use leptos::{context::Provider, ev, html, prelude::*};
use std::collections::HashMap;
use thaw_components::{Binder, Follower, FollowerPlacement, FollowerWidth};
use thaw_utils::{call_on_click_outside_with_list, class_list, mount_style, BoxCallback, Model};

#[component]
pub fn TagPicker(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// An array of selected option keys.
    #[prop(optional, into)]
    selected_options: Model<Vec<String>>,
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
        selected_options,
        input_ref,
        options,
        is_show_listbox,
        listbox_hidden_callback,
    };
    let on_click = move |e: ev::MouseEvent| {
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
    view! {
        <Binder target_ref=trigger_ref>
            <div
                class=class_list!["thaw-tag-picker-control", class]
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
            >
                <Provider value=tag_picker_injection>
                    <Listbox
                        open=is_show_listbox.read_only()
                        set_listbox
                        listbox_ref
                        on_hidden=listbox_hidden_callback
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

#[derive(Clone)]
pub(crate) struct TagPickerControlInjection(pub ActiveDescendantController);

impl TagPickerControlInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

#[derive(Clone, Copy)]
pub(crate) struct TagPickerInjection {
    pub input_ref: NodeRef<html::Input>,
    selected_options: Model<Vec<String>>,
    pub options: StoredValue<HashMap<String, (String, String, Signal<bool>)>>,
    is_show_listbox: RwSignal<bool>,
    listbox_hidden_callback: StoredValue<Vec<BoxCallback>>,
}

impl TagPickerInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    /// value: (value, text, disabled)
    pub fn insert_option(&self, id: String, value: (String, String, Signal<bool>)) {
        self.options.update_value(|options| {
            options.insert(id, value);
        });
    }

    pub fn remove_option(&self, id: &String) {
        self.options.update_value(|options| {
            options.remove(id);
        });
    }

    pub fn is_selected(&self, value: &String) -> bool {
        self.selected_options
            .with(|options| options.contains(value))
    }

    pub fn select_option(&self, value: &String) {
        self.selected_options.update(|options| {
            if let Some(index) = options.iter().position(|v| v == value) {
                options.remove(index);
            } else {
                options.push(value.clone());
                if let Some(input_el) = self.input_ref.get_untracked() {
                    input_el.set_value("");
                }
            }
        });
        self.is_show_listbox.set(false);
    }

    pub fn remove_selected_option(&self, value: String) {
        if self.is_show_listbox.get_untracked() {
            let selected_options = self.selected_options;
            self.listbox_hidden_callback.update_value(|list| {
                list.push(BoxCallback::new(move || {
                    selected_options.try_update(|options| {
                        if let Some(index) = options.iter().position(|v| v == &value) {
                            options.remove(index);
                        }
                    });
                }));
            });
        } else {
            self.selected_options.update(|options| {
                if let Some(index) = options.iter().position(|v| v == &value) {
                    options.remove(index);
                }
            });
        }
    }
}
