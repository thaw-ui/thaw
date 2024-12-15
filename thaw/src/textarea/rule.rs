use leptos::prelude::*;
use crate::{FieldValidationState, Rule};
use std::ops::Deref;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TextareaRuleTrigger {
    #[default]
    Blur,
    Focus,
    Input,
    Change,
}

pub struct TextareaRule(Rule<String, TextareaRuleTrigger>);

impl TextareaRule {
    pub fn required(required: Signal<bool>) -> Self {
        Self::validator(move |value, name| {
            if required.get_untracked() && value.is_empty() {
                let message = name.get_untracked().map_or_else(
                    || String::from("Please input!"),
                    |name| format!("Please input {name}!"),
                );
                Err(FieldValidationState::Error(message))
            } else {
                Ok(())
            }
        })
    }

    pub fn required_with_message(required: Signal<bool>, message: Signal<String>) -> Self {
        Self::validator(move |value, _| {
            if required.get_untracked() && value.is_empty() {
                Err(FieldValidationState::Error(message.get_untracked()))
            } else {
                Ok(())
            }
        })
    }

    pub fn validator(
        f: impl Fn(&String, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: TextareaRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for TextareaRule {
    type Target = Rule<String, TextareaRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
