use leptos::{html, prelude::*};

#[slot]
pub struct InputPrefix {
    #[prop(default = true)]
    if_: bool,
    children: Children,
}

#[slot]
pub struct InputSuffix {
    #[prop(default = true)]
    if_: bool,
    children: Children,
}

#[derive(Default, Clone)]
pub enum InputType {
    #[default]
    Text,
    Password,
    Search,
    Tel,
    Url,
    Email,
    Time,
    Date,
    DatetimeLocal,
    Month,
    Week,
    Number,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Email => "email",
            Self::Time => "time",
            Self::Date => "date",
            Self::DatetimeLocal => "datetime-local",
            Self::Month => "month",
            Self::Week => "week",
            Self::Number => "number",
        }
    }
}

#[derive(Clone)]
pub struct InputRef {
    pub(super) input_ref: NodeRef<html::Input>,
}

impl InputRef {
    /// Focus the input element.
    pub fn focus(&self) {
        if let Some(input_el) = self.input_ref.get_untracked() {
            _ = input_el.focus();
        }
    }

    /// Blur the input element.
    pub fn blur(&self) {
        if let Some(input_el) = self.input_ref.get_untracked() {
            _ = input_el.blur();
        }
    }

    /// Select the input element.
    pub fn select(&self) {
        if let Some(input_el) = self.input_ref.get_untracked() {
            _ = input_el.select();
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum InputSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl InputSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
