use crate::{InputRef, InputSize};
use leptos::prelude::*;
use std::collections::HashMap;
use thaw_utils::{ArcOneCallback, ComponentRef, Model};

#[slot]
pub struct AutoCompletePrefix {
    children: Children,
}

#[slot]
pub struct AutoCompleteSuffix {
    children: Children,
}

#[derive(Clone)]
pub(crate) struct AutoCompleteInjection {
    pub(super) value: Model<String>,
    pub(super) select_option: ArcOneCallback<String>,
    pub(super) options: StoredValue<HashMap<String, String>>,
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
        self.options.update_value(|options| {
            options.insert(id, value);
        });
    }

    pub fn remove_option(&self, id: &String) {
        self.options.update_value(|options| {
            options.remove(id);
        });
    }
}

#[derive(Clone)]
pub struct AutoCompleteRef {
    pub(super) input_ref: ComponentRef<InputRef>,
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

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum AutoCompleteSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl AutoCompleteSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

impl From<AutoCompleteSize> for InputSize {
    fn from(value: AutoCompleteSize) -> Self {
        match value {
            AutoCompleteSize::Small => Self::Small,
            AutoCompleteSize::Medium => Self::Medium,
            AutoCompleteSize::Large => Self::Large,
        }
    }
}
