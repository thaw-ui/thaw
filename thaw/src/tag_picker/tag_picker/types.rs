use crate::_aria::ActiveDescendantController;
use leptos::{html, prelude::*};
use std::collections::HashMap;
use thaw_utils::{BoxCallback, Model};

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
    pub disabled: Signal<bool>,
    pub size: Signal<TagPickerSize>,
    pub input_ref: NodeRef<html::Input>,
    pub(super) selected_options: Model<Vec<String>>,
    pub options: StoredValue<HashMap<String, (String, String, Signal<bool>)>>,
    pub(super) is_show_listbox: RwSignal<bool>,
    pub(super) listbox_hidden_callback: StoredValue<Vec<BoxCallback>>,
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

#[derive(Default, Clone)]
pub enum TagPickerSize {
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl TagPickerSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Medium => "medium",
            Self::Large => "large",
            Self::ExtraLarge => "extra-large",
        }
    }
}
