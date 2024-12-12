use std::ops::Deref;
use leptos::prelude::*;
use crate::{FieldValidationState, Rule};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum InputRuleTrigger {
    #[default]
    Blur,
    Focus,
    Input,
    Change,
}

pub struct InputRule(Rule<String, InputRuleTrigger>);

impl InputRule {
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

    pub fn with_trigger(self, trigger: InputRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for InputRule {
    type Target = Rule<String, InputRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}