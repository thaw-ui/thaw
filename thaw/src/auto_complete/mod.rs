mod auto_complete_option;

pub use auto_complete_option::AutoCompleteOption;

use crate::{
    combobox::listbox::{listbox_keyboard_event, Listbox},
    ComponentRef, Input, InputPrefix, InputRef, InputSuffix,
    _aria::use_active_descendant,
};
use leptos::{context::Provider, either::Either, html, prelude::*};
use std::collections::HashMap;
use thaw_components::{Binder, Follower, FollowerPlacement, FollowerWidth};
use thaw_utils::{class_list, mount_style, ArcOneCallback, BoxOneCallback, Model};

#[slot]
pub struct AutoCompletePrefix {
    children: Children,
}

#[slot]
pub struct AutoCompleteSuffix {
    children: Children,
}

#[component]
pub fn AutoComplete(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Input of autocomplete.
    #[prop(optional, into)]
    value: Model<String>,
    /// Autocomplete's placeholder.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    // Whether to clear after selection.
    #[prop(optional, into)] clear_after_select: MaybeSignal<bool>,
    /// Whether to blur after selection.
    #[prop(optional, into)]
    blur_after_select: MaybeSignal<bool>,
    // On select callback function.
    #[prop(optional, into)] on_select: Option<BoxOneCallback<String>>,
    /// Whether the input is disabled.
    #[prop(optional, into)]
    disabled: MaybeSignal<bool>,
    #[prop(optional)] auto_complete_prefix: Option<AutoCompletePrefix>,
    #[prop(optional)] auto_complete_suffix: Option<AutoCompleteSuffix>,
    #[prop(optional)] comp_ref: ComponentRef<AutoCompleteRef>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("auto-complete", include_str!("./auto-complete.css"));
    let input_ref = ComponentRef::<InputRef>::new();
    let listbox_ref = NodeRef::<html::Div>::new();
    let auto_complete_ref = NodeRef::<html::Div>::new();
    let open_listbox = RwSignal::new(false);
    let options = StoredValue::new(HashMap::<String, String>::new());

    let allow_value = move |_| {
        if !open_listbox.get_untracked() {
            open_listbox.set(true);
        }
        true
    };

    let select_option = ArcOneCallback::new(move |option_value: String| {
        if clear_after_select.get_untracked() {
            value.set(String::new());
        } else {
            value.set(option_value.clone());
        }
        if let Some(on_select) = on_select.as_ref() {
            on_select(option_value);
        }

        open_listbox.set(false);
        if blur_after_select.get_untracked() {
            if let Some(input_ref) = input_ref.get_untracked() {
                input_ref.blur();
            }
        }
    });

    let (set_listbox, active_descendant_controller) =
        use_active_descendant(move |el| el.class_list().contains("thaw-auto-complete-option"));
    let on_blur = {
        let active_descendant_controller = active_descendant_controller.clone();
        move |_| {
            active_descendant_controller.blur();
            open_listbox.set(false);
        }
    };
    let on_keydown = {
        let select_option = select_option.clone();
        move |e| {
            let select_option = select_option.clone();
            listbox_keyboard_event(
                e,
                open_listbox,
                false,
                &active_descendant_controller,
                move |option| {
                    options.with_value(|options| {
                        if let Some(value) = options.get(&option.id()) {
                            select_option(value.clone());
                        }
                    });
                },
            );
        }
    };

    comp_ref.load(AutoCompleteRef { input_ref });

    view! {
        <Binder target_ref=auto_complete_ref>
            <div
                class=class_list!["thaw-auto-complete", class]
                node_ref=auto_complete_ref
                on:keydown=on_keydown
            >
                <Input
                    value
                    placeholder
                    disabled
                    on_focus=move |_| open_listbox.set(true)
                    on_blur=on_blur
                    allow_value
                    comp_ref=input_ref
                >
                    <InputPrefix if_=auto_complete_prefix.is_some() slot>

                        {if let Some(auto_complete_prefix) = auto_complete_prefix {
                            Some((auto_complete_prefix.children)())
                        } else {
                            None
                        }}

                    </InputPrefix>
                    <InputSuffix if_=auto_complete_suffix.is_some() slot>

                        {if let Some(auto_complete_suffix) = auto_complete_suffix {
                            Some((auto_complete_suffix.children)())
                        } else {
                            None
                        }}

                    </InputSuffix>
                </Input>
            </div>
            <Follower
                slot
                show=open_listbox
                placement=FollowerPlacement::BottomStart
                width=FollowerWidth::Target
            >
                <Provider value=AutoCompleteInjection {
                    value,
                    select_option,
                    options,
                }>
                    <Listbox
                        open=open_listbox.read_only()
                        set_listbox
                        listbox_ref
                        class="thaw-auto-complete__listbox"
                    >
                        {if let Some(children) = children {
                            Either::Left(children())
                        } else {
                            Either::Right(())
                        }}
                    </Listbox>
                </Provider>
            </Follower>
        </Binder>
    }
}

#[derive(Clone)]
pub(crate) struct AutoCompleteInjection {
    value: Model<String>,
    select_option: ArcOneCallback<String>,
    options: StoredValue<HashMap<String, String>>,
}

impl AutoCompleteInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn is_selected(&self, key: &String) -> bool {
        self.value.with(|value| value == key)
    }

    pub fn select_option(&self, value: String) {
        (self.select_option)(value);
    }

    pub fn insert_option(&self, id: String, value: String) {
        self.options
            .update_value(|options| { options.insert(id, value); });
    }

    pub fn remove_option(&self, id: &String) {
        self.options.update_value(|options| { options.remove(id); });
    }
}

#[derive(Clone)]
pub struct AutoCompleteRef {
    input_ref: ComponentRef<InputRef>,
}

impl AutoCompleteRef {
    /// Focus the input element.
    pub fn focus(&self) {
        if let Some(input_ref) = self.input_ref.get_untracked() {
            input_ref.focus();
        }
    }

    /// Blur the input element.
    pub fn blur(&self) {
        if let Some(input_ref) = self.input_ref.get_untracked() {
            input_ref.blur();
        }
    }
}
