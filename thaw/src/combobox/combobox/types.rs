use std::collections::HashMap;
use thaw_utils::{Model, VecModel, VecModelWithValue};
use leptos::prelude::*;
use super::ComboboxRuleTrigger;

#[derive(Clone, Copy)]
pub(crate) struct ComboboxInjection {
    pub(super) value: Model<String>,
    pub(super) selected_options: VecModel<String>,
    pub(super) options: StoredValue<HashMap<String, (String, String, Signal<bool>)>>,
    pub(super) is_show_listbox: RwSignal<bool>,
    pub(super) validate: Callback<Option<ComboboxRuleTrigger>, bool>,
    pub multiselect: bool,
}

impl ComboboxInjection {
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
        self.selected_options.with(|options| match options {
            VecModelWithValue::T(v) => v == value,
            VecModelWithValue::Option(v) => {
                if let Some(v) = v.as_ref() {
                    v == value
                } else {
                    false
                }
            }
            VecModelWithValue::Vec(v) => v.contains(value),
        })
    }

    pub fn select_option(&self, value: &String, text: &String) {
        self.selected_options.update(|options| match options {
            (None, None, Some(v)) => {
                if let Some(index) = v.iter().position(|v| v == value) {
                    v.remove(index);
                    return;
                }
                v.push(value.clone());
            }
            (None, Some(v), None) => {
                *v = Some(value.clone());
                self.value.set(text.clone());
                self.is_show_listbox.set(false);
            }
            (Some(v), None, None) => {
                *v = value.clone();
                self.value.set(text.clone());
                self.is_show_listbox.set(false);
            }
            _ => unreachable!(),
        });
        self.validate.run(Some(ComboboxRuleTrigger::Change));
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ComboboxSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ComboboxSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}